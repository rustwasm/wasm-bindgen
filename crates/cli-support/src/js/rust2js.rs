use crate::descriptor::Descriptor;
use crate::intrinsic::Intrinsic;
use crate::js::{Context, Js2Rust};
use crate::webidl::{AuxImport, AuxValue, ImportBinding};
use failure::{bail, Error};

/// Helper struct for manufacturing a shim in JS used to translate Rust types to
/// JS, then invoking an imported JS function.
pub struct Rust2Js<'a, 'b: 'a> {
    cx: &'a mut Context<'b>,

    /// Arguments of the JS shim that we're generating, aka the variables passed
    /// from Rust which are only numbers.
    shim_arguments: Vec<String>,

    /// Arguments which are forwarded to the imported JS function
    js_arguments: Vec<String>,

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

    /// Expression used to generate the return value. The string "JS" in this
    /// expression is replaced with the actual JS invocation eventually.
    ret_expr: String,

    /// Whether or not we're catching JS exceptions
    catch: bool,
    catch_and_rethrow: bool,

    /// Whether or not the last argument is a slice representing variadic arguments.
    variadic: bool,

    /// What sort of style this invocation will be like, see the variants of
    /// this enum for more information.
    style: Style,

    /// list of arguments that are anyref, and whether they're an owned anyref
    /// or not.
    anyref_args: Vec<(usize, bool)>,
    ret_anyref: bool,
}

#[derive(PartialEq)]
enum Style {
    /// The imported function is expected to be invoked with `new` to create a
    /// JS object.
    Constructor,
    /// The imported function is expected to be invoked where the first
    /// parameter is the `this` and the rest of the arguments are the
    /// function's arguments.
    Method,
    /// Just a normal function call.
    Function,
}

impl<'a, 'b> Rust2Js<'a, 'b> {
    pub fn new(cx: &'a mut Context<'b>) -> Rust2Js<'a, 'b> {
        Rust2Js {
            cx,
            shim_arguments: Vec::new(),
            js_arguments: Vec::new(),
            prelude: String::new(),
            finally: String::new(),
            global_idx: 0,
            arg_idx: 0,
            ret_expr: String::new(),
            catch: false,
            catch_and_rethrow: false,
            variadic: false,
            anyref_args: Vec::new(),
            ret_anyref: false,
            style: Style::Function,
        }
    }

    pub fn catch(&mut self, catch: bool) -> &mut Self {
        self.catch = catch;
        self
    }

    pub fn catch_and_rethrow(&mut self, catch_and_rethrow: bool) -> &mut Self {
        self.catch_and_rethrow = catch_and_rethrow;
        self
    }

    pub fn variadic(&mut self, variadic: bool) -> &mut Self {
        self.variadic = variadic;
        self
    }

    /// Generates all bindings necessary for the signature in `Function`,
    /// creating necessary argument conversions and return value processing.
    pub fn process(&mut self, binding: &ImportBinding) -> Result<&mut Self, Error> {
        let function = match binding {
            ImportBinding::Constructor(f) => {
                self.style = Style::Constructor;
                f
            }
            ImportBinding::Method(f) => {
                self.style = Style::Method;
                f
            }
            ImportBinding::Function(f) => {
                self.style = Style::Function;
                f
            }
        };
        for arg in function.arguments.iter() {
            // Process the function argument and assert that the metadata about
            // the number of arguments on the Rust side required is correct.
            let before = self.shim_arguments.len();
            self.argument(arg)?;
            arg.assert_abi_arg_correct(before, self.shim_arguments.len());
        }
        // Process the return argument, and assert that the metadata returned
        // about the descriptor is indeed correct.
        let before = self.shim_arguments.len();
        self.ret(&function.ret)?;
        function
            .ret
            .assert_abi_return_correct(before, self.shim_arguments.len());
        Ok(self)
    }

    /// Get a generated name for an argument.
    fn shim_argument(&mut self) -> String {
        let s = format!("arg{}", self.arg_idx);
        self.arg_idx += 1;
        self.shim_arguments.push(s.clone());
        s
    }

