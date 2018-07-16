extern crate parity_wasm;
extern crate wasm_bindgen_shared as shared;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate wasm_gc;
extern crate wasmi;
#[macro_use]
extern crate failure;

use std::collections::BTreeSet;
use std::fmt;
use std::fs::File;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

use failure::{Error, ResultExt};
use parity_wasm::elements::*;

mod descriptor;
mod js;
pub mod wasm2es6js;

pub struct Bindgen {
    path: Option<PathBuf>,
    nodejs: bool,
    nodejs_experimental_modules: bool,
    browser: bool,
    no_modules: bool,
    no_modules_global: Option<String>,
    debug: bool,
    typescript: bool,
    demangle: bool,
    keep_debug: bool,
}

impl Bindgen {
    pub fn new() -> Bindgen {
        Bindgen {
            path: None,
            nodejs: false,
            nodejs_experimental_modules: false,
            browser: false,
            no_modules: false,
            no_modules_global: None,
            debug: false,
            typescript: false,
            demangle: true,
            keep_debug: false,
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

    pub fn nodejs_experimental_modules(&mut self, node: bool) -> &mut Bindgen {
        self.nodejs_experimental_modules = node;
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

    pub fn no_modules_global(&mut self, name: &str) -> &mut Bindgen {
        self.no_modules_global = Some(name.to_string());
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

    pub fn keep_debug(&mut self, keep_debug: bool) -> &mut Bindgen {
        self.keep_debug = keep_debug;
        self
    }

    pub fn generate<P: AsRef<Path>>(&mut self, path: P) -> Result<(), Error> {
        self._generate(path.as_ref())
    }

    fn _generate(&mut self, out_dir: &Path) -> Result<(), Error> {
        let input = match self.path {
            Some(ref path) => path,
            None => bail!("must have a path input for now"),
        };
        let stem = input.file_stem().unwrap().to_str().unwrap();
        let mut contents = Vec::new();
        File::open(&input)
            .and_then(|mut f| f.read_to_end(&mut contents))
            .with_context(|_| format!("failed to read `{}`", input.display()))?;
        let mut module = parity_wasm::deserialize_buffer::<Module>(&contents)
            .with_context(|_| "failed to parse input file as wasm")?;
        let programs = extract_programs(&mut module)
            .with_context(|_| "failed to extract wasm-bindgen custom sections")?;

        // Here we're actually instantiating the module we've parsed above for
        // execution. Why, you might be asking, are we executing wasm code? A
        // good question!
        //
        // Transmitting information from `#[wasm_bindgen]` here to the CLI tool
        // is pretty tricky. Specifically information about the types involved
        // with a function signature (especially generic ones) can be hefty to
        // translate over. As a result, the macro emits a bunch of shims which,
        // when executed, will describe to us what the types look like.
        //
        // This means that whenever we encounter an import or export we'll
        // execute a shim function which informs us about its type so we can
        // then generate the appropriate bindings.
        let instance = wasmi::Module::from_buffer(&contents)
            .with_context(|_| "failed to create wasmi module")?;
        let instance = wasmi::ModuleInstance::new(&instance, &MyResolver)
            .with_context(|_| "failed to instantiate wasm module")?;
        let instance = instance.not_started_instance();

        let (js, ts) = {
            let mut cx = js::Context {
                globals: String::new(),
                imports: String::new(),
                footer: String::new(),
                typescript: format!("/* tslint:disable */\n"),
                exposed_globals: Default::default(),
                required_internal_exports: Default::default(),
                imported_names: Default::default(),
                exported_classes: Default::default(),
                config: &self,
                module: &mut module,
                function_table_needed: false,
                module_versions: Default::default(),
                run_descriptor: &|name| {
                    let mut v = MyExternals(Vec::new());
                    match instance.invoke_export(name, &[], &mut v) {
                        Ok(None) => Some(v.0),
                        Ok(Some(_)) => unreachable!(
                            "there is only one export, and we only return None from it"
                        ),
                        // Allow missing exported describe functions. This can
                        // happen when a nested dependency crate exports things
                        // but the root crate doesn't use them.
                        Err(wasmi::Error::Function(_)) => None,
                        Err(e) => panic!("unexpected error running descriptor: {}", e),
                    }
                },
            };
            for program in programs.iter() {
                js::SubContext {
                    program,
                    cx: &mut cx,
                }.generate()?;
            }
            cx.finalize(stem)?
        };

        let extension = if self.nodejs_experimental_modules { "mjs" } else { "js" };
        let js_path = out_dir.join(stem).with_extension(extension);
        File::create(&js_path)
            .and_then(|mut f| f.write_all(reset_indentation(&js).as_bytes()))
            .with_context(|_| format!("failed to write `{}`", js_path.display()))?;

        if self.typescript {
            let ts_path = out_dir.join(stem).with_extension("d.ts");
            File::create(&ts_path)
                .and_then(|mut f| f.write_all(ts.as_bytes()))
                .with_context(|_| format!("failed to write `{}`", ts_path.display()))?;
        }

        let wasm_path = out_dir.join(format!("{}_bg", stem)).with_extension("wasm");

        if self.nodejs {
            let js_path = wasm_path.with_extension(extension);
            let shim = self.generate_node_wasm_import(&module, &wasm_path);
            File::create(&js_path)
                .and_then(|mut f| f.write_all(shim.as_bytes()))
                .with_context(|_| format!("failed to write `{}`", js_path.display()))?;
        }

        let wasm_bytes = parity_wasm::serialize(module)?;
        File::create(&wasm_path)
            .and_then(|mut f| f.write_all(&wasm_bytes))
            .with_context(|_| format!("failed to write `{}`", wasm_path.display()))?;
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

        if self.nodejs_experimental_modules {
            for (i, module) in imports.iter().enumerate() {
                shim.push_str(&format!("import * as import{} from '{}';\n",
                                       i, module));
            }
            // On windows skip the leading `/` which comes out when we parse a
            // url to use `C:\...` instead of `\C:\...`
            shim.push_str(&format!("
                import * as path from 'path';
                import * as fs from 'fs';
                import * as url from 'url';
                import * as process from 'process';

                let file = path.dirname(url.parse(import.meta.url).pathname);
                if (process.platform === 'win32') {{
                    file = file.substring(1);
                }}
                const bytes = fs.readFileSync(path.join(file, '{}'));
            ", path.file_name().unwrap().to_str().unwrap()));
        } else {
            shim.push_str(&format!("
                const path = require('path').join(__dirname, '{}');
                const bytes = require('fs').readFileSync(path);
            ", path.file_name().unwrap().to_str().unwrap()));
        }
        shim.push_str("let imports = {};\n");
        for (i, module) in imports.iter().enumerate() {
            if self.nodejs_experimental_modules {
                shim.push_str(&format!("imports['{}'] = import{};\n", module, i));
            } else {
                shim.push_str(&format!("imports['{0}'] = require('{0}');\n", module));
            }
        }

        shim.push_str(&format!(
            "
                const wasmModule = new WebAssembly.Module(bytes);
                const wasmInstance = new WebAssembly.Instance(wasmModule, imports);
            ",
        ));

        if self.nodejs_experimental_modules {
            if let Some(e) = m.export_section() {
                for name in e.entries().iter().map(|e| e.field()) {
                    shim.push_str("export const ");
                    shim.push_str(name);
                    shim.push_str(" = wasmInstance.exports.");
                    shim.push_str(name);
                    shim.push_str(";\n");
                }
            }
        } else {
            shim.push_str("module.exports = wasmInstance.exports;\n");
        }

        reset_indentation(&shim)
    }
}

fn extract_programs(module: &mut Module) -> Result<Vec<shared::Program>, Error> {
    let version = shared::version();
    let mut ret = Vec::new();
    let mut to_remove = Vec::new();

    for (i, s) in module.sections().iter().enumerate() {
        let custom = match *s {
            Section::Custom(ref s) => s,
            _ => continue,
        };
        if custom.name() != "__wasm_bindgen_unstable" {
            continue;
        }
        to_remove.push(i);

        let mut payload = custom.payload();
        let mut added_programs = Vec::new();
        while payload.len() > 0 {
            let len = ((payload[0] as usize) << 0)
                | ((payload[1] as usize) << 8)
                | ((payload[2] as usize) << 16)
                | ((payload[3] as usize) << 24);
            let (a, b) = payload[4..].split_at(len as usize);
            payload = b;

            // Due to a nasty LLVM bug it's currently possible for LLVM to
            // duplicate custom section directives in intermediate object files.
            // This means that we could see multiple program directives when in
            // fact we were originally only meant to see one!
            //
            // Work around the issue here until the upstream bug,
            // https://bugs.llvm.org/show_bug.cgi?id=38184, is hopefully fixed
            // via some other means.
            if added_programs.iter().any(|p| a == *p) {
                continue
            }
            added_programs.push(a);

            let p: shared::ProgramOnlySchema = match serde_json::from_slice(&a) {
                Ok(f) => f,
                Err(e) => bail!("failed to decode what looked like wasm-bindgen data: {}", e),
            };
            if p.schema_version != shared::SCHEMA_VERSION {
                bail!(
                    "

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

    cargo install -f wasm-bindgen-cli

if this warning fails to go away though and you're not sure what to do feel free
to open an issue at https://github.com/alexcrichton/wasm-bindgen/issues!
",
                    p.version,
                    version
                );
            }
            let p: shared::Program = match serde_json::from_slice(&a) {
                Ok(f) => f,
                Err(e) => bail!("failed to decode what looked like wasm-bindgen data: {}", e),
            };
            ret.push(p);
        }
    }

    for i in to_remove.into_iter().rev() {
        module.sections_mut().remove(i);
    }
    Ok(ret)
}

struct MyResolver;

impl wasmi::ImportResolver for MyResolver {
    fn resolve_func(
        &self,
        module_name: &str,
        field_name: &str,
        signature: &wasmi::Signature,
    ) -> Result<wasmi::FuncRef, wasmi::Error> {
        // Route our special "describe" export to 1 and everything else to 0.
        // That way whenever the function 1 is invoked we know what to do and
        // when 0 is invoked (by accident) we'll trap and produce an error.
        let idx = (module_name == "__wbindgen_placeholder__" && field_name == "__wbindgen_describe")
            as usize;
        Ok(wasmi::FuncInstance::alloc_host(signature.clone(), idx))
    }

    fn resolve_global(
        &self,
        _module_name: &str,
        _field_name: &str,
        descriptor: &wasmi::GlobalDescriptor,
    ) -> Result<wasmi::GlobalRef, wasmi::Error> {
        // dummy implementation to ensure instantiation succeeds
        let val = match descriptor.value_type() {
            wasmi::ValueType::I32 => wasmi::RuntimeValue::I32(0),
            wasmi::ValueType::I64 => wasmi::RuntimeValue::I64(0),
            wasmi::ValueType::F32 => wasmi::RuntimeValue::F32(0.0.into()),
            wasmi::ValueType::F64 => wasmi::RuntimeValue::F64(0.0.into()),
        };
        Ok(wasmi::GlobalInstance::alloc(val, descriptor.is_mutable()))
    }

    fn resolve_memory(
        &self,
        _module_name: &str,
        _field_name: &str,
        descriptor: &wasmi::MemoryDescriptor,
    ) -> Result<wasmi::MemoryRef, wasmi::Error> {
        // dummy implementation to ensure instantiation succeeds
        use wasmi::memory_units::Pages;
        let initial = Pages(descriptor.initial() as usize);
        let maximum = descriptor.maximum().map(|i| Pages(i as usize));
        wasmi::MemoryInstance::alloc(initial, maximum)
    }

    fn resolve_table(
        &self,
        _module_name: &str,
        _field_name: &str,
        descriptor: &wasmi::TableDescriptor,
    ) -> Result<wasmi::TableRef, wasmi::Error> {
        // dummy implementation to ensure instantiation succeeds
        let initial = descriptor.initial();
        let maximum = descriptor.maximum();
        wasmi::TableInstance::alloc(initial, maximum)
    }
}

struct MyExternals(Vec<u32>);

#[derive(Debug)]
struct MyError(String);

impl wasmi::Externals for MyExternals {
    fn invoke_index(
        &mut self,
        index: usize,
        args: wasmi::RuntimeArgs,
    ) -> Result<Option<wasmi::RuntimeValue>, wasmi::Trap> {
        macro_rules! bail {
            ($($t:tt)*) => ({
                let s = MyError(format!($($t)*));
                return Err(wasmi::Trap::new(wasmi::TrapKind::Host(Box::new(s))))
            })
        }
        // We only recognize one function here which was mapped to the index 1
        // by the resolver above.
        if index != 1 {
            bail!("only __wbindgen_describe can be run at this time")
        }
        if args.len() != 1 {
            bail!("must have exactly one argument");
        }
        match args.nth_value_checked(0)? {
            wasmi::RuntimeValue::I32(i) => self.0.push(i as u32),
            _ => bail!("expected one argument of i32 type"),
        }
        Ok(None)
    }
}

impl wasmi::HostError for MyError {}

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

fn reset_indentation(s: &str) -> String {
    let mut indent: u32 = 0;
    let mut dst = String::new();

    for line in s.lines() {
        let line = line.trim();
        if line.starts_with('}') || (line.ends_with('}') && !line.starts_with('*')) {
            indent = indent.saturating_sub(1);
        }
        let extra = if line.starts_with(':') || line.starts_with('?') { 1 } else { 0 };
        for _ in 0..indent + extra {
            dst.push_str("    ");
        }
        dst.push_str(line);
        dst.push_str("\n");
        if line.ends_with('{') {
            indent += 1;
        }
    }
    return dst
}
