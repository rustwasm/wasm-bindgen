use std::collections::{HashSet, HashMap};
use std::fmt::Write;
use std::mem;

use parity_wasm::elements::*;
use parity_wasm;
use shared;
use wasm_gc;

use super::Bindgen;
use descriptor::{Descriptor, VectorKind};

mod js2rust;
use self::js2rust::Js2Rust;
mod rust2js;
use self::rust2js::Rust2Js;

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
    pub constructor: Option<String>,
}

pub struct SubContext<'a, 'b: 'a> {
    pub program: &'a shared::Program,
    pub cx: &'a mut Context<'b>,
}

impl<'a> Context<'a> {
    fn export(&mut self, name: &str, contents: &str) {
        let contents = deindent(contents);
        let contents = contents.trim();
        let global = if self.config.nodejs {
            format!("module.exports.{} = {};\n", name, contents)
        } else if self.config.no_modules {
            format!("__exports.{} = {}\n", name, contents)
        } else {
            if contents.starts_with("function") {
                format!("export function {}{}\n", name, &contents[8..])
            } else if contents.starts_with("class") {
                format!("export {}\n", contents)
            } else {
                format!("export const {} = {};\n", name, contents)
            }
        };
        self.global(&global);
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
                self.export(name, &contents);
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
                String::from("
                    function(p, l) {
                        return addHeapObject(getStringFromWasm(p, l));
                    }
                ")
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
                String::from("
                    function() {
                        return addHeapObject(null);
                    }
                ")
            });

            bind("__wbindgen_is_null", &|me| {
                me.expose_get_object();
                String::from("
                    function(idx) {
                        return getObject(idx) === null ? 1 : 0;
                    }
                ")
            });

            bind("__wbindgen_is_undefined", &|me| {
                me.expose_get_object();
                String::from("
                    function(idx) {
                        return getObject(idx) === undefined ? 1 : 0;
                    }
                ")
            });

            bind("__wbindgen_boolean_new", &|me| {
                me.expose_add_heap_object();
                String::from("
                    function(v) {
                        return addHeapObject(v === 1);
                    }
                ")
            });

            bind("__wbindgen_boolean_get", &|me| {
                me.expose_get_object();
                String::from("
                    function(i) {
                        let v = getObject(i);
                        if (typeof(v) === 'boolean') {
                            return v ? 1 : 0;
                        } else {
                            return 2;
                        }
                    }
                ")
            });

            bind("__wbindgen_symbol_new", &|me| {
                me.expose_get_string_from_wasm();
                me.expose_add_heap_object();
                format!("
                    function(ptr, len) {{
                        let a;
                        console.log(ptr, len);
                        if (ptr === 0) {{
                            a = Symbol();
                        }} else {{
                            a = Symbol(getStringFromWasm(ptr, len));
                        }}
                        return addHeapObject(a);
                    }}
                ")
            });

            bind("__wbindgen_is_symbol", &|me| {
                me.expose_get_object();
                String::from("
                    function(i) {
                        return typeof(getObject(i)) === 'symbol' ? 1 : 0;
                    }
                ")
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
                String::from("
                    function(i, len_ptr) {
                        let obj = getObject(i);
                        if (typeof(obj) !== 'string')
                            return 0;
                        const [ptr, len] = passStringToWasm(obj);
                        getUint32Memory()[len_ptr / 4] = len;
                        return ptr;
                    }
                ")
            });

            bind("__wbindgen_cb_drop", &|me| {
                me.expose_drop_ref();
                String::from("
                    function(i) {
                        let obj = getObject(i).original;
                        obj.a = obj.b = 0;
                        dropRef(i);
                    }
                ")
            });
            bind("__wbindgen_cb_forget", &|me| {
                me.expose_drop_ref();
                String::from("
                    function(i) {
                        dropRef(i);
                    }
                ")
            });
        }

        self.rewrite_imports(module_name);

        let mut js = if self.config.no_modules {
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

            format!("\
                /* tslint:disable */\n\
                {import_wasm}\n\
                {imports}\n\

                {globals}\n\
                {footer}",
                    import_wasm = import_wasm,
                    globals = self.globals,
                    imports = self.imports,
                    footer = self.footer,
            )
        };

        self.export_table();
        self.gc();

        while js.contains("\n\n\n") {
            js = js.replace("\n\n\n", "\n\n");
        }

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

            if self.config.debug || exports.constructor.is_some() {
                self.expose_constructor_token();

                dst.push_str(&format!("
                    static __construct(ptr) {{
                        return new {}(new ConstructorToken(ptr));
                    }}

                    constructor(...args) {{
                        if (args.length === 1 && args[0] instanceof ConstructorToken) {{
                            this.ptr = args[0].ptr;
                            return;
                        }}
                ", class));

                if let Some(constructor) = exports.constructor {
                    ts_dst.push_str(&format!("constructor(...args: [any]);\n"));

                    dst.push_str(&format!("
                        // This invocation of new will call this constructor with a ConstructorToken
                        let instance = {class}.{constructor}(...args);
                        this.ptr = instance.ptr;
                    ", class = class, constructor = constructor));
                } else {
                    dst.push_str("throw new Error('you cannot invoke `new` directly without having a \
                method annotated a constructor');");
                }

                dst.push_str("}");
            } else {
                dst.push_str(&format!("
                    static __construct(ptr) {{
                        return new {}(ptr);
                    }}

                    constructor(ptr) {{
                        this.ptr = ptr;
                    }}
                ", class));
            }

            let new_name = shared::new_function(&class);
            if self.wasm_import_needed(&new_name) {
                self.expose_add_heap_object();

                self.export(&new_name, &format!("
                    function(ptr) {{
                        return addHeapObject({}.__construct(ptr));
                    }}
                ", class));
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
        self.global(&format!("
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
        self.global(&format!("
            let stack = [];
        "));
    }

    fn expose_global_slab(&mut self) {
        if !self.exposed_globals.insert("slab") {
            return;
        }
        self.global(&format!("let slab = [];"));
    }

    fn expose_global_slab_next(&mut self) {
        if !self.exposed_globals.insert("slab_next") {
            return;
        }
        self.global(&format!("
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
        self.global(&format!("
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

    fn expose_assert_num(&mut self) {
        if !self.exposed_globals.insert("assert_num") {
            return;
        }
        self.global(&format!("
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
        self.global(&format!("
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
        self.global(&format!("
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
        self.global(&format!("
            function passArray8ToWasm(arg) {{
                const ptr = wasm.__wbindgen_malloc(arg.length);
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
        self.global(&format!("
            function passArray16ToWasm(arg) {{
                const ptr = wasm.__wbindgen_malloc(arg.length * 2);
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
        self.global(&format!("
            function passArray32ToWasm(arg) {{
                const ptr = wasm.__wbindgen_malloc(arg.length * 4);
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
        self.global(&format!("
            function passArrayF32ToWasm(arg) {{
                const ptr = wasm.__wbindgen_malloc(arg.length * 4);
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
        self.global(&format!("
            function passArrayF64ToWasm(arg) {{
                const ptr = wasm.__wbindgen_malloc(arg.length * 8);
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
            self.global(&format!("
                const TextEncoder = require('util').TextEncoder;
            "));
        } else if !(self.config.browser || self.config.no_modules) {
            self.global(&format!("
                const TextEncoder = typeof window === 'object' && window.TextEncoder
                    ? window.TextEncoder
                    : require('util').TextEncoder;
            "));
        }
        self.global(&format!("
            let cachedEncoder = new TextEncoder('utf-8');
        "));
    }

    fn expose_text_decoder(&mut self) {
        if !self.exposed_globals.insert("text_decoder") {
            return;
        }
        if self.config.nodejs {
            self.global(&format!("
                const TextDecoder = require('util').TextDecoder;
            "));
        } else if !(self.config.browser || self.config.no_modules) {
            self.global(&format!("
                const TextDecoder = typeof window === 'object' && window.TextDecoder
                    ? window.TextDecoder
                    : require('util').TextDecoder;
            "));
        }
        self.global(&format!("
            let cachedDecoder = new TextDecoder('utf-8');
        "));
    }

    fn expose_constructor_token(&mut self) {
        if !self.exposed_globals.insert("ConstructorToken") {
            return;
        }

        self.global("
            class ConstructorToken {
                constructor(ptr) {
                    this.ptr = ptr;
                }
            }
        ");
    }

    fn expose_get_string_from_wasm(&mut self) {
        if !self.exposed_globals.insert("get_string_from_wasm") {
            return;
        }
        self.expose_text_decoder();
        self.expose_uint8_memory();
        self.global(&format!("
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
        self.global(&format!("
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
        self.global(&format!("
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
        self.global(&format!("
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
        self.global(&format!("
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
        self.global(&format!("
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
        self.global(&format!("
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
        self.global(&format!("
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
        self.global(&format!("
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
        self.global(&format!("
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
        self.global(&format!("
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
        self.global(&format!("
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
        self.global(&format!("
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
        self.global(&format!("
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
        self.global(&format!("
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
        self.global(&format!("
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
        self.global(&format!("
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
        self.global("
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
        self.global("
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
        self.global("
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
        self.global("
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

    fn global(&mut self, s: &str) {
        let s = deindent(s);
        let s = s.trim();

        // Ensure a blank line between adjacent items, and ensure everything is
        // terminated with a newline.
        while !self.globals.ends_with("\n\n\n") {
            self.globals.push_str("\n");
        }
        self.globals.push_str(s);
        self.globals.push_str("\n");
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
        for s in self.program.structs.iter() {
            self.cx.exported_classes
                .entry(s.clone())
                .or_insert_with(Default::default);
        }
    }

    pub fn generate_export(&mut self, export: &shared::Export) {
        if let Some(ref class) = export.class {
            return self.generate_export_for_class(class, export);
        }
        let descriptor = self.cx.describe(&export.function.name);
        let (js, ts) = Js2Rust::new(&export.function.name, self.cx)
            .process(descriptor.unwrap_function())
            .finish("function", &format!("wasm.{}", export.function.name));
        self.cx.export(&export.function.name, &js);
        self.cx.globals.push_str("\n");
        self.cx.typescript.push_str("export ");
        self.cx.typescript.push_str(&ts);
        self.cx.typescript.push_str("\n");
    }

    pub fn generate_export_for_class(&mut self, class_name: &str, export: &shared::Export) {
        let wasm_name = shared::struct_function_export_name(class_name, &export.function.name);
        let descriptor = self.cx.describe(&wasm_name);
        let (js, ts) = Js2Rust::new(&export.function.name, self.cx)
            .method(export.method)
            .process(descriptor.unwrap_function())
            .finish("", &format!("wasm.{}", wasm_name));
        let class = self.cx.exported_classes.entry(class_name.to_string())
            .or_insert(ExportedClass::default());
        if !export.method {
            class.contents.push_str("static ");
            class.typescript.push_str("static ");
        }

        let constructors: Vec<String> = self.program.exports
            .iter()
            .filter(|x| x.class == Some(class_name.to_string()))
            .filter_map(|x| x.constructor.clone())
            .collect();

        class.constructor = match constructors.len() {
            0 => None,
            1 => Some(constructors[0].clone()),
            x @ _ => panic!("There must be only one constructor, not {}", x),
        };

        class.contents.push_str(&export.function.name);
        class.contents.push_str(&js);
        class.contents.push_str("\n");
        class.typescript.push_str(&ts);
        class.typescript.push_str("\n");
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

        let target = match import.class {
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
                        let nargs = descriptor.unwrap_function().arguments.len();
                        let mut s = format!("function(");
                        for i in 0..nargs - 1 {
                            if i > 0 {
                                drop(write!(s, ", "));
                            }
                            drop(write!(s, "x{}", i));
                        }
                        s.push_str(") { return this.");
                        s.push_str(&import.function.name);
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
                        format!("{}.prototype.{}", class, import.function.name)
                    }
                };
                self.cx.global(&format!("
                    const {}_target = {};
                ", import.shim, target));
                format!("{}_target.call", import.shim)
            }
            Some(ref class) => {
                let class = self.import_name(info, class);
                self.cx.global(&format!("
                    const {}_target = {}.{};
                ", import.shim, class, import.function.name));
                format!("{}_target", import.shim)
            }
            None => {
                let name = self.import_name(info, &import.function.name);
                if name.contains(".") {
                    self.cx.global(&format!("
                        const {}_target = {};
                    ", import.shim, name));
                    format!("{}_target", import.shim)
                } else {
                    name
                }
            }
        };

        let js = Rust2Js::new(self.cx)
            .catch(import.catch)
            .process(descriptor.unwrap_function())
            .finish(&target);
        self.cx.export(&import.shim, &js);
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
                    self.cx.imports.push_str(&format!("\
                        const {} = require('{}').{};\n\
                    ", name, module, name));
                } else {
                    self.cx.imports.push_str(&format!("\
                        import {{ {} }} from '{}';\n\
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

fn indent(s: &str) -> String {
    let mut ret = String::new();
    for line in s.lines() {
        ret.push_str("    ");
        ret.push_str(line);
        ret.push_str("\n");
    }
    return ret
}

fn deindent(s: &str) -> String {
    let amt_to_strip = s.lines()
        .filter(|l| !l.trim().is_empty())
        .map(|s| s.len() - s.trim_left().len())
        .min()
        .unwrap_or(0);
    let mut ret = String::new();
    for line in s.lines() {
        if !line.trim().is_empty() {
            ret.push_str(&line[amt_to_strip..]);
        }
        ret.push_str("\n");
    }
    ret
}
