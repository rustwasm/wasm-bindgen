use failure::Error;

use super::Context;
use descriptor::{Descriptor, Function};

/// Helper struct for manufacturing a shim in JS used to translate JS types to
/// Rust, aka pass from JS back into Rust
pub struct Js2Rust<'a, 'b: 'a> {
    cx: &'a mut Context<'b>,

    /// Arguments passed to the invocation of the wasm function, aka things that
    /// are only numbers.
    rust_arguments: Vec<String>,

    /// Arguments and their types to the JS shim.
    pub js_arguments: Vec<(String, String)>,

    /// Conversions that happen before we invoke the wasm function, such as
    /// converting a string to a ptr/length pair.
    prelude: String,

    /// "Destructors" or cleanup that must happen after the wasm function
    /// finishes. This is scheduled in a `finally` block.
    finally: String,

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
            arg_idx: 0,
            ret_ty: String::new(),
            ret_expr: String::new(),
        }
    }

    /// Generates all bindings necessary for the signature in `Function`,
    /// creating necessary argument conversions and return value processing.
    pub fn process(&mut self, function: &Function) -> Result<&mut Self, Error> {
        for arg in function.arguments.iter() {
            self.argument(arg)?;
        }
        self.ret(&function.ret)?;
        Ok(self)
    }

    /// Flag this shim as a method call into Rust, so the first Rust argument
    /// passed should be `this.ptr`.
    pub fn method(&mut self, method: bool, consumed: bool) -> &mut Self {
        if method {
            self.prelude(
                "if (this.ptr === 0) {
                    throw new Error('Attempt to use a moved value');
                }"
            );
            if consumed {
                self.prelude("\
                    const ptr = this.ptr;\n\
                    this.ptr = 0;\n\
                ");
                self.rust_arguments.insert(0, "ptr".to_string());
            } else {
                self.rust_arguments.insert(0, "this.ptr".to_string());
            }
        }
        self
    }

    /// Add extra processing to the prelude of this shim.
    pub fn prelude(&mut self, s: &str) -> &mut Self {
        for line in s.lines() {
            self.prelude.push_str(line);
            self.prelude.push_str("\n");
        }
        self
    }

    /// Add extra processing to the finally block of this shim.
    pub fn finally(&mut self, s: &str) -> &mut Self {
        for line in s.lines() {
            self.finally.push_str(line);
            self.finally.push_str("\n");
        }
        self
    }

    /// Add an Rust argument to be passed manually.
    pub fn rust_argument(&mut self, s: &str) -> &mut Self {
        self.rust_arguments.push(s.to_string());
        self
    }

    fn abi_arg(&mut self) -> String {
        let s = format!("arg{}", self.arg_idx);
        self.arg_idx += 1;
        s
    }

    pub fn argument(&mut self, arg: &Descriptor) -> Result<&mut Self, Error> {
        let i = self.arg_idx;
        let name = self.abi_arg();

        if let Some(kind) = arg.vector_kind() {
            self.js_arguments
                .push((name.clone(), kind.js_ty().to_string()));

            let func = self.cx.pass_to_wasm_function(kind)?;
            self.prelude(&format!(
                "\
                 const [ptr{i}, len{i}] = {func}({arg});\n\
                 ",
                i = i,
                func = func,
                arg = name
            ));
            if arg.is_by_ref() {
                if arg.is_mut_ref() {
                    let get = self.cx.memview_function(kind);
                    self.finally(&format!(
                        "\
                         {arg}.set({get}().subarray(\
                         ptr{i} / {size}, \
                         ptr{i} / {size} + len{i}\
                         ));\n\
                         ",
                        i = i,
                        arg = name,
                        get = get,
                        size = kind.size()
                    ));
                }
                self.finally(&format!(
                    "\
                     wasm.__wbindgen_free(ptr{i}, len{i} * {size});\n\
                     ",
                    i = i,
                    size = kind.size()
                ));
                self.cx.require_internal_export("__wbindgen_free")?;
            }
            self.rust_arguments.push(format!("ptr{}", i));
            self.rust_arguments.push(format!("len{}", i));
            return Ok(self);
        }

        if let Some(s) = arg.rust_struct() {
            self.js_arguments.push((name.clone(), s.to_string()));

            if self.cx.config.debug {
                self.cx.expose_assert_class();
                self.prelude(&format!(
                    "\
                     _assertClass({arg}, {struct_});\n\
                     ",
                    arg = name,
                    struct_ = s
                ));
            }

            if arg.is_by_ref() {
                self.rust_arguments.push(format!("{}.ptr", name));
            } else {
                self.prelude(&format!(
                    "\
                    const ptr{i} = {arg}.ptr;\n\
                    if (ptr{i} === 0) {{
                        throw new Error('Attempt to use a moved value');
                    }}
                    {arg}.ptr = 0;\n\
                ",
                    i = i,
                    arg = name
                ));
                self.rust_arguments.push(format!("ptr{}", i));
            }
            return Ok(self);
        }

        if arg.is_number() {
            self.js_arguments.push((name.clone(), "number".to_string()));

            if self.cx.config.debug {
                self.cx.expose_assert_num();
                self.prelude(&format!("_assertNum({});", name));
            }

            self.rust_arguments.push(name);
            return Ok(self);
        }

        if let Some(signed) = arg.get_64bit() {
            let f = if signed {
                self.cx.expose_int64_cvt_shim()
            } else {
                self.cx.expose_uint64_cvt_shim()
            };
            self.cx.expose_uint32_memory();
            self.cx.expose_global_argument_ptr()?;
            self.js_arguments.push((name.clone(), "BigInt".to_string()));
            self.prelude(&format!(
                "\
                 {f}[0] = {name};\n\
                 const lo{i} = u32CvtShim[0];\n\
                 const hi{i} = u32CvtShim[1];\n\
                 ",
                i = i,
                f = f,
                name = name,
            ));
            self.rust_arguments.push(format!("lo{}", i));
            self.rust_arguments.push(format!("hi{}", i));
            return Ok(self);
        }

        if arg.is_ref_anyref() {
            self.js_arguments.push((name.clone(), "any".to_string()));
            self.cx.expose_borrowed_objects();
            self.finally("stack.pop();");
            self.rust_arguments
                .push(format!("addBorrowedObject({})", name));
            return Ok(self);
        }

        match *arg {
            Descriptor::Boolean => {
                self.js_arguments
                    .push((name.clone(), "boolean".to_string()));
                if self.cx.config.debug {
                    self.cx.expose_assert_bool();
                    self.prelude(&format!(
                        "\
                         _assertBoolean({name});\n\
                         ",
                        name = name
                    ));
                }
                self.rust_arguments.push(format!("{} ? 1 : 0", name));
            }
            Descriptor::Char => {
                self.js_arguments.push((name.clone(), "string".to_string()));
                self.rust_arguments.push(format!("{}.codePointAt(0)", name))
            }
            Descriptor::Anyref => {
                self.js_arguments.push((name.clone(), "any".to_string()));
                self.cx.expose_add_heap_object();
                self.rust_arguments.push(format!("addHeapObject({})", name));
            }
            _ => bail!("unsupported argument to rust function {:?}", arg),
        }
        Ok(self)
    }

    pub fn ret(&mut self, ret: &Option<Descriptor>) -> Result<&mut Self, Error> {
        let ty = match *ret {
            Some(ref t) => t,
            None => {
                self.ret_ty = "void".to_string();
                self.ret_expr = format!("return RET;");
                return Ok(self);
            }
        };

        if ty.is_ref_anyref() {
            self.ret_ty = "any".to_string();
            self.cx.expose_get_object();
            self.ret_expr = format!("return getObject(RET);");
            return Ok(self);
        }

        if ty.is_by_ref() {
            bail!("cannot return references from Rust to JS yet")
        }

        if let Some(ty) = ty.vector_kind() {
            self.ret_ty = ty.js_ty().to_string();
            let f = self.cx.expose_get_vector_from_wasm(ty);
            self.cx.expose_global_argument_ptr()?;
            self.cx.expose_uint32_memory();
            self.cx.require_internal_export("__wbindgen_free")?;
            self.prelude("const retptr = globalArgumentPtr();");
            self.rust_arguments.insert(0, "retptr".to_string());
            self.ret_expr = format!(
                "\
                 RET;\n\
                 const mem = getUint32Memory();\n\
                 const ptr = mem[retptr / 4];\n\
                 const len = mem[retptr / 4 + 1];\n\
                 const realRet = {}(ptr, len).slice();\n\
                 wasm.__wbindgen_free(ptr, len * {});\n\
                 return realRet;\n\
                 ",
                f,
                ty.size()
            );
            return Ok(self);
        }

        if let Some(name) = ty.rust_struct() {
            self.ret_ty = name.to_string();
            self.ret_expr = format!("return {name}.__construct(RET);", name = name);
            return Ok(self);
        }

        if ty.is_number() {
            self.ret_ty = "number".to_string();
            self.ret_expr = format!("return RET;");
            return Ok(self);
        }

        if let Some(signed) = ty.get_64bit() {
            self.ret_ty = "BigInt".to_string();
            self.cx.expose_global_argument_ptr()?;
            let f = if signed {
                self.cx.expose_int64_memory();
                "getInt64Memory"
            } else {
                self.cx.expose_uint64_memory();
                "getUint64Memory"
            };
            self.prelude("const retptr = globalArgumentPtr();");
            self.rust_arguments.insert(0, "retptr".to_string());
            self.ret_expr = format!(
                "\
                 RET;\n\
                 return {}()[retptr / 8];\n\
                 ",
                f
            );
            return Ok(self);
        }

        match *ty {
            Descriptor::Boolean => {
                self.ret_ty = "boolean".to_string();
                self.ret_expr = format!("return (RET) !== 0;");
            }
            Descriptor::Char => {
                self.ret_ty = "string".to_string();
                self.ret_expr = format!("return String.fromCodePoint(RET);")
            }
            Descriptor::Anyref => {
                self.ret_ty = "any".to_string();
                self.cx.expose_take_object();
                self.ret_expr = format!("return takeObject(RET);");
            }
            _ => bail!("unsupported return from Rust to JS {:?}", ty),
        }
        Ok(self)
    }

    /// Generate the actual function.
    ///
    /// The `prefix` specified is typically the string "function" but may be
    /// different for classes. The `invoc` is the function expression that we're
    /// invoking, like `wasm.bar` or `this.f`.
    ///
    /// Returns two strings, the first of which is the JS expression for the
    /// generated function shim and the second is a TypeScript signature of the
    /// JS expression.
    pub fn finish(&self, prefix: &str, invoc: &str) -> (String, String) {
        let js_args = self
            .js_arguments
            .iter()
            .map(|s| &s.0[..])
            .collect::<Vec<_>>()
            .join(", ");
        let mut js = format!("{}({}) {{\n", prefix, js_args);
        js.push_str(&self.prelude);
        let rust_args = self.rust_arguments.join(", ");

        let invoc = self
            .ret_expr
            .replace("RET", &format!("{}({})", invoc, rust_args));
        let invoc = if self.finally.len() == 0 {
            invoc
        } else {
            format!(
                "\
                try {{\n\
                    {}
                \n}} finally {{\n\
                    {}
                }}\n\
                ",
                &invoc, &self.finally,
            )
        };
        js.push_str(&invoc);
        js.push_str("\n}");
        let ts_args = self
            .js_arguments
            .iter()
            .map(|s| format!("{}: {}", s.0, s.1))
            .collect::<Vec<_>>()
            .join(", ");
        let ts = format!(
            "{} {}({}): {};\n",
            prefix, self.js_name, ts_args, self.ret_ty
        );
        (js, ts)
    }
}