    fn argument(&mut self, arg: &Descriptor) -> Result<(), Error> {
        let abi = self.shim_argument();

        let (arg, optional) = match arg {
            Descriptor::Option(t) => (&**t, true),
            _ => (arg, false),
        };

        if let Some(ty) = arg.vector_kind() {
            let abi2 = self.shim_argument();
            let f = self.cx.expose_get_vector_from_wasm(ty)?;
            self.prelude(&format!(
                "let v{0} = {prefix}{func}({0}, {1});",
                abi,
                abi2,
                func = f,
                prefix = if optional {
                    format!("{} == 0 ? undefined : ", abi)
                } else {
                    String::new()
                },
            ));

            if !arg.is_by_ref() {
                self.prelude(&format!(
                    "\
                     {start}
                     v{0} = v{0}.slice();
                     wasm.__wbindgen_free({0}, {1} * {size});
                     {end}\
                     ",
                    abi,
                    abi2,
                    size = ty.size(),
                    start = if optional {
                        format!("if ({} !== 0) {{", abi)
                    } else {
                        String::new()
                    },
                    end = if optional { "}" } else { "" },
                ));
                self.cx.require_internal_export("__wbindgen_free")?;
            }
            self.js_arguments.push(format!("v{}", abi));
            return Ok(());
        }

        // No need to special case `optional` here because `takeObject` will
        // naturally work.
        if arg.is_anyref() {
            let arg = self.cx.take_object(&abi);
            self.js_arguments.push(arg);
            self.anyref_args.push((self.arg_idx - 1, true));
            return Ok(());
        } else if arg.is_ref_anyref() {
            let arg = self.cx.get_object(&abi);
            self.js_arguments.push(arg);
            self.anyref_args.push((self.arg_idx - 1, false));
            return Ok(());
        }

        if optional {
            if arg.is_wasm_native() {
                let value = self.shim_argument();
                self.js_arguments.push(format!(
                    "{present} === 0 ? undefined : {value}",
                    value = value,
                    present = abi,
                ));
                return Ok(());
            }

            if arg.is_abi_as_u32() {
                self.js_arguments
                    .push(format!("{0} === 0xFFFFFF ? undefined : {0}", abi));
                return Ok(());
            }

            if let Some(signed) = arg.get_64() {
                let f = if signed {
                    self.cx.expose_int64_cvt_shim()
                } else {
                    self.cx.expose_uint64_cvt_shim()
                };
                self.shim_argument();
                let low = self.shim_argument();
                let high = self.shim_argument();
                let name = format!("n{}", abi);
                self.prelude(&format!(
                    "
                        u32CvtShim[0] = {present} === 0 ? 0 : {low};
                        u32CvtShim[1] = {present} === 0 ? 0 : {high};
                        const {name} = {present} === 0 ? undefined : {f}[0];
                    ",
                    present = abi,
                    low = low,
                    high = high,
                    f = f,
                    name = name,
                ));
                self.js_arguments.push(name);
                return Ok(());
            }

            match *arg {
                Descriptor::Boolean => {
                    self.js_arguments
                        .push(format!("{0} === 0xFFFFFF ? undefined : {0} !== 0", abi));
                    return Ok(());
                }
                Descriptor::Enum { hole } => {
                    self.js_arguments
                        .push(format!("{0} === {1} ? undefined : {0}", abi, hole));
                    return Ok(());
                }
                Descriptor::Char => {
                    self.js_arguments.push(format!(
                        "{0} === 0xFFFFFF ? undefined : String.fromCodePoint({0})",
                        abi
                    ));
                    return Ok(());
                }
                Descriptor::RustStruct(ref class) => {
                    self.cx.require_class_wrap(class);
                    let assign = format!(
                        "let c{0} = {0} === 0 ? undefined : {1}.__wrap({0});",
                        abi, class
                    );
                    self.prelude(&assign);
                    self.js_arguments.push(format!("c{}", abi));
                    return Ok(());
                }
                _ => bail!(
                    "unsupported optional argument type for calling JS function from Rust: {:?}",
                    arg
                ),
            };
        }

        if let Some(signed) = arg.get_64() {
            let f = if signed {
                self.cx.expose_int64_cvt_shim()
            } else {
                self.cx.expose_uint64_cvt_shim()
            };
            let high = self.shim_argument();
            let name = format!("n{}", abi);
            self.prelude(&format!(
                "\
                 u32CvtShim[0] = {low};
                 u32CvtShim[1] = {high};
                 const {name} = {f}[0];
                 ",
                low = abi,
                high = high,
                f = f,
                name = name,
            ));
            self.js_arguments.push(name);
            return Ok(());
        }

        if let Some(class) = arg.rust_struct() {
            if arg.is_by_ref() {
                bail!("cannot invoke JS functions with custom ref types yet")
            }
            self.cx.require_class_wrap(class);
            let assign = format!("let c{0} = {1}.__wrap({0});", abi, class);
            self.prelude(&assign);
            self.js_arguments.push(format!("c{}", abi));
            return Ok(());
        }

        if let Some((f, mutable)) = arg.stack_closure() {
            let arg2 = self.shim_argument();
            let (js, _ts, _js_doc) = {
                let mut builder = Js2Rust::new("", self.cx);
                if mutable {
                    builder
                        .prelude("let a = this.a;\n")
                        .prelude("this.a = 0;\n")
                        .rust_argument("a")
                        .finally("this.a = a;\n");
                } else {
                    builder.rust_argument("this.a");
                }
                builder
                    .rust_argument("this.b")
                    .process(f, &None)?
                    .finish("function", "this.f")
            };
            self.cx.export_function_table()?;
            self.global_idx();
            self.prelude(&format!(
                "\
                 let cb{0} = {js};\n\
                 cb{0}.f = wasm.__wbg_function_table.get({idx});\n\
                 cb{0}.a = {0};\n\
                 cb{0}.b = {1};\n\
                 ",
                abi,
                arg2,
                js = js,
                idx = f.shim_idx,
            ));
            self.finally(&format!("cb{0}.a = cb{0}.b = 0;", abi));
            self.js_arguments.push(format!("cb{0}.bind(cb{0})", abi));
            return Ok(());
        }

        if let Some(num) = arg.number() {
            if num.is_u32() {
                self.js_arguments.push(format!("{} >>> 0", abi));
            } else {
                self.js_arguments.push(abi);
            }
            return Ok(());
        }

        let invoc_arg = match *arg {
            Descriptor::Boolean => format!("{} !== 0", abi),
            Descriptor::Char => format!("String.fromCodePoint({})", abi),
            _ => bail!(
                "unsupported argument type for calling JS function from Rust: {:?}",
                arg
            ),
        };
        self.js_arguments.push(invoc_arg);
        Ok(())
    }

