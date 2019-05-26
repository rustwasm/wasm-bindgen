use crate::descriptor::{Descriptor, Function};
use crate::js::Context;
use failure::{bail, Error};

pub struct JsArgument {
    pub optional: bool,
    pub name: String,
    pub type_: String,
}

impl JsArgument {
    fn required(name: String, type_: String) -> Self {
        Self { optional: false, name, type_ }
    }

    fn optional(name: String, type_: String) -> Self {
        Self { optional: true, name, type_ }
    }
}

/// Helper struct for manufacturing a shim in JS used to translate JS types to
/// Rust, aka pass from JS back into Rust
pub struct Js2Rust<'a, 'b: 'a> {
    cx: &'a mut Context<'b>,

    /// Arguments passed to the invocation of the wasm function, aka things that
    /// are only numbers.
    rust_arguments: Vec<String>,

    /// Arguments and their types to the JS shim.
    pub js_arguments: Vec<JsArgument>,

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

    /// whether or not this generated function body will act like a constructor,
    /// meaning it doesn't actually return something but rather assigns to
    /// `this`
    ///
    /// The string value here is the class that this should be a constructor
    /// for.
    constructor: Option<String>,

    /// metadata for anyref transformations
    anyref_args: Vec<(usize, bool)>,
    ret_anyref: bool,
}

