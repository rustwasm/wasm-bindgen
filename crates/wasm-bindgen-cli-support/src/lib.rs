#[macro_use]
extern crate failure;
extern crate parity_wasm;
extern crate wasm_bindgen_shared as shared;
extern crate serde_json;
extern crate wasm_gc;

use std::char;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::slice;

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
        let programs = extract_programs(&mut module);

        let (js, ts) = {
            let mut cx = js::Context {
                globals: String::new(),
                imports: String::new(),
                typescript: format!("/* tslint:disable */\n"),
                exposed_globals: Default::default(),
                required_internal_exports: Default::default(),
                imports_to_rewrite: Default::default(),
                custom_type_names: Default::default(),
                config: &self,
                module: &mut module,
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

        let wasm_path = out_dir.join(format!("{}_wasm", stem)).with_extension("wasm");
        let wasm_bytes = parity_wasm::serialize(module).map_err(|e| {
            format_err!("{:?}", e)
        })?;
        let bytes = wasm_gc::Config::new()
            .demangle(false)
            .gc(&wasm_bytes)?;
        File::create(&wasm_path)?.write_all(&bytes)?;
        Ok(())
    }
}

fn extract_programs(module: &mut Module) -> Vec<shared::Program> {
    let data = module.sections_mut()
        .iter_mut()
        .filter_map(|s| {
            match *s {
                Section::Data(ref mut s) => Some(s),
                _ => None,
            }
        })
        .next();

    let mut ret = Vec::new();
    let data = match data {
        Some(data) => data,
        None => return ret,
    };

    'outer:
    for i in (0..data.entries().len()).rev() {
        {
            let mut value = bytes_to_u32(data.entries()[i].value());
            loop {
                match value.iter().position(|i| i.0 == (b'w' as u32)) {
                    Some(i) => value = &value[i + 1..],
                    None => continue 'outer,
                }
                match value.iter().position(|i| i.0 == (b'b' as u32)) {
                    Some(i) => value = &value[i + 1..],
                    None => continue 'outer,
                }
                match value.iter().position(|i| i.0 == (b'g' as u32)) {
                    Some(i) => value = &value[i + 1..],
                    None => continue 'outer,
                }
                match value.iter().position(|i| i.0 == (b':' as u32)) {
                    Some(i) => value = &value[i + 1..],
                    None => continue 'outer,
                }
                break
            }
            // TODO: shouldn't take the rest of the value
            let json = value.iter()
                .map(|i| char::from_u32(i.0).unwrap())
                .collect::<String>();
            let p = match serde_json::from_str(&json) {
                Ok(f) => f,
                Err(e) => {
                    panic!("failed to decode what looked like wasm-bindgen data: {}", e)
                }
            };
            ret.push(p);
        }
        data.entries_mut().remove(i);
    }
    return ret
}

#[repr(packed)]
struct Unaligned(u32);

fn bytes_to_u32(a: &[u8]) -> &[Unaligned] {
    unsafe {
        slice::from_raw_parts(a.as_ptr() as *const Unaligned, a.len() / 4)
    }
}
