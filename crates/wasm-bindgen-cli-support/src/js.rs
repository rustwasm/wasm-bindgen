use std::collections::HashSet;

use shared;
use parity_wasm::elements::*;

use super::Bindgen;

pub struct Js<'a> {
    pub globals: String,
    pub imports: String,
    pub typescript: String,
    pub exposed_globals: HashSet<&'static str>,
    pub config: &'a Bindgen,
    pub module: &'a mut Module,
    pub program: &'a shared::Program,
}

impl<'a> Js<'a> {
    pub fn generate(&mut self, module_name: &str) -> (String, String) {
        for f in self.program.free_functions.iter() {
            self.generate_free_function(f);
        }
        for f in self.program.imports.iter() {
            self.generate_import(&f.0, &f.1);
        }
        for s in self.program.structs.iter() {
            self.generate_struct(s);
        }
        for s in self.program.imported_structs.iter() {
            self.generate_import_struct(s);
        }

        {
            let mut bind = |name: &str, f: &Fn(&mut Self) -> String| {
                if !self.wasm_import_needed(name) {
                    return
                }
                let global = format!("export const {} = {};", name, f(self));
                self.globals.push_str(&global);
            };

            bind("__wbindgen_object_clone_ref", &|me| {
                me.expose_add_heap_object();
                me.expose_get_object();
                let bump_cnt = if me.config.debug {
                    String::from("
                        if (typeof(val) === 'number')
                            throw new Error('corrupt slab');
                        val.cnt += 1;
                    ")
                } else {
                    String::from("val.cnt += 1;")
                };
                format!("
                    function(idx) {{
                        // If this object is on the stack promote it to the heap.
                        if ((idx & 1) === 1)
                            return addHeapObject(getObject(idx));

                        // Otherwise if the object is on the heap just bump the
                        // refcount and move on
                        const val = slab[idx >> 1];
                        {}
                        return idx;
                    }}
                ", bump_cnt)
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
                me.expose_get_object();
                format!("
                    function(n, invalid) {{
                        let obj = getObject(n);
                        if (typeof(obj) === 'number')
                            return obj;
                        (new Uint8Array(wasm.memory.buffer))[invalid] = 1;
                        return 0;
                    }}
                ")
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
                format!("(ptr, len) => {{
                    let a;
                    console.log(ptr, len);
                    if (ptr === 0) {{
                        a = Symbol();
                    }} else {{
                        a = Symbol(getStringFromWasm(ptr, len));
                    }}
                    return addHeapObject(a);
                }}")
            });

            bind("__wbindgen_is_symbol", &|me| {
                me.expose_get_object();
                String::from("(i) => typeof(getObject(i)) == 'symbol' ? 1 : 0")
            });

            bind("__wbindgen_throw", &|me| {
                me.expose_get_string_from_wasm();
                format!("
                    function(ptr, len) {{
                        throw new Error(getStringFromWasm(ptr, len));
                    }}
                ")
            });

            bind("__wbindgen_string_get", &|me| {
                me.expose_pass_string_to_wasm();
                me.expose_get_object();
                String::from("(i, len_ptr) => {
                    let obj = getObject(i);
                    if (typeof(obj) !== 'string')
                        return 0;
                    const [ptr, len] = passStringToWasm(obj);
                    (new Uint32Array(wasm.memory.buffer))[len_ptr / 4] = len;
                    return ptr;
                }")
            });
        }

        let js = format!("
            /* tslint:disable */
            import * as wasm from './{module_name}_wasm'; // imports from wasm file
            {imports}

            {globals}
        ",
            module_name = module_name,
            globals = self.globals,
            imports = self.imports,
        );

        self.rewrite_imports(module_name);

        (js, self.typescript.clone())
    }

    pub fn generate_free_function(&mut self, func: &shared::Function) {
        let (js, ts) = self.generate_function("function",
                                              &func.name,
                                              &func.name,
                                              false,
                                              &func.arguments,
                                              func.ret.as_ref());
        self.globals.push_str("export ");
        self.globals.push_str(&js);
        self.globals.push_str("\n");
        self.typescript.push_str("export ");
        self.typescript.push_str(&ts);
        self.typescript.push_str("\n");
    }

    pub fn generate_struct(&mut self, s: &shared::Struct) {
        let mut dst = String::new();
        dst.push_str(&format!("export class {} {{", s.name));
        let mut ts_dst = dst.clone();
        ts_dst.push_str("
            public ptr: number;
        ");
        if self.config.debug {
            self.expose_check_token();
            dst.push_str(&format!("
                constructor(ptr, sym) {{
                    _checkToken(sym);
                    this.ptr = ptr;
                }}
            "));
            ts_dst.push_str("constructor(ptr: number, sym: Symbol);\n");
        } else {
            dst.push_str(&format!("
                constructor(ptr) {{
                    this.ptr = ptr;
                }}
            "));
            ts_dst.push_str("constructor(ptr: number);\n");
        }

        dst.push_str(&format!("
            free() {{
                const ptr = this.ptr;
                this.ptr = 0;
                wasm.{}(ptr);
            }}
        ", s.free_function()));
        ts_dst.push_str("free(): void;\n");

        for function in s.functions.iter() {
            let (js, ts) = self.generate_function(
                "static",
                &function.name,
                &function.struct_function_export_name(&s.name),
                false,
                &function.arguments,
                function.ret.as_ref(),
            );
            dst.push_str(&js);
            dst.push_str("\n");
            ts_dst.push_str(&ts);
            ts_dst.push_str("\n");
        }
        for method in s.methods.iter() {
            let (js, ts) = self.generate_function(
                "",
                &method.function.name,
                &method.function.struct_function_export_name(&s.name),
                true,
                &method.function.arguments,
                method.function.ret.as_ref(),
            );
            dst.push_str(&js);
            dst.push_str("\n");
            ts_dst.push_str(&ts);
            ts_dst.push_str("\n");
        }
        dst.push_str("}\n");
        ts_dst.push_str("}\n");

        self.globals.push_str(&dst);
        self.globals.push_str("\n");
        self.typescript.push_str(&ts_dst);
        self.typescript.push_str("\n");
    }

    fn generate_function(&mut self,
                         prefix: &str,
                         name: &str,
                         wasm_name: &str,
                         is_method: bool,
                         arguments: &[shared::Type],
                         ret: Option<&shared::Type>) -> (String, String) {
        let mut dst = format!("{}(", name);
        let mut dst_ts = format!("{}(", name);
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
                dst_ts.push_str(", ");
            }
            dst.push_str(&name);
            dst_ts.push_str(&name);

            let mut pass = |arg: &str| {
                if passed_args.len() > 0 {
                    passed_args.push_str(", ");
                }
                passed_args.push_str(arg);
            };
            match *arg {
                shared::Type::Number => {
                    dst_ts.push_str(": number");
                    if self.config.debug {
                        self.expose_assert_num();
                        arg_conversions.push_str(&format!("_assertNum({});\n", name));
                    }
                    pass(&name)
                }
                shared::Type::Boolean => {
                    dst_ts.push_str(": boolean");
                    if self.config.debug {
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
                    dst_ts.push_str(": string");
                    self.expose_pass_string_to_wasm();
                    arg_conversions.push_str(&format!("\
                        const [ptr{i}, len{i}] = passStringToWasm({arg});
                    ", i = i, arg = name));
                    pass(&format!("ptr{}", i));
                    pass(&format!("len{}", i));
                    if let shared::Type::BorrowedStr = *arg {
                        destructors.push_str(&format!("\n\
                            wasm.__wbindgen_free(ptr{i}, len{i});\n\
                        ", i = i));
                    }
                }
                shared::Type::ByRef(ref s) |
                shared::Type::ByMutRef(ref s) => {
                    dst_ts.push_str(&format!(": {}", s));
                    if self.config.debug {
                        self.expose_assert_class();
                        arg_conversions.push_str(&format!("\
                            _assertClass({arg}, {struct_});
                        ", arg = name, struct_ = s));
                    }
                    pass(&format!("{}.ptr", name));
                }
                shared::Type::ByValue(ref s) => {
                    dst_ts.push_str(&format!(": {}", s));
                    if self.config.debug {
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
                    dst_ts.push_str(": any");
                    self.expose_add_heap_object();
                    arg_conversions.push_str(&format!("\
                        const idx{i} = addHeapObject({arg});
                    ", i = i, arg = name));
                    pass(&format!("idx{}", i));
                }
                shared::Type::JsObjectRef => {
                    dst_ts.push_str(": any");
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
        dst_ts.push_str(")");
        let convert_ret = match ret {
            None => {
                dst_ts.push_str(": void");
                format!("return ret;")
            }
            Some(&shared::Type::Number) => {
                dst_ts.push_str(": number");
                format!("return ret;")
            }
            Some(&shared::Type::Boolean) => {
                dst_ts.push_str(": boolean");
                format!("return ret != 0;")
            }
            Some(&shared::Type::JsObject) => {
                dst_ts.push_str(": any");
                self.expose_take_object();
                format!("return takeObject(ret);")
            }
            Some(&shared::Type::JsObjectRef) |
            Some(&shared::Type::BorrowedStr) |
            Some(&shared::Type::ByMutRef(_)) |
            Some(&shared::Type::ByRef(_)) => panic!(),
            Some(&shared::Type::ByValue(ref name)) => {
                dst_ts.push_str(": ");
                dst_ts.push_str(name);
                if self.config.debug {
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
                dst_ts.push_str(": string");
                self.expose_get_string_from_wasm();
                format!("
                    const ptr = wasm.__wbindgen_boxed_str_ptr(ret);
                    const len = wasm.__wbindgen_boxed_str_len(ret);
                    const realRet = getStringFromWasm(ptr, len);
                    wasm.__wbindgen_boxed_str_free(ret);
                    return realRet;
                ")
            }
        };
        dst_ts.push_str(";");
        dst.push_str(" {\n        ");
        dst.push_str(&arg_conversions);
        if destructors.len() == 0 {
            dst.push_str(&format!("\
                const ret = wasm.{}({passed});
                {convert_ret}
            ",
                f = wasm_name,
                passed = passed_args,
                convert_ret = convert_ret,
            ));
        } else {
            dst.push_str(&format!("\
                try {{
                    const ret = wasm.{f}({passed});
                    {convert_ret}
                }} finally {{
                    {destructors}
                }}
            ",
                f = wasm_name,
                passed = passed_args,
                destructors = destructors,
                convert_ret = convert_ret,
            ));
        }
        dst.push_str("}");
        (format!("{} {}", prefix, dst), format!("{} {}", prefix, dst_ts))
    }

    pub fn generate_import(&mut self, module: &str, import: &shared::Function) {
        let imported_name = format!("import{}", self.imports.len());

        self.imports.push_str(&format!("
            import {{ {} as {} }} from '{}';
        ", import.name, imported_name, module));

        self.gen_import_shim(&import.mangled_import_name(None),
                             &imported_name,
                             import)
    }

    pub fn generate_import_struct(&mut self, import: &shared::ImportStruct) {
        if let Some(ref module) = import.module {
            self.imports.push_str(&format!("
                import {{ {} }} from '{}';
            ", import.name, module));
        }

        for &(method, ref function) in import.functions.iter() {
            self.generate_import_struct_function(&import.name,
                                                 method,
                                                 function);
        }
    }

    fn generate_import_struct_function(
        &mut self,
        class: &str,
        is_method: bool,
        function: &shared::Function,
    ) {
        let delegate = if is_method {
            format!("{}.prototype.{}.call", class, function.name)
        } else {
            format!("{}.{}", class, function.name)
        };
        self.gen_import_shim(&function.mangled_import_name(Some(class)),
                             &delegate,
                             function)
    }

    fn gen_import_shim(
        &mut self,
        shim_name: &str,
        shim_delegate: &str,
        import: &shared::Function,
    ) {
        let mut dst = String::new();

        dst.push_str(&format!("function {}(", shim_name));

        let mut invocation = String::new();
        for (i, arg) in import.arguments.iter().enumerate() {
            if invocation.len() > 0 {
                invocation.push_str(", ");
            }
            if i > 0 {
                dst.push_str(", ");
            }
            match *arg {
                shared::Type::Number => {
                    invocation.push_str(&format!("arg{}", i));
                    dst.push_str(&format!("arg{}", i));
                }
                shared::Type::Boolean => {
                    invocation.push_str(&format!("arg{} != 0", i));
                    dst.push_str(&format!("arg{}", i));
                }
                shared::Type::BorrowedStr => {
                    self.expose_get_string_from_wasm();
                    invocation.push_str(&format!("getStringFromWasm(ptr{0}, len{0})", i));
                    dst.push_str(&format!("ptr{0}, len{0}", i));
                }
                shared::Type::JsObject => {
                    self.expose_take_object();
                    invocation.push_str(&format!("takeObject(arg{})", i));
                    dst.push_str(&format!("arg{}", i));
                }
                shared::Type::JsObjectRef => {
                    self.expose_get_object();
                    invocation.push_str(&format!("getObject(arg{})", i));
                    dst.push_str(&format!("arg{}", i));
                }
                shared::Type::String |
                shared::Type::ByRef(_) |
                shared::Type::ByMutRef(_) |
                shared::Type::ByValue(_) => {
                    panic!("unsupported type in import");
                }
            }
        }
        dst.push_str(")");
        let invoc = format!("{}({})", shim_delegate, invocation);
        let invoc = match import.ret {
            Some(shared::Type::Number) => invoc,
            Some(shared::Type::Boolean) => format!("{} ? 1 : 0", invoc),
            Some(shared::Type::JsObject) => {
                self.expose_add_heap_object();
                format!("addHeapObject({})", invoc)
            }
            None => invoc,
            _ => unimplemented!(),
        };
        dst.push_str(" {\n");
        dst.push_str(&format!("return {};\n}}", invoc));

        self.globals.push_str("export ");
        self.globals.push_str(&dst);
        self.globals.push_str("\n");
    }

    fn wasm_import_needed(&self, name: &str) -> bool {
        let imports = match self.module.import_section() {
            Some(s) => s,
            None => return false,
        };

        imports.entries().iter().any(|i| {
            i.module() == "env" && i.field() == name
        })
    }

    fn rewrite_imports(&mut self, module_name: &str) {
        for section in self.module.sections_mut() {
            let imports = match *section {
                Section::Import(ref mut s) => s,
                _ => continue,
            };
            for import in imports.entries_mut() {
                if import.module() != "env" {
                    continue
                }
                if import.field().starts_with("__wbindgen") {
                    import.module_mut().truncate(0);
                    import.module_mut().push_str("./");
                    import.module_mut().push_str(module_name);
                    continue
                }

                // rustc doesn't have support for importing from anything other
                // than the module `env` so let's use the metadata here to
                // rewrite the imports if they import from `env` until it's
                // fixed upstream.
                let program_import = self.program.imports
                    .iter()
                    .any(|&(_, ref f)| f.mangled_import_name(None) == import.field());
                let struct_import = self.program.imported_structs
                    .iter()
                    .flat_map(|s| s.functions.iter().map(move |f| (s, &f.1)))
                    .any(|(s, f)| f.mangled_import_name(Some(&s.name)) == import.field());
                if program_import || struct_import {
                    import.module_mut().truncate(0);
                    import.module_mut().push_str("./");
                    import.module_mut().push_str(module_name);
                    continue
                }
            }
        }
    }

    fn expose_drop_ref(&mut self) {
        if !self.exposed_globals.insert("drop_ref") {
            return
        }
        self.expose_global_slab();
        self.expose_global_slab_next();
        let validate_owned = if self.config.debug {
            String::from("
                if ((idx & 1) === 1)
                    throw new Error('cannot drop ref of stack objects');
            ")
        } else {
            String::new()
        };
        let dec_ref = if self.config.debug {
            String::from("
                if (typeof(obj) === 'number')
                    throw new Error('corrupt slab');
                obj.cnt -= 1;
                if (obj.cnt > 0)
                    return;
            ")
        } else {
            String::from("
                obj.cnt -= 1;
                if (obj.cnt > 0)
                    return;
            ")
        };
        self.globals.push_str(&format!("
            function dropRef(idx) {{
                {}

                let obj = slab[idx >> 1];
                {}

                // If we hit 0 then free up our space in the slab
                slab[idx >> 1] = slab_next;
                slab_next = idx >> 1;
            }}
        ", validate_owned, dec_ref));
    }

    fn expose_global_stack(&mut self) {
        if !self.exposed_globals.insert("stack") {
            return
        }
        self.globals.push_str(&format!("
            let stack = [];
        "));
    }

    fn expose_global_slab(&mut self) {
        if !self.exposed_globals.insert("slab") {
            return
        }
        self.globals.push_str(&format!("let slab = [];"));
    }

    fn expose_global_slab_next(&mut self) {
        if !self.exposed_globals.insert("slab_next") {
            return
        }
        self.globals.push_str(&format!("
            let slab_next = 0;
        "));
    }

    fn expose_get_object(&mut self) {
        if !self.exposed_globals.insert("get_object") {
            return
        }
        self.expose_global_stack();
        self.expose_global_slab();

        let get_obj = if self.config.debug {
            String::from("
                if (typeof(val) === 'number')
                    throw new Error('corrupt slab');
                return val.obj;
            ")
        } else {
            String::from("
                return val.obj;
            ")
        };
        self.globals.push_str(&format!("
            function getObject(idx) {{
                if ((idx & 1) === 1) {{
                    return stack[idx >> 1];
                }} else {{
                    const val = slab[idx >> 1];
                    {}
                }}
            }}
        ", get_obj));
    }

    fn expose_check_token(&mut self) {
        if !self.exposed_globals.insert("check_token") {
            return
        }
        self.globals.push_str(&format!("
            const token = Symbol('foo');
            function _checkToken(sym) {{
                if (token !== sym)
                    throw new Error('cannot invoke `new` directly');
            }}
        "));
    }

    fn expose_assert_num(&mut self) {
        if !self.exposed_globals.insert("assert_num") {
            return
        }
        self.globals.push_str(&format!("
            function _assertNum(n) {{
                if (typeof(n) !== 'number')
                    throw new Error('expected a number argument');
            }}
        "));
    }

    fn expose_assert_bool(&mut self) {
        if !self.exposed_globals.insert("assert_bool") {
            return
        }
        self.globals.push_str(&format!("
            function _assertBoolean(n) {{
                if (typeof(n) !== 'boolean')
                    throw new Error('expected a boolean argument');
            }}
        "));
    }

    fn expose_pass_string_to_wasm(&mut self) {
        if !self.exposed_globals.insert("pass_string_to_wasm") {
            return
        }
        if self.config.nodejs {
            self.globals.push_str(&format!("
                function passStringToWasm(arg) {{
                    if (typeof(arg) !== 'string')
                        throw new Error('expected a string argument');
                    const buf = Buffer.from(arg);
                    const len = buf.length;
                    const ptr = wasm.__wbindgen_malloc(len);
                    buf.copy(Buffer.from(wasm.memory.buffer), ptr);
                    return [ptr, len];
                }}
            "));
        } else {
            self.globals.push_str(&format!("
                function passStringToWasm(arg) {{
                    if (typeof(arg) !== 'string')
                        throw new Error('expected a string argument');
                    const buf = new TextEncoder('utf-8').encode(arg);
                    const len = buf.length;
                    const ptr = wasm.__wbindgen_malloc(len);
                    let array = new Uint8Array(wasm.memory.buffer);
                    array.set(buf, ptr);
                    return [ptr, len];
                }}
            "));
        }
    }

    fn expose_get_string_from_wasm(&mut self) {
        if !self.exposed_globals.insert("get_string_from_wasm") {
            return
        }
        if self.config.nodejs {
            self.globals.push_str(&format!("
                function getStringFromWasm(ptr, len) {{
                    const buf = Buffer.from(wasm.memory.buffer).slice(ptr, ptr + len);
                    const ret = buf.toString();
                    return ret;
                }}
            "));
        } else {
            self.globals.push_str(&format!("
                function getStringFromWasm(ptr, len) {{
                    const mem = new Uint8Array(wasm.memory.buffer);
                    const slice = mem.slice(ptr, ptr + len);
                    const ret = new TextDecoder('utf-8').decode(slice);
                    return ret;
                }}
            "));
        }
    }

    fn expose_assert_class(&mut self) {
        if !self.exposed_globals.insert("assert_class") {
            return
        }
        self.globals.push_str(&format!("
            function _assertClass(instance, klass) {{
                if (!(instance instanceof klass))
                    throw new Error(`expected instance of ${{klass.name}}`);
                return instance.ptr;
            }}
        "));
    }

    fn expose_borrowed_objects(&mut self) {
        if !self.exposed_globals.insert("borrowed_objects") {
            return
        }
        self.expose_global_stack();
        self.globals.push_str(&format!("
            function addBorrowedObject(obj) {{
                stack.push(obj);
                return ((stack.length - 1) << 1) | 1;
            }}
        "));
    }

    fn expose_take_object(&mut self) {
        if !self.exposed_globals.insert("take_object") {
            return
        }
        self.expose_get_object();
        self.expose_drop_ref();
        self.globals.push_str(&format!("
            function takeObject(idx) {{
                const ret = getObject(idx);
                dropRef(idx);
                return ret;
            }}
        "));
    }

    fn expose_add_heap_object(&mut self) {
        if !self.exposed_globals.insert("add_heap_object") {
            return
        }
        self.expose_global_slab();
        self.expose_global_slab_next();
        let set_slab_next = if self.config.debug {
            String::from("
                if (typeof(next) !== 'number')
                    throw new Error('corrupt slab');
                slab_next = next;
            ")
        } else {
            String::from("
                slab_next = next;
            ")
        };
        self.globals.push_str(&format!("
            function addHeapObject(obj) {{
                if (slab_next == slab.length)
                    slab.push(slab.length + 1);
                const idx = slab_next;
                const next = slab[idx];
                {}
                slab[idx] = {{ obj, cnt: 1 }};
                return idx << 1;
            }}
        ", set_slab_next));
    }
}
