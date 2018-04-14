use std::collections::{HashSet, HashMap};
use std::fmt::Write;
use std::mem;

use parity_wasm::elements::*;
use parity_wasm;
use shared;
use wasm_gc;

use super::Bindgen;
use descriptor::{Descriptor, VectorKind, Function};

pub struct Context<'a> {
    pub globals: String,
    pub imports: String,
    pub footer: String,
    pub typescript: String,
    pub exposed_globals: HashSet<&'static str>,
    pub required_internal_exports: HashSet<&'static str>,
    pub config: &'a Bindgen,
    pub module: &'a mut Module,
    pub imported_names: HashSet<String>,
    pub exported_classes: HashMap<String, ExportedClass>,
    pub function_table_needed: bool,
    pub run_descriptor: &'a Fn(&str) -> Vec<u32>,
}

#[derive(Default)]
pub struct ExportedClass {
    pub contents: String,
    pub typescript: String,
}

pub struct SubContext<'a, 'b: 'a> {
    pub program: &'a shared::Program,
    pub cx: &'a mut Context<'b>,
}

impl<'a> Context<'a> {
    fn export(&mut self, name: &str, contents: &str) {
        let contents = contents.trim();
        let global = if self.config.nodejs {
            format!("module.exports.{} = {};\n", name, contents)
        } else if self.config.no_modules {
            format!("__exports.{} = {}\n", name, contents)
        } else {
            if contents.starts_with("function") {
                format!("export function {} {}\n", name, &contents[8..])
            } else if contents.starts_with("class") {
                format!("export {}\n", contents)
            } else {
                format!("export const {} = {};\n", name, contents)
            }
        };
        self.globals.push_str(&global);
    }

    pub fn finalize(&mut self, module_name: &str) -> (String, String) {
        self.unexport_unused_internal_exports();
        self.gc();
        self.write_classes();
        {
            let mut bind = |name: &str, f: &Fn(&mut Self) -> String| {
                if !self.wasm_import_needed(name) {
                    return;
                }
                let contents = f(self);
                let contents = contents.trim();
                self.export(name, contents);
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
                "function(i) { dropRef(i); }".to_string()
            });

            bind("__wbindgen_string_new", &|me| {
                me.expose_add_heap_object();
                me.expose_get_string_from_wasm();
                String::from("function(p, l) {
                    return addHeapObject(getStringFromWasm(p, l));
                }")
            });

            bind("__wbindgen_number_new", &|me| {
                me.expose_add_heap_object();
                String::from("function(i) { return addHeapObject(i); }")
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
                String::from("function() { return addHeapObject(undefined); }")
            });

            bind("__wbindgen_null_new", &|me| {
                me.expose_add_heap_object();
                String::from("function() {
                    return addHeapObject(null);
                }")
            });

            bind("__wbindgen_is_null", &|me| {
                me.expose_get_object();
                String::from("function(idx) {
                    return getObject(idx) === null ? 1 : 0;
                }")
            });

            bind("__wbindgen_is_undefined", &|me| {
                me.expose_get_object();
                String::from("function(idx) {
                    return getObject(idx) === undefined ? 1 : 0;
                }")
            });

            bind("__wbindgen_boolean_new", &|me| {
                me.expose_add_heap_object();
                String::from("function(v) {
                    return addHeapObject(v === 1);
                }")
            });

            bind("__wbindgen_boolean_get", &|me| {
                me.expose_get_object();
                String::from("function(i) {
                    let v = getObject(i);
                    if (typeof(v) === 'boolean') {
                        return v ? 1 : 0;
                    } else {
                        return 2;
                    }
                }")
            });

            bind("__wbindgen_symbol_new", &|me| {
                me.expose_get_string_from_wasm();
                me.expose_add_heap_object();
                format!("function(ptr, len) {{
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
                String::from("function(i) {
                    return typeof(getObject(i)) === 'symbol' ? 1 : 0;
                }")
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
                String::from("function(i, len_ptr) {
                    let obj = getObject(i);
                    if (typeof(obj) !== 'string')
                        return 0;
                    const [ptr, len] = passStringToWasm(obj);
                    getUint32Memory()[len_ptr / 4] = len;
                    return ptr;
                }")
            });

            for i in 0..8 {
                let name = format!("__wbindgen_cb_arity{}", i);
                bind(&name, &|me| {
                    me.expose_add_heap_object();
                    me.function_table_needed = true;
                    let args = (0..i)
                        .map(|x| format!("arg{}", x))
                        .collect::<Vec<_>>()
                        .join(", ");
                    format!("function(a, b, c) {{
                        const cb = function({0}) {{
                            return this.f(this.a, this.b {1} {0});
                        }};
                        cb.a = b;
                        cb.b = c;
                        cb.f = wasm.__wbg_function_table.get(a);
                        let real = cb.bind(cb);
                        real.original = cb;
                        return addHeapObject(real);
                    }}", args, if i == 0 {""} else {","})
                });
            }
            bind("__wbindgen_cb_drop", &|me| {
                me.expose_drop_ref();
                String::from("function(i) {
                    let obj = getObject(i).original;
                    obj.a = obj.b = 0;
                    dropRef(i);
                }")
            });
            bind("__wbindgen_cb_forget", &|me| {
                me.expose_drop_ref();
                String::from("function(i) {
                    dropRef(i);
                }")
            });
        }

        self.rewrite_imports(module_name);

