use std::char;
use std::collections::{HashSet, HashMap};

use shared;
use parity_wasm::elements::*;

use super::Bindgen;

pub struct Context<'a> {
    pub globals: String,
    pub imports: String,
    pub typescript: String,
    pub exposed_globals: HashSet<&'static str>,
    pub required_internal_exports: HashSet<&'static str>,
    pub config: &'a Bindgen,
    pub module: &'a mut Module,
    pub imports_to_rewrite: HashSet<String>,
    pub custom_type_names: HashMap<char, String>,
}

pub struct SubContext<'a, 'b: 'a> {
    pub program: &'a shared::Program,
    pub cx: &'a mut Context<'b>,
}

impl<'a> Context<'a> {
    pub fn add_custom_type_names(&mut self, program: &shared::Program) {
        for custom in program.custom_type_names.iter() {
            assert!(self.custom_type_names.insert(custom.descriptor,
                                                  custom.name.clone()).is_none());
            let val = custom.descriptor as u32;
            assert!(val & 1 == 0);
            let descriptor = char::from_u32(val | 1).unwrap();
            assert!(self.custom_type_names.insert(descriptor,
                                                  custom.name.clone()).is_none());
        }
    }

    pub fn finalize(&mut self, module_name: &str) -> (String, String) {
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
                me.expose_uint8_memory();
                format!("
                    function(n, invalid) {{
                        let obj = getObject(n);
                        if (typeof(obj) === 'number')
                            return obj;
                        getUint8Memory()[invalid] = 1;
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
                me.expose_uint32_memory();
                String::from("(i, len_ptr) => {
                    let obj = getObject(i);
                    if (typeof(obj) !== 'string')
                        return 0;
                    const [ptr, len] = passStringToWasm(obj);
                    getUint32Memory()[len_ptr / 4] = len;
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
        self.unexport_unused_internal_exports();

        (js, self.typescript.clone())
    }

    fn rewrite_imports(&mut self, module_name: &str) {
        for section in self.module.sections_mut() {
            let imports = match *section {
                Section::Import(ref mut s) => s,
                _ => continue,
            };
            for import in imports.entries_mut() {
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
                if self.imports_to_rewrite.contains(import.field()) {
                    import.module_mut().truncate(0);
                    import.module_mut().push_str("./");
                    import.module_mut().push_str(module_name);
                    continue
                }
            }
        }
    }

    fn unexport_unused_internal_exports(&mut self) {
        let required = &self.required_internal_exports;
        for section in self.module.sections_mut() {
            let exports = match *section {
                Section::Export(ref mut s) => s,
                _ => continue,
            };
            exports.entries_mut().retain(|export| {
                !export.field().starts_with("__wbindgen") ||
                    required.contains(export.field())
            });
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
        self.required_internal_exports.insert("__wbindgen_malloc");
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
            self.expose_text_encoder();
            self.expose_uint8_memory();
            self.globals.push_str(&format!("
                function passStringToWasm(arg) {{
                    if (typeof(arg) !== 'string')
                        throw new Error('expected a string argument');
                    const buf = textEncoder().encode(arg);
                    const len = buf.length;
                    const ptr = wasm.__wbindgen_malloc(len);
                    getUint8Memory().set(buf, ptr);
                    return [ptr, len];
                }}
            "));
        }
    }

    fn expose_text_encoder(&mut self) {
        if !self.exposed_globals.insert("text_encoder") {
            return
        }
        self.globals.push_str(&format!("
            let cachedEncoder = null;
            function textEncoder() {{
                if (cachedEncoder)
                    return cachedEncoder;
                cachedEncoder = new TextEncoder('utf-8');
                return cachedEncoder;
            }}
        "));
    }

    fn expose_text_decoder(&mut self) {
        if !self.exposed_globals.insert("text_decoder") {
            return
        }
        self.globals.push_str(&format!("
            let cachedDecoder = null;
            function textDecoder() {{
                if (cachedDecoder)
                    return cachedDecoder;
                cachedDecoder = new TextDecoder('utf-8');
                return cachedDecoder;
            }}
        "));
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
            self.expose_text_decoder();
            self.expose_uint8_memory();
            self.globals.push_str(&format!("
                function getStringFromWasm(ptr, len) {{
                    const mem = getUint8Memory();
                    const slice = mem.slice(ptr, ptr + len);
                    const ret = textDecoder().decode(slice);
                    return ret;
                }}
            "));
        }
    }

    fn expose_uint8_memory(&mut self) {
        if !self.exposed_globals.insert("uint8_memory") {
            return
        }
        self.globals.push_str(&format!("
            let cachedUint8Memory = null;
            function getUint8Memory() {{
                if (cachedUint8Memory === null ||
                    cachedUint8Memory.buffer !== wasm.memory.buffer)
                    cachedUint8Memory = new Uint8Array(wasm.memory.buffer);
                return cachedUint8Memory;
            }}
        "));
    }

    fn expose_uint32_memory(&mut self) {
        if !self.exposed_globals.insert("uint32_memory") {
            return
        }
        self.globals.push_str(&format!("
            let cachedUint32Memory = null;
            function getUint32Memory() {{
                if (cachedUint32Memory === null ||
                    cachedUint32Memory.buffer !== wasm.memory.buffer)
                    cachedUint32Memory = new Uint32Array(wasm.memory.buffer);
                return cachedUint32Memory;
            }}
        "));
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

    fn wasm_import_needed(&self, name: &str) -> bool {
        let imports = match self.module.import_section() {
            Some(s) => s,
            None => return false,
        };

        imports.entries().iter().any(|i| {
            i.module() == "env" && i.field() == name
        })
    }
}

impl<'a, 'b> SubContext<'a, 'b> {
    pub fn generate(&mut self) {
        for f in self.program.free_functions.iter() {
            self.generate_free_function(f);
        }
        for f in self.program.imports.iter() {
            self.generate_import(&f.module, &f.function);
        }
        for s in self.program.structs.iter() {
            self.generate_struct(s);
        }
        for s in self.program.imported_structs.iter() {
            self.generate_import_struct(s);
        }
    }

    pub fn generate_free_function(&mut self, func: &shared::Function) {
        let (js, ts) = self.generate_function("function",
                                              &func.name,
                                              &func.name,
                                              false,
                                              &func.arguments,
                                              func.ret.as_ref());
        self.cx.globals.push_str("export ");
        self.cx.globals.push_str(&js);
        self.cx.globals.push_str("\n");
        self.cx.typescript.push_str("export ");
        self.cx.typescript.push_str(&ts);
        self.cx.typescript.push_str("\n");
    }

    pub fn generate_struct(&mut self, s: &shared::Struct) {
        let mut dst = String::new();
        dst.push_str(&format!("export class {} {{", s.name));
        let mut ts_dst = dst.clone();
        ts_dst.push_str("
            public ptr: number;
        ");
        if self.cx.config.debug {
            self.cx.expose_check_token();
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
        ", shared::free_function(&s.name)));
        ts_dst.push_str("free(): void;\n");

        for function in s.functions.iter() {
            let (js, ts) = self.generate_function(
                "static",
                &function.name,
                &shared::struct_function_export_name(&s.name, &function.name),
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
                &shared::struct_function_export_name(&s.name, &method.function.name),
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

        self.cx.globals.push_str(&dst);
        self.cx.globals.push_str("\n");
        self.cx.typescript.push_str(&ts_dst);
        self.cx.typescript.push_str("\n");
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
                shared::TYPE_NUMBER => {
                    dst_ts.push_str(": number");
                    if self.cx.config.debug {
                        self.cx.expose_assert_num();
                        arg_conversions.push_str(&format!("_assertNum({});\n", name));
                    }
                    pass(&name)
                }
                shared::TYPE_BOOLEAN => {
                    dst_ts.push_str(": boolean");
                    if self.cx.config.debug {
                        self.cx.expose_assert_bool();
                        arg_conversions.push_str(&format!("\
                            _assertBoolean({name});
                        ", name = name));
                    } else {
                    }
                    pass(&format!("arg{i} ? 1 : 0", i = i))
                }
                shared::TYPE_BORROWED_STR |
                shared::TYPE_STRING => {
                    dst_ts.push_str(": string");
                    self.cx.expose_pass_string_to_wasm();
                    arg_conversions.push_str(&format!("\
                        const [ptr{i}, len{i}] = passStringToWasm({arg});
                    ", i = i, arg = name));
                    pass(&format!("ptr{}", i));
                    pass(&format!("len{}", i));
                    if *arg == shared::TYPE_BORROWED_STR {
                        destructors.push_str(&format!("\n\
                            wasm.__wbindgen_free(ptr{i}, len{i});\n\
                        ", i = i));
                        self.cx.required_internal_exports.insert("__wbindgen_free");
                    }
                }
                shared::TYPE_JS_OWNED => {
                    dst_ts.push_str(": any");
                    self.cx.expose_add_heap_object();
                    arg_conversions.push_str(&format!("\
                        const idx{i} = addHeapObject({arg});
                    ", i = i, arg = name));
                    pass(&format!("idx{}", i));
                }
                shared::TYPE_JS_REF => {
                    dst_ts.push_str(": any");
                    self.cx.expose_borrowed_objects();
                    arg_conversions.push_str(&format!("\
                        const idx{i} = addBorrowedObject({arg});
                    ", i = i, arg = name));
                    destructors.push_str("stack.pop();\n");
                    pass(&format!("idx{}", i));
                }
                custom if (custom as u32) & shared::TYPE_CUSTOM_REF_FLAG != 0 => {
                    let s = self.cx.custom_type_names[&custom].clone();
                    dst_ts.push_str(&format!(": {}", s));
                    if self.cx.config.debug {
                        self.cx.expose_assert_class();
                        arg_conversions.push_str(&format!("\
                            _assertClass({arg}, {struct_});
                        ", arg = name, struct_ = s));
                    }
                    pass(&format!("{}.ptr", name));
                }
                custom => {
                    let s = self.cx.custom_type_names[&custom].clone();
                    dst_ts.push_str(&format!(": {}", s));
                    if self.cx.config.debug {
                        self.cx.expose_assert_class();
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
            }
        }
        dst.push_str(")");
        dst_ts.push_str(")");
        let convert_ret = match ret {
            None => {
                dst_ts.push_str(": void");
                format!("return ret;")
            }
            Some(&shared::TYPE_NUMBER) => {
                dst_ts.push_str(": number");
                format!("return ret;")
            }
            Some(&shared::TYPE_BOOLEAN) => {
                dst_ts.push_str(": boolean");
                format!("return ret != 0;")
            }
            Some(&shared::TYPE_JS_OWNED) => {
                dst_ts.push_str(": any");
                self.cx.expose_take_object();
                format!("return takeObject(ret);")
            }
            Some(&shared::TYPE_STRING) => {
                dst_ts.push_str(": string");
                self.cx.expose_get_string_from_wasm();
                self.cx.required_internal_exports.insert("__wbindgen_boxed_str_ptr");
                self.cx.required_internal_exports.insert("__wbindgen_boxed_str_len");
                self.cx.required_internal_exports.insert("__wbindgen_boxed_str_free");
                format!("
                    const ptr = wasm.__wbindgen_boxed_str_ptr(ret);
                    const len = wasm.__wbindgen_boxed_str_len(ret);
                    const realRet = getStringFromWasm(ptr, len);
                    wasm.__wbindgen_boxed_str_free(ret);
                    return realRet;
                ")
            }
            Some(&shared::TYPE_JS_REF) |
            Some(&shared::TYPE_BORROWED_STR) => panic!(),
            Some(&t) if (t as u32) & shared::TYPE_CUSTOM_REF_FLAG != 0 => panic!(),
            Some(custom) => {
                let name = &self.cx.custom_type_names[custom];
                dst_ts.push_str(": ");
                dst_ts.push_str(name);
                if self.cx.config.debug {
                    format!("\
                        return new {name}(ret, token);
                    ", name = name)
                } else {
                    format!("\
                        return new {name}(ret);
                    ", name = name)
                }
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
        let imported_name = format!("import{}", self.cx.imports.len());

        self.cx.imports.push_str(&format!("
            import {{ {} as {} }} from '{}';
        ", import.name, imported_name, module));

        let name = shared::mangled_import_name(None, &import.name);
        self.gen_import_shim(&name,
                             &imported_name,
                             false,
                             import);
        self.cx.imports_to_rewrite.insert(name);
    }

    pub fn generate_import_struct(&mut self, import: &shared::ImportStruct) {
        if let Some(ref module) = import.module {
            self.cx.imports.push_str(&format!("
                import {{ {} }} from '{}';
            ", import.name, module));
        }

        for f in import.functions.iter() {
            self.generate_import_struct_function(&import.name, f);
        }
    }

    fn generate_import_struct_function(
        &mut self,
        class: &str,
        f: &shared::ImportStructFunction,
    ) {
        let delegate = if f.method {
            format!("{}.prototype.{}.call", class, f.function.name)
        } else if f.js_new {
            format!("new {}", class)
        } else {
            format!("{}.{}", class, f.function.name)
        };

        let name = shared::mangled_import_name(Some(class), &f.function.name);
        self.gen_import_shim(&name,
                             &delegate,
                             f.method,
                             &f.function);
        self.cx.imports_to_rewrite.insert(name);
    }

    fn gen_import_shim(
        &mut self,
        shim_name: &str,
        shim_delegate: &str,
        is_method: bool,
        import: &shared::Function,
    ) {
        let mut dst = String::new();

        dst.push_str(&format!("function {}(", shim_name));
        let mut invocation = String::new();

        if is_method {
            dst.push_str("ptr");
            invocation.push_str("getObject(ptr)");
            self.cx.expose_get_object();
        }

        let mut extra = String::new();

        for (i, arg) in import.arguments.iter().enumerate() {
            if invocation.len() > 0 {
                invocation.push_str(", ");
            }
            if i > 0 || is_method {
                dst.push_str(", ");
            }
            match *arg {
                shared::TYPE_NUMBER => {
                    invocation.push_str(&format!("arg{}", i));
                    dst.push_str(&format!("arg{}", i));
                }
                shared::TYPE_BOOLEAN => {
                    invocation.push_str(&format!("arg{} != 0", i));
                    dst.push_str(&format!("arg{}", i));
                }
                shared::TYPE_BORROWED_STR => {
                    self.cx.expose_get_string_from_wasm();
                    invocation.push_str(&format!("getStringFromWasm(ptr{0}, len{0})", i));
                    dst.push_str(&format!("ptr{0}, len{0}", i));
                }
                shared::TYPE_STRING => {
                    self.cx.expose_get_string_from_wasm();
                    dst.push_str(&format!("ptr{0}, len{0}", i));
                    extra.push_str(&format!("
                        let arg{0} = getStringFromWasm(ptr{0}, len{0});
                        wasm.__wbindgen_free(ptr{0}, len{0});
                    ", i));
                    invocation.push_str(&format!("arg{}", i));
                    self.cx.required_internal_exports.insert("__wbindgen_free");
                }
                shared::TYPE_JS_OWNED => {
                    self.cx.expose_take_object();
                    invocation.push_str(&format!("takeObject(arg{})", i));
                    dst.push_str(&format!("arg{}", i));
                }
                shared::TYPE_JS_REF => {
                    self.cx.expose_get_object();
                    invocation.push_str(&format!("getObject(arg{})", i));
                    dst.push_str(&format!("arg{}", i));
                }
                _ => {
                    panic!("unsupported type in import");
                }
            }
        }
        let invoc = format!("{}({})", shim_delegate, invocation);
        let invoc = match import.ret {
            Some(shared::TYPE_NUMBER) => format!("return {};", invoc),
            Some(shared::TYPE_BOOLEAN) => format!("return {} ? 1 : 0;", invoc),
            Some(shared::TYPE_JS_OWNED) => {
                self.cx.expose_add_heap_object();
                format!("return addHeapObject({});", invoc)
            }
            Some(shared::TYPE_STRING) => {
                self.cx.expose_pass_string_to_wasm();
                self.cx.expose_uint32_memory();
                if import.arguments.len() > 0 || is_method {
                    dst.push_str(", ");
                }
                dst.push_str("wasmretptr");
                format!("
                    const [retptr, retlen] = passStringToWasm({});
                    getUint32Memory()[wasmretptr / 4] = retlen;
                    return retptr;
                ", invoc)
            }
            None => invoc,
            _ => unimplemented!(),
        };
        dst.push_str(") {\n");
        dst.push_str(&extra);
        dst.push_str(&format!("{}\n}}", invoc));

        self.cx.globals.push_str("export ");
        self.cx.globals.push_str(&dst);
        self.cx.globals.push_str("\n");
    }
}
