use failure::Error;

use super::{Context, ImportTarget, Js2Rust};
use descriptor::{Descriptor, Function};

/// Helper struct for manufacturing a shim in JS used to translate Rust types to
/// JS, then invoking an imported JS function.
pub struct Rust2Js<'a, 'b: 'a> {
    pub cx: &'a mut Context<'b>,

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

    /// Whether or not the last argument is a slice representing variadic arguments.
    variadic: bool,
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
            variadic: false,
        }
    }

    pub fn catch(&mut self, catch: bool) -> &mut Self {
        self.catch = catch;
        self
    }

    pub fn variadic(&mut self, variadic: bool) -> &mut Self {
        self.variadic = variadic;
        self
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
            let f = self.cx.expose_get_vector_from_wasm(ty);
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

            if !arg.is_by_ref() && !arg.is_clamped_by_ref() {
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
            self.cx.expose_take_object();
            self.js_arguments.push(format!("takeObject({})", abi));
            return Ok(());
        } else if arg.is_ref_anyref() {
            self.cx.expose_get_object();
            self.js_arguments.push(format!("getObject({})", abi));
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
                Descriptor::Char => {
                    let value = self.shim_argument();
                    self.js_arguments.push(format!(
                        "{present} === 0 ? undefined : String.fromCodePoint({value})",
                        value = value,
                        present = abi,
                    ));
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
                    .process(f)?
                    .finish("function", "this.f")
            };
            self.cx.function_table_needed = true;
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

        let invoc_arg = match *arg {
            ref d if d.is_number() => abi,
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
            return Ok(());
        }
        if optional {
            if ty.is_wasm_native() {
                self.cx.expose_is_like_none();
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
                self.cx.expose_is_like_none();
                self.ret_expr = "
                    const val = JS;
                    return isLikeNone(val) ? 0xFFFFFF : val;
                "
                .to_string();
                return Ok(());
            }

            if let Some(signed) = ty.get_64() {
                self.cx.expose_is_like_none();
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
                    self.cx.expose_is_like_none();
                    self.ret_expr = "
                        const val = JS;
                        return isLikeNone(val) ? 0xFFFFFF : val ? 1 : 0;
                    "
                    .to_string();
                    return Ok(());
                }
                Descriptor::Char => {
                    self.cx.expose_is_like_none();
                    self.cx.expose_uint32_memory();
                    self.shim_arguments.insert(0, "ret".to_string());
                    self.ret_expr = "
                        const val = JS;
                        getUint32Memory()[ret / 4] = !isLikeNone(val);
                        getUint32Memory()[ret / 4 + 1] = isLikeNone(val) ? 0 : val.codePointAt(0);
                    "
                    .to_string();
                    return Ok(());
                }
                _ => bail!(
                    "unsupported optional return type for calling JS function from Rust: {:?}",
                    ty
                ),
            };
        }
        if ty.is_number() {
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
    pub fn is_noop(&self) -> bool {
        let Rust2Js {
            // fields which may affect whether we do nontrivial work
            catch,
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
        } = self;

        !catch &&
            !variadic &&
            prelude.is_empty() &&
            finally.is_empty() &&
            // make sure our faux return expression is "simple" by not
            // performing any sort of transformation on the return value
            (ret_expr == "JS;" || ret_expr == "return JS;") &&
            // similarly we want to make sure that all the arguments are simply
            // forwarded from the shim we would generate to the import,
            // requiring no transformations
            js_arguments == shim_arguments
    }

    pub fn finish(&mut self, invoc: &ImportTarget) -> Result<String, Error> {
        let mut ret = String::new();
        ret.push_str("function(");
        ret.push_str(&self.shim_arguments.join(", "));
        if self.catch {
            if self.shim_arguments.len() > 0 {
                ret.push_str(", ")
            }
            ret.push_str("exnptr");
        }
        ret.push_str(") {\n");
        ret.push_str(&self.prelude);

        let variadic = self.variadic;
        let ret_expr = &self.ret_expr;
        let js_arguments = &self.js_arguments;
        let handle_variadic = |invoc: &str, js_arguments: &[String]| {
            let ret = if variadic {
                let (last_arg, args) = match js_arguments.split_last() {
                    Some(pair) => pair,
                    None => bail!("a function with no arguments cannot be variadic"),
                };
                if args.len() > 0 {
                    ret_expr.replace(
                        "JS",
                        &format!("{}({}, ...{})", invoc, args.join(", "), last_arg),
                    )
                } else {
                    ret_expr.replace("JS", &format!("{}(...{})", invoc, last_arg))
                }
            } else {
                ret_expr.replace("JS", &format!("{}({})", invoc, js_arguments.join(", ")))
            };
            Ok(ret)
        };

        let fixed = |desc: &str, class: &Option<String>, amt: usize| {
            if variadic {
                bail!("{} cannot be variadic", desc);
            }
            match (class, js_arguments.len()) {
                (None, n) if n == amt + 1 => Ok((js_arguments[0].clone(), &js_arguments[1..])),
                (None, _) => bail!("setters must have {} arguments", amt + 1),
                (Some(class), n) if n == amt => Ok((class.clone(), &js_arguments[..])),
                (Some(_), _) => bail!("static setters must have {} arguments", amt),
            }
        };

        let mut invoc = match invoc {
            ImportTarget::Function(f) => handle_variadic(&f, &self.js_arguments)?,
            ImportTarget::Constructor(c) => {
                handle_variadic(&format!("new {}", c), &self.js_arguments)?
            }
            ImportTarget::Method(f) => handle_variadic(&format!("{}.call", f), &self.js_arguments)?,
            ImportTarget::StructuralMethod(f) => {
                let (receiver, args) = match self.js_arguments.split_first() {
                    Some(pair) => pair,
                    None => bail!("methods must have at least one argument"),
                };
                handle_variadic(&format!("{}.{}", receiver, f), args)?
            }
            ImportTarget::StructuralGetter(class, field) => {
                let (receiver, _) = fixed("getter", class, 0)?;
                let expr = format!("{}.{}", receiver, field);
                self.ret_expr.replace("JS", &expr)
            }
            ImportTarget::StructuralSetter(class, field) => {
                let (receiver, val) = fixed("setter", class, 1)?;
                let expr = format!("{}.{} = {}", receiver, field, val[0]);
                self.ret_expr.replace("JS", &expr)
            }
            ImportTarget::StructuralIndexingGetter(class) => {
                let (receiver, field) = fixed("indexing getter", class, 1)?;
                let expr = format!("{}[{}]", receiver, field[0]);
                self.ret_expr.replace("JS", &expr)
            }
            ImportTarget::StructuralIndexingSetter(class) => {
                let (receiver, field) = fixed("indexing setter", class, 2)?;
                let expr = format!("{}[{}] = {}", receiver, field[0], field[1]);
                self.ret_expr.replace("JS", &expr)
            }
            ImportTarget::StructuralIndexingDeleter(class) => {
                let (receiver, field) = fixed("indexing deleter", class, 1)?;
                let expr = format!("delete {}[{}]", receiver, field[0]);
                self.ret_expr.replace("JS", &expr)
            }
        };

        if self.catch {
            self.cx.expose_uint32_memory();
            self.cx.expose_add_heap_object();
            let catch = "\
                         const view = getUint32Memory();\n\
                         view[exnptr / 4] = 1;\n\
                         view[exnptr / 4 + 1] = addHeapObject(e);\n\
                         ";

            invoc = format!(
                "\
                try {{\n\
                    {}
                }} catch (e) {{\n\
                    {}
                }}\
                ",
                &invoc, catch
            );
        };

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
}
