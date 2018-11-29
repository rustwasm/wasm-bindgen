extern crate base64;
extern crate tempfile;

use std::collections::HashSet;

use failure::Error;
use parity_wasm::elements::*;

pub struct Config {
    base64: bool,
    fetch_path: Option<String>,
}

pub struct Output {
    module: Module,
    base64: bool,
    fetch_path: Option<String>,
}

impl Config {
    pub fn new() -> Config {
        Config {
            base64: false,
            fetch_path: None,
        }
    }

    pub fn base64(&mut self, base64: bool) -> &mut Self {
        self.base64 = base64;
        self
    }

    pub fn fetch(&mut self, path: Option<String>) -> &mut Self {
        self.fetch_path = path;
        self
    }

    pub fn generate(&mut self, wasm: &[u8]) -> Result<Output, Error> {
        if !self.base64 && !self.fetch_path.is_some() {
            bail!("one of --base64 or --fetch is required");
        }
        let module = deserialize_buffer(wasm)?;
        Ok(Output {
            module,
            base64: self.base64,
            fetch_path: self.fetch_path.clone(),
        })
    }
}

pub fn typescript(module: &Module) -> String {
    let mut exports = format!("/* tslint:disable */\n");

    if let Some(i) = module.export_section() {
        let imported_functions = module
            .import_section()
            .map(|m| m.functions() as u32)
            .unwrap_or(0);
        for entry in i.entries() {
            let idx = match *entry.internal() {
                Internal::Function(i) => i - imported_functions,
                Internal::Memory(_) => {
                    exports.push_str(&format!(
                        "export const {}: WebAssembly.Memory;\n",
                        entry.field()
                    ));
                    continue;
                }
                Internal::Table(_) => {
                    exports.push_str(&format!(
                        "export const {}: WebAssembly.Table;\n",
                        entry.field()
                    ));
                    continue;
                }
                Internal::Global(_) => continue,
            };

            let functions = module
                .function_section()
                .expect("failed to find function section");
            let idx = functions.entries()[idx as usize].type_ref();

            let types = module
                .type_section()
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

            exports.push_str(&format!(
                "export function {name}({args}): {ret};\n",
                name = entry.field(),
                args = args,
                ret = if ty.return_type().is_some() {
                    "number"
                } else {
                    "void"
                },
            ));
        }
    }

    return exports;
}

impl Output {
    pub fn typescript(&self) -> String {
        let mut ts = typescript(&self.module);
        if self.base64 {
            ts.push_str("export const booted: Promise<boolean>;\n");
        }
        return ts
    }

    pub fn js_and_wasm(mut self) -> Result<(String, Option<Vec<u8>>), Error> {
        let mut js_imports = String::new();
        let mut exports = String::new();
        let mut set_exports = String::new();
        let mut imports = String::new();

        if let Some(i) = self.module.import_section() {
            let mut set = HashSet::new();
            for entry in i.entries() {
                if !set.insert(entry.module()) {
                    continue;
                }

                let name = (b'a' + (set.len() as u8)) as char;
                js_imports.push_str(&format!(
                    "import * as import_{} from '{}';\n",
                    name,
                    entry.module()
                ));
                imports.push_str(&format!("'{}': import_{}, ", entry.module(), name));
            }
        }

        if let Some(i) = self.module.export_section() {
            for entry in i.entries() {
                exports.push_str("export let ");
                exports.push_str(entry.field());
                exports.push_str(";\n");
                set_exports.push_str(entry.field());
                set_exports.push_str(" = wasm.exports.");
                set_exports.push_str(entry.field());
                set_exports.push_str(";\n");
            }
        }

        // This is sort of tricky, but the gist of it is that if there's a start
        // function we want to defer execution of the start function until after
        // all our module's exports are bound. That way we'll execute it as soon
        // as we're ready, but the module's imports and such will be able to
        // work as everything is wired up.
        //
        // This ends up helping out in situations such as:
        //
        // * The start function calls an imported function
        // * That imported function in turn tries to access the wasm module
        //
        // If we don't do this then the second step won't work because the start
        // function is automatically executed before the promise of
        // instantiation resolves, meaning that we won't actually have anything
        // bound for it to access.
        //
        // If we remove the start function here (via `unstart`) then we'll
        // reexport it as `__wasm2es6js_start` so be manually executed here.
        if self.unstart() {
            set_exports.push_str("wasm.exports.__wasm2es6js_start();\n");
        }

        let inst = format!(
            "
            WebAssembly.instantiate(bytes,{{ {imports} }})
                .then(obj => {{
                    const wasm = obj.instance;
                    {set_exports}
                }})
            ",
            imports = imports,
            set_exports = set_exports,
        );
        let wasm = serialize(self.module).expect("failed to serialize");
        let (bytes, booted) = if self.base64 {
            (
                format!(
                    "
                    let bytes;
                    const base64 = \"{base64}\";
                    if (typeof Buffer === 'undefined') {{
                        bytes = Uint8Array.from(atob(base64), c => c.charCodeAt(0));
                    }} else {{
                        bytes = Buffer.from(base64, 'base64');
                    }}
                    ",
                    base64 = base64::encode(&wasm)
                ),
                inst,
            )
        } else if let Some(ref path) = self.fetch_path {
            (
                String::new(),
                format!(
                    "
                    fetch('{path}')
                        .then(res => res.arrayBuffer())
                        .then(bytes => {inst})
                    ",
                    path = path,
                    inst = inst
                ),
            )
        } else {
            bail!("the option --base64 or --fetch is required");
        };
        let js = format!(
            "\
            {js_imports}
            {bytes}
            export const booted = {booted};
            {exports}
            ",
            bytes = bytes,
            booted = booted,
            js_imports = js_imports,
            exports = exports,
        );
        let wasm = if self.base64 { None } else { Some(wasm) };
        Ok((js, wasm))
    }

    /// See comments above for what this is doing, but in a nutshell this
    /// removes the start section, if any, and moves it to an exported function.
    /// Returns whether a start function was found and removed.
    fn unstart(&mut self) -> bool {
        let mut start = None;
        for (i, section) in self.module.sections().iter().enumerate() {
            if let Section::Start(idx) = section  {
                start = Some((i, *idx));
                break;
            }
        }
        let (i, idx) = match start {
            Some(p) => p,
            None => return false,
        };
        self.module.sections_mut().remove(i);
        let entry = ExportEntry::new(
            "__wasm2es6js_start".to_string(),
            Internal::Function(idx),
        );
        self.module.export_section_mut().unwrap().entries_mut().push(entry);
        true
    }
}
