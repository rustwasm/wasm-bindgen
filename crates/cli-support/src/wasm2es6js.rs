extern crate base64;
extern crate tempfile;

use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{self, Read, Write};
use std::process::Command;

use failure::{Error, ResultExt};
use parity_wasm::elements::*;

pub struct Config {
    base64: bool,
    wasm2asm: bool,
    fetch_path: Option<String>,
}

pub struct Output {
    module: Module,
    base64: bool,
    wasm2asm: bool,
    fetch_path: Option<String>,
}

impl Config {
    pub fn new() -> Config {
        Config {
            base64: false,
            wasm2asm: false,
            fetch_path: None,
        }
    }

    pub fn base64(&mut self, base64: bool) -> &mut Self {
        self.base64 = base64;
        self
    }

    pub fn wasm2asm(&mut self, wasm2asm: bool) -> &mut Self {
        self.wasm2asm = wasm2asm;
        self
    }

    pub fn fetch(&mut self, path: Option<String>) -> &mut Self {
        self.fetch_path = path;
        self
    }

    pub fn generate(&mut self, wasm: &[u8]) -> Result<Output, Error> {
        if !self.base64 && !self.fetch_path.is_some() && !self.wasm2asm {
            bail!("one of --base64, --fetch, or --wasm2asm is required");
        }
        let module = deserialize_buffer(wasm)?;
        Ok(Output {
            module,
            base64: self.base64,
            wasm2asm: self.wasm2asm,
            fetch_path: self.fetch_path.clone(),
        })
    }
}

