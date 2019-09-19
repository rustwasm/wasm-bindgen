#![doc(html_root_url = "https://docs.rs/wasm-bindgen-cli-support/0.2")]

use failure::{bail, Error, ResultExt};
use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::env;
use std::fs;
use std::mem;
use std::path::{Path, PathBuf};
use std::str;
use walrus::Module;
use wasm_bindgen_wasm_conventions as wasm_conventions;

mod anyref;
mod decode;
mod descriptor;
mod descriptors;
mod intrinsic;
mod js;
pub mod wasm2es6js;
mod webidl;

pub struct Bindgen {
    input: Input,
    out_name: Option<String>,
    mode: OutputMode,
    debug: bool,
    typescript: bool,
    demangle: bool,
    keep_debug: bool,
    remove_name_section: bool,
    remove_producers_section: bool,
    emit_start: bool,
    // Experimental support for weakrefs, an upcoming ECMAScript feature.
    // Currently only enable-able through an env var.
    weak_refs: bool,
    // Support for the wasm threads proposal, transforms the wasm module to be
    // "ready to be instantiated on any thread"
    threads: wasm_bindgen_threads_xform::Config,
    anyref: bool,
    multi_value: bool,
    wasm_interface_types: bool,
    encode_into: EncodeInto,
}

pub struct Output {
    module: walrus::Module,
    stem: String,
    js: String,
    ts: String,
    mode: OutputMode,
    typescript: bool,
    snippets: HashMap<String, Vec<String>>,
    local_modules: HashMap<String, String>,
    npm_dependencies: HashMap<String, (PathBuf, String)>,
    wasm_interface_types: bool,
}

#[derive(Clone)]
enum OutputMode {
    Bundler { browser_only: bool },
    Web,
    NoModules { global: String },
    Node { experimental_modules: bool },
}

enum Input {
    Path(PathBuf),
    Module(Module, String),
    None,
}

pub enum EncodeInto {
    Test,
    Always,
    Never,
}

impl Bindgen {
    pub fn new() -> Bindgen {
        let anyref = env::var("WASM_BINDGEN_ANYREF").is_ok();
        let wasm_interface_types = env::var("WASM_INTERFACE_TYPES").is_ok();
        let multi_value = env::var("WASM_BINDGEN_MULTI_VALUE").is_ok();
        Bindgen {
            input: Input::None,
            out_name: None,
            mode: OutputMode::Bundler {
                browser_only: false,
            },
            debug: false,
            typescript: false,
            demangle: true,
            keep_debug: false,
            remove_name_section: false,
            remove_producers_section: false,
            emit_start: true,
            weak_refs: env::var("WASM_BINDGEN_WEAKREF").is_ok(),
            threads: threads_config(),
            anyref: anyref || wasm_interface_types,
            multi_value,
            wasm_interface_types,
            encode_into: EncodeInto::Test,
        }
    }

    pub fn input_path<P: AsRef<Path>>(&mut self, path: P) -> &mut Bindgen {
        self.input = Input::Path(path.as_ref().to_path_buf());
        self
    }

    pub fn out_name(&mut self, name: &str) -> &mut Bindgen {
        self.out_name = Some(name.to_string());
        self
    }

    /// Explicitly specify the already parsed input module.
    pub fn input_module(&mut self, name: &str, module: Module) -> &mut Bindgen {
        let name = name.to_string();
        self.input = Input::Module(module, name);
        return self;
    }

    fn switch_mode(&mut self, mode: OutputMode, flag: &str) -> Result<(), Error> {
        match self.mode {
            OutputMode::Bundler { .. } => self.mode = mode,
            _ => bail!(
                "cannot specify `{}` with another output mode already specified",
                flag
            ),
        }
        Ok(())
    }

    pub fn nodejs(&mut self, node: bool) -> Result<&mut Bindgen, Error> {
        if node {
            self.switch_mode(
                OutputMode::Node {
                    experimental_modules: false,
                },
                "--target nodejs",
            )?;
        }
        Ok(self)
    }

