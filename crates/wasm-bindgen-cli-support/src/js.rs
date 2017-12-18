use shared;

#[derive(Default)]
pub struct Js {
    pub expose_global_memory: bool,
    pub expose_global_exports: bool,
    pub exports: Vec<(String, String)>,
    pub nodejs: bool,
}

impl Js {

    pub fn generate_program(&mut self, program: &shared::Program) {
        for f in program.free_functions.iter() {
            self.generate_free_function(f);
        }
    }

    pub fn generate_free_function(&mut self, func: &shared::Function) {
        let simple = func.arguments.iter().all(|t| t.is_number()) &&
            func.ret.as_ref().map(|t| t.is_number()).unwrap_or(true);

        if simple {
            self.exports.push((
                func.name.clone(),
                format!("obj.instance.exports.{}", func.name),
            ));
            return
        }

        let mut dst = format!("function {}(", func.name);
        let mut passed_args = String::new();
        let mut arg_conversions = String::new();
        let mut destructors = String::new();

        for (i, arg) in func.arguments.iter().enumerate() {
            let name = format!("arg{}", i);
            if i > 0 {
                dst.push_str(", ");
            }
            dst.push_str(&name);

            let mut pass = |arg: &str| {
                if passed_args.len() > 0 {
                    passed_args.push_str(", ");
                }
                passed_args.push_str(arg);
            };
            match *arg {
                shared::Type::Number => pass(&name),
                shared::Type::BorrowedStr |
                shared::Type::String => {
                    if self.nodejs {
                        arg_conversions.push_str(&format!("\
                            const buf{i} = Buffer.from({arg});
                            const len{i} = buf{i}.length;
                            const ptr{i} = exports.__wbindgen_malloc(len{i});
                            let memory{i} = new Uint8Array(memory.buffer);
                            buf{i}.copy(memory{i}, ptr{i});
                        ", i = i, arg = name));
                        pass(&format!("ptr{}", i));
                        pass(&format!("len{}", i));
                        if let shared::Type::BorrowedStr = *arg {
                            destructors.push_str(&format!("\n\
                                exports.__wbindgen_free(ptr{i}, len{i});\n\
                            ", i = i));
                        }
                    } else {
                        panic!("strings not implemented for browser");
                    }
                }
                shared::Type::ByRef(_) |
                shared::Type::ByMutRef(_) => {
                    arg_conversions.push_str(&format!("\
                        const ptr{i} = {arg}.__wasmPtr;
                    ", i = i, arg = name));
                    pass(&format!("ptr{}", i));
                }
                shared::Type::ByValue(_) => {
                    arg_conversions.push_str(&format!("\
                        const ptr{i} = {arg}.__wasmPtr;
                        {arg}.__wasmPtr = 0;
                    ", i = i, arg = name));
                    pass(&format!("ptr{}", i));
                }
            }
        }
        let convert_ret = match func.ret {
            None |
            Some(shared::Type::Number) => format!("return ret;"),
            Some(shared::Type::BorrowedStr) |
            Some(shared::Type::ByMutRef(_)) |
            Some(shared::Type::ByRef(_)) => panic!(),
            Some(shared::Type::ByValue(ref name)) => {
                format!("\
                    return {name}.__wasmWrap(ret);
                ", name = name)
            }
            Some(shared::Type::String) => {
                if self.nodejs {
                    format!("\
                        const mem = new Uint8Array(memory.buffer);
                        const ptr = exports.__wbindgen_boxed_str_ptr(ret);
                        const len = exports.__wbindgen_boxed_str_len(ret);
                        const buf = Buffer.from(mem.slice(ptr, ptr + len));
                        const realRet = buf.toString();
                        exports.__wbindgen_boxed_str_free(ret);
                        return realRet;
                    ")
                } else {
                    panic!("strings not implemented for browser");
                }
            }
        };
        dst.push_str(") {\n        ");
        dst.push_str(&arg_conversions);
        dst.push_str(&format!("\
            try {{
                const ret = exports.{f}({passed});
                {convert_ret}
            }} finally {{
                {destructors}
            }}
        ", f = func.name, passed = passed_args, destructors = destructors,
            convert_ret = convert_ret));
        dst.push_str("};");

        self.exports.push((func.name.clone(), dst));
    }

    pub fn to_string(&self) -> String {
        let mut globals = String::new();
        if self.expose_global_memory {
            globals.push_str("const memory = obj.instance.exports.memory;\n");
        }
        if self.expose_global_exports {
            globals.push_str("const exports = obj.instance.exports;\n");
        }

        let mut exports = String::new();
        for &(ref name, ref body) in self.exports.iter() {
            exports.push_str("obj.");
            exports.push_str(name);
            exports.push_str(" = ");
            exports.push_str(body);
            exports.push_str(";\n");
        }
        format!("
            const function xform(obj) {{
                {}
                {}
                return obj;
            }}
            export const function instantiate(bytes, imports) {{
                return WebAssembly.instantiate(bytes, imports).then(xform);
            }}
        ", globals, exports)
    }
}
