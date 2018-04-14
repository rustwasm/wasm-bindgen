use super::Context;
use descriptor::{Descriptor, Function};

/// Helper struct for manfuacturing a shim in JS used to translate JS types to
/// Rust, aka pass from JS back into Rust
pub struct Js2Rust<'a, 'b: 'a> {
    cx: &'a mut Context<'b>,

    /// Arguments passed to the invocation of the wasm function, aka things that
    /// are only numbers.
    rust_arguments: Vec<String>,

    /// Arguments and their types to the JS shim.
    js_arguments: Vec<(String, String)>,

    /// Conversions that happen before we invoke the wasm function, such as
    /// converting a string to a ptr/length pair.
    prelude: String,

    /// "Destructors" or cleanup that must happen after the wasm function
    /// finishes. This is scheduled in a `finally` block.
    finally: String,

    /// Next global index to write to when passing arguments via the single
    /// global stack.
    global_idx: usize,

    /// Index of the next argument for unique name generation purposes.
    arg_idx: usize,

    /// Typescript expression representing the type of the return value of this
    /// function.
    ret_ty: String,

    /// Expression used to generate the return value. The string "RET" in this
    /// expression is replaced with the actual wasm invocation eventually.
    ret_expr: String,

    /// Name of the JS shim/function that we're generating, primarily for
    /// TypeScript right now.
    js_name: String,
}

impl<'a, 'b> Js2Rust<'a, 'b> {
    pub fn new(js_name: &str, cx: &'a mut Context<'b>) -> Js2Rust<'a, 'b> {
        Js2Rust {
            cx,
            js_name: js_name.to_string(),
            rust_arguments: Vec::new(),
            js_arguments: Vec::new(),
            prelude: String::new(),
            finally: String::new(),
            global_idx: 0,
            arg_idx: 0,
            ret_ty: String::new(),
            ret_expr: String::new(),
        }
    }

    /// Generates all bindings necessary for the signature in `Function`,
    /// creating necessary argument conversions and return value processing.
    pub fn process(&mut self, function: &Function) -> &mut Self {
        for arg in function.arguments.iter() {
            self.argument(arg);
        }
        self.ret(&function.ret);
        self
    }

    /// Flag this shim as a method call into Rust, so the first Rust argument
    /// passed should be `this.ptr`.
    pub fn method(&mut self, method: bool) -> &mut Self {
        if method {
            self.rust_arguments.insert(0, "this.ptr".to_string());
        }
        self
    }

    /// Add extra processing to the prelude of this shim.
    pub fn prelude(&mut self, s: &str) -> &mut Self {
        self.prelude.push_str(s);
        self
    }

    /// Add extra processing to the finally block of this shim.
    pub fn finally(&mut self, s: &str) -> &mut Self {
        self.finally.push_str(s);
        self
    }

    /// Add an Rust argument to be passed manually.
    pub fn rust_argument(&mut self, s: &str) -> &mut Self {
        self.rust_arguments.push(s.to_string());
        self
    }

