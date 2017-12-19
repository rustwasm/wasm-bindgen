use shared;

#[derive(Default)]
pub struct Js {
    expose_global_memory: bool,
    expose_global_exports: bool,
    expose_get_string_from_wasm: bool,
    expose_pass_string_to_wasm: bool,
    expose_assert_num: bool,
    expose_assert_class: bool,
    expose_token: bool,
    exports: Vec<(String, String)>,
    classes: Vec<String>,
    imports: Vec<String>,
    pub nodejs: bool,
}

impl Js {

    pub fn generate_program(&mut self, program: &shared::Program) {
        for f in program.free_functions.iter() {
            self.generate_free_function(f);
        }
        for s in program.structs.iter() {
            self.generate_struct(s);
        }
        for s in program.imports.iter() {
            self.generate_import(s);
        }
    }

    pub fn generate_free_function(&mut self, func: &shared::Function) {
        let ret = self.generate_function(&format!("function {}", func.name),
                                         &func.name,
                                         false,
                                         &func.arguments,
                                         func.ret.as_ref());

        self.exports.push((func.name.clone(), ret));
    }

    pub fn generate_struct(&mut self, s: &shared::Struct) {
        let mut dst = String::new();
        self.expose_token = true;
        self.expose_global_exports = true;
        dst.push_str(&format!("
            class {} {{
                constructor(ptr, sym) {{
                    _checkToken(sym);
                    this.__wasmPtr = ptr;
                }}

                free() {{
                    const ptr = this.__wasmPtr;
                    this.__wasmPtr = 0;
                    exports.{}(ptr);
                }}
        ", s.name, s.free_function()));

        for function in s.functions.iter() {
            let f = self.generate_function(
                &format!("static {}", function.name),
                &function.struct_function_export_name(&s.name),
                false,
                &function.arguments,
                function.ret.as_ref(),
            );
            dst.push_str(&f);
            dst.push_str("\n");
        }
        for method in s.methods.iter() {
            let f = self.generate_function(
                &format!("{}", method.function.name),
                &method.function.struct_function_export_name(&s.name),
                true,
                &method.function.arguments,
                method.function.ret.as_ref(),
            );
            dst.push_str(&f);
            dst.push_str("\n");
        }
        dst.push_str("}\n");
        self.classes.push(dst);
        self.exports.push((s.name.clone(), s.name.clone()));
    }

    fn generate_function(&mut self,
                         name: &str,
                         wasm_name: &str,
                         is_method: bool,
                         arguments: &[shared::Type],
                         ret: Option<&shared::Type>) -> String {
        let mut dst = format!("{}(", name);
        let mut passed_args = String::new();
        let mut arg_conversions = String::new();
        let mut destructors = String::new();

        if is_method {
            passed_args.push_str("this.__wasmPtr");
        }

        for (i, arg) in arguments.iter().enumerate() {
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
                shared::Type::Number => {
                    self.expose_assert_num = true;
                    arg_conversions.push_str(&format!("_assertNum({});\n", name));
                    pass(&name)
                }
                shared::Type::BorrowedStr |
                shared::Type::String => {
                    self.expose_global_exports = true;
                    self.expose_pass_string_to_wasm = true;
                    arg_conversions.push_str(&format!("\
                        const [ptr{i}, len{i}] = passStringToWasm({arg});
                    ", i = i, arg = name));
                    pass(&format!("ptr{}", i));
                    pass(&format!("len{}", i));
                    if let shared::Type::BorrowedStr = *arg {
                        destructors.push_str(&format!("\n\
                            exports.__wbindgen_free(ptr{i}, len{i});\n\
                        ", i = i));
                    }
                }
                shared::Type::ByRef(ref s) |
                shared::Type::ByMutRef(ref s) => {
                    self.expose_assert_class = true;
                    arg_conversions.push_str(&format!("\
                        const ptr{i} = _assertClass({arg}, {struct_});
                    ", i = i, arg = name, struct_ = s));
                    pass(&format!("ptr{}", i));
                }
                shared::Type::ByValue(ref s) => {
                    self.expose_assert_class = true;
                    arg_conversions.push_str(&format!("\
                        const ptr{i} = _assertClass({arg}, {struct_});
                        {arg}.__wasmPtr = 0;
                    ", i = i, arg = name, struct_ = s));
                    pass(&format!("ptr{}", i));
                }
            }
        }
        let convert_ret = match ret {
            None |
            Some(&shared::Type::Number) => format!("return ret;"),
            Some(&shared::Type::BorrowedStr) |
            Some(&shared::Type::ByMutRef(_)) |
            Some(&shared::Type::ByRef(_)) => panic!(),
            Some(&shared::Type::ByValue(ref name)) => {
                format!("\
                    return new {name}(ret, token);
                ", name = name)
            }
            Some(&shared::Type::String) => {
                self.expose_get_string_from_wasm = true;
                self.expose_global_exports = true;
                format!("
                    const ptr = exports.__wbindgen_boxed_str_ptr(ret);
                    const len = exports.__wbindgen_boxed_str_len(ret);
                    const realRet = getStringFromWasm(ptr, len);
                    exports.__wbindgen_boxed_str_free(ret);
                    return realRet;
                ")
            }
        };
        dst.push_str(") {\n        ");
        dst.push_str(&arg_conversions);
        self.expose_global_exports = true;
        if destructors.len() == 0 {
            dst.push_str(&format!("\
                const ret = exports.{f}({passed});
                {convert_ret}
            ", f = wasm_name, passed = passed_args, convert_ret = convert_ret));
        } else {
            dst.push_str(&format!("\
                try {{
                    const ret = exports.{f}({passed});
                    {convert_ret}
                }} finally {{
                    {destructors}
                }}
            ", f = wasm_name, passed = passed_args, destructors = destructors,
                convert_ret = convert_ret));
        }
        dst.push_str("}");
        return dst
    }

    pub fn generate_import(&mut self, import: &shared::Function) {
        let mut dst = String::new();

        dst.push_str(&format!("const {0} = imports.env.{0};\n", import.name));
        dst.push_str(&format!("imports.env.{0} = function {0}_shim(", import.name));

        let mut invocation = String::new();
        for (i, arg) in import.arguments.iter().enumerate() {
            if invocation.len() > 0 {
                invocation.push_str(", ");
            }
            if i > 0 {
                dst.push_str(", ");
            }
            match *arg {
                shared::Type::Number => {
                    invocation.push_str(&format!("arg{}", i));
                    dst.push_str(&format!("arg{}", i));
                }
                shared::Type::BorrowedStr => {
                    self.expose_get_string_from_wasm = true;
                    invocation.push_str(&format!("getStringFromWasm(ptr{0}, len{0})", i));
                    dst.push_str(&format!("ptr{0}, len{0}", i));
                }
                shared::Type::String |
                shared::Type::ByRef(_) |
                shared::Type::ByMutRef(_) |
                shared::Type::ByValue(_) => {
                    panic!("unsupported type in import");
                }
            }
        }
        dst.push_str(") {\n");
        dst.push_str(&format!("return {}({});\n}}", import.name, invocation));

        self.imports.push(dst);
    }

    pub fn to_string(&self) -> String {
        let mut globals = String::new();
        let mut real_globals = String::new();
        if self.expose_global_memory ||
            self.expose_pass_string_to_wasm ||
            self.expose_get_string_from_wasm
        {
            globals.push_str("const memory = obj.instance.exports.memory;\n");
        }
        if self.expose_global_exports ||
            self.expose_pass_string_to_wasm ||
            self.expose_get_string_from_wasm
        {
            globals.push_str("const exports = obj.instance.exports;\n");
        }
        if self.expose_token {
            globals.push_str("\
                const token = Symbol('foo');
                function _checkToken(sym) {
                    if (token !== sym)
                        throw new Error('cannot invoke `new` directly');
                }
            ");
        }
        if self.expose_assert_num {
            globals.push_str("\
                function _assertNum(n) {
                    if (typeof(n) !== 'number')
                        throw new Error('expected a number argument');
                }
            ");
        }
        if self.expose_pass_string_to_wasm {
            if self.nodejs {
                globals.push_str("
                    function passStringToWasm(arg) {
                        if (typeof(arg) !== 'string')
                            throw new Error('expected a string argument');
                        const buf = Buffer.from(arg);
                        const len = buf.length;
                        const ptr = exports.__wbindgen_malloc(len);
                        let array = new Uint8Array(memory.buffer);
                        buf.copy(array, ptr);
                        return [ptr, len];
                    }
                ");
            } else {
                globals.push_str("
                    function passStringToWasm(arg) {
                        if (typeof(arg) !== 'string')
                            throw new Error('expected a string argument');
                        const buf = new TextEncoder('utf-8').encode(arg);
                        const len = buf.length;
                        const ptr = exports.__wbindgen_malloc(len);
                        let array = new Uint8Array(memory.buffer);
                        array.set(buf, ptr);
                        return [ptr, len];
                    }
                ");
            }
        }
        if self.expose_get_string_from_wasm {
            real_globals.push_str("let getStringFromWasm = null;\n");
            if self.nodejs {
                globals.push_str("
                    getStringFromWasm = function getStringFromWasm(ptr, len) {
                        const mem = new Uint8Array(memory.buffer);
                        const buf = Buffer.from(mem.slice(ptr, ptr + len));
                        const ret = buf.toString();
                        return ret;
                    }
                ");
            } else {
                globals.push_str("
                    getStringFromWasm = function getStringFromWasm(ptr, len) {
                        const mem = new Uint8Array(memory.buffer);
                        const slice = mem.slice(ptr, ptr + len);
                        const ret = new TextDecoder('utf-8').decode(slice);
                        return ret;
                    }
                ");
            }
        }
        if self.expose_assert_class {
            globals.push_str("
                function _assertClass(instance, klass) {
                    if (!(instance instanceof klass))
                        throw new Error(`expected instance of ${klass.name}`);
                    return instance.__wasmPtr;
                }
            ");
        }

        let mut exports = String::new();
        for class in self.classes.iter() {
            exports.push_str(class);
            exports.push_str("\n");
        }
        for &(ref name, ref body) in self.exports.iter() {
            exports.push_str("obj.");
            exports.push_str(name);
            exports.push_str(" = ");
            exports.push_str(body);
            exports.push_str(";\n");
        }
        let mut imports = String::new();
        for import in self.imports.iter() {
            imports.push_str(import);
            imports.push_str("\n");
        }
        format!("
            {}
            function xform(obj) {{
                {}
                {}
                return obj;
            }}
            export function instantiate(bytes, imports) {{
                {}
                return WebAssembly.instantiate(bytes, imports).then(xform);
            }}
        ", real_globals, globals, exports, imports)
    }
}