pub enum ExportedShim<'a> {
    Named(&'a str),
    TableElement(&'a mut u32),
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
            constructor: None,
            anyref_args: Vec::new(),
            ret_anyref: false,
        }
    }

    /// Generates all bindings necessary for the signature in `Function`,
    /// creating necessary argument conversions and return value processing.
    pub fn process<'c, I>(
        &mut self,
        function: &Function,
        opt_arg_names: I,
    ) -> Result<&mut Self, Error>
    where
        I: Into<Option<&'c Vec<String>>>,
    {
        if let Some(arg_names) = opt_arg_names.into() {
            for (arg, arg_name) in function.arguments.iter().zip(arg_names) {
                self.argument(arg, arg_name.as_str())?;
            }
        } else {
            for arg in function.arguments.iter() {
                self.argument(arg, None)?;
            }
        }
        self.ret(&function.ret)?;
        Ok(self)
    }

    pub fn constructor(&mut self, class: Option<&str>) -> &mut Self {
        self.constructor = class.map(|s| s.to_string());
        self
    }

    /// Flag this shim as a method call into Rust, so the first Rust argument
    /// passed should be `this.ptr`.
    pub fn method(&mut self, consumed: bool) -> &mut Self {
        if self.cx.config.debug {
            self.prelude(
                "if (this.ptr === 0) {
                        throw new Error('Attempt to use a moved value');
                }",
            );
        }
        if consumed {
            self.prelude(
                "\
                 const ptr = this.ptr;\n\
                 this.ptr = 0;\n\
                 ",
            );
            self.rust_arguments.insert(0, "ptr".to_string());
        } else {
            self.rust_arguments.insert(0, "this.ptr".to_string());
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

    fn abi_arg(&mut self, opt_arg_name: Option<&str>) -> String {
        let ret = if let Some(x) = opt_arg_name {
            x.into()
        } else {
            format!("arg{}", self.arg_idx)
        };
        self.arg_idx += 1;
        ret
    }

    pub fn argument<'c, I>(&mut self, arg: &Descriptor, opt_arg_name: I) -> Result<&mut Self, Error>
    where
        I: Into<Option<&'c str>>,
    {
        let i = self.arg_idx;
        let name = self.abi_arg(opt_arg_name.into());

        let (arg, optional) = match arg {
            Descriptor::Option(t) => (&**t, true),
            _ => (arg, false),
        };

        if let Some(kind) = arg.vector_kind() {
            self.js_arguments
                .push(JsArgument::required(name.clone(), kind.js_ty().to_string()));

            let func = self.cx.pass_to_wasm_function(kind)?;
            let val = if optional {
                self.cx.expose_is_like_none();
                format!("isLikeNone({}) ? [0, 0] : {}({})", name, func, name)
            } else {
                format!("{}({})", func, name)
            };
            self.prelude(&format!(
                "const ptr{i} = {val};\nconst len{i} = WASM_VECTOR_LEN;",
                i = i,
                val = val,
            ));
            if arg.is_by_ref() || arg.is_clamped_by_ref() {
                if optional {
                    bail!("optional slices aren't currently supported");
                }
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

        if arg.is_anyref() {
            self.js_arguments.push(JsArgument::required(name.clone(), "any".to_string()));
            if self.cx.config.anyref {
                if optional {
                    self.cx.expose_add_to_anyref_table()?;
                    self.cx.expose_is_like_none();
                    self.rust_arguments
                        .push(format!("isLikeNone({0}) ? 0 : addToAnyrefTable({0})", name));
                } else {
                    self.anyref_args.push((self.rust_arguments.len(), true));
                    self.rust_arguments.push(name);
                }
            } else {
                self.cx.expose_add_heap_object();
                if optional {
                    self.cx.expose_is_like_none();
                    self.rust_arguments
                        .push(format!("isLikeNone({0}) ? 0 : addHeapObject({0})", name));
                } else {
                    self.rust_arguments.push(format!("addHeapObject({})", name));
                }
            }
            return Ok(self);
        }

        if optional {
            self.cx.expose_is_like_none();

            if arg.is_wasm_native() {
                self.js_arguments
                    .push(JsArgument::optional(name.clone(), "number".to_string()));

                if self.cx.config.debug {
                    self.cx.expose_assert_num();
                    self.prelude(&format!(
                        "
                            if (!isLikeNone({0})) {{
                                _assertNum({0});
                            }}
                        ",
                        name
                    ));
                }

                self.rust_arguments.push(format!("!isLikeNone({0})", name));
                self.rust_arguments
                    .push(format!("isLikeNone({0}) ? 0 : {0}", name));
                return Ok(self);
            }

            if arg.is_abi_as_u32() {
                self.js_arguments
                    .push(JsArgument::optional(name.clone(), "number".to_string()));

                if self.cx.config.debug {
                    self.cx.expose_assert_num();
                    self.prelude(&format!(
                        "
                            if (!isLikeNone({0})) {{
                                _assertNum({0});
                            }}
                        ",
                        name
                    ));
                }

                self.rust_arguments
                    .push(format!("isLikeNone({0}) ? 0xFFFFFF : {0}", name));
                return Ok(self);
            }

            if let Some(signed) = arg.get_64() {
                let f = if signed {
                    self.cx.expose_int64_cvt_shim()
                } else {
                    self.cx.expose_uint64_cvt_shim()
                };
                self.cx.expose_uint32_memory();
                self.js_arguments
                    .push(JsArgument::optional(name.clone(), "BigInt".to_string()));
                self.prelude(&format!(
                    "
                        {f}[0] = isLikeNone({name}) ? BigInt(0) : {name};
                        const low{i} = isLikeNone({name}) ? 0 : u32CvtShim[0];
                        const high{i} = isLikeNone({name}) ? 0 : u32CvtShim[1];
                    ",
                    i = i,
                    f = f,
                    name = name,
                ));
                self.rust_arguments.push(format!("!isLikeNone({})", name));
                self.rust_arguments.push(format!("0"));
                self.rust_arguments.push(format!("low{}", i));
                self.rust_arguments.push(format!("high{}", i));
                return Ok(self);
            }

            match *arg {
                Descriptor::Boolean => {
                    self.js_arguments
                        .push(JsArgument::optional(name.clone(), "boolean".to_string()));
                    if self.cx.config.debug {
                        self.cx.expose_assert_bool();
                        self.prelude(&format!(
                            "
                                if (!isLikeNone({0})) {{
                                    _assertBoolean({0});
                                }}
                            ",
                            name,
                        ));
                    }
                    self.rust_arguments
                        .push(format!("isLikeNone({0}) ? 0xFFFFFF : {0} ? 1 : 0", name));
                }
                Descriptor::Char => {
                    self.js_arguments
                        .push(JsArgument::optional(name.clone(), "string".to_string()));
                    self.rust_arguments.push(format!(
                        "isLikeNone({0}) ? 0xFFFFFF : {0}.codePointAt(0)",
                        name
                    ));
                }
                Descriptor::Enum { hole } => {
                    self.js_arguments
                        .push(JsArgument::optional(name.clone(), "number".to_string()));
                    self.rust_arguments
                        .push(format!("isLikeNone({0}) ? {1} : {0}", name, hole));
                }
                Descriptor::RustStruct(ref s) => {
                    self.js_arguments
                        .push(JsArgument::optional(name.clone(), s.to_string()));
                    self.prelude(&format!("let ptr{} = 0;", i));
                    self.prelude(&format!("if (!isLikeNone({0})) {{", name));
                    self.assert_class(&name, s);
                    self.assert_not_moved(&name);
                    self.prelude(&format!("ptr{} = {}.ptr;", i, name));
                    self.prelude(&format!("{}.ptr = 0;", name));
                    self.prelude("}");
                    self.rust_arguments.push(format!("ptr{}", i));
                }
                _ => bail!(
                    "unsupported optional argument type for calling Rust function from JS: {:?}",
                    arg
                ),
            }

            return Ok(self);
        }

        if let Some(s) = arg.rust_struct() {
            self.js_arguments.push(JsArgument::required(name.clone(), s.to_string()));
            self.assert_class(&name, s);
            self.assert_not_moved(&name);
            if arg.is_by_ref() {
                self.rust_arguments.push(format!("{}.ptr", name));
            } else {
                self.prelude(&format!("const ptr{} = {}.ptr;", i, name));
                self.prelude(&format!("{}.ptr = 0;", name));
                self.rust_arguments.push(format!("ptr{}", i));
            }
            return Ok(self);
        }

        if arg.number().is_some() {
            self.js_arguments.push(JsArgument::required(name.clone(), "number".to_string()));

            if self.cx.config.debug {
                self.cx.expose_assert_num();
                self.prelude(&format!("_assertNum({});", name));
            }

            self.rust_arguments.push(name);
            return Ok(self);
        }

        if let Some(signed) = arg.get_64() {
            let f = if signed {
                self.cx.expose_int64_cvt_shim()
            } else {
                self.cx.expose_uint64_cvt_shim()
            };
            self.cx.expose_uint32_memory();
            self.js_arguments.push(JsArgument::required(name.clone(), "BigInt".to_string()));
            self.prelude(&format!(
                "
                 {f}[0] = {name};
                 const low{i} = u32CvtShim[0];
                 const high{i} = u32CvtShim[1];
                 ",
                i = i,
                f = f,
                name = name,
            ));
            self.rust_arguments.push(format!("low{}", i));
            self.rust_arguments.push(format!("high{}", i));
            return Ok(self);
        }

        if arg.is_ref_anyref() {
            self.js_arguments.push(JsArgument::required(name.clone(), "any".to_string()));
            if self.cx.config.anyref {
                self.anyref_args.push((self.rust_arguments.len(), false));
                self.rust_arguments.push(name);
            } else {
                // the "stack-ful" nature means that we're always popping from the
                // stack, and make sure that we actually clear our reference to
                // allow stale values to get GC'd
                self.cx.expose_borrowed_objects();
                self.cx.expose_global_stack_pointer();
                self.finally("heap[stack_pointer++] = undefined;");
                self.rust_arguments
                    .push(format!("addBorrowedObject({})", name));
            }
            return Ok(self);
        }

        match *arg {
            Descriptor::Boolean => {
                self.js_arguments
                    .push(JsArgument::required(name.clone(), "boolean".to_string()));
                if self.cx.config.debug {
                    self.cx.expose_assert_bool();
                    self.prelude(&format!(
                        "\
                         _assertBoolean({name});\n\
                         ",
                        name = name
                    ));
                }
                self.rust_arguments.push(format!("{}", name));
            }
            Descriptor::Char => {
                self.js_arguments.push(JsArgument::required(name.clone(), "string".to_string()));
                self.rust_arguments.push(format!("{}.codePointAt(0)", name))
            }
            _ => bail!(
                "unsupported argument type for calling Rust function from JS: {:?}",
                arg
            ),
        }
        Ok(self)
    }

    pub fn ret(&mut self, ty: &Descriptor) -> Result<&mut Self, Error> {
        if let Some(name) = ty.rust_struct() {
            match &self.constructor {
                Some(class) if class == name => {
                    self.ret_expr = format!("this.ptr = RET;");
                    if self.cx.config.weak_refs {
                        self.ret_expr.push_str(&format!(
                            "\
                             {}FinalizationGroup.register(this, this.ptr, this.ptr);
                             ",
                            name
                        ));
                    }
                }
                Some(class) => bail!("constructor for `{}` cannot return `{}`", class, name),
                None => {
                    self.ret_ty = name.to_string();
                    self.cx.require_class_wrap(name);
                    self.ret_expr = format!("return {name}.__wrap(RET);", name = name);
                }
            }
            return Ok(self);
        }

        if self.constructor.is_some() {
            bail!("constructor functions must return a Rust structure")
        }

        if let Descriptor::Unit = ty {
            self.ret_ty = "void".to_string();
            self.ret_expr = format!("return RET;");
            return Ok(self);
        }

        let (ty, optional) = match ty {
            Descriptor::Option(t) => (&**t, true),
            _ => (ty, false),
        };

        if let Some(ty) = ty.vector_kind() {
            self.ret_ty = ty.js_ty().to_string();
            let f = self.cx.expose_get_vector_from_wasm(ty)?;
            self.cx.expose_global_argument_ptr()?;
            self.cx.expose_uint32_memory();
            self.cx.require_internal_export("__wbindgen_free")?;
            self.prelude("const retptr = globalArgumentPtr();");
            self.rust_arguments.insert(0, "retptr".to_string());
            self.ret_expr = format!(
                "\
                 RET;\n\
                 const mem = getUint32Memory();\n\
                 const rustptr = mem[retptr / 4];\n\
                 const rustlen = mem[retptr / 4 + 1];\n\
                 {guard}
                 const realRet = {}(rustptr, rustlen).slice();\n\
                 wasm.__wbindgen_free(rustptr, rustlen * {});\n\
                 return realRet;\n\
                 ",
                f,
                ty.size(),
                guard = if optional {
                    "if (rustptr === 0) return;"
                } else {
                    ""
                },
            );
            return Ok(self);
        }

        // No need to worry about `optional` here, the abi representation means
        // that `takeObject` will naturally pluck out `undefined`.
        if ty.is_anyref() {
            self.ret_ty = "any".to_string();
            self.ret_expr = format!("return {};", self.cx.take_object("RET"));
            self.ret_anyref = true;
            return Ok(self);
        }

        if optional {
            if ty.is_wasm_native() {
                self.ret_ty = "number | undefined".to_string();
                self.cx.expose_global_argument_ptr()?;
                self.cx.expose_uint32_memory();
                match ty {
                    Descriptor::I32 => self.cx.expose_int32_memory(),
                    Descriptor::U32 => (),
                    Descriptor::F32 => self.cx.expose_f32_memory(),
                    Descriptor::F64 => self.cx.expose_f64_memory(),
                    _ => (),
                };
                self.prelude("const retptr = globalArgumentPtr();");
                self.rust_arguments.insert(0, "retptr".to_string());
                self.ret_expr = format!(
                    "
                        RET;
                        const present = getUint32Memory()[retptr / 4];
                        const value = {mem}[retptr / {size} + 1];
                        return present === 0 ? undefined : value;
                    ",
                    size = match ty {
                        Descriptor::I32 => 4,
                        Descriptor::U32 => 4,
                        Descriptor::F32 => 4,
                        Descriptor::F64 => 8,
                        _ => unreachable!(),
                    },
                    mem = match ty {
                        Descriptor::I32 => "getInt32Memory()",
                        Descriptor::U32 => "getUint32Memory()",
                        Descriptor::F32 => "getFloat32Memory()",
                        Descriptor::F64 => "getFloat64Memory()",
                        _ => unreachable!(),
                    }
                );
                return Ok(self);
            }

            if ty.is_abi_as_u32() {
                self.ret_ty = "number | undefined".to_string();
                self.ret_expr = "
                    const ret = RET;
                    return ret === 0xFFFFFF ? undefined : ret;
                "
                .to_string();
                return Ok(self);
            }

            if let Some(signed) = ty.get_64() {
                self.ret_ty = "BigInt | undefined".to_string();
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
                    "
                        RET;
                        const present = getUint32Memory()[retptr / 4];
                        const value = {}()[retptr / 8 + 1];
                        return present === 0 ? undefined : value;
                    ",
                    f
                );
                return Ok(self);
            }

            match *ty {
                Descriptor::Boolean => {
                    self.ret_ty = "boolean | undefined".to_string();
                    self.ret_expr = "
                        const ret = RET;
                        return ret === 0xFFFFFF ? undefined : ret !== 0;
                    "
                    .to_string();
                    return Ok(self);
                }
                Descriptor::Char => {
                    self.ret_ty = "string | undefined".to_string();
                    self.ret_expr = "
                        const ret = RET;
                        return ret === 0xFFFFFF ? undefined : String.fromCodePoint(ret);
                    "
                    .to_string();
                    return Ok(self);
                }
                Descriptor::Enum { hole } => {
                    self.ret_ty = "number | undefined".to_string();
                    self.ret_expr = format!(
                        "
                        const ret = RET;
                        return ret === {} ? undefined : ret;
                    ",
                        hole
                    );
                    return Ok(self);
                }
                Descriptor::RustStruct(ref name) => {
                    self.ret_ty = format!("{} | undefined", name);
                    self.cx.require_class_wrap(name);
                    self.ret_expr = format!(
                        "
                        const ptr = RET;
                        return ptr === 0 ? undefined : {}.__wrap(ptr);
                    ",
                        name,
                    );
                    return Ok(self);
                }
                _ => bail!(
                    "unsupported optional return type for calling Rust function from JS: {:?}",
                    ty
                ),
            };
        }

        if ty.is_ref_anyref() {
            self.ret_ty = "any".to_string();
            self.cx.expose_get_object();
            self.ret_expr = format!("return getObject(RET);");
            return Ok(self);
        }

        if ty.is_by_ref() {
            bail!("cannot return references from Rust to JS yet")
        }

        if let Some(name) = ty.rust_struct() {
            self.ret_ty = name.to_string();
            self.cx.require_class_wrap(name);
            self.ret_expr = format!("return {name}.__wrap(RET);", name = name);
            return Ok(self);
        }

        if let Some(num) = ty.number() {
            self.ret_ty = "number".to_string();
            if num.is_u32() {
                self.ret_expr = format!("return RET >>> 0;");
            } else {
                self.ret_expr = format!("return RET;");
            }
            return Ok(self);
        }

        if let Some(signed) = ty.get_64() {
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
            _ => bail!(
                "unsupported return type for calling Rust function from JS: {:?}",
                ty
            ),
        }
        Ok(self)
    }

    pub fn js_doc_comments(&self) -> String {
        let mut ret: String = self
            .js_arguments
            .iter()
            .map(|a| if a.optional {
                format!("@param {{{} | undefined}} {}\n", a.type_, a.name)
            } else {
                format!("@param {{{}}} {}\n", a.type_, a.name)
            })
            .collect();
        ret.push_str(&format!("@returns {{{}}}", self.ret_ty));
        ret
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
    pub fn finish(
        &mut self,
        prefix: &str,
        invoc: &str,
        exported_shim: ExportedShim,
    ) -> (String, String, String) {
        let js_args = self
            .js_arguments
            .iter()
            .map(|s| &s.name[..])
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

        // Determine TS parameter list
        let mut omittable = true;
        let mut ts_args = Vec::with_capacity(self.js_arguments.len());
        for arg in self.js_arguments.iter().rev() {
            // In TypeScript, we can mark optional parameters as omittable
            // using the `?` suffix, but only if they're not followed by
            // non-omittable parameters. Therefore iterate the parameter list
            // in reverse and stop using the `?` suffix for optional params as
            // soon as a non-optional parameter is encountered.
            if arg.optional {
                if omittable {
                    ts_args.push(format!("{}?: {}", arg.name, arg.type_));
                } else {
                    ts_args.push(format!("{}: {} | undefined", arg.name, arg.type_));
                }
            } else {
                omittable = false;
                ts_args.push(format!("{}: {}", arg.name, arg.type_));
            }
        }
        ts_args.reverse();
        let ts_args = ts_args.join(", ");

        let mut ts = if prefix.is_empty() {
            format!("{}({})", self.js_name, ts_args)
        } else {
            format!("{} {}({})", prefix, self.js_name, ts_args)
        };
        if self.constructor.is_none() {
            ts.push_str(": ");
            ts.push_str(&self.ret_ty);
        }
        ts.push(';');

        if self.ret_anyref || self.anyref_args.len() > 0 {
            match exported_shim {
                ExportedShim::Named(name) => {
                    self.cx
                        .anyref
                        .export_xform(name, &self.anyref_args, self.ret_anyref);
                }
                ExportedShim::TableElement(idx) => {
                    *idx = self.cx.anyref.table_element_xform(
                        *idx,
                        &self.anyref_args,
                        self.ret_anyref,
                    );
                }
            }
        }

        (js, ts, self.js_doc_comments())
    }

    fn assert_class(&mut self, arg: &str, class: &str) {
        if !self.cx.config.debug {
            return;
        }
        self.cx.expose_assert_class();
        self.prelude(&format!("_assertClass({}, {});", arg, class));
    }

    fn assert_not_moved(&mut self, arg: &str) {
        if !self.cx.config.debug {
            return;
        }
        self.prelude(&format!(
            "\
                if ({0}.ptr === 0) {{
                    throw new Error('Attempt to use a moved value');
                }}
            ",
            arg,
        ));
    }
}
