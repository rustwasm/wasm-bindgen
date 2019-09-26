use failure::{bail, Error};
use std::collections::HashSet;
use walrus::Module;

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
        let module = Module::from_buffer(wasm)?;
        Ok(Output {
            module,
            base64: self.base64,
            fetch_path: self.fetch_path.clone(),
        })
    }
}

pub fn typescript(module: &Module) -> Result<String, Error> {
    let mut exports = format!("/* tslint:disable */\n");

    for entry in module.exports.iter() {
        let id = match entry.item {
            walrus::ExportItem::Function(i) => i,
            walrus::ExportItem::Memory(_) => {
                exports.push_str(&format!(
                    "export const {}: WebAssembly.Memory;\n",
                    entry.name,
                ));
                continue;
            }
            walrus::ExportItem::Table(_) => {
                exports.push_str(&format!(
                    "export const {}: WebAssembly.Table;\n",
                    entry.name,
                ));
                continue;
            }
            walrus::ExportItem::Global(_) => continue,
        };

        let func = module.funcs.get(id);
        let ty = module.types.get(func.ty());
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
            name = entry.name,
            args = args,
            ret = match ty.results().len() {
                0 => "void",
                1 => "number",
                _ => bail!("cannot support multi-return yet"),
            },
        ));
    }

    Ok(exports)
}

impl Output {
    pub fn typescript(&self) -> Result<String, Error> {
        let mut ts = typescript(&self.module)?;
        if self.base64 {
            ts.push_str("export const booted: Promise<boolean>;\n");
        }
        Ok(ts)
    }

    pub fn js_and_wasm(mut self) -> Result<(String, Option<Vec<u8>>), Error> {
        let mut js_imports = String::new();
        let mut exports = String::new();
        let mut set_exports = String::new();
        let mut imports = String::new();

        let mut set = HashSet::new();
        for entry in self.module.imports.iter() {
            if !set.insert(&entry.module) {
                continue;
            }

            let name = (b'a' + (set.len() as u8)) as char;
            js_imports.push_str(&format!(
                "import * as import_{} from '{}';\n",
                name, entry.module
            ));
            imports.push_str(&format!("'{}': import_{}, ", entry.module, name));
        }

        for entry in self.module.exports.iter() {
            exports.push_str("export let ");
            exports.push_str(&entry.name);
            exports.push_str(";\n");
            set_exports.push_str(&entry.name);
            set_exports.push_str(" = wasm.exports.");
            set_exports.push_str(&entry.name);
            set_exports.push_str(";\n");
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
        let wasm = self.module.emit_wasm();
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
        let start = match self.module.start.take() {
            Some(id) => id,
            None => return false,
        };
        self.module.exports.add("__wasm2es6js_start", start);
        true
    }
}