    fn argument(&mut self, arg: &Descriptor) {
        let i = self.arg_idx;
        self.arg_idx += 1;
        let name = format!("arg{}", i);

        if let Some(kind) = arg.vector_kind() {
            self.js_arguments.push((name.clone(), kind.js_ty().to_string()));

            let func = self.cx.pass_to_wasm_function(kind);
            self.cx.expose_set_global_argument();
            let global_idx = self.global_idx();
            self.prelude.push_str(&format!("\
                const [ptr{i}, len{i}] = {func}({arg});
                setGlobalArgument(len{i}, {global_idx});
            ", i = i, func = func, arg = name, global_idx = global_idx));
            if arg.is_by_ref() {
                self.finally.push_str(&format!("\n\
                    wasm.__wbindgen_free(ptr{i}, len{i} * {size});\n\
                ", i = i, size = kind.size()));
                self.cx.required_internal_exports.insert(
                    "__wbindgen_free",
                );
            }
            self.rust_arguments.push(format!("ptr{}", i));
            return
        }

        if let Some(s) = arg.rust_struct() {
            self.js_arguments.push((name.clone(), s.to_string()));

            if self.cx.config.debug {
                self.cx.expose_assert_class();
                self.prelude.push_str(&format!("\
                    _assertClass({arg}, {struct_});
                ", arg = name, struct_ = s));
            }

            if arg.is_by_ref() {
                self.rust_arguments.push(format!("{}.ptr", name));
            } else {
                self.prelude.push_str(&format!("\
                    const ptr{i} = {arg}.ptr;
                    {arg}.ptr = 0;
                ", i = i, arg = name));
                self.rust_arguments.push(format!("ptr{}", i));
            }
            return
        }

        if arg.is_number() {
            self.js_arguments.push((name.clone(), "number".to_string()));

            if self.cx.config.debug {
                self.cx.expose_assert_num();
                self.prelude.push_str(&format!("_assertNum({});\n", name));
            }

            self.rust_arguments.push(name);
            return
        }

        if arg.is_ref_anyref() {
            self.js_arguments.push((name.clone(), "any".to_string()));
            self.cx.expose_borrowed_objects();
            self.finally.push_str("stack.pop();\n");
            self.rust_arguments.push(format!("addBorrowedObject({})", name));
            return
        }

        match *arg {
            Descriptor::Boolean => {
                self.js_arguments.push((name.clone(), "boolean".to_string()));
                if self.cx.config.debug {
                    self.cx.expose_assert_bool();
                    self.prelude.push_str(&format!("\
                        _assertBoolean({name});
                    ", name = name));
                }
                self.rust_arguments.push(format!("arg{i} ? 1 : 0", i = i));
            }
            Descriptor::Anyref => {
                self.js_arguments.push((name.clone(), "any".to_string()));
                self.cx.expose_add_heap_object();
                self.rust_arguments.push(format!("addHeapObject({})", name));
            }
            _ => {
                panic!("unsupported argument to rust function {:?}", arg)
            }
        }
    }

    fn ret(&mut self, ret: &Option<Descriptor>) {
        let ty = match *ret {
            Some(ref t) => t,
            None => {
                self.ret_ty = "void".to_string();
                self.ret_expr = format!("return RET;");
                return
            }
        };

        if ty.is_ref_anyref() {
            self.ret_ty = "any".to_string();
            self.cx.expose_get_object();
            self.ret_expr = format!("return getObject(RET);");
            return
        }

        if ty.is_by_ref() {
            panic!("cannot return references from Rust to JS yet")
        }

        if let Some(ty) = ty.vector_kind() {
            self.ret_ty = ty.js_ty().to_string();
            let f = self.cx.expose_get_vector_from_wasm(ty);
            self.cx.expose_get_global_argument();
            self.cx.required_internal_exports.insert("__wbindgen_free");
            self.ret_expr = format!("
                const ret = RET;
                const len = getGlobalArgument(0);
                const realRet = {}(ret, len);
                wasm.__wbindgen_free(ret, len * {});
                return realRet;
            ", f, ty.size());
            return
        }

        if let Some(name) = ty.rust_struct() {
            self.ret_ty = name.to_string();
            self.ret_expr = format!("return {name}.__construct(RET);", name = name);
            return
        }

        if ty.is_number() {
            self.ret_ty = "number".to_string();
            self.ret_expr = format!("return RET;");
            return
        }

        match *ty {
            Descriptor::Boolean => {
                self.ret_ty = "boolean".to_string();
                self.ret_expr = format!("return (RET) !== 0;");
            }
            Descriptor::Anyref => {
                self.ret_ty = "any".to_string();
                self.cx.expose_take_object();
                self.ret_expr = format!("return takeObject(RET);");
            }
            _ => panic!("unsupported return from Rust to JS {:?}", ty),
        }
    }

    /// Generate the actual function.
    ///
    /// The `prefix` specified is typically the string "function" but may be
    /// different for classes. The `invoc` is the function expression that we're
    /// invoking, like `wasm.bar` or `this.f`.
    ///
    /// Returns two strings, the first of which is the JS expression for the
    /// generated function shim and the second is a TyepScript signature of rthe
    /// JS expression.
    pub fn finish(&self, prefix: &str, invoc: &str) -> (String, String) {
        let js_args = self.js_arguments
            .iter()
            .map(|s| &s.0[..])
            .collect::<Vec<_>>()
            .join(", ");
        let mut js = format!("{}({}) {{\n", prefix, js_args);
        js.push_str(&self.prelude);
        let rust_args = self.rust_arguments.join(", ");

        let invoc = self.ret_expr.replace("RET", &format!("{}({})", invoc, rust_args));
        if self.finally.len() == 0 {
            js.push_str(&invoc);
        } else {
            js.push_str(&format!("\
                try {{
                    {}
                }} finally {{
                    {}
                }}
            ",
                invoc,
                self.finally,
            ));
        }
        js.push_str("}");

        let ts_args = self.js_arguments
            .iter()
            .map(|s| format!("{}: {}", s.0, s.1))
            .collect::<Vec<_>>()
            .join(", ");
        let ts = format!("{} {}({}): {};\n", prefix, self.js_name, ts_args, self.ret_ty);
        (js, ts)
    }

    fn global_idx(&mut self) -> usize {
        let ret = self.global_idx;
        self.global_idx += 1;
        ret
    }
}
