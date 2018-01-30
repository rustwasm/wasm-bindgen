#[macro_use]
extern crate failure;
extern crate parity_wasm;
extern crate wasm_bindgen_shared as shared;
extern crate serde_json;

use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};

use failure::Error;
use parity_wasm::elements::*;

mod js;
pub mod wasm2es6js;

pub struct Bindgen {
    path: Option<PathBuf>,
    nodejs: bool,
    debug: bool,
    typescript: bool,
}

impl Bindgen {
    pub fn new() -> Bindgen {
        Bindgen {
            path: None,
            nodejs: false,
            debug: false,
            typescript: false,
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

    pub fn debug(&mut self, debug: bool) -> &mut Bindgen {
        self.debug = debug;
        self
    }

    pub fn typescript(&mut self, typescript: bool) -> &mut Bindgen {
        self.typescript = typescript;
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
            format_err!("{:?}", e)
        })?;
        let program = extract_program(&mut module);

        let (js, ts) = js::Js {
            globals: String::new(),
            imports: String::new(),
            typescript: format!("/* tslint:disable */\n"),
            exposed_globals: Default::default(),
            config: &self,
            module: &mut module,
            program: &program,
        }.generate(stem);

        let js_path = out_dir.join(stem).with_extension("js");
        File::create(&js_path).unwrap()
            .write_all(js.as_bytes()).unwrap();

        if self.typescript {
            let ts_path = out_dir.join(stem).with_extension("d.ts");
            File::create(&ts_path).unwrap()
                .write_all(ts.as_bytes()).unwrap();
        }

        let wasm_path = out_dir.join(format!("{}_wasm", stem)).with_extension("wasm");
        parity_wasm::serialize_to_file(wasm_path, module).map_err(|e| {
            format_err!("{:?}", e)
        })?;
        Ok(())
    }
}

fn extract_program(module: &mut Module) -> shared::Program {
    let data = module.sections_mut()
        .iter_mut()
        .filter_map(|s| {
            match *s {
                Section::Data(ref mut s) => Some(s),
                _ => None,
            }
        })
        .next();

    let mut ret = shared::Program {
        structs: Vec::new(),
        free_functions: Vec::new(),
        imports: Vec::new(),
    };
    let data = match data {
        Some(data) => data,
        None => return ret,
    };

    for i in (0..data.entries().len()).rev() {
        {
            let value = data.entries()[i].value();
            if !value.starts_with(b"wbg:") {
                continue
            }
            let json = &value[4..];
            let p = match serde_json::from_slice(json) {
                Ok(f) => f,
                Err(_) => continue,
            };
            let shared::Program { structs, free_functions, imports } = p;
            ret.structs.extend(structs);
            ret.free_functions.extend(free_functions);
            ret.imports.extend(imports);
        }
        data.entries_mut().remove(i);
    }
    return ret
}