    pub fn nodejs_experimental_modules(&mut self, node: bool) -> Result<&mut Bindgen, Error> {
        if node {
            self.switch_mode(
                OutputMode::Node {
                    experimental_modules: true,
                },
                "--nodejs-experimental-modules",
            )?;
        }
        Ok(self)
    }

    pub fn bundler(&mut self, bundler: bool) -> Result<&mut Bindgen, Error> {
        if bundler {
            self.switch_mode(
                OutputMode::Bundler {
                    browser_only: false,
                },
                "--target bundler",
            )?;
        }
        Ok(self)
    }

    pub fn web(&mut self, web: bool) -> Result<&mut Bindgen, Error> {
        if web {
            self.switch_mode(OutputMode::Web, "--target web")?;
        }
        Ok(self)
    }

    pub fn no_modules(&mut self, no_modules: bool) -> Result<&mut Bindgen, Error> {
        if no_modules {
            self.switch_mode(
                OutputMode::NoModules {
                    global: "wasm_bindgen".to_string(),
                },
                "--target no-modules",
            )?;
        }
        Ok(self)
    }

    pub fn browser(&mut self, browser: bool) -> Result<&mut Bindgen, Error> {
        if browser {
            match &mut self.mode {
                OutputMode::Bundler { browser_only } => *browser_only = true,
                _ => bail!("cannot specify `--browser` with other output types"),
            }
        }
        Ok(self)
    }