    fn ret(&mut self, ty: &Descriptor) -> Result<(), Error> {
        if let Descriptor::Unit = ty {
            self.ret_expr = "JS;".to_string();
            return Ok(());
        }
        let (ty, optional) = match ty {
            Descriptor::Option(t) => (&**t, true),
            _ => (ty, false),
        };
        if ty.is_by_ref() {
            bail!("cannot return a reference from JS to Rust")
        }
        if let Some(ty) = ty.vector_kind() {
            let f = self.cx.pass_to_wasm_function(ty)?;
            self.cx.expose_uint32_memory();
            self.shim_arguments.insert(0, "ret".to_string());
            let mut prelude = String::new();
            let expr = if optional {
                prelude.push_str("const val = JS;");
                self.cx.expose_is_like_none();
                format!("isLikeNone(val) ? [0, 0] : {}(val)", f)
            } else {
                format!("{}(JS)", f)
            };
            self.ret_expr = format!(
                "\
                {}
                const retptr = {};
                const retlen = WASM_VECTOR_LEN;
                const mem = getUint32Memory();
                mem[ret / 4] = retptr;
                mem[ret / 4 + 1] = retlen;
                ",
                prelude, expr
            );
            return Ok(());
        }
        if ty.is_anyref() {
            if self.cx.config.anyref {
                if optional {
                    self.cx.expose_add_to_anyref_table()?;
                    self.cx.expose_is_like_none();
                    self.ret_expr = "
                        const val = JS;
                        return isLikeNone(val) ? 0 : addToAnyrefTable(val);
                    "
                    .to_string();
                } else {
                    self.ret_anyref = true;
                    self.ret_expr = "return JS;".to_string()
                }
            } else {
                self.cx.expose_add_heap_object();
                if optional {
                    self.cx.expose_is_like_none();
                    self.ret_expr = "
                        const val = JS;
                        return isLikeNone(val) ? 0 : addHeapObject(val);
                    "
                    .to_string();
                } else {
                    self.ret_expr = "return addHeapObject(JS);".to_string()
                }
            }
            return Ok(());
        }
        if optional {
            self.cx.expose_is_like_none();

            if ty.is_wasm_native() {
                self.cx.expose_uint32_memory();
                match ty {
                    Descriptor::I32 => self.cx.expose_int32_memory(),
                    Descriptor::U32 => (),
                    Descriptor::F32 => self.cx.expose_f32_memory(),
                    Descriptor::F64 => self.cx.expose_f64_memory(),
                    _ => (),
                };
                self.shim_arguments.insert(0, "ret".to_string());
                self.ret_expr = format!(
                    "
                        const val = JS;
                        getUint32Memory()[ret / 4] = !isLikeNone(val);
                        {mem}[ret / {size} + 1] = isLikeNone(val) ? 0 : val;
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
                return Ok(());
            }

            if ty.is_abi_as_u32() {
                self.ret_expr = "
                    const val = JS;
                    return isLikeNone(val) ? 0xFFFFFF : val;
                "
                .to_string();
                return Ok(());
            }

            if let Some(signed) = ty.get_64() {
                self.cx.expose_uint32_memory();
                let f = if signed {
                    self.cx.expose_int64_memory();
                    "getInt64Memory"
                } else {
                    self.cx.expose_uint64_memory();
                    "getUint64Memory"
                };
                self.shim_arguments.insert(0, "ret".to_string());
                self.ret_expr = format!(
                    "
                        const val = JS;
                        getUint32Memory()[ret / 4] = !isLikeNone(val);
                        {}()[ret / 8 + 1] = isLikeNone(val) ? BigInt(0) : val;
                    ",
                    f
                );
                return Ok(());
            }

            match *ty {
                Descriptor::Boolean => {
                    self.ret_expr = "
                        const val = JS;
                        return isLikeNone(val) ? 0xFFFFFF : val ? 1 : 0;
                    "
                    .to_string();
                }
                Descriptor::Char => {
                    self.ret_expr = "
                        const val = JS;
                        return isLikeNone(val) ? 0xFFFFFF : val.codePointAt(0);
                    "
                    .to_string();
                }
                Descriptor::Enum { hole } => {
                    self.ret_expr = format!(
                        "
                        const val = JS;
                        return isLikeNone(val) ? {} : val;
                    ",
                        hole
                    );
                }
                Descriptor::RustStruct(ref class) => {
                    // Like below, assert the type
                    self.ret_expr = format!(
                        "\
                        const val = JS;
                        if (isLikeNone(val))
                            return 0;
                        if (!(val instanceof {0})) {{
                            throw new Error('expected value of type {0}');
                        }}
                        const ret = val.ptr;
                        val.ptr = 0;
                        return ret;\
                    ",
                        class
                    );
                }
                _ => bail!(
                    "unsupported optional return type for calling JS function from Rust: {:?}",
                    ty
                ),
            };

            return Ok(());
        }
        if ty.number().is_some() {
            self.ret_expr = "return JS;".to_string();
            return Ok(());
        }
        if let Some(signed) = ty.get_64() {
            let f = if signed {
                self.cx.expose_int64_memory();
                "getInt64Memory"
            } else {
                self.cx.expose_uint64_memory();
                "getUint64Memory"
            };
            self.shim_arguments.insert(0, "ret".to_string());
            self.ret_expr = format!(
                "\
                 const val = JS;\n\
                 {}()[ret / 8] = val;\n\
                 ",
                f
            );
            return Ok(());
        }

        if let Some(class) = ty.rust_struct() {
            if ty.is_by_ref() {
                bail!("cannot invoke JS functions returning custom ref types yet")
            }
            // Insert an assertion to the type of the returned value as
            // otherwise this will cause memory unsafety on the Rust side of
            // things.
            self.ret_expr = format!(
                "\
                const val = JS;
                if (!(val instanceof {0})) {{
                    throw new Error('expected value of type {0}');
                }}
                const ret = val.ptr;
                val.ptr = 0;
                return ret;\
            ",
                class
            );
            return Ok(());
        }

        self.ret_expr = match *ty {
            Descriptor::Boolean => "return JS;".to_string(),
            Descriptor::Char => "return JS.codePointAt(0);".to_string(),
            _ => bail!(
                "unsupported return type for calling JS function from Rust: {:?}",
                ty
            ),
        };
        Ok(())
    }

    /// Returns whether this shim won't actually do anything when called other
    /// than forward the invocation somewhere else.
    ///
    /// This is used as an optimization to wire up imports directly where
    /// possible and avoid a shim in some circumstances.
    fn is_noop(&self) -> bool {
        let Rust2Js {
            // fields which may affect whether we do nontrivial work
            catch,
            catch_and_rethrow,
            finally,
            js_arguments,
            prelude,
            ret_expr,
            variadic,
            shim_arguments,

            // all other fields, listed explicitly here so if one is added we'll
            // trigger a nonexhaustive error.
            arg_idx: _,
            cx: _,
            global_idx: _,
            anyref_args: _,
            ret_anyref: _,
            style,
        } = self;

        !catch &&
            !catch_and_rethrow &&
            !variadic &&
            prelude.is_empty() &&
            finally.is_empty() &&
            // make sure our faux return expression is "simple" by not
            // performing any sort of transformation on the return value
            (ret_expr == "JS;" || ret_expr == "return JS;") &&
            // similarly we want to make sure that all the arguments are simply
            // forwarded from the shim we would generate to the import,
            // requiring no transformations
            js_arguments == shim_arguments &&
            // method/constructor invocations require some JS shimming right
            // now, so only normal function-style invocations may get wired up
            *style == Style::Function
    }

    pub fn finish(&mut self, target: &AuxImport) -> Result<String, Error> {
        let variadic = self.variadic;
        let variadic_args = |js_arguments: &[String]| {
            Ok(if !variadic {
                format!("{}", js_arguments.join(", "))
            } else {
                let (last_arg, args) = match js_arguments.split_last() {
                    Some(pair) => pair,
                    None => bail!("a function with no arguments cannot be variadic"),
                };
                if args.len() > 0 {
                    format!("{}, ...{}", args.join(", "), last_arg)
                } else {
                    format!("...{}", last_arg)
                }
            })
        };

        let invoc = match target {
            AuxImport::Value(val) => match self.style {
                Style::Constructor => {
                    let js = match val {
                        AuxValue::Bare(js) => self.cx.import_name(js)?,
                        _ => bail!("invalid import set for constructor"),
                    };
                    format!("new {}({})", js, variadic_args(&self.js_arguments)?)
                }
                Style::Method => {
                    let descriptor = |anchor: &str, extra: &str, field: &str, which: &str| {
                        format!(
                            "GetOwnOrInheritedPropertyDescriptor({}{}, '{}').{}",
                            anchor, extra, field, which
                        )
                    };
                    let js = match val {
                        AuxValue::Bare(js) => self.cx.import_name(js)?,
                        AuxValue::Getter(class, field) => {
                            self.cx.expose_get_inherited_descriptor();
                            let class = self.cx.import_name(class)?;
                            descriptor(&class, ".prototype", field, "get")
                        }
                        AuxValue::ClassGetter(class, field) => {
                            self.cx.expose_get_inherited_descriptor();
                            let class = self.cx.import_name(class)?;
                            descriptor(&class, "", field, "get")
                        }
                        AuxValue::Setter(class, field) => {
                            self.cx.expose_get_inherited_descriptor();
                            let class = self.cx.import_name(class)?;
                            descriptor(&class, ".prototype", field, "set")
                        }
                        AuxValue::ClassSetter(class, field) => {
                            self.cx.expose_get_inherited_descriptor();
                            let class = self.cx.import_name(class)?;
                            descriptor(&class, "", field, "set")
                        }
                    };
                    format!("{}.call({})", js, variadic_args(&self.js_arguments)?)
                }
                Style::Function => {
                    let js = match val {
                        AuxValue::Bare(js) => self.cx.import_name(js)?,
                        _ => bail!("invalid import set for constructor"),
                    };
                    if self.is_noop() {
                        self.cx.expose_does_not_exist();
                        // TODO: comment this
                        let js = format!("typeof {} === 'undefined' ? doesNotExist : {0}", js);
                        return Ok(js);
                    }
                    format!("{}({})", js, variadic_args(&self.js_arguments)?)
                }
            },

            AuxImport::Instanceof(js) => {
                let js = self.cx.import_name(js)?;
                assert!(self.style == Style::Function);
                assert!(!variadic);
                assert_eq!(self.js_arguments.len(), 1);
                format!("{} instanceof {}", self.js_arguments[0], js)
            }

            AuxImport::Static(js) => {
                assert!(self.style == Style::Function);
                assert!(!variadic);
                assert_eq!(self.js_arguments.len(), 0);
                self.cx.import_name(js)?
            }

            AuxImport::Closure(closure) => {
                assert!(self.style == Style::Function);
                assert!(!variadic);
                assert_eq!(self.js_arguments.len(), 3);
                let (js, _ts, _js_doc) = {
                    let mut builder = Js2Rust::new("", self.cx);

                    // First up with a closure we increment the internal reference
                    // count. This ensures that the Rust closure environment won't
                    // be deallocated while we're invoking it.
                    builder.prelude("this.cnt++;");

                    if closure.mutable {
                        // For mutable closures they can't be invoked recursively.
                        // To handle that we swap out the `this.a` pointer with zero
                        // while we invoke it. If we finish and the closure wasn't
                        // destroyed, then we put back the pointer so a future
                        // invocation can succeed.
                        builder
                            .prelude("let a = this.a;")
                            .prelude("this.a = 0;")
                            .rust_argument("a")
                            .rust_argument("b")
                            .finally("if (--this.cnt === 0) d(a, b);")
                            .finally("else this.a = a;");
                    } else {
                        // For shared closures they can be invoked recursively so we
                        // just immediately pass through `this.a`. If we end up
                        // executing the destructor, however, we clear out the
                        // `this.a` pointer to prevent it being used again the
                        // future.
                        builder
                            .rust_argument("this.a")
                            .rust_argument("b")
                            .finally("if (--this.cnt === 0) {")
                            .finally("d(this.a, b);")
                            .finally("this.a = 0;")
                            .finally("}");
                    }
                    builder
                        .process(&closure.function, &None)?
                        .finish("function", "f")
                };
                self.cx.export_function_table()?;
                let body = format!(
                    "
                        const f = wasm.__wbg_function_table.get({});
                        const d = wasm.__wbg_function_table.get({});
                        const b = {};
                        const cb = {};
                        cb.a = {};
                        cb.cnt = 1;
                        let real = cb.bind(cb);
                        real.original = cb;
                    ",
                    closure.shim_idx,
                    closure.dtor_idx,
                    &self.js_arguments[1],
                    js,
                    &self.js_arguments[0],
                );
                self.prelude(&body);
                "real".to_string()
            }

            AuxImport::StructuralMethod(name) => {
                assert!(self.style == Style::Function);
                let (receiver, args) = match self.js_arguments.split_first() {
                    Some(pair) => pair,
                    None => bail!("structural method calls must have at least one argument"),
                };
                format!("{}.{}({})", receiver, name, variadic_args(args)?)
            }

            AuxImport::StructuralGetter(field) => {
                assert!(self.style == Style::Function);
                assert!(!variadic);
                assert_eq!(self.js_arguments.len(), 1);
                format!("{}.{}", self.js_arguments[0], field)
            }

            AuxImport::StructuralClassGetter(class, field) => {
                assert!(self.style == Style::Function);
                assert!(!variadic);
                assert_eq!(self.js_arguments.len(), 0);
                let class = self.cx.import_name(class)?;
                format!("{}.{}", class, field)
            }

            AuxImport::StructuralSetter(field) => {
                assert!(self.style == Style::Function);
                assert!(!variadic);
                assert_eq!(self.js_arguments.len(), 2);
                format!(
                    "{}.{} = {}",
                    self.js_arguments[0], field, self.js_arguments[1]
                )
            }

            AuxImport::StructuralClassSetter(class, field) => {
                assert!(self.style == Style::Function);
                assert!(!variadic);
                assert_eq!(self.js_arguments.len(), 1);
                let class = self.cx.import_name(class)?;
                format!("{}.{} = {}", class, field, self.js_arguments[0])
            }

            AuxImport::IndexingGetterOfClass(class) => {
                assert!(self.style == Style::Function);
                assert!(!variadic);
                assert_eq!(self.js_arguments.len(), 1);
                let class = self.cx.import_name(class)?;
                format!("{}[{}]", class, self.js_arguments[0])
            }

            AuxImport::IndexingGetterOfObject => {
                assert!(self.style == Style::Function);
                assert!(!variadic);
                assert_eq!(self.js_arguments.len(), 2);
                format!("{}[{}]", self.js_arguments[0], self.js_arguments[1])
            }

            AuxImport::IndexingSetterOfClass(class) => {
                assert!(self.style == Style::Function);
                assert!(!variadic);
                assert_eq!(self.js_arguments.len(), 2);
                let class = self.cx.import_name(class)?;
                format!(
                    "{}[{}] = {}",
                    class, self.js_arguments[0], self.js_arguments[1]
                )
            }

            AuxImport::IndexingSetterOfObject => {
                assert!(self.style == Style::Function);
                assert!(!variadic);
                assert_eq!(self.js_arguments.len(), 3);
                format!(
                    "{}[{}] = {}",
                    self.js_arguments[0], self.js_arguments[1], self.js_arguments[2]
                )
            }

            AuxImport::IndexingDeleterOfClass(class) => {
                assert!(self.style == Style::Function);
                assert!(!variadic);
                assert_eq!(self.js_arguments.len(), 1);
                let class = self.cx.import_name(class)?;
                format!("delete {}[{}]", class, self.js_arguments[0])
            }

            AuxImport::IndexingDeleterOfObject => {
                assert!(self.style == Style::Function);
                assert!(!variadic);
                assert_eq!(self.js_arguments.len(), 2);
                format!("delete {}[{}]", self.js_arguments[0], self.js_arguments[1])
            }

            AuxImport::WrapInExportedClass(class) => {
                assert!(self.style == Style::Function);
                assert!(!variadic);
                assert_eq!(self.js_arguments.len(), 1);
                self.cx.require_class_wrap(class);
                if self.is_noop() {
                    return Ok(format!("{}.__wrap", class));
                }
                format!("{}.__wrap({})", class, self.js_arguments[0])
            }

            AuxImport::Intrinsic(intrinsic) => {
                assert!(self.style == Style::Function);
                assert!(!variadic);
                self.intrinsic_expr(intrinsic)?
            }
        };
        let mut invoc = self.ret_expr.replace("JS", &invoc);

        if self.catch {
            self.cx.expose_handle_error()?;
            invoc = format!(
                "\
                try {{\n\
                    {}
                }} catch (e) {{\n\
                    handleError(e);\n\
                }}\
                ",
                &invoc
            );
        } else if self.catch_and_rethrow {
            invoc = format!(
                "\
                try {{\n\
                    {}
                }} catch (e) {{\n\
                    let error = (function () {{
                        try {{
                            return e instanceof Error \
                                ? `${{e.message}}\\n\\nStack:\\n${{e.stack}}` \
                                : e.toString();
                        }} catch(_) {{
                            return \"<failed to stringify thrown value>\";
                        }}
                    }}());
                    console.error(\"wasm-bindgen: imported JS function that \
                                    was not marked as `catch` threw an error:\", \
                                    error);
                    throw e;
                }}\
                ",
                &invoc,
            );
        }

        if self.finally.len() > 0 {
            invoc = format!(
                "\
                try {{\n\
                    {}
                }} finally {{\n\
                    {}
                }}\
                ",
                &invoc, &self.finally
            );
        }
        let mut ret = String::new();
        ret.push_str("function(");
        ret.push_str(&self.shim_arguments.join(", "));
        ret.push_str(") {\n");
        ret.push_str(&self.prelude);

        ret.push_str(&invoc);
        ret.push_str("\n}\n");

        Ok(ret)
    }

    fn global_idx(&mut self) -> usize {
        let ret = self.global_idx;
        self.global_idx += 1;
        ret
    }

    fn prelude(&mut self, s: &str) -> &mut Self {
        for line in s.lines() {
            self.prelude.push_str(line);
            self.prelude.push_str("\n");
        }
        self
    }

    fn finally(&mut self, s: &str) -> &mut Self {
        for line in s.lines() {
            self.finally.push_str(line);
            self.finally.push_str("\n");
        }
        self
    }

    fn intrinsic_expr(&mut self, intrinsic: &Intrinsic) -> Result<String, Error> {
        let expr = match intrinsic {
            Intrinsic::JsvalEq => {
                assert_eq!(self.js_arguments.len(), 2);
                format!("{} === {}", self.js_arguments[0], self.js_arguments[1])
            }

            Intrinsic::IsFunction => {
                assert_eq!(self.js_arguments.len(), 1);
                format!("typeof({}) === 'function'", self.js_arguments[0])
            }

            Intrinsic::IsUndefined => {
                assert_eq!(self.js_arguments.len(), 1);
                format!("{} === undefined", self.js_arguments[0])
            }

            Intrinsic::IsNull => {
                assert_eq!(self.js_arguments.len(), 1);
                format!("{} === null", self.js_arguments[0])
            }

            Intrinsic::IsObject => {
                assert_eq!(self.js_arguments.len(), 1);
                self.prelude(&format!("const val = {};", self.js_arguments[0]));
                format!("typeof(val) === 'object' && val !== null ? 1 : 0")
            }

            Intrinsic::IsSymbol => {
                assert_eq!(self.js_arguments.len(), 1);
                format!("typeof({}) === 'symbol'", self.js_arguments[0])
            }

            Intrinsic::IsString => {
                assert_eq!(self.js_arguments.len(), 1);
                format!("typeof({}) === 'string'", self.js_arguments[0])
            }

            Intrinsic::ObjectCloneRef => {
                assert_eq!(self.js_arguments.len(), 1);
                self.js_arguments[0].clone()
            }

            Intrinsic::ObjectDropRef => {
                assert_eq!(self.js_arguments.len(), 1);
                self.js_arguments[0].clone()
            }

            Intrinsic::CallbackDrop => {
                assert_eq!(self.js_arguments.len(), 1);
                self.prelude(&format!("const obj = {}.original;", self.js_arguments[0]));
                self.prelude("if (obj.cnt-- == 1) {");
                self.prelude("obj.a = 0;");
                self.prelude("return true;");
                self.prelude("}");
                "false".to_string()
            }

            Intrinsic::CallbackForget => {
                assert_eq!(self.js_arguments.len(), 1);
                self.js_arguments[0].clone()
            }

            Intrinsic::NumberNew => {
                assert_eq!(self.js_arguments.len(), 1);
                self.js_arguments[0].clone()
            }

            Intrinsic::StringNew => {
                assert_eq!(self.js_arguments.len(), 1);
                self.js_arguments[0].clone()
            }

            Intrinsic::SymbolNamedNew => {
                assert_eq!(self.js_arguments.len(), 1);
                format!("Symbol({})", self.js_arguments[0])
            }

            Intrinsic::SymbolAnonymousNew => {
                assert_eq!(self.js_arguments.len(), 0);
                "Symbol()".to_string()
            }

            Intrinsic::NumberGet => {
                assert_eq!(self.js_arguments.len(), 2);
                self.cx.expose_uint8_memory();
                self.prelude(&format!("const obj = {};", self.js_arguments[0]));
                self.prelude("if (typeof(obj) === 'number') return obj;");
                self.prelude(&format!("getUint8Memory()[{}] = 1;", self.js_arguments[1]));
                "0".to_string()
            }

            Intrinsic::StringGet => {
                self.cx.expose_pass_string_to_wasm()?;
                self.cx.expose_uint32_memory();
                assert_eq!(self.js_arguments.len(), 2);
                self.prelude(&format!("const obj = {};", self.js_arguments[0]));
                self.prelude("if (typeof(obj) !== 'string') return 0;");
                self.prelude("const ptr = passStringToWasm(obj);");
                self.prelude(&format!(
                    "getUint32Memory()[{} / 4] = WASM_VECTOR_LEN;",
                    self.js_arguments[1],
                ));
                "ptr".to_string()
            }

            Intrinsic::BooleanGet => {
                assert_eq!(self.js_arguments.len(), 1);
                self.prelude(&format!("const v = {};", self.js_arguments[0]));
                format!("typeof(v) === 'boolean' ? (v ? 1 : 0) : 2")
            }
            Intrinsic::Throw => {
                assert_eq!(self.js_arguments.len(), 1);
                format!("throw new Error({})", self.js_arguments[0])
            }

            Intrinsic::Rethrow => {
                assert_eq!(self.js_arguments.len(), 1);
                format!("throw {}", self.js_arguments[0])
            }

            Intrinsic::Module => {
                assert_eq!(self.js_arguments.len(), 0);
                if !self.cx.config.mode.no_modules() && !self.cx.config.mode.web() {
                    bail!(
                        "`wasm_bindgen::module` is currently only supported with \
                         `--target no-modules` and `--target web`"
                    );
                }
                format!("init.__wbindgen_wasm_module")
            }

            Intrinsic::Memory => {
                assert_eq!(self.js_arguments.len(), 0);
                self.cx.memory().to_string()
            }

            Intrinsic::FunctionTable => {
                assert_eq!(self.js_arguments.len(), 0);
                self.cx.export_function_table()?;
                format!("wasm.__wbg_function_table")
            }

            Intrinsic::DebugString => {
                assert_eq!(self.js_arguments.len(), 1);
                self.cx.expose_debug_string();
                format!("debugString({})", self.js_arguments[0])
            }

            Intrinsic::JsonParse => {
                assert_eq!(self.js_arguments.len(), 1);
                format!("JSON.parse({})", self.js_arguments[0])
            }

            Intrinsic::JsonSerialize => {
                assert_eq!(self.js_arguments.len(), 1);
                format!("JSON.stringify({})", self.js_arguments[0])
            }

            Intrinsic::AnyrefHeapLiveCount => {
                assert_eq!(self.js_arguments.len(), 0);
                if self.cx.config.anyref {
                    // Eventually we should add support to the anyref-xform to
                    // re-write calls to the imported
                    // `__wbindgen_anyref_heap_live_count` function into calls to
                    // the exported `__wbindgen_anyref_heap_live_count_impl`
                    // function, and to un-export that function.
                    //
                    // But for now, we just bounce wasm -> js -> wasm because it is
                    // easy.
                    self.cx.require_internal_export("__wbindgen_anyref_heap_live_count_impl")?;
                    "wasm.__wbindgen_anyref_heap_live_count_impl()".into()
                } else {
                    self.cx.expose_global_heap();
                    self.prelude(
                        "
                            let free_count = 0;
                            let next = heap_next;
                            while (next < heap.length) {
                                free_count += 1;
                                next = heap[next];
                            }
                        ",
                    );
                    format!(
                        "heap.length - free_count - {} - {}",
                        super::INITIAL_HEAP_OFFSET,
                        super::INITIAL_HEAP_VALUES.len(),
                    )
                }
            }

            Intrinsic::InitAnyrefTable => {
                self.cx.expose_anyref_table();
                String::from(
                    "
                      const table = wasm.__wbg_anyref_table;
                      const offset = table.grow(4);
                      table.set(offset + 0, undefined);
                      table.set(offset + 1, null);
                      table.set(offset + 2, true);
                      table.set(offset + 3, false);
                    ",
                )
            }
        };
        Ok(expr)
    }
}