        let js = if self.config.no_modules {
            format!("
                (function() {{
                    var wasm;
                    const __exports = {{}};
                    {globals}
                    function init(wasm_path) {{
                        return fetch(wasm_path)
                            .then(response => response.arrayBuffer())
                            .then(buffer => WebAssembly.instantiate(buffer, {{ './{module}': __exports }}))
                            .then(({{instance}}) => {{
                                wasm = init.wasm = instance.exports;
                                return;
                            }});
                    }};
                    window.wasm_bindgen = Object.assign(init, __exports);
                }})();
            ",
                    globals = self.globals,
                    module = module_name,
            )
        } else {
            let import_wasm = if self.config.nodejs {
                self.footer.push_str(&format!("wasm = require('./{}_bg');",
                                              module_name));
                format!("var wasm;")
            } else {
                format!("import * as wasm from './{}_bg';", module_name)
            };

            format!("
                /* tslint:disable */
                {import_wasm}
                {imports}

                {globals}
                {footer}",
                    import_wasm = import_wasm,
                    globals = self.globals,
                    imports = self.imports,
                    footer = self.footer,
            )
        };

        self.export_table();
        self.gc();

        (js, self.typescript.clone())
    }

    fn write_classes(&mut self) {
        let classes = mem::replace(&mut self.exported_classes, Default::default());
        for (class, exports) in classes {
            let mut dst = format!("class {} {{\n", class);
            let mut ts_dst = format!("export {}", dst);
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

                let new_name = shared::new_function(&class);
                if self.wasm_import_needed(&new_name) {
                    self.expose_add_heap_object();
                    self.export(&new_name, &format!("
                        function(ptr) {{
                            return addHeapObject(new {class}(ptr, token));
                        }}
                    ", class = class));
                }
            } else {
                dst.push_str(&format!("
                    constructor(ptr) {{
                        this.ptr = ptr;
                    }}
                "));
                ts_dst.push_str("constructor(ptr: number);\n");

                let new_name = shared::new_function(&class);
                if self.wasm_import_needed(&new_name) {
                    self.expose_add_heap_object();
                    self.export(&new_name, &format!("
                        function(ptr) {{
                            return addHeapObject(new {class}(ptr));
                        }}
                    ", class = class));
                }
            }

            dst.push_str(&format!("
                free() {{
                    const ptr = this.ptr;
                    this.ptr = 0;
                    wasm.{}(ptr);
                }}
            ", shared::free_function(&class)));
            ts_dst.push_str("free(): void;\n");

            dst.push_str(&exports.contents);
            ts_dst.push_str(&exports.typescript);
            dst.push_str("}\n");
            ts_dst.push_str("}\n");

            self.export(&class, &dst);
            self.typescript.push_str(&ts_dst);
        }
    }

    fn export_table(&mut self) {
        if !self.function_table_needed {
            return
        }
        for section in self.module.sections_mut() {
            let exports = match *section {
                Section::Export(ref mut s) => s,
                _ => continue,
            };
            let entry = ExportEntry::new("__wbg_function_table".to_string(),
                                         Internal::Table(0));
            exports.entries_mut().push(entry);
            break
        }
    }

    fn rewrite_imports(&mut self, module_name: &str) {
        for (name, contents) in self._rewrite_imports(module_name) {
            self.export(&name, &contents);
        }
    }

    fn _rewrite_imports(&mut self, module_name: &str)
                        -> Vec<(String, String)>
    {
        let mut math_imports = Vec::new();
        let imports = self.module.sections_mut()
            .iter_mut()
            .filter_map(|s| {
                match *s {
                    Section::Import(ref mut s) => Some(s),
                    _ => None,
                }
            })
            .flat_map(|s| s.entries_mut());

        for import in imports {
            if import.module() == "__wbindgen_placeholder__" {
                import.module_mut().truncate(0);
                import.module_mut().push_str("./");
                import.module_mut().push_str(module_name);
                continue;
            }

            if import.module() != "env" {
                continue;
            }

            let renamed_import = format!("__wbindgen_{}", import.field());
            let mut bind_math = |expr: &str| {
                math_imports.push((
                    renamed_import.clone(),
                    format!("function{}", expr),
                ));
            };

            // FIXME(#32): try to not use function shims
            match import.field() {
                "Math_acos" => bind_math("(x) { return Math.acos(x); }"),
                "Math_asin" => bind_math("(x) { return Math.asin(x); }"),
                "Math_atan" => bind_math("(x) { return Math.atan(x); }"),
                "Math_atan2" => bind_math("(x, y) { return Math.atan2(x, y); }"),
                "Math_cbrt" => bind_math("(x) { return Math.cbrt(x); }"),
                "Math_cosh" => bind_math("(x) { return Math.cosh(x); }"),
                "Math_expm1" => bind_math("(x) { return Math.expm1(x); }"),
                "Math_hypot" => bind_math("(x, y) { return Math.hypot(x, y); }"),
                "Math_log1p" => bind_math("(x) { return Math.log1p(x); }"),
                "Math_sinh" => bind_math("(x) { return Math.sinh(x); }"),
                "Math_tan" => bind_math("(x) { return Math.tan(x); }"),
                "Math_tanh" => bind_math("(x) { return Math.tanh(x); }"),
                "cos" => bind_math("(x) { return Math.cos(x); }"),
                "cosf" => bind_math("(x) { return Math.cos(x); }"),
                "exp" => bind_math("(x) { return Math.exp(x); }"),
                "expf" => bind_math("(x) { return Math.exp(x); }"),
                "log2" => bind_math("(x) { return Math.log2(x); }"),
                "log2f" => bind_math("(x) { return Math.log2(x); }"),
                "log10" => bind_math("(x) { return Math.log10(x); }"),
                "log10f" => bind_math("(x) { return Math.log10(x); }"),
                "log" => bind_math("(x) { return Math.log(x); }"),
                "logf" => bind_math("(x) { return Math.log(x); }"),
                "round" => bind_math("(x) { return Math.round(x); }"),
                "roundf" => bind_math("(x) { return Math.round(x); }"),
                "sin" => bind_math("(x) { return Math.sin(x); }"),
                "sinf" => bind_math("(x) { return Math.sin(x); }"),
                "pow" => bind_math("(x, y) { return Math.pow(x, y); }"),
                "powf" => bind_math("(x, y) { return Math.pow(x, y); }"),
                "exp2" => bind_math("(a) { return Math.pow(2, a); }"),
                "exp2f" => bind_math("(a) { return Math.pow(2, a); }"),
                "fmod" => bind_math("(a, b) { return a % b; }"),
                "fmodf" => bind_math("(a, b) { return a % b; }"),
                "fma" => bind_math("(a, b, c) { return (a * b) + c; }"),
                "fmaf" => bind_math("(a, b, c) { return (a * b) + c; }"),
                _ => continue,
            }

            import.module_mut().truncate(0);
            import.module_mut().push_str("./");
            import.module_mut().push_str(module_name);
            *import.field_mut() = renamed_import.clone();
        }

        math_imports
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
            return;
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
            return;
        }
        self.globals.push_str(&format!("
            let stack = [];
        "));
    }

    fn expose_global_slab(&mut self) {
        if !self.exposed_globals.insert("slab") {
            return;
        }
        self.globals.push_str(&format!("let slab = [];"));
    }

    fn expose_global_slab_next(&mut self) {
        if !self.exposed_globals.insert("slab_next") {
            return;
        }
        self.globals.push_str(&format!("
            let slab_next = 0;
        "));
    }

    fn expose_get_object(&mut self) {
        if !self.exposed_globals.insert("get_object") {
            return;
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
            return;
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
            return;
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
            return;
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
            return;
        }
        self.required_internal_exports.insert("__wbindgen_malloc");
        self.expose_text_encoder();
        self.expose_uint8_memory();
        let debug = if self.config.debug {
            "
                if (typeof(arg) !== 'string')
                    throw new Error('expected a string argument');
            "
        } else {
            ""
        };
        self.globals.push_str(&format!("
            function passStringToWasm(arg) {{
                {}
                const buf = cachedEncoder.encode(arg);
                const ptr = wasm.__wbindgen_malloc(buf.length);
                getUint8Memory().set(buf, ptr);
                return [ptr, buf.length];
            }}
        ", debug));
    }

    fn expose_pass_array8_to_wasm(&mut self) {
        if !self.exposed_globals.insert("pass_array8_to_wasm") {
            return;
        }
        self.required_internal_exports.insert("__wbindgen_malloc");
        self.expose_uint8_memory();
        self.globals.push_str(&format!("
            function passArray8ToWasm(arg) {{
                const ptr = wasm.__wbindgen_malloc(arg.byteLength);
                getUint8Memory().set(arg, ptr);
                return [ptr, arg.length];
            }}
        "));
    }

    fn expose_pass_array16_to_wasm(&mut self) {
        if !self.exposed_globals.insert("pass_array16_to_wasm") {
            return;
        }
        self.required_internal_exports.insert("__wbindgen_malloc");
        self.expose_uint16_memory();
        self.globals.push_str(&format!("
            function passArray16ToWasm(arg) {{
                const ptr = wasm.__wbindgen_malloc(arg.byteLength);
                getUint16Memory().set(arg, ptr / 2);
                return [ptr, arg.length];
            }}
        "));
    }

    fn expose_pass_array32_to_wasm(&mut self) {
        if !self.exposed_globals.insert("pass_array32_to_wasm") {
            return;
        }
        self.required_internal_exports.insert("__wbindgen_malloc");
        self.expose_uint32_memory();
        self.globals.push_str(&format!("
            function passArray32ToWasm(arg) {{
                const ptr = wasm.__wbindgen_malloc(arg.byteLength);
                getUint32Memory().set(arg, ptr / 4);
                return [ptr, arg.length];
            }}
        "));
    }

    fn expose_pass_array_f32_to_wasm(&mut self) {
        if !self.exposed_globals.insert("pass_array_f32_to_wasm") {
            return;
        }
        self.required_internal_exports.insert("__wbindgen_malloc");
        self.globals.push_str(&format!("
            function passArrayF32ToWasm(arg) {{
                const ptr = wasm.__wbindgen_malloc(arg.byteLength);
                new Float32Array(wasm.memory.buffer).set(arg, ptr / 4);
                return [ptr, arg.length];
            }}
        "));
    }

    fn expose_pass_array_f64_to_wasm(&mut self) {
        if !self.exposed_globals.insert("pass_array_f64_to_wasm") {
            return;
        }
        self.required_internal_exports.insert("__wbindgen_malloc");
        self.globals.push_str(&format!("
            function passArrayF64ToWasm(arg) {{
                const ptr = wasm.__wbindgen_malloc(arg.byteLength);
                new Float64Array(wasm.memory.buffer).set(arg, ptr / 8);
                return [ptr, arg.length];
            }}
        "));
    }

    fn expose_text_encoder(&mut self) {
        if !self.exposed_globals.insert("text_encoder") {
            return;
        }
        if self.config.nodejs {
            self.globals.push_str(&format!("
                const TextEncoder = require('util').TextEncoder;
            "));
        } else if !(self.config.browser || self.config.no_modules) {
            self.globals.push_str(&format!("
                const TextEncoder = typeof window === 'object' && window.TextEncoder
                    ? window.TextEncoder
                    : require('util').TextEncoder;
            "));
        }
        self.globals.push_str(&format!("
            let cachedEncoder = new TextEncoder('utf-8');
        "));
    }

    fn expose_text_decoder(&mut self) {
        if !self.exposed_globals.insert("text_decoder") {
            return;
        }
        if self.config.nodejs {
            self.globals.push_str(&format!("
                const TextDecoder = require('util').TextDecoder;
            "));
        } else if !(self.config.browser || self.config.no_modules) {
            self.globals.push_str(&format!("
                const TextDecoder = typeof window === 'object' && window.TextDecoder
                    ? window.TextDecoder
                    : require('util').TextDecoder;
            "));
        }
        self.globals.push_str(&format!("
            let cachedDecoder = new TextDecoder('utf-8');
        "));
    }

    fn expose_get_string_from_wasm(&mut self) {
        if !self.exposed_globals.insert("get_string_from_wasm") {
            return;
        }
        self.expose_text_decoder();
        self.expose_uint8_memory();
        self.globals.push_str(&format!("
            function getStringFromWasm(ptr, len) {{
                return cachedDecoder.decode(getUint8Memory().slice(ptr, ptr + len));
            }}
        "));
    }

    fn expose_get_array_js_value_from_wasm(&mut self) {
        if !self.exposed_globals.insert("get_array_js_value_from_wasm") {
            return;
        }
        self.expose_get_array_u32_from_wasm();
        self.expose_get_object();
        self.globals.push_str(&format!("
            function getArrayJsValueFromWasm(ptr, len) {{
                const mem = getUint32Memory();
                const slice = mem.slice(ptr / 4, ptr / 4 + len);
                const result = [];
                for (ptr in slice) {{
                    result.push(getObject(ptr))
                }}
                return result;
            }}
        "));
    }

    fn expose_get_array_i8_from_wasm(&mut self) {
        self.expose_uint8_memory();
        if !self.exposed_globals.insert("get_array_i8_from_wasm") {
            return;
        }
        self.globals.push_str(&format!("
            function getArrayI8FromWasm(ptr, len) {{
                const mem = getUint8Memory();
                const slice = mem.slice(ptr, ptr + len);
                return new Int8Array(slice);
            }}
        "));
    }

    fn expose_get_array_u8_from_wasm(&mut self) {
        self.expose_uint8_memory();
        if !self.exposed_globals.insert("get_array_u8_from_wasm") {
            return;
        }
        self.globals.push_str(&format!("
            function getArrayU8FromWasm(ptr, len) {{
                const mem = getUint8Memory();
                const slice = mem.slice(ptr, ptr + len);
                return new Uint8Array(slice);
            }}
        "));
    }

    fn expose_get_array_i16_from_wasm(&mut self) {
        self.expose_uint16_memory();
        if !self.exposed_globals.insert("get_array_i16_from_wasm") {
            return;
        }
        self.globals.push_str(&format!("
            function getArrayI16FromWasm(ptr, len) {{
                const mem = getUint16Memory();
                const slice = mem.slice(ptr / 2, ptr / 2 + len);
                return new Int16Array(slice);
            }}
        "));
    }

    fn expose_get_array_u16_from_wasm(&mut self) {
        self.expose_uint16_memory();
        if !self.exposed_globals.insert("get_array_u16_from_wasm") {
            return;
        }
        self.globals.push_str(&format!("
            function getArrayU16FromWasm(ptr, len) {{
                const mem = getUint16Memory();
                const slice = mem.slice(ptr / 2, ptr / 2 + len);
                return new Uint16Array(slice);
            }}
        "));
    }

    fn expose_get_array_i32_from_wasm(&mut self) {
        self.expose_uint32_memory();
        if !self.exposed_globals.insert("get_array_i32_from_wasm") {
            return;
        }
        self.globals.push_str(&format!("
            function getArrayI32FromWasm(ptr, len) {{
                const mem = getUint32Memory();
                const slice = mem.slice(ptr / 4, ptr / 4 + len);
                return new Int32Array(slice);
            }}
        "));
    }

    fn expose_get_array_u32_from_wasm(&mut self) {
        self.expose_uint32_memory();
        if !self.exposed_globals.insert("get_array_u32_from_wasm") {
            return;
        }
        self.globals.push_str(&format!("
            function getArrayU32FromWasm(ptr, len) {{
                const mem = getUint32Memory();
                const slice = mem.slice(ptr / 4, ptr / 4 + len);
                return new Uint32Array(slice);
            }}
        "));
    }

    fn expose_get_array_f32_from_wasm(&mut self) {
        if !self.exposed_globals.insert("get_array_f32_from_wasm") {
            return;
        }
        self.globals.push_str(&format!("
            function getArrayF32FromWasm(ptr, len) {{
                const mem = new Float32Array(wasm.memory.buffer);
                const slice = mem.slice(ptr / 4,  ptr / 4 + len);
                return new Float32Array(slice);
            }}
        "));
    }

    fn expose_get_array_f64_from_wasm(&mut self) {
        if !self.exposed_globals.insert("get_array_f64_from_wasm") {
            return;
        }
        self.globals.push_str(&format!("
            function getArrayF64FromWasm(ptr, len) {{
                const mem = new Float64Array(wasm.memory.buffer);
                const slice = mem.slice(ptr / 8,  ptr / 8 + len);
                return new Float64Array(slice);
            }}
        "));
    }

    fn expose_uint8_memory(&mut self) {
        if !self.exposed_globals.insert("uint8_memory") {
            return;
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

    fn expose_uint16_memory(&mut self) {
        if !self.exposed_globals.insert("uint16_memory") {
            return;
        }
        self.globals.push_str(&format!("
            let cachedUint16Memory = null;
            function getUint16Memory() {{
                if (cachedUint16Memory === null ||
                    cachedUint16Memory.buffer !== wasm.memory.buffer)
                    cachedUint16Memory = new Uint16Array(wasm.memory.buffer);
                return cachedUint16Memory;
            }}
        "));
    }

    fn expose_uint32_memory(&mut self) {
        if !self.exposed_globals.insert("uint32_memory") {
            return;
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
            return;
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
            return;
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
            return;
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
            return;
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
                if (slab_next === slab.length)
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
            i.module() == "__wbindgen_placeholder__" && i.field() == name
        })
    }

    fn pass_to_wasm_function(&mut self, t: VectorKind) -> &'static str {
        match t {
            VectorKind::String => {
                self.expose_pass_string_to_wasm();
                "passStringToWasm"
            }
            VectorKind::I8 |
            VectorKind::U8 => {
                self.expose_pass_array8_to_wasm();
                "passArray8ToWasm"
            }
            VectorKind::U16 |
            VectorKind::I16 => {
                self.expose_pass_array16_to_wasm();
                "passArray16ToWasm"
            }
            VectorKind::I32 |
            VectorKind::U32 => {
                self.expose_pass_array32_to_wasm();
                "passArray32ToWasm"
            }
            VectorKind::F32 => {
                self.expose_pass_array_f32_to_wasm();
                "passArrayF32ToWasm"
            }
            VectorKind::F64 => {
                self.expose_pass_array_f64_to_wasm();
                "passArrayF64ToWasm"
            }
            VectorKind::Anyref => {
                panic!("cannot pass list of JsValue to wasm yet")
            }
        }
    }

    fn expose_get_vector_from_wasm(&mut self, ty: VectorKind) -> &'static str {
        match ty {
            VectorKind::String => {
                self.expose_get_string_from_wasm();
                "getStringFromWasm"
            }
            VectorKind::I8 => {
                self.expose_get_array_i8_from_wasm();
                "getArrayI8FromWasm"
            }
            VectorKind::U8 => {
                self.expose_get_array_u8_from_wasm();
                "getArrayU8FromWasm"
            }
            VectorKind::I16 => {
                self.expose_get_array_i16_from_wasm();
                "getArrayI16FromWasm"
            }
            VectorKind::U16 => {
                self.expose_get_array_u16_from_wasm();
                "getArrayU16FromWasm"
            }
            VectorKind::I32 => {
                self.expose_get_array_i32_from_wasm();
                "getArrayI32FromWasm"
            }
            VectorKind::U32 => {
                self.expose_get_array_u32_from_wasm();
                "getArrayU32FromWasm"
            }
            VectorKind::F32 => {
                self.expose_get_array_f32_from_wasm();
                "getArrayF32FromWasm"
            }
            VectorKind::F64 => {
                self.expose_get_array_f64_from_wasm();
                "getArrayF64FromWasm"
            }
            VectorKind::Anyref => {
                self.expose_get_array_js_value_from_wasm();
                "getArrayJsValueFromWasm"
            }
        }
    }

    fn expose_set_global_argument(&mut self) {
        if !self.exposed_globals.insert("set_global_argument") {
            return;
        }
        self.expose_uint32_memory();
        self.expose_global_argument_ptr();
        self.globals.push_str("
            function setGlobalArgument(arg, i) {
                const idx = globalArgumentPtr() / 4 + i;
                getUint32Memory()[idx] = arg;
            }
        ");
    }

    fn expose_get_global_argument(&mut self) {
        if !self.exposed_globals.insert("get_global_argument") {
            return;
        }
        self.expose_uint32_memory();
        self.expose_global_argument_ptr();
        self.globals.push_str("
            function getGlobalArgument(arg) {
                const idx = globalArgumentPtr() / 4 + arg;
                return getUint32Memory()[idx];
            }
        ");
    }

    fn expose_global_argument_ptr(&mut self) {
        if !self.exposed_globals.insert("global_argument_ptr") {
            return;
        }
        self.required_internal_exports.insert("__wbindgen_global_argument_ptr");
        self.globals.push_str("
            let cachedGlobalArgumentPtr = null;
            function globalArgumentPtr() {
                if (cachedGlobalArgumentPtr === null)
                    cachedGlobalArgumentPtr = wasm.__wbindgen_global_argument_ptr();
                return cachedGlobalArgumentPtr;
            }
        ");
    }

    fn expose_get_inherited_descriptor(&mut self) {
        if !self.exposed_globals.insert("get_inherited_descriptor") {
            return
        }
        // It looks like while rare some browsers will move descriptors up the
        // property chain which runs the risk of breaking wasm-bindgen-generated
        // code because we're looking for precise descriptor functions rather
        // than relying on the prototype chain like most "normal JS" projects
        // do.
        //
        // As a result we have a small helper here which will walk the prototype
        // chain looking for a descriptor. For some more information on this see
        // #109
        self.globals.push_str("
            function GetOwnOrInheritedPropertyDescriptor(obj, id) {
              while (obj) {
                let desc = Object.getOwnPropertyDescriptor(obj, id);
                if (desc) return desc;
                obj = Object.getPrototypeOf(obj);
              }
              throw \"descriptor not found\";
            }
        ");
    }

    fn gc(&mut self) {
        let module = mem::replace(self.module, Module::default());
        let wasm_bytes = parity_wasm::serialize(module).unwrap();
        let bytes = wasm_gc::Config::new()
            .demangle(self.config.demangle)
            .gc(&wasm_bytes)
            .unwrap();
        *self.module = deserialize_buffer(&bytes).unwrap();
    }

    fn describe(&self, name: &str) -> Descriptor {
        let name = format!("__wbindgen_describe_{}", name);
        let ret = (self.run_descriptor)(&name);
        Descriptor::decode(&ret)
    }

    fn return_from_rust(&mut self, ty: &Option<Descriptor>, dst_ts: &mut String)
        -> String
    {
        let ty = match *ty {
            Some(ref t) => t,
            None => {
                dst_ts.push_str(": void");
                return format!("return ret;")
            }
        };

        if ty.is_ref_anyref() {
            dst_ts.push_str(": any");
            self.expose_get_object();
            return format!("return getObject(ret);")
        }

        if ty.is_by_ref() {
            panic!("cannot return references from Rust to JS yet")
        }

        if let Some(ty) = ty.vector_kind() {
            dst_ts.push_str(": ");
            dst_ts.push_str(ty.js_ty());
            let f = self.expose_get_vector_from_wasm(ty);
            self.expose_get_global_argument();
            self.required_internal_exports.insert("__wbindgen_free");
            return format!("
                const len = getGlobalArgument(0);
                const realRet = {}(ret, len);
                wasm.__wbindgen_free(ret, len * {});
                return realRet;
            ", f, ty.size())
        }

        if let Some(name) = ty.rust_struct() {
            dst_ts.push_str(": ");
            dst_ts.push_str(name);
            return if self.config.debug {
                format!("return new {name}(ret, token);", name = name)
            } else {
                format!("return new {name}(ret);", name = name)
            }
        }

        if ty.is_number() {
            dst_ts.push_str(": number");
            return format!("return ret;")
        }

        match *ty {
            Descriptor::Boolean => {
                dst_ts.push_str(": boolean");
                format!("return ret !== 0;")
            }
            Descriptor::Anyref => {
                dst_ts.push_str(": any");
                self.expose_take_object();
                format!("return takeObject(ret);")
            }
            _ => panic!("unsupported return from Rust to JS {:?}", ty),
        }
    }

    fn return_from_js(&mut self, ty: &Option<Descriptor>, invoc: &str) -> String {
        let ty = match *ty {
            Some(ref t) => t,
            None => return invoc.to_string(),
        };
        if ty.is_by_ref() {
            panic!("cannot return a reference from JS to Rust")
        }
        if let Some(ty) = ty.vector_kind() {
            let f = self.pass_to_wasm_function(ty);
            self.expose_uint32_memory();
            self.expose_set_global_argument();
            return format!("
                const [retptr, retlen] = {}({});
                setGlobalArgument(retlen, 0);
                return retptr;
            ", f, invoc)
        }
        if ty.is_number() {
            return format!("return {};", invoc)
        }
        match *ty {
            Descriptor::Boolean => format!("return {} ? 1 : 0;", invoc),
            Descriptor::Anyref => {
                self.expose_add_heap_object();
                format!("return addHeapObject({});", invoc)
            }
            _ => panic!("unimplemented return from JS to Rust: {:?}", ty),
        }
    }
}

impl<'a, 'b> SubContext<'a, 'b> {
    pub fn generate(&mut self) {
        for f in self.program.exports.iter() {
            self.generate_export(f);
        }
        for f in self.program.imports.iter() {
            self.generate_import(f);
        }
        for e in self.program.enums.iter() {
            self.generate_enum(e);
        }
    }

    pub fn generate_export(&mut self, export: &shared::Export) {
        if let Some(ref class) = export.class {
            return self.generate_export_for_class(class, export);
        }
        let descriptor = self.cx.describe(&export.function.name);
        let (js, ts) = self.generate_function("function",
                                              &export.function.name,
                                              &export.function.name,
                                              false,
                                              descriptor.unwrap_function());
        self.cx.export(&export.function.name, &js);
        self.cx.globals.push_str("\n");
        self.cx.typescript.push_str("export ");
        self.cx.typescript.push_str(&ts);
        self.cx.typescript.push_str("\n");
    }

    pub fn generate_export_for_class(&mut self, class: &str, export: &shared::Export) {
        let wasm_name = shared::struct_function_export_name(class, &export.function.name);
        let descriptor = self.cx.describe(&wasm_name);
        let (js, ts) = self.generate_function(
            "",
            &export.function.name,
            &wasm_name,
            export.method,
            &descriptor.unwrap_function(),
        );
        let class = self.cx.exported_classes.entry(class.to_string())
            .or_insert(ExportedClass::default());
        if !export.method {
            class.contents.push_str("static ");
            class.typescript.push_str("static ");
        }
        class.contents.push_str(&export.function.name);
        class.contents.push_str(&js);
        class.contents.push_str("\n");
        class.typescript.push_str(&ts);
        class.typescript.push_str("\n");
    }

    fn generate_function(&mut self,
                         prefix: &str,
                         js_name: &str,
                         wasm_name: &str,
                         is_method: bool,
                         function: &Function) -> (String, String) {
        let mut dst = String::from("(");
        let mut dst_ts = format!("{}(", js_name);
        let mut passed_args = String::new();
        let mut arg_conversions = String::new();
        let mut destructors = String::new();

        if is_method {
            passed_args.push_str("this.ptr");
        }

        let mut global_idx = 0;
        for (i, arg) in function.arguments.iter().enumerate() {
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

            if let Some(kind) = arg.vector_kind() {
                dst_ts.push_str(": ");
                dst_ts.push_str(kind.js_ty());
                let func = self.cx.pass_to_wasm_function(kind);
                self.cx.expose_set_global_argument();
                arg_conversions.push_str(&format!("\
                    const [ptr{i}, len{i}] = {func}({arg});
                    setGlobalArgument(len{i}, {global_idx});
                ", i = i, func = func, arg = name, global_idx = global_idx));
                global_idx += 1;
                pass(&format!("ptr{}", i));
                if arg.is_by_ref() {
                    destructors.push_str(&format!("\n\
                        wasm.__wbindgen_free(ptr{i}, len{i} * {size});\n\
                    ", i = i, size = kind.size()));
                    self.cx.required_internal_exports.insert(
                        "__wbindgen_free",
                    );
                }
                continue
            }

            if let Some(s) = arg.rust_struct() {
                dst_ts.push_str(&format!(": {}", s));
                if self.cx.config.debug {
                    self.cx.expose_assert_class();
                    arg_conversions.push_str(&format!("\
                        _assertClass({arg}, {struct_});
                    ", arg = name, struct_ = s));
                }

                if arg.is_by_ref() {
                    pass(&format!("{}.ptr", name));
                } else {
                    arg_conversions.push_str(&format!("\
                        const ptr{i} = {arg}.ptr;
                        {arg}.ptr = 0;
                    ", i = i, arg = name));
                    pass(&format!("ptr{}", i));
                }
                continue
            }

            match *arg {
                ref d if d.is_number() => {
                    dst_ts.push_str(": number");
                    if self.cx.config.debug {
                        self.cx.expose_assert_num();
                        arg_conversions.push_str(&format!("_assertNum({});\n", name));
                    }
                    pass(&name);
                    continue
                }
                Descriptor::Boolean => {
                    dst_ts.push_str(": boolean");
                    if self.cx.config.debug {
                        self.cx.expose_assert_bool();
                        arg_conversions.push_str(&format!("\
                            _assertBoolean({name});
                        ", name = name));
                    }
                    pass(&format!("arg{i} ? 1 : 0", i = i));
                    continue
                }
                Descriptor::Anyref => {
                    dst_ts.push_str(": any");
                    self.cx.expose_add_heap_object();
                    pass(&format!("addHeapObject({})", name));
                    continue
                }
                ref r if r.is_ref_anyref() => {
                    dst_ts.push_str(": any");
                    self.cx.expose_borrowed_objects();
                    destructors.push_str("stack.pop();\n");
                    pass(&format!("addBorrowedObject({})", name));
                    continue
                }
                _ => {}
            }
            panic!("unsupported argument to rust function {:?}", arg)
        }
        dst.push_str(")");
        dst_ts.push_str(")");
        let convert_ret = self.cx.return_from_rust(&function.ret, &mut dst_ts);
        dst_ts.push_str(";");
        dst.push_str(" {\n        ");
        dst.push_str(&arg_conversions);
        if destructors.len() == 0 {
            dst.push_str(&format!("\
                const ret = wasm.{f}({passed});
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

    pub fn generate_import(&mut self, import: &shared::Import) {
        match import.kind {
            shared::ImportKind::Function(ref f) => {
                self.generate_import_function(import, f)
            }
            shared::ImportKind::Static(ref s) => {
                self.generate_import_static(import, s)
            }
            shared::ImportKind::Type(_) => {}
        }
    }

    pub fn generate_import_static(&mut self,
                                  info: &shared::Import,
                                  import: &shared::ImportStatic) {
        // TODO: should support more types to import here
        let obj = self.import_name(info, &import.name);
        self.cx.expose_add_heap_object();
        self.cx.export(&import.shim, &format!("
            function() {{
                return addHeapObject({});
            }}
        ", obj));
    }

    pub fn generate_import_function(&mut self,
                                    info: &shared::Import,
                                    import: &shared::ImportFunction) {
        let descriptor = self.cx.describe(&import.shim);
        let desc_function = descriptor.unwrap_function();

        let mut dst = String::new();

        dst.push_str("function(");
        let mut invoc_args = Vec::new();
        let mut abi_args = Vec::new();

        let mut extra = String::new();
        let mut finally = String::new();

        let mut next_global = 0;
        for (i, arg) in desc_function.arguments.iter().enumerate() {
            abi_args.push(format!("arg{}", i));

            if let Some(ty) = arg.vector_kind() {
                let f = self.cx.expose_get_vector_from_wasm(ty);
                self.cx.expose_get_global_argument();
                extra.push_str(&format!("
                    let len{0} = getGlobalArgument({next_global});
                    let v{0} = {func}(arg{0}, len{0});
                ", i, func = f, next_global = next_global));
                next_global += 1;

                if !arg.is_by_ref() {
                    extra.push_str(&format!("
                        wasm.__wbindgen_free(arg{0}, len{0} * {size});
                    ", i, size = ty.size()));
                    self.cx.required_internal_exports.insert(
                        "__wbindgen_free"
                    );
                }
                invoc_args.push(format!("v{}", i));
                continue
            }

            if let Some(s) = arg.rust_struct() {
                if arg.is_by_ref() {
                    panic!("cannot invoke JS functions with custom ref types yet")
                }
                let assign = if self.cx.config.debug {
                    format!("let c{0} = new {class}(arg{0}, token);", i, class = s)
                } else {
                    format!("let c{0} = new {class}(arg{0});", i, class = s)
                };
                extra.push_str(&assign);
                invoc_args.push(format!("c{}", i));
                continue
            }

            if let Some((f, mutable)) = arg.stack_closure() {
                let args = (0..f.arguments.len())
                    .map(|i| format!("arg{}", i))
                    .collect::<Vec<_>>()
                    .join(", ");
                self.cx.expose_get_global_argument();
                self.cx.function_table_needed = true;
                let sep = if f.arguments.len() == 0 {""} else {","};
                let body = if mutable {
                    format!("
                        let a = this.a;
                        this.a = 0;
                        try {{
                            return this.f(a, this.b {} {});
                        }} finally {{
                            this.a = a;
                        }}
                    ", sep, args)
                } else {
                    format!("return this.f(this.a, this.b {} {});", sep, args)
                };
                extra.push_str(&format!("
                    let cb{0} = function({args}) {{ {body} }};
                    cb{0}.f = wasm.__wbg_function_table.get(arg{0});
                    cb{0}.a = getGlobalArgument({next_global});
                    cb{0}.b = getGlobalArgument({next_global} + 1);
                ", i, next_global = next_global, body = body, args = args));
                next_global += 2;
                finally.push_str(&format!("
                    cb{0}.a = cb{0}.b = 0;
                ", i));
                invoc_args.push(format!("cb{0}.bind(cb{0})", i));
                continue
            }

            if let Some(_f) = arg.ref_closure() {
                self.cx.expose_get_object();
                invoc_args.push(format!("getObject(arg{})", i));
                continue
            }

            let invoc_arg = match *arg {
                ref d if d.is_number() => format!("arg{}", i),
                Descriptor::Boolean => format!("arg{} !== 0", i),
                Descriptor::Anyref => {
                    self.cx.expose_take_object();
                    format!("takeObject(arg{})", i)
                }
                ref d if d.is_ref_anyref() => {
                    self.cx.expose_get_object();
                    format!("getObject(arg{})", i)
                }
                _ => panic!("unimplemented argument type in imported function: {:?}", arg),
            };
            invoc_args.push(invoc_arg);
        }

        let nargs = invoc_args.len();
        let invoc_args = invoc_args.join(", ");
        let function_name = &import.function.name;
        let invoc = match import.class {
            Some(ref class) if import.js_new => {
                format!("new {}", self.import_name(info, class))
            }
            Some(ref class) if import.method => {
                let class = self.import_name(info, class);
                let target = if let Some(ref g) = import.getter {
                    if import.structural {
                        format!("function() {{ return this.{}; }}", g)
                    } else {
                        self.cx.expose_get_inherited_descriptor();
                        format!(
                            "GetOwnOrInheritedPropertyDescriptor\
                                ({}.prototype, '{}').get;",
                            class,
                            g,
                        )
                    }
                } else if let Some(ref s) = import.setter {
                    if import.structural {
                        format!("function(y) {{ this.{} = y; }}", s)
                    } else {
                        self.cx.expose_get_inherited_descriptor();
                        format!(
                            "GetOwnOrInheritedPropertyDescriptor\
                                ({}.prototype, '{}').set;",
                            class,
                            s,
                        )
                    }
                } else {
                    if import.structural {
                        let mut s = format!("function(");
                        for i in 0..nargs - 1 {
                            if i > 0 {
                                drop(write!(s, ", "));
                            }
                            drop(write!(s, "x{}", i));
                        }
                        s.push_str(") { return this.");
                        s.push_str(function_name);
                        s.push_str("(");
                        for i in 0..nargs - 1 {
                            if i > 0 {
                                drop(write!(s, ", "));
                            }
                            drop(write!(s, "x{}", i));
                        }
                        s.push_str("); }");
                        s
                    } else {
                        format!("{}.prototype.{}", class, function_name)
                    }
                };
                self.cx.globals.push_str(&format!("
                    const {}_target = {};
                ", import.shim, target));
                format!("{}_target.call", import.shim)
            }
            Some(ref class) => {
                let class = self.import_name(info, class);
                self.cx.globals.push_str(&format!("
                    const {}_target = {}.{};
                ", import.shim, class, function_name));
                format!("{}_target", import.shim)
            }
            None => {
                let name = self.import_name(info, function_name);
                if name.contains(".") {
                    self.cx.globals.push_str(&format!("
                        const {}_target = {};
                    ", import.shim, name));
                    format!("{}_target", import.shim)
                } else {
                    name
                }
            }
        };
        let invoc = format!("{}({})", invoc, invoc_args);
        let invoc = self.cx.return_from_js(&desc_function.ret, &invoc);

        let invoc = if import.catch {
            self.cx.expose_uint32_memory();
            self.cx.expose_add_heap_object();
            abi_args.push("exnptr".to_string());
            format!("
                try {{
                    {}
                }} catch (e) {{
                    const view = getUint32Memory();
                    view[exnptr / 4] = 1;
                    view[exnptr / 4 + 1] = addHeapObject(e);
                }}
            ", invoc)
        } else {
            invoc
        };
        let invoc = if finally.len() > 0 {
            format!("
                try {{
                    {}
                }} finally {{
                    {}
                }}
            ", invoc, finally)
        } else {
            invoc
        };

        dst.push_str(&abi_args.join(", "));
        dst.push_str(") {\n");
        dst.push_str(&extra);
        dst.push_str(&format!("{}\n}}", invoc));
        self.cx.export(&import.shim, &dst);
    }

    pub fn generate_enum(&mut self, enum_: &shared::Enum) {
        let mut variants = String::new();

        for variant in enum_.variants.iter() {
            variants.push_str(&format!("{}:{},", variant.name, variant.value));
        }
        self.cx.export(&enum_.name, &format!("Object.freeze({{ {} }})", variants));
        self.cx.typescript.push_str(&format!("export enum {} {{", enum_.name));

        variants.clear();
        for variant in enum_.variants.iter() {
            variants.push_str(&format!("{},", variant.name));
        }
        self.cx.typescript.push_str(&variants);
        self.cx.typescript.push_str("}\n");
    }

    fn import_name(&mut self, import: &shared::Import, item: &str) -> String {
        if let Some(ref module) = import.module {
            if self.cx.config.no_modules {
                panic!("import from `{}` module not allowed in `--no-modules`. use `--nodejs` or `--browser` instead", module);
            }

            let name = import.js_namespace.as_ref().map(|s| &**s).unwrap_or(item);

            if self.cx.imported_names.insert(name.to_string()) {
                if self.cx.config.nodejs {
                    self.cx.imports.push_str(&format!("
                        const {} = require('{}').{};
                    ", name, module, name));
                } else {
                    self.cx.imports.push_str(&format!("
                        import {{ {} }} from '{}';
                    ", name, module));
                }
            }
        }
        match import.js_namespace {
            Some(ref s) => format!("{}.{}", s, item),
            None => item.to_string(),
        }
    }
}