    pub fn no_modules_global(&mut self, name: &str) -> Result<&mut Bindgen, Error> {
        match &mut self.mode {
            OutputMode::NoModules { global } => *global = name.to_string(),
            _ => bail!("can only specify `--no-modules-global` with `--target no-modules`"),
        }
        Ok(self)
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

    pub fn remove_name_section(&mut self, remove: bool) -> &mut Bindgen {
        self.remove_name_section = remove;
        self
    }

    pub fn remove_producers_section(&mut self, remove: bool) -> &mut Bindgen {
        self.remove_producers_section = remove;
        self
    }

    pub fn emit_start(&mut self, emit: bool) -> &mut Bindgen {
        self.emit_start = emit;
        self
    }

    pub fn encode_into(&mut self, mode: EncodeInto) -> &mut Bindgen {
        self.encode_into = mode;
        self
    }

    pub fn generate<P: AsRef<Path>>(&mut self, path: P) -> Result<(), Error> {
        self.generate_output()?.emit(path.as_ref())
    }

    pub fn generate_output(&mut self) -> Result<Output, Error> {
        let (mut module, stem) = match self.input {
            Input::None => bail!("must have an input by now"),
            Input::Module(ref mut m, ref name) => {
                let blank_module = Module::default();
                (mem::replace(m, blank_module), &name[..])
            }
            Input::Path(ref path) => {
                let contents = fs::read(&path)
                    .with_context(|_| format!("failed to read `{}`", path.display()))?;
                let module = walrus::ModuleConfig::new()
                    // Skip validation of the module as LLVM's output is
                    // generally already well-formed and so we won't gain much
                    // from re-validating. Additionally LLVM's current output
                    // for threads includes atomic instructions but doesn't
                    // include shared memory, so it fails that part of
                    // validation!
                    .strict_validate(false)
                    .generate_dwarf(self.keep_debug)
                    .generate_name_section(!self.remove_name_section)
                    .generate_producers_section(!self.remove_producers_section)
                    .on_parse(wasm_webidl_bindings::binary::on_parse)
                    .parse(&contents)
                    .context("failed to parse input file as wasm")?;
                let stem = match &self.out_name {
                    Some(name) => &name,
                    None => path.file_stem().unwrap().to_str().unwrap(),
                };
                (module, stem)
            }
        };

        // Our threads and multi-value xforms rely on the presence of the stack
        // pointer, so temporarily export it so that our many GC's don't remove
        // it before the xform runs.
        let mut exported_shadow_stack_pointer = false;
        if self.multi_value || self.threads.is_enabled() {
            wasm_conventions::export_shadow_stack_pointer(&mut module)?;
            exported_shadow_stack_pointer = true;
        }

        // This isn't the hardest thing in the world too support but we
        // basically don't know how to rationalize #[wasm_bindgen(start)] and
        // the actual `start` function if present. Figure this out later if it
        // comes up, but otherwise we should continue to be compatible with
        // LLVM's output today.
        //
        // Note that start function handling in `js/mod.rs` will need to be
        // updated as well, because `#[wasm_bindgen(start)]` is inserted *after*
        // a module's start function, if any, because we assume start functions
        // only show up when injected on behalf of wasm-bindgen's passes.
        if module.start.is_some() {
            bail!(
                "wasm-bindgen is currently incompatible with modules that \
                 already have a start function"
            );
        }

        self.threads
            .run(&mut module)
            .with_context(|_| "failed to prepare module for threading")?;

        // If requested, turn all mangled symbols into prettier unmangled
        // symbols with the help of `rustc-demangle`.
        if self.demangle {
            demangle(&mut module);
        }
        unexported_unused_lld_things(&mut module);

        // We're making quite a few changes, list ourselves as a producer.
        module
            .producers
            .add_processed_by("wasm-bindgen", &wasm_bindgen_shared::version());

        // Learn about the type signatures of all wasm-bindgen imports and
        // exports by executing `__wbindgen_describe_*` functions. This'll
        // effectively move all the descriptor functions to their own custom
        // sections.
        descriptors::execute(&mut module)?;

        // Process and remove our raw custom sections emitted by the
        // #[wasm_bindgen] macro and the compiler. In their stead insert a
        // forward-compatible WebIDL bindings section (forward-compatible with
        // the webidl bindings proposal) as well as an auxiliary section for all
        // sorts of miscellaneous information and features #[wasm_bindgen]
        // supports that aren't covered by WebIDL bindings.
        webidl::process(
            &mut module,
            self.anyref,
            self.wasm_interface_types,
            self.emit_start,
        )?;

        // Now that we've got type information from the webidl processing pass,
        // touch up the output of rustc to insert anyref shims where necessary.
        // This is only done if the anyref pass is enabled, which it's
        // currently off-by-default since `anyref` is still in development in
        // engines.
        if self.anyref {
            anyref::process(&mut module, self.wasm_interface_types)?;
        }

        let aux = module
            .customs
            .delete_typed::<webidl::WasmBindgenAux>()
            .expect("aux section should be present");
        let mut bindings = module
            .customs
            .delete_typed::<webidl::NonstandardWebidlSection>()
            .unwrap();

        // Now that our module is massaged and good to go, feed it into the JS
        // shim generation which will actually generate JS for all this.
        let (npm_dependencies, (js, ts)) = {
            let mut cx = js::Context::new(&mut module, self)?;
            cx.generate(&aux, &bindings)?;
            let npm_dependencies = cx.npm_dependencies.clone();
            (npm_dependencies, cx.finalize(stem)?)
        };

        if self.wasm_interface_types {
            if self.multi_value {
                webidl::standard::add_multi_value(&mut module, &mut bindings)
                    .context("failed to transform return pointers into multi-value Wasm")?;
            }
            webidl::standard::add_section(&mut module, &aux, &bindings)
                .with_context(|_| "failed to generate a standard wasm bindings custom section")?;
        } else {
            if self.multi_value {
                failure::bail!(
                    "Wasm multi-value is currently only available when \
                     Wasm interface types is also enabled"
                );
            }
        }

        // If we exported the shadow stack pointer earlier, remove it from the
        // export set now.
        if exported_shadow_stack_pointer {
            wasm_conventions::unexport_shadow_stack_pointer(&mut module)?;
            // The shadow stack pointer is potentially unused now, but since it
            // most likely _is_ in use, we don't pay the cost of a full GC here
            // just to remove one potentially unnecessary global.
            //
            // walrus::passes::gc::run(&mut module);
        }

        Ok(Output {
            module,
            stem: stem.to_string(),
            snippets: aux.snippets.clone(),
            local_modules: aux.local_modules.clone(),
            npm_dependencies,
            js,
            ts,
            mode: self.mode.clone(),
            typescript: self.typescript,
            wasm_interface_types: self.wasm_interface_types,
        })
    }

    fn local_module_name(&self, module: &str) -> String {
        format!("./snippets/{}", module)
    }

    fn inline_js_module_name(
        &self,
        unique_crate_identifier: &str,
        snippet_idx_in_crate: usize,
    ) -> String {
        format!(
            "./snippets/{}/inline{}.js",
            unique_crate_identifier, snippet_idx_in_crate,
        )
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
        let extra = if line.starts_with(':') || line.starts_with('?') {
            1
        } else {
            0
        };
        if !line.is_empty() {
            for _ in 0..indent + extra {
                dst.push_str("    ");
            }
            dst.push_str(line);
        }
        dst.push_str("\n");
        if line.ends_with('{') {
            indent += 1;
        }
    }
    return dst;
}

// Eventually these will all be CLI options, but while they're unstable features
// they're left as environment variables. We don't guarantee anything about
// backwards-compatibility with these options.
fn threads_config() -> wasm_bindgen_threads_xform::Config {
    let mut cfg = wasm_bindgen_threads_xform::Config::new();
    if let Ok(s) = env::var("WASM_BINDGEN_THREADS_MAX_MEMORY") {
        cfg.maximum_memory(s.parse().unwrap());
    }
    if let Ok(s) = env::var("WASM_BINDGEN_THREADS_STACK_SIZE") {
        cfg.thread_stack_size(s.parse().unwrap());
    }
    cfg
}

fn demangle(module: &mut Module) {
    for func in module.funcs.iter_mut() {
        let name = match &func.name {
            Some(name) => name,
            None => continue,
        };
        if let Ok(sym) = rustc_demangle::try_demangle(name) {
            func.name = Some(sym.to_string());
        }
    }
}

impl OutputMode {
    fn uses_es_modules(&self) -> bool {
        match self {
            OutputMode::Bundler { .. }
            | OutputMode::Web
            | OutputMode::Node {
                experimental_modules: true,
            } => true,
            _ => false,
        }
    }

