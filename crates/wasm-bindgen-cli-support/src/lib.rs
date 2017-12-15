#[macro_use]
extern crate failure;
extern crate parity_wasm;
extern crate wasm_bindgen_shared as shared;
extern crate serde_json;

use std::path::{Path, PathBuf};
use std::fs::File;
use std::io::Write;

use failure::{Error, ResultExt};
use parity_wasm::elements::*;

pub struct Bindgen {
    path: Option<PathBuf>,
    nodejs: bool,
}

pub struct Object {
    module: Module,
    items: Vec<shared::Function>,
    nodejs: bool,
}

impl Bindgen {
    pub fn new() -> Bindgen {
        Bindgen {
            path: None,
            nodejs: false,
        }
    }

    pub fn input_path<P: AsRef<Path>>(&mut self, path: P) -> &mut Bindgen {
        self.path = Some(path.as_ref().to_path_buf());
        self
    }

    pub fn nodejs(&mut self, node: bool) -> &mut Bindgen {
        self.nodejs = node;
        self
    }

    pub fn generate(&mut self) -> Result<Object, Error> {
        let input = match self.path {
            Some(ref path) => path,
            None => panic!("must have a path input for now"),
        };
        let mut module = parity_wasm::deserialize_file(input).map_err(|e| {
            format_err!("{:?}", e)
        })?;
        let items = extract_items(&mut module);
        Ok(Object {
            module,
            items,
            nodejs: self.nodejs,
        })
    }
}

impl Object {
    pub fn write_js_to<P: AsRef<Path>>(&self, path: P) -> Result<(), Error> {
        self._write_js_to(path.as_ref())
    }

    fn _write_js_to(&self, path: &Path) -> Result<(), Error> {
        let js = self.generate_js();
        let mut f = File::create(path).with_context(|_| {
            format!("failed to create file at {:?}", path)
        })?;
        f.write_all(js.as_bytes()).with_context(|_| {
            format!("failed to write file at {:?}", path)
        })?;
        Ok(())
    }

    pub fn write_wasm_to<P: AsRef<Path>>(self, path: P) -> Result<(), Error> {
        self._write_wasm_to(path.as_ref())
    }

    fn _write_wasm_to(self, path: &Path) -> Result<(), Error> {
        parity_wasm::serialize_to_file(path, self.module).map_err(|e| {
            format_err!("{:?}", e)
        })?;
        Ok(())
    }

    pub fn generate_js(&self) -> String {
        let mut set_exports = String::new();
        for func in self.items.iter() {
            self.add_export(func, &mut set_exports);
        }

        format!("\
const xform = (obj) => {{
    const exports = obj.instance.exports;
    const memory = obj.instance.exports.memory;
    {set_exports}
    return obj;
}};
export const instantiate = (bytes, imports) => {{
    return WebAssembly.instantiate(bytes, imports).then(xform);
}};
",
    set_exports = set_exports,
)
    }

    fn add_export(&self, func: &shared::Function, dst: &mut String) {
        let simple = func.arguments.iter().all(|t| t.is_number()) &&
            func.ret.as_ref().map(|t| t.is_number()).unwrap_or(true);

        if simple {
            dst.push_str(&format!("\n\
                obj.{f} = obj.instance.exports.{f};\
            ", f = func.name));
            return
        }

        dst.push_str(&format!("\n    obj.{f} = function {f}(",
                                      f = func.name));
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
            }
        }
        let convert_ret = match func.ret {
            None |
            Some(shared::Type::Number) => format!("return ret;"),
            Some(shared::Type::BorrowedStr) => panic!(),
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
    }
}

fn extract_items(module: &mut Module) -> Vec<shared::Function> {
    let data = module.sections_mut()
        .iter_mut()
        .filter_map(|s| {
            match *s {
                Section::Data(ref mut s) => Some(s),
                _ => None,
            }
        })
        .next();
    let data = match data {
        Some(data) => data,
        None => return Vec::new(),
    };

    let mut ret = Vec::new();
    for i in (0..data.entries().len()).rev() {
        {
            let value = data.entries()[i].value();
            if !value.starts_with(b"wbg:") {
                continue
            }
            let json = &value[4..];
            let func: shared::Function = match serde_json::from_slice(json) {
                Ok(f) => f,
                Err(_) => continue,
            };
            ret.push(func);
        }
        data.entries_mut().remove(i);
    }
    return ret
}
