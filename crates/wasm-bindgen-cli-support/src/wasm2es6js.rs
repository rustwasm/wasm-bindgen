extern crate base64;

use std::collections::HashSet;

use parity_wasm::elements::*;

use super::Error;

pub struct Config {
    base64: bool,
}

pub struct Output {
    module: Module,
    base64: bool,
}

impl Config {
    pub fn new() -> Config {
        Config {
            base64: false,
        }
    }

    pub fn base64(&mut self, base64: bool) -> &mut Self {
        self.base64 = base64;
        self
    }

    pub fn generate(&mut self, wasm: &[u8]) -> Result<Output, Error> {
        assert!(self.base64);
        let module = deserialize_buffer(wasm).map_err(|e| {
            format_err!("{:?}", e)
        })?;
        Ok(Output {
            module,
            base64: self.base64,
        })
    }
}

impl Output {
    pub fn typescript(&self) -> String {
        let mut exports = format!("/* tslint:disable */\n");

        if let Some(i) = self.module.export_section() {
            let imported_functions = self.module.import_section()
                .map(|m| m.functions() as u32)
                .unwrap_or(0);
            for entry in i.entries() {
                let idx = match *entry.internal() {
                    Internal::Function(i) => i - imported_functions,
                    Internal::Memory(_) => {
                        exports.push_str(&format!("
                            export const {}: WebAssembly.Memory;
                        ", entry.field()));
                        continue
                    }
                    Internal::Table(_) => {
                        panic!("wasm exports a table which isn't supported yet");
                    }
                    Internal::Global(_) => {
                        panic!("wasm exports globals which aren't supported yet");
                    }
                };

                let functions = self.module.function_section()
                    .expect("failed to find function section");
                let idx = functions.entries()[idx as usize].type_ref();

                let types = self.module.type_section()
                    .expect("failed to find type section");
                let ty = match types.types()[idx as usize] {
                    Type::Function(ref f) => f,
                };
                let mut args = String::new();
                for (i, _) in ty.params().iter().enumerate() {
                    if i > 0 {
                        args.push_str(", ");
                    }
                    args.push((b'a' + (i as u8)) as char);
                    args.push_str(": number");
                }

                exports.push_str(&format!("
                    export function {name}({args}): {ret};
                ",
                    name = entry.field(),
                    args = args,
                    ret = if ty.return_type().is_some() { "number" } else { "void" },
                ));
            }
        }

        if self.base64 {
            exports.push_str("export const booted: Promise<boolean>;");
        }

        return exports
    }

    pub fn js(self) -> String {
        let mut js_imports = String::new();
        let mut exports = String::new();
        let mut imports = String::new();
        let mut export_mem = false;

        if let Some(i) = self.module.import_section() {
            let mut set = HashSet::new();
            for entry in i.entries() {
                match *entry.external() {
                    External::Function(_) => {}
                    External::Table(_) => {
                        panic!("wasm imports a table which isn't supported yet");
                    }
                    External::Memory(_) => {
                        panic!("wasm imports memory which isn't supported yet");
                    }
                    External::Global(_) => {
                        panic!("wasm imports globals which aren't supported yet");
                    }
                }

                if !set.insert(entry.module()) {
                    continue
                }

                let name = (b'a' + (set.len() as u8)) as char;
                js_imports.push_str(&format!("import * as import_{} from '{}';",
                                             name,
                                             entry.module()));
                imports.push_str(&format!("'{}': import_{}, ", entry.module(), name));
            }
        }

        if let Some(i) = self.module.export_section() {
            let imported_functions = self.module.import_section()
                .map(|m| m.functions() as u32)
                .unwrap_or(0);
            for entry in i.entries() {
                let idx = match *entry.internal() {
                    Internal::Function(i) => i - imported_functions,
                    Internal::Memory(_) => {
                        export_mem = true;
                        continue
                    }
                    Internal::Table(_) => {
                        panic!("wasm exports a table which isn't supported yet");
                    }
                    Internal::Global(_) => {
                        panic!("wasm exports globals which aren't supported yet");
                    }
                };

                let functions = self.module.function_section()
                    .expect("failed to find function section");
                let idx = functions.entries()[idx as usize].type_ref();

                let types = self.module.type_section()
                    .expect("failed to find type section");
                let ty = match types.types()[idx as usize] {
                    Type::Function(ref f) => f,
                };
                let mut args = String::new();
                for (i, _) in ty.params().iter().enumerate() {
                    if i > 0 {
                        args.push_str(", ");
                    }
                    args.push((b'a' + (i as u8)) as char);
                }

                exports.push_str(&format!("
                    export function {name}({args}) {{
                        {ret} wasm.exports.{name}({args});
                    }}
                ",
                    name = entry.field(),
                    args = args,
                    ret = if ty.return_type().is_some() { "return" } else { "" },
                ));
            }
        }

        let wasm = serialize(self.module)
            .expect("failed to serialize");

        format!("
            {js_imports}
            let wasm;
            let bytes;
            const base64 = \"{base64}\";
            if (typeof Buffer === 'undefined') {{
                bytes = Uint8Array.from(atob(base64), c => c.charCodeAt(0));
            }} else {{
                bytes = Buffer.from(base64, 'base64');
            }}
            {mem_export}
            export const booted = WebAssembly.instantiate(bytes, {{ {imports} }})
                .then(obj => {{
                    wasm = obj.instance;
                    {memory}
                }});

            {exports}
        ",
            base64 = base64::encode(&wasm),
            js_imports = js_imports,
            imports = imports,
            exports = exports,
            mem_export = if export_mem { "export let memory;" } else { "" },
            memory = if export_mem { "memory = wasm.exports.memory;" } else { "" },
        )
    }
}
