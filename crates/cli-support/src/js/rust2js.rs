use failure::Error;

use super::{Context, Js2Rust};
use descriptor::{Descriptor, Function};

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
        }
    }

    pub fn catch(&mut self, catch: bool) -> &mut Self {
        if catch {
            self.cx.expose_uint32_memory();
            self.cx.expose_add_heap_object();
        }
        self.catch = catch;
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
                prefix = if optional { format!("{} == 0 ? undefined : ", abi) } else { String::new() },
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
                    start = if optional { format!("if ({} !== 0) {{", abi) } else { String::new() },
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
            return Ok(())
        }

        if optional {
            bail!("unsupported optional argument {:?}", arg);
        }

        if let Some(signed) = arg.get_64bit() {
            let f = if signed {
                self.cx.expose_int64_cvt_shim()
            } else {
                self.cx.expose_uint64_cvt_shim()
            };
            let hi = self.shim_argument();
            let name = format!("n{}", abi);
            self.prelude(&format!(
                "\
                 u32CvtShim[0] = {lo};\n\
                 u32CvtShim[1] = {hi};\n\
                 const {name} = {f}[0];\n\
                 ",
                lo = abi,
                hi = hi,
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
            let assign = format!("let c{0} = {1}.__construct({0});", abi, class);
            self.prelude(&assign);
            self.js_arguments.push(format!("c{}", abi));
            return Ok(());
        }

        if let Some((f, mutable)) = arg.stack_closure() {
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
            self.cx.expose_get_global_argument()?;
            self.cx.function_table_needed = true;
            let next_global = self.global_idx();
            self.global_idx();
            self.prelude(&format!(
                "\
                 let cb{0} = {js};\n\
                 cb{0}.f = wasm.__wbg_function_table.get({0});\n\
                 cb{0}.a = getGlobalArgument({next_global});\n\
                 cb{0}.b = getGlobalArgument({next_global} + 1);\n\
                 ",
                abi,
                js = js,
                next_global = next_global
            ));
            self.finally(&format!("cb{0}.a = cb{0}.b = 0;", abi));
            self.js_arguments.push(format!("cb{0}.bind(cb{0})", abi));
            return Ok(());
        }

        if let Some(closure) = arg.ref_closure() {
            let (js, _ts, _js_doc) = {
                let mut builder = Js2Rust::new("", self.cx);
                if closure.mutable {
                    builder
                        .prelude("let a = this.a;\n")
                        .prelude("this.a = 0;\n")
                        .rust_argument("a")
                        .finally("this.a = a;\n");
                } else {
                    builder.rust_argument("this.a");
                }
                builder
                    .process(&closure.function)?
                    .finish("function", "this.f")
            };
            self.cx.expose_get_global_argument()?;
            self.cx.expose_uint32_memory();
            self.cx.expose_add_heap_object();
            self.cx.function_table_needed = true;
            let reset_idx = format!(
                "\
                 let cb{0} = {js};\n\
                 cb{0}.f = wasm.__wbg_function_table.get(getGlobalArgument({f}));\n\
                 cb{0}.a = getGlobalArgument({a});\n\
                 let real = cb{0}.bind(cb{0});\n\
                 real.original = cb{0};\n\
                 idx{0} = getUint32Memory()[{0} / 4] = addHeapObject(real);\n\
                 ",
                abi,
                js = js,
                f = self.global_idx(),
                a = self.global_idx(),
            );
            self.prelude(&format!(
                "\
                 let idx{0} = getUint32Memory()[{0} / 4];\n\
                 if (idx{0} === 0xffffffff) {{\n\
                 {1}\
                 }}\n\
                 ",
                abi, &reset_idx
            ));
            self.cx.expose_get_object();
            self.js_arguments.push(format!("getObject(idx{})", abi));
            return Ok(());
        }

        let invoc_arg = match *arg {
            ref d if d.is_number() => abi,
            Descriptor::Boolean => format!("{} !== 0", abi),
            Descriptor::Char => format!("String.fromCodePoint({})", abi),
            ref d if d.is_ref_anyref() => {
                self.cx.expose_get_object();
                format!("getObject({})", abi)
            }
            _ => bail!(
                "unimplemented argument type in imported function: {:?}",
                arg
            ),
        };
        self.js_arguments.push(invoc_arg);
        Ok(())
    }

    fn ret(&mut self, ret: &Option<Descriptor>) -> Result<(), Error> {
        let ty = match *ret {
            Some(ref t) => t,
            None => {
                self.ret_expr = "JS;".to_string();
                return Ok(());
            }
        };
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
                const [retptr, retlen] = {};
                const mem = getUint32Memory();
                mem[ret / 4] = retptr;
                mem[ret / 4 + 1] = retlen;
                ",
                prelude,
                expr
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
                ".to_string();
            } else {
                self.ret_expr = "return addHeapObject(JS);".to_string()
            }
            return Ok(())
        }
        if optional {
            bail!("unsupported optional return type {:?}", ty);
        }
        if ty.is_number() {
            self.ret_expr = "return JS;".to_string();
            return Ok(());
        }
        if let Some(signed) = ty.get_64bit() {
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
            Descriptor::Boolean => "return JS ? 1 : 0;".to_string(),
            Descriptor::Char => "return JS.codePointAt(0);".to_string(),
            _ => bail!("unimplemented return from JS to Rust: {:?}", ty),
        };
        Ok(())
    }

    pub fn finish(&self, invoc: &str) -> String {
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

        let mut invoc = self.ret_expr.replace(
            "JS",
            &format!("{}({})", invoc, self.js_arguments.join(", ")),
        );
        if self.catch {
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
        return ret;
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