    fn nodejs_experimental_modules(&self) -> bool {
        match self {
            OutputMode::Node {
                experimental_modules,
            } => *experimental_modules,
            _ => false,
        }
    }

    fn nodejs(&self) -> bool {
        match self {
            OutputMode::Node { .. } => true,
            _ => false,
        }
    }

    fn no_modules(&self) -> bool {
        match self {
            OutputMode::NoModules { .. } => true,
            _ => false,
        }
    }

    fn always_run_in_browser(&self) -> bool {
        match self {
            OutputMode::Web => true,
            OutputMode::NoModules { .. } => true,
            OutputMode::Bundler { browser_only } => *browser_only,
            _ => false,
        }
    }

    fn web(&self) -> bool {
        match self {
            OutputMode::Web => true,
            _ => false,
        }
    }

    fn bundler(&self) -> bool {
        match self {
            OutputMode::Bundler { .. } => true,
            _ => false,
        }
    }
}

/// Remove a number of internal exports that are synthesized by Rust's linker,
/// LLD. These exports aren't typically ever needed and just add extra space to
/// the binary.
fn unexported_unused_lld_things(module: &mut Module) {
    let mut to_remove = Vec::new();
    for export in module.exports.iter() {
        match export.name.as_str() {
            "__heap_base" | "__data_end" | "__indirect_function_table" => {
                to_remove.push(export.id());
            }
            _ => {}
        }
    }
    for id in to_remove {
        module.exports.delete(id);
    }
}

impl Output {
    pub fn js(&self) -> &str {
        assert!(!self.wasm_interface_types);
        &self.js
    }

    pub fn wasm(&self) -> &walrus::Module {
        &self.module
    }

    pub fn emit(&self, out_dir: impl AsRef<Path>) -> Result<(), Error> {
        self._emit(out_dir.as_ref())
    }

