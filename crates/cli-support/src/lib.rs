extern crate parity_wasm;
extern crate wasm_bindgen_shared as shared;
extern crate serde_json;
extern crate wasm_gc;

use std::collections::BTreeSet;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};

use parity_wasm::elements::*;

mod js;
pub mod wasm2es6js;

pub struct Bindgen {
    path: Option<PathBuf>,
    nodejs: bool,
    browser: bool,
    no_modules: bool,
    debug: bool,
    typescript: bool,
    demangle: bool,
}

#[derive(Debug)]
pub struct Error(String);

impl<E: std::error::Error> From<E> for Error {
    fn from(e: E) -> Error {
        Error(e.to_string())
    }
}

impl Bindgen {
    pub fn new() -> Bindgen {
        Bindgen {
            path: None,
            nodejs: false,
            browser: false,
            no_modules: false,
            debug: false,
            typescript: false,
            demangle: true,
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

    pub fn browser(&mut self, browser: bool) -> &mut Bindgen {
        self.browser = browser;
        self
    }

    pub fn no_modules(&mut self, no_modules: bool) -> &mut Bindgen {
        self.no_modules = no_modules;
        self
    }

    pub fn debug(&mut self, debug: bool) -> &mut Bindgen {
        self.debug = debug;
        self
    }

    pub fn typescript(&mut self, typescript: bool) -> &mut Bindgen {
        self.typescript = typescript;
        self
    }

    pub fn demangle(&mut self, demangle: bool) -> &mut Bindgen {
        self.demangle = demangle;
        self
    }

    pub fn generate<P: AsRef<Path>>(&mut self, path: P) -> Result<(), Error> {
        self._generate(path.as_ref())
    }

    fn _generate(&mut self, out_dir: &Path) -> Result<(), Error> {
        let input = match self.path {
            Some(ref path) => path,
            None => panic!("must have a path input for now"),
        };
        let stem = input.file_stem().unwrap().to_str().unwrap();
        let mut module = parity_wasm::deserialize_file(input).map_err(|e| {
            Error(format!("{:?}", e))
        })?;
        let programs = extract_programs(&mut module);

        let (js, ts) = {
            let mut cx = js::Context {
                globals: String::new(),
                imports: String::new(),
                footer: String::new(),
                typescript: format!("/* tslint:disable */\n"),
                exposed_globals: Default::default(),
                required_internal_exports: Default::default(),
                custom_type_names: Default::default(),
                imported_names: Default::default(),
                exported_classes: Default::default(),
                config: &self,
                module: &mut module,
                function_table_needed: false,
            };
            for program in programs.iter() {
                cx.add_custom_type_names(program);
            }
            for program in programs.iter() {
                js::SubContext {
                    program,
                    cx: &mut cx,
                }.generate();
            }
            cx.finalize(stem)
        };

        let js_path = out_dir.join(stem).with_extension("js");
        File::create(&js_path).unwrap()
            .write_all(js.as_bytes()).unwrap();

        if self.typescript {
            let ts_path = out_dir.join(stem).with_extension("d.ts");
            File::create(&ts_path).unwrap()
                .write_all(ts.as_bytes()).unwrap();
        }

        let wasm_path = out_dir.join(format!("{}_bg", stem)).with_extension("wasm");

        if self.nodejs {
            let js_path = wasm_path.with_extension("js");
            let shim = self.generate_node_wasm_import(&module, &wasm_path);
            File::create(&js_path)?.write_all(shim.as_bytes())?;
        }

        let wasm_bytes = parity_wasm::serialize(module).map_err(|e| {
            Error(format!("{:?}", e))
        })?;
        File::create(&wasm_path)?.write_all(&wasm_bytes)?;
        Ok(())
    }

    fn generate_node_wasm_import(&self, m: &Module, path: &Path) -> String {
        let mut imports = BTreeSet::new();
        if let Some(i) = m.import_section() {
            for i in i.entries() {
                imports.insert(i.module());
            }
        }

        let mut shim = String::new();
        shim.push_str("let imports = {};\n");
        for module in imports {
            shim.push_str(&format!("imports['{0}'] = require('{0}');\n", module));
        }

        shim.push_str(&format!("
            const join = require('path').join;
            const bytes = require('fs').readFileSync(join(__dirname, '{}'));
            const wasmModule = new WebAssembly.Module(bytes);
            const wasmInstance = new WebAssembly.Instance(wasmModule, imports);
            module.exports = wasmInstance.exports;
        ", path.file_name().unwrap().to_str().unwrap()));

        shim
    }
}

fn extract_programs(module: &mut Module) -> Vec<shared::Program> {
    let version = shared::version();
    let mut ret = Vec::new();

    module.sections_mut().retain(|s| {
        let custom = match *s {
            Section::Custom(ref s) => s,
            _ => return true,
        };
        if custom.name() != "__wasm_bindgen_unstable" {
            return true
        }

        let mut payload = custom.payload();
        while payload.len() > 0 {
            let len =
                ((payload[0] as usize) << 0) |
                ((payload[1] as usize) << 8) |
                ((payload[2] as usize) << 16) |
                ((payload[3] as usize) << 24);
            let (a, b) = payload[4..].split_at(len as usize);
            payload = b;
            let p: shared::Program = match serde_json::from_slice(&a) {
                Ok(f) => f,
                Err(e) => {
                    panic!("failed to decode what looked like wasm-bindgen data: {}", e)
                }
            };
            if p.schema_version != shared::SCHEMA_VERSION {
                panic!("

it looks like the Rust project used to create this wasm file was linked against
a different version of wasm-bindgen than this binary:

  rust wasm file: {}
     this binary: {}

Currently the bindgen format is unstable enough that these two version must
exactly match, so it's required that these two version are kept in sync by
either updating the wasm-bindgen dependency or this binary. You should be able
to update the wasm-bindgen dependency with:

    cargo update -p wasm-bindgen

or you can update the binary with

    cargo install -f --git https://github.com/alexcrichton/wasm-bindgen

if this warning fails to go away though and you're not sure what to do feel free
to open an issue at https://github.com/alexcrichton/wasm-bindgen/issues!
",
    p.version, version);
            }
            ret.push(p);
        }

        false
    });
    return ret
}
