use std::collections::{HashSet, HashMap};
use std::fmt;

use shared;
use parity_wasm::elements::*;

use super::Mapped;

#[derive(Default)]
pub struct Js {
    globals: String,
    exposed_globals: HashSet<&'static str>,
    exports: Vec<(String, String, String)>,
    wasm_exports_bound: HashSet<String>,
    classes: Vec<String>,
    pub nodejs: bool,
    pub debug: bool,
    pub ts: bool,
}

impl Js {
    pub fn generate_program(&mut self,
                            program: &shared::Program,
                            m: &Mapped) {
        for f in program.free_functions.iter() {
            self.generate_free_function(f, m);
        }
        for s in program.structs.iter() {
            self.generate_struct(s, m);
        }
    }

    pub fn generate_free_function(&mut self,
                                  func: &shared::Function,
                                  m: &Mapped) {
        let (js, ts) = self.generate_function("function",
                                              &func.name,
                                              &func.name,
                                              false,
                                              &func.arguments,
                                              func.ret.as_ref(),
                                              m);

        self.exports.push((func.name.clone(), js, ts));
    }

    pub fn generate_struct(&mut self,
                           s: &shared::Struct,
                           m: &Mapped) {
        let mut dst = String::new();
        self.expose_wasm_exports();
        dst.push_str(&format!("
            export class {} {{
        ", s.name));
        let number = self.typed("number");
        let symbol = self.typed("Symbol");
        let void = self.typed("void");
        if self.ts {
            dst.push_str("
                public ptr: number;
            ");
        }
        if self.debug {
            self.expose_check_token();
            dst.push_str(&format!("
                constructor(ptr{}, sym{}) {{
                    _checkToken(sym);
                    this.ptr = ptr;
                }}
            ", number, symbol));
        } else {
            dst.push_str(&format!("
                constructor(ptr{}) {{
                    this.ptr = ptr;
                }}
            ", number));
        }

        dst.push_str(&format!("
            free(){} {{
                const ptr = this.ptr;
                this.ptr = 0;
                wasm_exports.{}(ptr);
            }}
        ", void, m.export_name(&s.free_function())));

        self.wasm_exports_bound.insert(s.name.clone());

        for function in s.functions.iter() {
            let (js, _ts) = self.generate_function(
                "static",
                &function.name,
                &function.struct_function_export_name(&s.name),
                false,
                &function.arguments,
                function.ret.as_ref(),
                m,
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
                m,
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
                         ret: Option<&shared::Type>,
                         m: &Mapped) -> (String, String) {
        let mut dst = format!("{}(", name);
        let mut passed_args = String::new();
        let mut arg_conversions = String::new();
        let mut destructors = String::new();

        if is_method {
            passed_args.push_str("this.ptr");
        }

        for (i, arg) in arguments.iter().enumerate() {
            let name = format!("arg{}", i);
            if i > 0 {
                dst.push_str(", ");
            }
            dst.push_str(&name);

            let mut pass = |arg: &str| {
                if passed_args.len() > 0 {
                    passed_args.push_str(", ");
                }
                passed_args.push_str(arg);
            };
            match *arg {
                shared::Type::Number => {
                    dst.push_str(&format!("{}", self.typed("number")));
                    if self.debug {
                        self.expose_assert_num();
                        arg_conversions.push_str(&format!("_assertNum({});\n", name));
                    }
                    pass(&name)
                }
                shared::Type::Boolean => {
                    dst.push_str(&format!("{}", self.typed("boolean")));
                    if self.debug {
                        self.expose_assert_bool();
                        arg_conversions.push_str(&format!("\
                            _assertBoolean({name});
                        ", name = name));
                    } else {
                    }
                    pass(&format!("arg{i} ? 1 : 0", i = i))
                }
                shared::Type::BorrowedStr |
                shared::Type::String => {
                    dst.push_str(&format!("{}", self.typed("string")));
                    self.expose_pass_string_to_wasm(m);
                    arg_conversions.push_str(&format!("\
                        const [ptr{i}, len{i}] = passStringToWasm({arg});
                    ", i = i, arg = name));
                    pass(&format!("ptr{}", i));
                    pass(&format!("len{}", i));
                    if let shared::Type::BorrowedStr = *arg {
                        self.expose_wasm_exports();
                        destructors.push_str(&format!("\n\
                            wasm_exports.{free}(ptr{i}, len{i});\n\
                        ", i = i, free = m.export_name("__wbindgen_free")));
                    }
                }
                shared::Type::ByRef(ref s) |
                shared::Type::ByMutRef(ref s) => {
                    dst.push_str(&format!("{}", self.typed(s)));
                    if self.debug {
                        self.expose_assert_class();
                        arg_conversions.push_str(&format!("\
                            _assertClass({arg}, {struct_});
                        ", arg = name, struct_ = s));
                    }
                    pass(&format!("{}.ptr", name));
                }
                shared::Type::ByValue(ref s) => {
                    dst.push_str(&format!("{}", self.typed(s)));
                    if self.debug {
                        self.expose_assert_class();
                        arg_conversions.push_str(&format!("\
                            _assertClass({arg}, {struct_});
                        ", arg = name, struct_ = s));
                    }
                    arg_conversions.push_str(&format!("\
                        const ptr{i} = {arg}.ptr;
                        {arg}.ptr = 0;
                    ", i = i, arg = name));
                    pass(&format!("ptr{}", i));
                }
                shared::Type::JsObject => {
                    dst.push_str(&format!("{}", self.typed("any")));
                    self.expose_add_heap_object();
                    arg_conversions.push_str(&format!("\
                        const idx{i} = addHeapObject({arg});
                    ", i = i, arg = name));
                    pass(&format!("idx{}", i));
                }
                shared::Type::JsObjectRef => {
                    dst.push_str(&format!("{}", self.typed("any")));
                    self.expose_borrowed_objects();
                    arg_conversions.push_str(&format!("\
                        const idx{i} = addBorrowedObject({arg});
                    ", i = i, arg = name));
                    destructors.push_str("stack.pop();\n");
                    pass(&format!("idx{}", i));
                }
            }
        }
        dst.push_str(")");
        let convert_ret = match ret {
            None => {
                dst.push_str(&format!("{}", self.typed("void")));
                format!("return ret;")
            }
            Some(&shared::Type::Number) => {
                dst.push_str(&format!("{}", self.typed("number")));
                format!("return ret;")
            }
            Some(&shared::Type::Boolean) => {
                dst.push_str(&format!("{}", self.typed("boolean")));
                format!("return ret != 0;")
            }
            Some(&shared::Type::JsObject) => {
                dst.push_str(&format!("{}", self.typed("any")));
                self.expose_take_object();
                format!("return takeObject(ret);")
            }
            Some(&shared::Type::JsObjectRef) |
            Some(&shared::Type::BorrowedStr) |
            Some(&shared::Type::ByMutRef(_)) |
            Some(&shared::Type::ByRef(_)) => panic!(),
            Some(&shared::Type::ByValue(ref name)) => {
                dst.push_str(&format!("{}", self.typed(name)));
                if self.debug {
                    format!("\
                        return new {name}(ret, token);
                    ", name = name)
                } else {
                    format!("\
                        return new {name}(ret);
                    ", name = name)
                }
            }
            Some(&shared::Type::String) => {
                dst.push_str(&format!("{}", self.typed("string")));
                self.expose_get_string_from_wasm();
                self.expose_wasm_exports();
                format!("
                    const ptr = wasm_exports.{}(ret);
                    const len = wasm_exports.{}(ret);
                    const realRet = getStringFromWasm(ptr, len);
                    wasm_exports.{}(ret);
                    return realRet;
                ",
                    m.export_name("__wbindgen_boxed_str_ptr"),
                    m.export_name("__wbindgen_boxed_str_len"),
                    m.export_name("__wbindgen_boxed_str_free"),
                )
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
            ",
                f = m.export_name(wasm_name),
                passed = passed_args,
                convert_ret = convert_ret,
            ));
        } else {
            dst.push_str(&format!("\
                try {{
                    const ret = wasm_exports.{f}({passed});
                    {convert_ret}
                }} finally {{
                    {destructors}
                }}
            ",
                f = m.export_name(wasm_name),
                passed = passed_args,
                destructors = destructors,
                convert_ret = convert_ret,
            ));
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
        let number = self.typed("number");

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
            ts_dst.push_str(&format!("arg{}", i));
            match *arg {
                shared::Type::Number => {
                    ts_dst.push_str(&self.typed("number").to_string());
                    invocation.push_str(&format!("arg{}", i));
                    dst.push_str(&format!("arg{}{}", i, number));
                }
                shared::Type::Boolean => {
                    ts_dst.push_str(&self.typed("boolean").to_string());
                    invocation.push_str(&format!("arg{} != 0", i));
                    dst.push_str(&format!("arg{}{}", i, number));
                }
                shared::Type::BorrowedStr => {
                    ts_dst.push_str(&self.typed("string").to_string());
                    self.expose_get_string_from_wasm();
                    invocation.push_str(&format!("getStringFromWasm(ptr{0}, len{0})", i));
                    dst.push_str(&format!("ptr{0}{1}, len{0}{1}", i, number));
                }
                shared::Type::JsObject => {
                    ts_dst.push_str(&self.typed("any").to_string());
                    self.expose_take_object();
                    invocation.push_str(&format!("takeObject(arg{})", i));
                    dst.push_str(&format!("arg{}{}", i, number));
                }
                shared::Type::JsObjectRef => {
                    ts_dst.push_str(&self.typed("any").to_string());
                    self.expose_get_object();
                    invocation.push_str(&format!("getObject(arg{})", i));
                    dst.push_str(&format!("arg{}{}", i, number));
                }
                shared::Type::String |
                shared::Type::ByRef(_) |
                shared::Type::ByMutRef(_) |
                shared::Type::ByValue(_) => {
                    panic!("unsupported type in import");
                }
            }
        }
        ts_dst.push_str(")");
        dst.push_str(")");
        let invoc = format!("_imports.{}({})", import.name, invocation);
        let invoc = match import.ret {
            Some(shared::Type::Number) => {
                ts_dst.push_str(&self.typed("number").to_string());
                dst.push_str(&self.typed("number").to_string());
                invoc
            }
            Some(shared::Type::Boolean) => {
                ts_dst.push_str(&self.typed("boolean").to_string());
                dst.push_str(&self.typed("number").to_string());
                format!("{} ? 1 : 0", invoc)
            }
            Some(shared::Type::JsObject) => {
                ts_dst.push_str(&self.typed("any").to_string());
                dst.push_str(&self.typed("number").to_string());
                self.expose_add_heap_object();
                format!("addHeapObject({})", invoc)
            }
            None => {
                ts_dst.push_str(&self.typed("void").to_string());
                dst.push_str(&self.typed("void").to_string());
                invoc
            }
            _ => unimplemented!(),
        };
        ts_dst.push_str("\n");
        dst.push_str(" {\n");
        dst.push_str(&format!("return {};\n}}", invoc));

        (dst, ts_dst)
    }

    pub fn to_string(&mut self, m: &Mapped, program: &shared::Program) -> String {
        if self.debug {
            self.expose_global_slab();
            self.expose_global_stack();
            let void = self.typed("void");
            self.exports.push(
                (
                    "assertHeapAndStackEmpty".to_string(),
                    format!("function(){} {{
                        if (stack.length > 0)
                            throw new Error('stack is not empty');
                        for (let i = 0; i < slab.length; i++) {{
                            if (typeof(slab[i]) !== 'number')
                                throw new Error('slab is not empty');
                        }}
                    }}", void),
                    format!("assertHeapAndStackEmpty(){};\n", void),
                )
            );
        }

        for class in self.classes.iter() {
            self.globals.push_str(class);
            self.globals.push_str("\n");
        }
        let wasm_exports = self.typescript_wasm_exports(&m.module);
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
            let orig_export = m.orig_export_name(export);
            if orig_export.starts_with("__wbindgen") {
                continue
            }
            // Ignore anything we just bound above,
            if self.wasm_exports_bound.contains(orig_export) {
                continue
            }

            assert_eq!(orig_export, export);
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
        let wasm_imports = self.typescript_wasm_imports(&m.module);

        let mut imports_object = String::new();
        let mut extra_imports_interface = String::new();
        let mut imports_bound = HashSet::new();
        let mut imports_interface = String::new();
        for import in program.imports.iter() {
            // Only actually generate this import if it ended up being used in
            // the wasm module, an optimization pass at some point may have
            // ended up removing the code that needed the import, removing the
            // import.
            let name = m.import_name(&import.name);
            if !wasm_imports.contains_key(name) {
                continue
            }
            imports_bound.insert(name.to_string());
            let (val, ts) = self.generate_import(import);
            imports_object.push_str(&name);
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
            let orig_import = m.orig_import_name(import);
            if orig_import.starts_with("__wbindgen") {
                continue
            }
            // Ignore anything we just bound above,
            if imports_bound.contains(import) {
                continue
            }

            assert_eq!(orig_import, import);
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

        {
            let mut bind = |name: &str, f: &Fn(&mut Self) -> String| {
                if !self.wasm_import_needed(name, m) {
                    return
                }
                imports_object.push_str(&format!("
                    {}: {},
                ", m.import_name(name), f(self)));
            };

            bind("__wbindgen_object_clone_ref", &|me| {
                me.expose_add_heap_object();
                me.expose_get_object();
                let bump_cnt = if me.debug {
                    String::from("
                        if (typeof(val) === 'number')
                            throw new Error('corrupt slab');
                        val.cnt += 1;
                    ")
                } else if me.ts {
                    String::from("(val as {cnt:number}).cnt += 1;")
                } else {
                    String::from("val.cnt += 1;")
                };
                let number = me.typed("number");
                format!("
                    function(idx{}){} {{
                        // If this object is on the stack promote it to the heap.
                        if ((idx & 1) === 1)
                            return addHeapObject(getObject(idx));

                        // Otherwise if the object is on the heap just bump the
                        // refcount and move on
                        const val = slab[idx >> 1];
                        {}
                        return idx;
                    }}
                ", number, number, bump_cnt)
            });

            bind("__wbindgen_object_drop_ref", &|me| {
                me.expose_drop_ref();
                "dropRef".to_string()
            });

            bind("__wbindgen_string_new", &|me| {
                me.expose_add_heap_object();
                me.expose_get_string_from_wasm();
                String::from("(p, l) => addHeapObject(getStringFromWasm(p, l))")
            });

            bind("__wbindgen_number_new", &|me| {
                me.expose_add_heap_object();
                String::from("addHeapObject")
            });

            bind("__wbindgen_number_get", &|me| {
                me.expose_global_memory();
                let number = me.typed("number");
                format!("
                    function(n{0}, invalid{0}){0} {{
                        let obj = getObject(n);
                        if (typeof(obj) === 'number')
                            return obj;
                        (new Uint8Array(memory.buffer))[invalid] = 1;
                        return 0;
                    }}
                ", number)
            });

            bind("__wbindgen_undefined_new", &|me| {
                me.expose_add_heap_object();
                String::from("() => addHeapObject(undefined)")
            });

            bind("__wbindgen_null_new", &|me| {
                me.expose_add_heap_object();
                String::from("() => addHeapObject(null)")
            });

            bind("__wbindgen_is_null", &|me| {
                me.expose_get_object();
                String::from("(idx) => getObject(idx) === null ? 1 : 0")
            });

            bind("__wbindgen_is_undefined", &|me| {
                me.expose_get_object();
                String::from("(idx) => getObject(idx) === undefined ? 1 : 0")
            });

            bind("__wbindgen_boolean_new", &|me| {
                me.expose_add_heap_object();
                String::from("(v) => addHeapObject(v == 1)")
            });

            bind("__wbindgen_boolean_get", &|me| {
                me.expose_get_object();
                String::from("(i) => {
                    let v = getObject(i);
                    if (typeof(v) == 'boolean') {
                        return v ? 1 : 0;
                    } else {
                        return 2;
                    }
                }")
            });

            bind("__wbindgen_symbol_new", &|me| {
                me.expose_get_string_from_wasm();
                me.expose_add_heap_object();
                let symbol = me.typed("Symbol");
                format!("(ptr, len) => {{
                    let a{};
                    console.log(ptr, len);
                    if (ptr === 0) {{
                        a = Symbol();
                    }} else {{
                        a = Symbol(getStringFromWasm(ptr, len));
                    }}
                    return addHeapObject(a);
                }}", symbol)
            });

            bind("__wbindgen_is_symbol", &|me| {
                me.expose_get_object();
                String::from("(i) => typeof(getObject(i)) == 'symbol' ? 1 : 0")
            });

            bind("__wbindgen_throw", &|me| {
                me.expose_get_string_from_wasm();
                let number = me.typed("number");
                format!("
                    function(ptr{}, len{}) {{
                        throw new Error(getStringFromWasm(ptr, len));
                    }}
                ", number, number)
            });

            bind("__wbindgen_string_get", &|me| {
                me.expose_pass_string_to_wasm(m);
                me.expose_get_object();
                me.expose_global_memory();
                String::from("(i, len_ptr) => {
                    let obj = getObject(i);
                    if (typeof(obj) !== 'string')
                        return 0;
                    const [ptr, len] = passStringToWasm(obj);
                    (new Uint32Array(memory.buffer))[len_ptr / 4] = len;
                    return ptr;
                }")
            });
        }

        let mut writes = String::new();
        if self.exposed_globals.contains(&"memory") {
            writes.push_str("memory = exports.memory;\n");
        }
        if self.exposed_globals.contains(&"wasm_exports") {
            writes.push_str("wasm_exports = exports;\n");
        }
        let mut interfaces = format!("
            /* tslint:disable */
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
        ",
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
        );
        if !self.ts {
            interfaces.truncate(0);
        }

        let any = self.typed("any");
        let imports_ty = self.typed("Imports");
        let exports_ty = self.typed("Exports");
        let result_ty = self.typed("WebAssembly.ResultObject");
        let promise_ty = self.typed("Promise<Exports>");
        let wasm_exports_ty = self.typed("WasmExports");
        let wasm_imports_ty = self.typed("WasmImportsTop");

        format!("
            /* tslint:disable */
            {globals}

            {interfaces}

            function xform(obj{result_ty}){exports_ty} {{
                let {{ module, instance }} = obj;
                let exports{wasm_exports_ty} = instance.exports;
                {writes}
                return {exports};
            }}
            export function instantiate(bytes{any}, _imports{imports_ty}){promise_ty} {{
                let wasm_imports{wasm_imports_ty} = {{
                    env: {{
                        {imports_object}
                    }},
                }};
                return WebAssembly.instantiate(bytes, wasm_imports).then(xform);
            }}
        ",
            globals = self.globals,
            interfaces = interfaces,
            any = any,
            result_ty = result_ty,
            exports_ty = exports_ty,
            promise_ty = promise_ty,
            imports_ty = imports_ty,
            wasm_imports_ty = wasm_imports_ty,
            wasm_exports_ty = wasm_exports_ty,
            exports = exports,
            imports_object = imports_object,
            writes = writes,
        )
    }

    fn wasm_import_needed(&self, name: &str, m: &Mapped) -> bool {
        let imports = match m.module.import_section() {
            Some(s) => s,
            None => return false,
        };

        imports.entries().iter().any(|i| {
            i.module() == "env" && i.field() == m.import_name(name)
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
                ts.push_str(&format!("arg{}{}", i, self.typed("number")));
            }
            ts.push_str(")");
            if ty.return_type().is_none() {
                ts.push_str(&self.typed("void").to_string());
            } else {
                ts.push_str(&self.typed("number").to_string());
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
                ts.push_str(&format!("arg{}{}", i, self.typed("number")));
            }
            ts.push_str(")");
            if ty.return_type().is_none() {
                ts.push_str(&self.typed("void").to_string());
            } else {
                ts.push_str(&self.typed("number").to_string());
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
        let validate_owned = if self.debug {
            String::from("
                if ((idx & 1) === 1)
                    throw new Error('cannot drop ref of stack objects');
            ")
        } else {
            String::new()
        };
        let dec_ref = if self.debug {
            String::from("
                if (typeof(obj) === 'number')
                    throw new Error('corrupt slab');
                obj.cnt -= 1;
                if (obj.cnt > 0)
                    return;
            ")
        } else if self.ts {
            String::from("
                (obj as {cnt:number}).cnt -= 1;
                if ((obj as {cnt:number}).cnt > 0)
                    return;
            ")
        } else {
            String::from("
                obj.cnt -= 1;
                if (obj.cnt > 0)
                    return;
            ")
        };
        let number = self.typed("number");
        let void = self.typed("void");
        self.globals.push_str(&format!("
            function dropRef(idx{}){} {{
                {}

                let obj = slab[idx >> 1];
                {}

                // If we hit 0 then free up our space in the slab
                slab[idx >> 1] = slab_next;
                slab_next = idx >> 1;
            }}
        ", number, void, validate_owned, dec_ref));
    }

    fn expose_global_stack(&mut self) {
        if !self.exposed_globals.insert("stack") {
            return
        }
        let ty = self.typed("any[]");
        self.globals.push_str(&format!("
            let stack{} = [];
        ", ty));
    }

    fn expose_global_slab(&mut self) {
        if !self.exposed_globals.insert("slab") {
            return
        }
        let ty = self.typed("({ obj: any, cnt: number } | number)[]");
        self.globals.push_str(&format!("let slab{} = [];", ty));
    }

    fn expose_global_slab_next(&mut self) {
        if !self.exposed_globals.insert("slab_next") {
            return
        }
        let ty = self.typed("number");
        self.globals.push_str(&format!("
            let slab_next{} = 0;
        ", ty));
    }

    fn expose_get_object(&mut self) {
        if !self.exposed_globals.insert("get_object") {
            return
        }
        self.expose_global_stack();
        self.expose_global_slab();

        let get_obj = if self.debug {
            String::from("
                if (typeof(val) === 'number')
                    throw new Error('corrupt slab');
                return val.obj;
            ")
        } else if self.ts {
            String::from("
                return (val as {obj:any}).obj;
            ")
        } else {
            String::from("
                return val.obj;
            ")
        };
        let number = self.typed("number");
        let any = self.typed("any");
        self.globals.push_str(&format!("
            function getObject(idx{}){} {{
                if ((idx & 1) === 1) {{
                    return stack[idx >> 1];
                }} else {{
                    const val = slab[idx >> 1];
                    {}
                }}
            }}
        ", number, any, get_obj));
    }

    fn expose_global_memory(&mut self) {
        if !self.exposed_globals.insert("memory") {
            return
        }
        let mem = self.typed("WebAssembly.Memory");
        self.globals.push_str(&format!("let memory{};\n", mem));
    }

    fn expose_wasm_exports(&mut self) {
        if !self.exposed_globals.insert("wasm_exports") {
            return
        }
        let ty = self.typed("WasmExports");
        self.globals.push_str(&format!("let wasm_exports{};\n", ty));
    }

    fn expose_check_token(&mut self) {
        if !self.exposed_globals.insert("check_token") {
            return
        }
        let symbol = self.typed("Symbol");
        let void = self.typed("void");
        self.globals.push_str(&format!("
            const token = Symbol('foo');
            function _checkToken(sym{}){} {{
                if (token !== sym)
                    throw new Error('cannot invoke `new` directly');
            }}
        ", symbol, void));
    }

    fn expose_assert_num(&mut self) {
        if !self.exposed_globals.insert("assert_num") {
            return
        }
        let number = self.typed("number");
        let void = self.typed("void");
        self.globals.push_str(&format!("
            function _assertNum(n{}){} {{
                if (typeof(n) !== 'number')
                    throw new Error('expected a number argument');
            }}
        ", number, void));
    }

    fn expose_assert_bool(&mut self) {
        if !self.exposed_globals.insert("assert_bool") {
            return
        }
        let boolean = self.typed("boolean");
        self.globals.push_str(&format!("
            function _assertBoolean(n{}) {{
                if (typeof(n) !== 'boolean')
                    throw new Error('expected a boolean argument');
            }}
        ", boolean));
    }

    fn expose_pass_string_to_wasm(&mut self, m: &Mapped) {
        if !self.exposed_globals.insert("pass_string_to_wasm") {
            return
        }
        self.expose_wasm_exports();
        self.expose_global_memory();
        let string = self.typed("string");
        let ret = self.typed("[number, number]");
        if self.nodejs {
            self.globals.push_str(&format!("
                function passStringToWasm(arg{}){} {{
                    if (typeof(arg) !== 'string')
                        throw new Error('expected a string argument');
                    const buf = Buffer.from(arg);
                    const len = buf.length;
                    const ptr = wasm_exports.{}(len);
                    buf.copy(Buffer.from(memory.buffer), ptr);
                    return [ptr, len];
                }}
            ", string, ret, m.export_name("__wbindgen_malloc")));
        } else {
            self.globals.push_str(&format!("
                function passStringToWasm(arg{}){} {{
                    if (typeof(arg) !== 'string')
                        throw new Error('expected a string argument');
                    const buf = new TextEncoder('utf-8').encode(arg);
                    const len = buf.length;
                    const ptr = wasm_exports.{}(len);
                    let array = new Uint8Array(memory.buffer);
                    array.set(buf, ptr);
                    return [ptr, len];
                }}
            ", string, ret, m.export_name("__wbindgen_malloc")));
        }
    }

    fn expose_get_string_from_wasm(&mut self) {
        if !self.exposed_globals.insert("get_string_from_wasm") {
            return
        }
        let number = self.typed("number");
        let string = self.typed("string");
        if self.nodejs {
            self.expose_global_memory();
            self.globals.push_str(&format!("
                function getStringFromWasm(ptr{}, len{}){} {{
                    const buf = Buffer.from(memory.buffer).slice(ptr, ptr + len);
                    const ret = buf.toString();
                    return ret;
                }}
            ", number, number, string));
        } else {
            self.expose_global_memory();
            self.globals.push_str(&format!("
                function getStringFromWasm(ptr{}, len{}){} {{
                    const mem = new Uint8Array(memory.buffer);
                    const slice = mem.slice(ptr, ptr + len);
                    const ret = new TextDecoder('utf-8').decode(slice);
                    return ret;
                }}
            ", number, number, string));
        }
    }

    fn expose_assert_class(&mut self) {
        if !self.exposed_globals.insert("assert_class") {
            return
        }
        let any = self.typed("any");
        self.globals.push_str(&format!("
            function _assertClass(instance{}, klass{}) {{
                if (!(instance instanceof klass))
                    throw new Error(`expected instance of ${{klass.name}}`);
                return instance.ptr;
            }}
        ", any, any));
    }

    fn expose_borrowed_objects(&mut self) {
        if !self.exposed_globals.insert("borrowed_objects") {
            return
        }
        self.expose_global_stack();
        let any = self.typed("any");
        let number = self.typed("number");
        self.globals.push_str(&format!("
            function addBorrowedObject(obj{}){} {{
                stack.push(obj);
                return ((stack.length - 1) << 1) | 1;
            }}
        ", any, number));
    }

    fn expose_take_object(&mut self) {
        if !self.exposed_globals.insert("take_object") {
            return
        }
        self.expose_get_object();
        self.expose_drop_ref();
        let number = self.typed("number");
        let any = self.typed("any");
        self.globals.push_str(&format!("
            function takeObject(idx{}){} {{
                const ret = getObject(idx);
                dropRef(idx);
                return ret;
            }}
        ", number, any));
    }

    fn expose_add_heap_object(&mut self) {
        if !self.exposed_globals.insert("add_heap_object") {
            return
        }
        self.expose_global_slab();
        self.expose_global_slab_next();
        let set_slab_next = if self.debug {
            String::from("
                if (typeof(next) !== 'number')
                    throw new Error('corrupt slab');
                slab_next = next;
            ")
        } else if self.ts {
            String::from("
                slab_next = next as number;
            ")
        } else {
            String::from("
                slab_next = next;
            ")
        };
        let any = self.typed("any");
        let number = self.typed("number");
        self.globals.push_str(&format!("
            function addHeapObject(obj{}){} {{
                if (slab_next == slab.length)
                    slab.push(slab.length + 1);
                const idx = slab_next;
                const next = slab[idx];
                {}
                slab[idx] = {{ obj, cnt: 1 }};
                return idx << 1;
            }}
        ", any, number, set_slab_next));
    }

    fn typed<T>(&self, t: T) -> MaybeEmit<Typed<T>> {
        self.maybe(Typed(t))
    }

    fn maybe<T>(&self, t: T) -> MaybeEmit<T> {
        MaybeEmit {
            enabled: self.ts,
            inner: t,
        }
    }
}

struct MaybeEmit<T> {
    enabled: bool,
    inner: T,
}

impl<T: fmt::Display> fmt::Display for MaybeEmit<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.enabled {
            self.inner.fmt(f)
        } else {
            Ok(())
        }
    }
}

struct Typed<T>(T);

impl<T: fmt::Display> fmt::Display for Typed<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, ": {}", self.0)
    }
}