    fn _emit(&self, out_dir: &Path) -> Result<(), Error> {
        let wasm_name = if self.wasm_interface_types {
            self.stem.clone()
        } else {
            format!("{}_bg", self.stem)
        };
        let wasm_path = out_dir.join(wasm_name).with_extension("wasm");
        fs::create_dir_all(out_dir)?;
        let wasm_bytes = self.module.emit_wasm();
        fs::write(&wasm_path, wasm_bytes)
            .with_context(|_| format!("failed to write `{}`", wasm_path.display()))?;

        if self.wasm_interface_types {
            return Ok(());
        }

        // Write out all local JS snippets to the final destination now that
        // we've collected them from all the programs.
        for (identifier, list) in self.snippets.iter() {
            for (i, js) in list.iter().enumerate() {
                let name = format!("inline{}.js", i);
                let path = out_dir.join("snippets").join(identifier).join(name);
                fs::create_dir_all(path.parent().unwrap())?;
                fs::write(&path, js)
                    .with_context(|_| format!("failed to write `{}`", path.display()))?;
            }
        }

        for (path, contents) in self.local_modules.iter() {
            let path = out_dir.join("snippets").join(path);
            fs::create_dir_all(path.parent().unwrap())?;
            fs::write(&path, contents)
                .with_context(|_| format!("failed to write `{}`", path.display()))?;
        }

        if self.npm_dependencies.len() > 0 {
            let map = self
                .npm_dependencies
                .iter()
                .map(|(k, v)| (k, &v.1))
                .collect::<BTreeMap<_, _>>();
            let json = serde_json::to_string_pretty(&map)?;
            fs::write(out_dir.join("package.json"), json)?;
        }

        // And now that we've got all our JS and TypeScript, actually write it
        // out to the filesystem.
        let extension = if self.mode.nodejs_experimental_modules() {
            "mjs"
        } else {
            "js"
        };
        let js_path = out_dir.join(&self.stem).with_extension(extension);
        fs::write(&js_path, reset_indentation(&self.js))
            .with_context(|_| format!("failed to write `{}`", js_path.display()))?;

        if self.typescript {
            let ts_path = js_path.with_extension("d.ts");
            fs::write(&ts_path, &self.ts)
                .with_context(|_| format!("failed to write `{}`", ts_path.display()))?;
        }

        if self.mode.nodejs() {
            let js_path = wasm_path.with_extension(extension);
            let shim = self.generate_node_wasm_import(&self.module, &wasm_path);
            fs::write(&js_path, shim)
                .with_context(|_| format!("failed to write `{}`", js_path.display()))?;
        }

        if self.typescript {
            let ts_path = wasm_path.with_extension("d.ts");
            let ts = wasm2es6js::typescript(&self.module)?;
            fs::write(&ts_path, ts)
                .with_context(|_| format!("failed to write `{}`", ts_path.display()))?;
        }

        Ok(())
    }

    fn generate_node_wasm_import(&self, m: &Module, path: &Path) -> String {
        let mut imports = BTreeSet::new();
        for import in m.imports.iter() {
            imports.insert(&import.module);
        }

        let mut shim = String::new();

        if self.mode.nodejs_experimental_modules() {
            for (i, module) in imports.iter().enumerate() {
                shim.push_str(&format!("import * as import{} from '{}';\n", i, module));
            }
            // On windows skip the leading `/` which comes out when we parse a
            // url to use `C:\...` instead of `\C:\...`
            shim.push_str(&format!(
                "
                import * as path from 'path';
                import * as fs from 'fs';
                import * as url from 'url';
                import * as process from 'process';

                let file = path.dirname(url.parse(import.meta.url).pathname);
                if (process.platform === 'win32') {{
                    file = file.substring(1);
                }}
                const bytes = fs.readFileSync(path.join(file, '{}'));
            ",
                path.file_name().unwrap().to_str().unwrap()
            ));
        } else {
            shim.push_str(&format!(
                "
                const path = require('path').join(__dirname, '{}');
                const bytes = require('fs').readFileSync(path);
            ",
                path.file_name().unwrap().to_str().unwrap()
            ));
        }
        shim.push_str("let imports = {};\n");
        for (i, module) in imports.iter().enumerate() {
            if self.mode.nodejs_experimental_modules() {
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

        if self.mode.nodejs_experimental_modules() {
            for entry in m.exports.iter() {
                shim.push_str("export const ");
                shim.push_str(&entry.name);
                shim.push_str(" = wasmInstance.exports.");
                shim.push_str(&entry.name);
                shim.push_str(";\n");
            }
        } else {
            shim.push_str("module.exports = wasmInstance.exports;\n");
        }

        reset_indentation(&shim)
    }
}