impl Output {
    pub fn typescript(&self) -> String {
        let mut exports = format!("/* tslint:disable */\n");

        if let Some(i) = self.module.export_section() {
            let imported_functions = self
                .module
                .import_section()
                .map(|m| m.functions() as u32)
                .unwrap_or(0);
            for entry in i.entries() {
                let idx = match *entry.internal() {
                    Internal::Function(i) => i - imported_functions,
                    Internal::Memory(_) => {
                        exports.push_str(&format!(
                            "
                            export const {}: WebAssembly.Memory;
                            ",
                            entry.field()
                        ));
                        continue;
                    }
                    Internal::Table(_) => {
                        exports.push_str(&format!(
                            "
                            export const {}: WebAssembly.Table;
                            ",
                            entry.field()
                        ));
                        continue;
                    }
                    Internal::Global(_) => continue,
                };

                let functions = self
                    .module
                    .function_section()
                    .expect("failed to find function section");
                let idx = functions.entries()[idx as usize].type_ref();

                let types = self
                    .module
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
                    "
                    export function {name}({args}): {ret};
                    ",
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

        if self.base64 {
            exports.push_str("export const booted: Promise<boolean>;");
        }

        return exports;
    }

    pub fn js(self) -> Result<String, Error> {
        if self.wasm2asm {
            return self.js_wasm2asm();
        }
        let mut js_imports = String::new();
        let mut exports = String::new();
        let mut set_exports = String::new();
        let mut imports = String::new();

        if let Some(i) = self.module.import_section() {
            let mut set = HashSet::new();
            for entry in i.entries() {
                match *entry.external() {
                    External::Function(_) => {}
                    External::Table(_) => {
                        bail!("wasm imports a table which isn't supported yet");
                    }
                    External::Memory(_) => {
                        bail!("wasm imports memory which isn't supported yet");
                    }
                    External::Global(_) => {
                        bail!("wasm imports globals which aren't supported yet");
                    }
                }

                if !set.insert(entry.module()) {
                    continue;
                }

                let name = (b'a' + (set.len() as u8)) as char;
                js_imports.push_str(&format!(
                    "import * as import_{} from '{}';",
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
        let (bytes, booted) = if self.base64 {
            let wasm = serialize(self.module).expect("failed to serialize");
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
        Ok(format!(
            "
            {js_imports}
            {bytes}
            export const booted = {booted};
            {exports}
            ",
            bytes = bytes,
            booted = booted,
            js_imports = js_imports,
            exports = exports,
        ))
    }

    fn js_wasm2asm(self) -> Result<String, Error> {
        let mut js_imports = String::new();
        let mut imported_items = Vec::new();
        if let Some(i) = self.module.import_section() {
            let mut name_map = HashMap::new();
            for (i, entry) in i.entries().iter().enumerate() {
                match *entry.external() {
                    External::Function(_) => {}
                    External::Table(_) => {
                        bail!("wasm imports a table which isn't supported yet");
                    }
                    External::Memory(_) => {
                        bail!("wasm imports memory which isn't supported yet");
                    }
                    External::Global(_) => {
                        bail!("wasm imports globals which aren't supported yet");
                    }
                }

                let m = name_map.entry(entry.field()).or_insert(entry.module());
                if *m != entry.module() {
                    bail!(
                        "the name `{}` is imported from two differnet \
                         modules which currently isn't supported in `wasm2asm` \
                         mode"
                    );
                }

                let name = format!("import{}", i);
                js_imports.push_str(&format!(
                    "import {{ {} as {} }} from '{}';\n",
                    entry.field(),
                    name,
                    entry.module()
                ));
                imported_items.push((entry.field().to_string(), name));
            }
        }

        let mut js_exports = String::new();
        if let Some(i) = self.module.export_section() {
            let mut export_mem = false;
            for entry in i.entries() {
                match *entry.internal() {
                    Internal::Function(_) => {}
                    Internal::Memory(_) => export_mem = true,
                    Internal::Table(_) => continue,
                    Internal::Global(_) => continue,
                };

                js_exports.push_str(&format!("export const {0} = ret.{0};\n", entry.field()));
            }
            if !export_mem {
                bail!(
                    "the `wasm2asm` mode is currently only compatible with \
                     modules that export memory"
                )
            }
        }

        let memory_size = self.module.memory_section().unwrap().entries()[0]
            .limits()
            .initial();

        let mut js_init_mem = String::new();
        if let Some(s) = self.module.data_section() {
            for entry in s.entries() {
                let offset = entry.offset().code();
                if offset.len() != 2 {
                    bail!("don't recognize data offset {:?}", offset)
                }
                if offset[1] != Instruction::End {
                    bail!("don't recognize data offset {:?}", offset)
                }
                let offset = match offset[0] {
                    Instruction::I32Const(x) => x,
                    _ => bail!("don't recognize data offset {:?}", offset),
                };

                let base64 = base64::encode(entry.value());
                js_init_mem.push_str(&format!("_assign({}, \"{}\");\n", offset, base64));
            }
        }

        let td = tempfile::tempdir()?;
        let wasm = serialize(self.module)?;
        let wasm_file = td.as_ref().join("foo.wasm");
        File::create(&wasm_file)
            .and_then(|mut f| f.write_all(&wasm))
            .with_context(|_| format!("failed to write wasm to `{}`", wasm_file.display()))?;

        let wast_file = td.as_ref().join("foo.wast");
        run(
            Command::new("wasm-dis")
                .arg(&wasm_file)
                .arg("-o")
                .arg(&wast_file),
            "wasm-dis",
        )?;
        let js_file = td.as_ref().join("foo.js");
        run(
            Command::new("wasm2asm")
                .arg(&wast_file)
                .arg("-o")
                .arg(&js_file),
            "wasm2asm",
        )?;

        let mut asm_func = String::new();
        File::open(&js_file)
            .and_then(|mut f| f.read_to_string(&mut asm_func))
            .with_context(|_| format!("failed to read `{}`", js_file.display()))?;

        let mut make_imports = String::from(
            "
            var imports = {};
        ",
        );
        for (name, import) in imported_items {
            make_imports.push_str(&format!("imports['{}'] = {};\n", name, import));
        }

        Ok(format!(
            "\
            {js_imports}

            {asm_func}

            const mem = new ArrayBuffer({mem_size});
            const _mem = new Uint8Array(mem);
            function _assign(offset, s) {{
                if (typeof Buffer === 'undefined') {{
                    const bytes = atob(s);
                    for (let i = 0; i < bytes.length; i++)
                        _mem[offset + i] = bytes.charCodeAt(i);
                }} else {{
                    const bytes = Buffer.from(s, 'base64');
                    for (let i = 0; i < bytes.length; i++)
                        _mem[offset + i] = bytes[i];
                }}
            }}
            {js_init_mem}
            {make_imports}
            const ret = asmFunc({{
                Math,
                Int8Array,
                Uint8Array,
                Int16Array,
                Uint16Array,
                Int32Array,
                Uint32Array,
                Float32Array,
                Float64Array,
                NaN,
                Infinity,
            }}, imports, mem);
            {js_exports}
            ",
            js_imports = js_imports,
            js_init_mem = js_init_mem,
            asm_func = asm_func,
            js_exports = js_exports,
            make_imports = make_imports,
            mem_size = memory_size * (1 << 16),
        ))
    }
}

fn run(cmd: &mut Command, program: &str) -> Result<(), Error> {
    let output = cmd.output().with_context(|e| {
        if e.kind() == io::ErrorKind::NotFound {
            format!(
                "failed to execute `{}`, is the tool installed \
                 from the binaryen project?\ncommand line: {:?}",
                program, cmd
            )
        } else {
            format!("failed to execute: {:?}", cmd)
        }
    })?;
    if output.status.success() {
        return Ok(());
    }

    let mut s = format!("failed to execute: {:?}\nstatus: {}\n", cmd, output.status);
    if !output.stdout.is_empty() {
        s.push_str(&format!(
            "----- stdout ------\n{}\n",
            String::from_utf8_lossy(&output.stdout)
        ));
    }
    if !output.stderr.is_empty() {
        s.push_str(&format!(
            "----- stderr ------\n{}\n",
            String::from_utf8_lossy(&output.stderr)
        ));
    }
    bail!("{}", s)
}
