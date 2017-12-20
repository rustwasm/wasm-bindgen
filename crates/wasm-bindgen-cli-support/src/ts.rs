use std::collections::{HashSet, HashMap};

use shared;
use parity_wasm::elements::*;

#[derive(Default)]
pub struct Js {
    globals: String,
    exposed_globals: HashSet<&'static str>,
    exports: Vec<(String, String, String)>,
    wasm_exports_bound: HashSet<String>,
    classes: Vec<String>,
    pub nodejs: bool,
    pub debug: bool,
}

impl Js {

    pub fn generate_program(&mut self,
                            program: &shared::Program,
                            _wasm: &Module) {
        for f in program.free_functions.iter() {
            self.generate_free_function(f);
        }
        for s in program.structs.iter() {
            self.generate_struct(s);
        }
    }

    pub fn generate_free_function(&mut self, func: &shared::Function) {
        let (js, ts) = self.generate_function("function",
                                              &func.name,
                                              &func.name,
                                              false,
                                              &func.arguments,
                                              func.ret.as_ref());

        self.exports.push((func.name.clone(), js, ts));
    }

    pub fn generate_struct(&mut self, s: &shared::Struct) {
        let mut dst = String::new();
        self.expose_check_token();
        self.expose_wasm_exports();
        dst.push_str(&format!("
            export class {} {{
                constructor(public __wasmPtr: number, sym: Symbol) {{
                    _checkToken(sym);
                }}

                free(): void {{
                    const ptr = this.__wasmPtr;
                    this.__wasmPtr = 0;
                    wasm_exports.{}(ptr);
                }}
        ", s.name, s.free_function()));
        self.wasm_exports_bound.insert(s.name.clone());

        for function in s.functions.iter() {
            let (js, _ts) = self.generate_function(
                "static",
                &function.name,
                &function.struct_function_export_name(&s.name),
                false,
                &function.arguments,
                function.ret.as_ref(),
            );
            dst.push_str(&js);
            dst.push_str("\n");
        }
        for method in s.methods.iter() {
            let (js, _ts) = self.generate_function(
                "",
                &method.function.name,
                &method.function.struct_function_export_name(&s.name),
                true,
                &method.function.arguments,
                method.function.ret.as_ref(),
            );
            dst.push_str(&js);
            dst.push_str("\n");
        }
        dst.push_str("}\n");
        self.classes.push(dst);

        let ts_export = format!("{0}: typeof {0};", s.name);
        self.exports.push((s.name.clone(), s.name.clone(), ts_export));
    }

    fn generate_function(&mut self,
                         prefix: &str,
                         name: &str,
                         wasm_name: &str,
                         is_method: bool,
                         arguments: &[shared::Type],
                         ret: Option<&shared::Type>) -> (String, String) {
        let mut dst = format!("{}(", name);
        let mut passed_args = String::new();
        let mut arg_conversions = String::new();
        let mut destructors = String::new();

        if is_method {
            passed_args.push_str("this.__wasmPtr");
        }

        for (i, arg) in arguments.iter().enumerate() {
            let name = format!("arg{}", i);
            if i > 0 {
                dst.push_str(", ");
            }
            dst.push_str(&name);
            dst.push_str(": ");

            let mut pass = |arg: &str| {
                if passed_args.len() > 0 {
                    passed_args.push_str(", ");
                }
                passed_args.push_str(arg);
            };
            match *arg {
                shared::Type::Number => {
                    dst.push_str("number");
                    self.expose_assert_num();
                    arg_conversions.push_str(&format!("_assertNum({});\n", name));
                    pass(&name)
                }
                shared::Type::Boolean => {
                    dst.push_str("boolean");
                    self.expose_assert_bool();
                    arg_conversions.push_str(&format!("\
                        const bool{i} = _assertBoolean({name});
                    ", name = name, i = i));
                    pass(&format!("bool{i}", i = i))
                }
                shared::Type::BorrowedStr |
                shared::Type::String => {
                    dst.push_str("string");
                    self.expose_pass_string_to_wasm();
                    arg_conversions.push_str(&format!("\
                        const [ptr{i}, len{i}] = passStringToWasm({arg});
                    ", i = i, arg = name));
                    pass(&format!("ptr{}", i));
                    pass(&format!("len{}", i));
                    if let shared::Type::BorrowedStr = *arg {
                        self.expose_wasm_exports();
                        destructors.push_str(&format!("\n\
                            wasm_exports.__wbindgen_free(ptr{i}, len{i});\n\
                        ", i = i));
                    }
                }
                shared::Type::ByRef(ref s) |
                shared::Type::ByMutRef(ref s) => {
                    dst.push_str(s);
                    self.expose_assert_class();
                    arg_conversions.push_str(&format!("\
                        const ptr{i} = _assertClass({arg}, {struct_});
                    ", i = i, arg = name, struct_ = s));
                    pass(&format!("ptr{}", i));
                }
                shared::Type::ByValue(ref s) => {
                    dst.push_str(s);
                    self.expose_assert_class();
                    arg_conversions.push_str(&format!("\
                        const ptr{i} = _assertClass({arg}, {struct_});
                        {arg}.__wasmPtr = 0;
                    ", i = i, arg = name, struct_ = s));
                    pass(&format!("ptr{}", i));
                }
                shared::Type::JsObject => {
                    dst.push_str("any");
                    self.expose_add_heap_object();
                    arg_conversions.push_str(&format!("\
                        const idx{i} = addHeapObject({arg});
                    ", i = i, arg = name));
                    pass(&format!("idx{}", i));
                }
                shared::Type::JsObjectRef => {
                    dst.push_str("any");
                    self.expose_borrowed_objects();
                    arg_conversions.push_str(&format!("\
                        const idx{i} = addBorrowedObject({arg});
                    ", i = i, arg = name));
                    destructors.push_str("popBorrowedObject();\n");
                    pass(&format!("idx{}", i));
                }
            }
        }
        dst.push_str("): ");
        let convert_ret = match ret {
            None => {
                dst.push_str("void");
                format!("return ret;")
            }
            Some(&shared::Type::Number) => {
                dst.push_str("number");
                format!("return ret;")
            }
            Some(&shared::Type::Boolean) => {
                dst.push_str("boolean");
                format!("return ret != 0;")
            }
            Some(&shared::Type::JsObject) => {
                dst.push_str("any");
                self.expose_take_object();
                format!("return takeObject(ret);")
            }
            Some(&shared::Type::JsObjectRef) |
            Some(&shared::Type::BorrowedStr) |
            Some(&shared::Type::ByMutRef(_)) |
            Some(&shared::Type::ByRef(_)) => panic!(),
            Some(&shared::Type::ByValue(ref name)) => {
                dst.push_str(name);
                format!("\
                    return new {name}(ret, token);
                ", name = name)
            }
            Some(&shared::Type::String) => {
                dst.push_str("string");
                self.expose_get_string_from_wasm();
                self.expose_wasm_exports();
                format!("
                    const ptr = wasm_exports.__wbindgen_boxed_str_ptr(ret);
                    const len = wasm_exports.__wbindgen_boxed_str_len(ret);
                    const realRet = getStringFromWasm(ptr, len);
                    wasm_exports.__wbindgen_boxed_str_free(ret);
                    return realRet;
                ")
            }
        };
        let mut dst_ts = dst.clone();
        dst_ts.push_str(";");
        dst.push_str(" {\n        ");
        dst.push_str(&arg_conversions);
        self.expose_wasm_exports();
        if destructors.len() == 0 {
            dst.push_str(&format!("\
                const ret = wasm_exports.{f}({passed});
                {convert_ret}
            ", f = wasm_name, passed = passed_args, convert_ret = convert_ret));
        } else {
            dst.push_str(&format!("\
                try {{
                    const ret = wasm_exports.{f}({passed});
                    {convert_ret}
                }} finally {{
                    {destructors}
                }}
            ", f = wasm_name, passed = passed_args, destructors = destructors,
                convert_ret = convert_ret));
        }
        dst.push_str("}");
        self.wasm_exports_bound.insert(wasm_name.to_string());
        (format!("{} {}", prefix, dst), dst_ts)
    }

    pub fn generate_import(&mut self, import: &shared::Function)
        -> (String, String)
    {
        let mut dst = String::new();
        let mut ts_dst = String::new();

        dst.push_str(&format!("function {0}_shim(", import.name));

        ts_dst.push_str(&import.name);
        ts_dst.push_str("(");

        let mut invocation = String::new();
        for (i, arg) in import.arguments.iter().enumerate() {
            if invocation.len() > 0 {
                invocation.push_str(", ");
            }
            if i > 0 {
                dst.push_str(", ");
                ts_dst.push_str(", ");
            }
            ts_dst.push_str(&format!("arg{}: ", i));
            match *arg {
                shared::Type::Number => {
                    ts_dst.push_str("number");
                    invocation.push_str(&format!("arg{}", i));
                    dst.push_str(&format!("arg{}: number", i));
                }
                shared::Type::Boolean => {
                    ts_dst.push_str("boolean");
                    invocation.push_str(&format!("arg{} != 0", i));
                    dst.push_str(&format!("arg{}: number", i));
                }
                shared::Type::BorrowedStr => {
                    ts_dst.push_str("string");
                    self.expose_get_string_from_wasm();
                    invocation.push_str(&format!("getStringFromWasm(ptr{0}, len{0})", i));
                    dst.push_str(&format!("ptr{0}: number, len{0}: number", i));
                }
                shared::Type::JsObject => {
                    ts_dst.push_str("any");
                    self.expose_take_object();
                    invocation.push_str(&format!("takeObject(arg{})", i));
                    dst.push_str(&format!("arg{}: number", i));
                }
                shared::Type::JsObjectRef => {
                    ts_dst.push_str("any");
                    self.expose_get_object();
                    invocation.push_str(&format!("getObject(arg{})", i));
                    dst.push_str(&format!("arg{}: number", i));
                }
                shared::Type::String |
                shared::Type::ByRef(_) |
                shared::Type::ByMutRef(_) |
                shared::Type::ByValue(_) => {
                    panic!("unsupported type in import");
                }
            }
        }
        ts_dst.push_str("): ");
        dst.push_str("): ");
        let mut convert = None;
        match import.ret {
            Some(shared::Type::Number) => {
                ts_dst.push_str("number");
                dst.push_str("number");
            }
            Some(shared::Type::Boolean) => {
                ts_dst.push_str("boolean");
                dst.push_str("number");
                convert = Some("_assertBoolean");
                self.expose_assert_bool();
            }
            Some(shared::Type::JsObject) => {
                ts_dst.push_str("any");
                dst.push_str("number");
                self.expose_add_heap_object();
                convert = Some("addHeapObject");
            }
            None => {
                ts_dst.push_str("void");
                dst.push_str("void");
            }
            _ => unimplemented!(),
        }
        ts_dst.push_str("\n");
        dst.push_str(" {\n");
        let invoc = format!("_imports.{}({})", import.name, invocation);
        let invoc = match convert {
            Some(s) => format!("{}({})", s, invoc),
            None => invoc,
        };
        dst.push_str(&format!("return {};\n}}", invoc));

        (dst, ts_dst)
    }

    pub fn to_string(&mut self, m: &Module, program: &shared::Program) -> String {
        if self.debug {
            self.expose_global_slab();
            self.expose_global_stack();
            self.exports.push(
                (
                    "assertHeapAndStackEmpty".to_string(),
                    "function(): void {
                        if (stack.length > 0)
                            throw new Error('stack is not empty');
                        for (let i = 0; i < slab.length; i++) {
                            if (typeof(slab[i]) !== 'number')
                                throw new Error('slab is not empty');
                        }
                    }".to_string(),
                    "assertHeapAndStackEmpty(): void;\n".to_string(),
                )
            );
        }

        for class in self.classes.iter() {
            self.globals.push_str(class);
            self.globals.push_str("\n");
        }
        let wasm_exports = self.typescript_wasm_exports(m);
        let mut exports_interface = String::new();
        let mut extra_exports_interface = String::new();
        let mut exports = format!("\
            {{
                module,
                instance,
        ");
        for &(ref name, ref body, ref ts_export) in self.exports.iter() {
            exports.push_str(name);
            exports.push_str(": ");
            exports.push_str(body);
            exports.push_str(",\n");
            exports_interface.push_str(ts_export);
            exports_interface.push_str("\n");
        }
        // If the user otherwise specified functions to export which *weren't*
        // part of wasm-bindgen we want to make sure they come through here as
        // well.
        for (export, typescript) in wasm_exports.iter() {
            // ignore any internal functions we have for ourselves
            if export.starts_with("__wbindgen") {
                continue
            }
            // Ignore anything we just bound above,
            if self.wasm_exports_bound.contains(export) {
                continue
            }

            if extra_exports_interface.len() == 0 {
                extra_exports_interface.push_str("export interface ExtraExports {\n");
                exports_interface.push_str("extra: ExtraExports;\n");
                exports.push_str("extra: {\n");
            }
            exports.push_str(export);
            exports.push_str(":");
            exports.push_str("exports.");
            exports.push_str(export);
            exports.push_str(",\n");
            extra_exports_interface.push_str(typescript);
            extra_exports_interface.push_str("\n");
        }
        if extra_exports_interface.len() > 0 {
            extra_exports_interface.push_str("}\n");
            exports.push_str("},\n");
        }
        exports.push_str("}");
        let wasm_imports = self.typescript_wasm_imports(m);

        let mut imports_object = String::new();
        let mut extra_imports_interface = String::new();
        let mut imports_bound = HashSet::new();
        let mut imports_interface = String::new();
        for import in program.imports.iter() {
            // Only actually generate this import if it ended up being used in
            // the wasm module, an optimization pass at some point may have
            // ended up removing the code that needed the import, removing the
            // import.
            if !wasm_imports.contains_key(&import.name) {
                continue
            }
            imports_bound.insert(import.name.clone());
            let (val, ts) = self.generate_import(import);
            imports_object.push_str(&import.name);
            imports_object.push_str(":");
            imports_object.push_str(&val);
            imports_object.push_str(",\n");
            imports_interface.push_str(&ts);
            imports_interface.push_str("\n");
        }

        // If the user otherwise specified functions to import which *weren't*
        // part of wasm-bindgen we want to make sure they come through here as
        // well.
        for (import, typescript) in wasm_imports.iter() {
            // ignore any internal functions we have for ourselves
            if import.starts_with("__wbindgen") {
                continue
            }
            // Ignore anything we just bound above,
            if imports_bound.contains(import) {
                continue
            }

            if extra_imports_interface.len() == 0 {
                extra_imports_interface.push_str("export interface ExtraImports {\n");
                imports_interface.push_str("env: ExtraImports;\n");
            }
            imports_object.push_str(import);
            imports_object.push_str(":");
            imports_object.push_str("_imports.env.");
            imports_object.push_str(import);
            imports_object.push_str(",\n");
            extra_imports_interface.push_str(typescript);
            extra_imports_interface.push_str("\n");
        }
        if extra_imports_interface.len() > 0 {
            extra_imports_interface.push_str("}\n");
        }

        if self.wasm_import_needed("__wbindgen_object_clone_ref", m) {
            self.expose_add_heap_object();
            self.expose_get_object();
            imports_object.push_str("
                __wbindgen_object_clone_ref: function(idx: number): number {
                    // If this object is on the stack promote it to the heap.
                    if ((idx & 1) === 1) {
                        return addHeapObject(getObject(idx));
                    }

                    // Otherwise if the object is on the heap just bump the
                    // refcount and move on
                    const val = slab[idx >> 1];
                    if (typeof(val) === 'number')
                        throw new Error('corrupt slab');
                    val.cnt += 1;
                    return idx;
                },
            ");
        }

        if self.wasm_import_needed("__wbindgen_object_drop_ref", m) {
            self.expose_drop_ref();
            imports_object.push_str("__wbindgen_object_drop_ref: dropRef,\n");
        }

        if self.wasm_import_needed("__wbindgen_throw", m) {
            self.expose_get_string_from_wasm();
            imports_object.push_str("__wbindgen_throw: function(ptr: number, len: number) {
                throw new Error(getStringFromWasm(ptr, len));
            },\n");
        }

        let mut writes = String::new();
        if self.exposed_globals.contains(&"memory") {
            writes.push_str("memory = exports.memory;\n");
        }
        if self.exposed_globals.contains(&"wasm_exports") {
            writes.push_str("wasm_exports = exports;\n");
        }
        format!("
            /* tslint:disable */
            {globals}

            interface WasmImportsTop {{
                env: WasmImports,
            }}

            interface WasmImports {{
                {wasm_imports}
            }}

            interface WasmExports {{
                {wasm_exports}
            }}

            export interface Imports {{
                {imports_interface}
            }}

            {extra_imports_interface}

            export interface Exports {{
                module: WebAssembly.Module;
                instance: WebAssembly.Module;
                {exports_interface}
            }}

            {extra_exports_interface}

            function xform(obj: WebAssembly.ResultObject): Exports {{
                let {{ module, instance }} = obj;
                let exports: WasmExports = instance.exports;
                {writes}
                return {exports};
            }}
            export function instantiate(bytes: any, _imports: Imports): Promise<Exports> {{
                let wasm_imports: WasmImportsTop = {{
                    env: {{
                        {imports_object}
                    }},
                }};
                return WebAssembly.instantiate(bytes, wasm_imports).then(xform);
            }}
        ",
            globals = self.globals,
            exports = exports,
            imports_object = imports_object,
            writes = writes,
            imports_interface = imports_interface,
            extra_imports_interface = extra_imports_interface,
            exports_interface = exports_interface,
            extra_exports_interface = extra_exports_interface,
            wasm_imports = wasm_imports.values()
                .map(|s| &**s)
                .collect::<Vec<_>>()
                .join("\n"),
            wasm_exports = wasm_exports.values()
                .map(|s| &**s)
                .collect::<Vec<_>>()
                .join("\n"),
        )
    }

    fn wasm_import_needed(&self, name: &str, m: &Module) -> bool {
        let imports = match m.import_section() {
            Some(s) => s,
            None => return false,
        };

        imports.entries().iter().any(|i| {
            i.module() == "env" && i.field() == name
        })
    }

    /// Returns a map of import name to the typescript definition for that name.
    ///
    /// This function generates the list of imports that a wasm module has,
    /// using the source of truth (the was module itself) to generate this list.
    fn typescript_wasm_imports(&self, m: &Module) -> HashMap<String, String> {
        let imports = match m.import_section() {
            Some(s) => s,
            None => return HashMap::new(),
        };
        let types = match m.type_section() {
            Some(s) => s,
            None => return HashMap::new(),
        };

        let mut map = HashMap::new();
        for import in imports.entries() {
            assert_eq!(import.module(), "env");

            let ty = match *import.external() {
                External::Function(i) => {
                    match types.types()[i as usize] {
                        Type::Function(ref t) => t,
                    }
                }
                _ => continue,
            };

            let mut ts = String::new();
            ts.push_str(import.field());
            ts.push_str("(");
            // TODO: probably match `arg` to catch exhaustive errors in the
            // future
            for (i, _arg) in ty.params().iter().enumerate() {
                if i > 0 {
                    ts.push_str(", ");
                }
                ts.push_str(&format!("arg{}: number", i));
            }
            ts.push_str("): ");
            if ty.return_type().is_none() {
                ts.push_str("void");
            } else {
                ts.push_str("number");
            }
            ts.push_str(";");

            map.insert(import.field().to_string(), ts);
        }
        return map;
    }

    /// Returns a map from export name to its typescript signature.
    ///
    /// This uses the module itself as the source of truth to help flesh out
    /// bugs in this program.
    fn typescript_wasm_exports(&self, m: &Module) -> HashMap<String, String> {
        let imported_functions = match m.import_section() {
            Some(s) => s.functions(),
            None => 0,
        };
        let functions = match m.function_section() {
            Some(s) => s,
            None => return HashMap::new(),
        };
        let types = match m.type_section() {
            Some(s) => s,
            None => return HashMap::new(),
        };
        let exports = match m.export_section() {
            Some(s) => s,
            None => return HashMap::new(),
        };

        let mut map = HashMap::new();
        for export in exports.entries() {
            let fn_idx = match *export.internal() {
                Internal::Function(i) => i as usize,
                Internal::Memory(_) => {
                    map.insert(export.field().to_string(),
                               format!("{}: WebAssembly.Memory;", export.field()));
                    continue
                }
                _ => continue,
            };
            assert!(fn_idx >= imported_functions);
            let function = &functions.entries()[fn_idx - imported_functions];
            let ty = match types.types()[function.type_ref() as usize] {
                Type::Function(ref t) => t,
            };

            let mut ts = String::new();
            ts.push_str(export.field());
            ts.push_str("(");
            // TODO: probably match `arg` to catch exhaustive errors in the
            // future
            for (i, _arg) in ty.params().iter().enumerate() {
                if i > 0 {
                    ts.push_str(", ");
                }
                ts.push_str(&format!("arg{}: number", i));
            }
            ts.push_str("): ");
            if ty.return_type().is_none() {
                ts.push_str("void");
            } else {
                ts.push_str("number");
            }
            ts.push_str(";");
            map.insert(export.field().to_string(), ts);
        }
        return map;
    }

    fn expose_drop_ref(&mut self) {
        if !self.exposed_globals.insert("drop_ref") {
            return
        }
        self.expose_global_slab();
        self.expose_global_slab_next();
        self.globals.push_str("
            function dropRef(idx: number): void {
                if ((idx & 1) == 1)
                    throw new Error('cannot drop ref of stack objects');

                // Decrement our refcount, but if it's still larger than one
                // keep going
                let obj = slab[idx >> 1];
                if (typeof(obj) === 'number')
                    throw new Error('corrupt slab');
                obj.cnt -= 1;
                if (obj.cnt > 0)
                    return;

                // If we hit 0 then free up our space in the slab
                slab[idx >> 1] = slab_next;
                slab_next = idx >> 1;
            }
        ");
    }

    fn expose_global_stack(&mut self) {
        if !self.exposed_globals.insert("stack") {
            return
        }
        self.globals.push_str("
            let stack: any[] = [];
        ");
    }

    fn expose_global_slab(&mut self) {
        if !self.exposed_globals.insert("slab") {
            return
        }
        self.globals.push_str("
            let slab: ({ obj: any, cnt: number } | number)[] = [];
        ");
    }

    fn expose_global_slab_next(&mut self) {
        if !self.exposed_globals.insert("slab_next") {
            return
        }
        self.globals.push_str("
            let slab_next: number = 0;
        ");
    }

    fn expose_get_object(&mut self) {
        if !self.exposed_globals.insert("get_object") {
            return
        }
        self.expose_global_stack();
        self.expose_global_slab();
        self.globals.push_str("
            function getObject(idx: number): any {
                if ((idx & 1) === 1) {
                    return stack[idx >> 1];
                } else {
                    const val = slab[idx >> 1];
                    if (typeof(val) === 'number')
                        throw new Error('corrupt slab');
                    return val.obj;
                }
            }
        ");
    }

    fn expose_global_memory(&mut self) {
        if !self.exposed_globals.insert("memory") {
            return
        }
        self.globals.push_str("let memory: WebAssembly.Memory;\n");
    }

    fn expose_wasm_exports(&mut self) {
        if !self.exposed_globals.insert("wasm_exports") {
            return
        }
        self.globals.push_str("let wasm_exports: WasmExports;\n");
    }

    fn expose_check_token(&mut self) {
        if !self.exposed_globals.insert("check_token") {
            return
        }
        self.globals.push_str("\
            const token = Symbol('foo');
            function _checkToken(sym: Symbol): void {
                if (token !== sym)
                    throw new Error('cannot invoke `new` directly');
            }
        ");
    }

    fn expose_assert_num(&mut self) {
        if !self.exposed_globals.insert("assert_num") {
            return
        }
        self.globals.push_str("\
            function _assertNum(n: number): void {
                if (typeof(n) !== 'number')
                    throw new Error('expected a number argument');
            }
        ");
    }

    fn expose_assert_bool(&mut self) {
        if !self.exposed_globals.insert("assert_bool") {
            return
        }
        self.globals.push_str("\
            function _assertBoolean(n: boolean): number {
                if (typeof(n) !== 'boolean')
                    throw new Error('expected a boolean argument');
                if (n) {
                    return 1;
                } else {
                    return 0;
                }
            }
        ");
    }

    fn expose_pass_string_to_wasm(&mut self) {
        if !self.exposed_globals.insert("pass_string_to_wasm") {
            return
        }
        self.expose_wasm_exports();
        self.expose_global_memory();
        if self.nodejs {
            self.globals.push_str("
                function passStringToWasm(arg: string): [number, number] {
                    if (typeof(arg) !== 'string')
                        throw new Error('expected a string argument');
                    const buf = Buffer.from(arg);
                    const len = buf.length;
                    const ptr = wasm_exports.__wbindgen_malloc(len);
                    buf.copy(Buffer.from(memory.buffer), ptr);
                    return [ptr, len];
                }
            ");
        } else {
            self.globals.push_str("
                function passStringToWasm(arg: string): [number, number] {
                    if (typeof(arg) !== 'string')
                        throw new Error('expected a string argument');
                    const buf = new TextEncoder('utf-8').encode(arg);
                    const len = buf.length;
                    const ptr = wasm_exports.__wbindgen_malloc(len);
                    let array = new Uint8Array(memory.buffer);
                    array.set(buf, ptr);
                    return [ptr, len];
                }
            ");
        }
    }

    fn expose_get_string_from_wasm(&mut self) {
        if !self.exposed_globals.insert("get_string_from_wasm") {
            return
        }
        if self.nodejs {
            self.expose_global_memory();
            self.globals.push_str("
                function getStringFromWasm(ptr: number, len: number): string {
                    const buf = Buffer.from(memory.buffer).slice(ptr, ptr + len);
                    const ret = buf.toString();
                    return ret;
                }
            ");
        } else {
            self.expose_global_memory();
            self.globals.push_str("
                function getStringFromWasm(ptr: number, len: number): string {
                    const mem = new Uint8Array(memory.buffer);
                    const slice = mem.slice(ptr, ptr + len);
                    const ret = new TextDecoder('utf-8').decode(slice);
                    return ret;
                }
            ");
        }
    }

    fn expose_assert_class(&mut self) {
        if !self.exposed_globals.insert("assert_class") {
            return
        }
        self.globals.push_str("
            function _assertClass(instance: any, klass: any) {
                if (!(instance instanceof klass))
                    throw new Error(`expected instance of ${klass.name}`);
                return instance.__wasmPtr;
            }
        ");
    }

    fn expose_borrowed_objects(&mut self) {
        if !self.exposed_globals.insert("borrowed_objects") {
            return
        }
        self.expose_global_stack();
        self.globals.push_str("
            function addBorrowedObject(obj: any): number {
                stack.push(obj);
                return ((stack.length - 1) << 1) | 1;
            }

            function popBorrowedObject(): void {
                stack.pop();
            }
        ");
    }

    fn expose_take_object(&mut self) {
        if !self.exposed_globals.insert("take_object") {
            return
        }
        self.expose_get_object();
        self.expose_drop_ref();
        self.globals.push_str("
            function takeObject(idx: number): any {
                const ret = getObject(idx);
                dropRef(idx);
                return ret;
            }
        ");
    }

    fn expose_add_heap_object(&mut self) {
        if !self.exposed_globals.insert("add_heap_object") {
            return
        }
        self.expose_global_slab();
        self.expose_global_slab_next();
        self.globals.push_str("
            function addHeapObject(obj: any): number {
                if (slab_next == slab.length) {
                    slab.push(slab.length + 1);
                }
                const idx = slab_next;
                const next = slab[idx];
                if (typeof(next) !== 'number')
                    throw new Error('corrupt slab');
                slab_next = next;
                slab[idx] = { obj, cnt: 1 };
                return idx << 1;
            }
        ");
    }
}
