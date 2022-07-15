#![doc(html_root_url = "https://docs.rs/wasm-bindgen-cli-support/0.2")]

use anyhow::{bail, Context, Error};
use std::collections::{BTreeMap, HashMap, HashSet};
use std::env;
use std::fs;
use std::mem;
use std::path::{Path, PathBuf};
use std::str;
use walrus::Module;

pub(crate) const PLACEHOLDER_MODULE: &str = "__wbindgen_placeholder__";

mod decode;
mod descriptor;
mod descriptors;
mod externref;
mod intrinsic;
mod js;
mod multivalue;
mod throw2unreachable;
pub mod wasm2es6js;
mod wit;

pub struct Bindgen {
    input: Input,
    out_name: Option<String>,
    mode: OutputMode,
    debug: bool,
    typescript: bool,
    omit_imports: bool,
    demangle: bool,
    keep_debug: bool,
    remove_name_section: bool,
    remove_producers_section: bool,
    omit_default_module_path: bool,
    emit_start: bool,
    // Experimental support for weakrefs, an upcoming ECMAScript feature.
    // Currently only enable-able through an env var.
    weak_refs: bool,
    // Support for the wasm threads proposal, transforms the wasm module to be
    // "ready to be instantiated on any thread"
    threads: wasm_bindgen_threads_xform::Config,
    externref: bool,
    multi_value: bool,
    wasm_interface_types: bool,
    encode_into: EncodeInto,
}

pub struct Output {
    module: walrus::Module,
    stem: String,
    generated: Generated,
}

enum Generated {
    InterfaceTypes,
    Js(JsGenerated),
}

struct JsGenerated {
    mode: OutputMode,
    js: String,
    ts: String,
    start: Option<String>,
    snippets: HashMap<String, Vec<String>>,
    local_modules: HashMap<String, String>,
    npm_dependencies: HashMap<String, (PathBuf, String)>,
    typescript: bool,
}

#[derive(Clone)]
enum OutputMode {
    Bundler { browser_only: bool },
    Web,
    NoModules { global: String },
    Node { experimental_modules: bool },
    Deno,
}

enum Input {
    Path(PathBuf),
    Module(Module, String),
    Bytes(Vec<u8>, String),
    None,
}

pub enum EncodeInto {
    Test,
    Always,
    Never,
}

impl Bindgen {
    pub fn new() -> Bindgen {
        let externref =
            env::var("WASM_BINDGEN_ANYREF").is_ok() || env::var("WASM_BINDGEN_EXTERNREF").is_ok();
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
            omit_imports: false,
            demangle: true,
            keep_debug: false,
            remove_name_section: false,
            remove_producers_section: false,
            emit_start: true,
            weak_refs: env::var("WASM_BINDGEN_WEAKREF").is_ok(),
            threads: threads_config(),
            externref: externref || wasm_interface_types,
            multi_value: multi_value || wasm_interface_types,
            wasm_interface_types,
            encode_into: EncodeInto::Test,
            omit_default_module_path: true,
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

    pub fn weak_refs(&mut self, enable: bool) -> &mut Bindgen {
        self.weak_refs = enable;
        self
    }

    pub fn reference_types(&mut self, enable: bool) -> &mut Bindgen {
        self.externref = enable;
        self
    }

    /// Explicitly specify the already parsed input module.
    pub fn input_module(&mut self, name: &str, module: Module) -> &mut Bindgen {
        let name = name.to_string();
        self.input = Input::Module(module, name);
        return self;
    }

    /// Specify the input as the provided Wasm bytes.
    pub fn input_bytes(&mut self, name: &str, bytes: Vec<u8>) -> &mut Bindgen {
        let name = name.to_string();
        self.input = Input::Bytes(bytes, name);
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

    pub fn deno(&mut self, deno: bool) -> Result<&mut Bindgen, Error> {
        if deno {
            self.switch_mode(OutputMode::Deno, "--target deno")?;
            self.encode_into(EncodeInto::Always);
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

    pub fn omit_imports(&mut self, omit_imports: bool) -> &mut Bindgen {
        self.omit_imports = omit_imports;
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

    pub fn omit_default_module_path(&mut self, omit_default_module_path: bool) -> &mut Bindgen {
        self.omit_default_module_path = omit_default_module_path;
        self
    }

    pub fn generate<P: AsRef<Path>>(&mut self, path: P) -> Result<(), Error> {
        self.generate_output()?.emit(path.as_ref())
    }

    pub fn stem(&self) -> Result<&str, Error> {
        Ok(match &self.input {
            Input::None => bail!("must have an input by now"),
            Input::Module(_, name) | Input::Bytes(_, name) => name,
            Input::Path(path) => match &self.out_name {
                Some(name) => name,
                None => path.file_stem().unwrap().to_str().unwrap(),
            },
        })
    }

    pub fn generate_output(&mut self) -> Result<Output, Error> {
        let mut module = match self.input {
            Input::None => bail!("must have an input by now"),
            Input::Module(ref mut m, _) => {
                let blank_module = Module::default();
                mem::replace(m, blank_module)
            }
            Input::Path(ref path) => {
                let bytes = std::fs::read(path)
                    .with_context(|| format!("failed reading '{}'", path.display()))?;
                self.module_from_bytes(&bytes).with_context(|| {
                    format!("failed getting Wasm module for '{}'", path.display())
                })?
            }
            Input::Bytes(ref bytes, _) => self
                .module_from_bytes(&bytes)
                .context("failed getting Wasm module")?,
        };

        self.threads
            .run(&mut module)
            .with_context(|| "failed to prepare module for threading")?;

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
        // forward-compatible wasm interface types section as well as an
        // auxiliary section for all sorts of miscellaneous information and
        // features #[wasm_bindgen] supports that aren't covered by wasm
        // interface types.
        wit::process(
            &mut module,
            self.externref,
            self.wasm_interface_types,
            self.emit_start,
        )?;

        // Now that we've got type information from the webidl processing pass,
        // touch up the output of rustc to insert externref shims where necessary.
        // This is only done if the externref pass is enabled, which it's
        // currently off-by-default since `externref` is still in development in
        // engines.
        //
        // If the externref pass isn't necessary, then we blanket delete the
        // export of all our externref intrinsics which will get cleaned up in the
        // GC pass before JS generation.
        if self.externref {
            externref::process(&mut module)?;
        } else {
            let ids = module
                .exports
                .iter()
                .filter(|e| e.name.starts_with("__externref"))
                .map(|e| e.id())
                .collect::<Vec<_>>();
            for id in ids {
                module.exports.delete(id);
            }
            // Clean up element segments as well if they have holes in them
            // after some of our transformations, because non-externref engines
            // only support contiguous arrays of function references in element
            // segments.
            externref::force_contiguous_elements(&mut module)?;
        }

        // If wasm interface types are enabled then the `__wbindgen_throw`
        // intrinsic isn't available but it may be used by our runtime, so
        // change all calls to this function to calls to `unreachable` instead.
        // See more documentation in the pass documentation itself.
        if self.wasm_interface_types {
            throw2unreachable::run(&mut module);
        }

        // Using all of our metadata convert our module to a multi-value using
        // module if applicable.
        if self.multi_value {
            if !self.wasm_interface_types {
                anyhow::bail!(
                    "Wasm multi-value is currently only available when \
                     Wasm interface types is also enabled"
                );
            }
            multivalue::run(&mut module)
                .context("failed to transform return pointers into multi-value Wasm")?;
        }

        // We've done a whole bunch of transformations to the wasm module, many
        // of which leave "garbage" lying around, so let's prune out all our
        // unnecessary things here.
        gc_module_and_adapters(&mut module);

        let stem = self.stem()?;

        // We're ready for the final emission passes now. If we're in wasm
        // interface types mode then we execute the various passes there and
        // generate a valid interface types section into the wasm module.
        //
        // Otherwise we execute the JS generation passes to actually emit
        // JS/TypeScript/etc. The output here is unused in wasm interfac
        let generated = if self.wasm_interface_types {
            wit::section::add(&mut module)
                .context("failed to generate a standard interface types section")?;
            Generated::InterfaceTypes
        } else {
            let aux = module
                .customs
                .delete_typed::<wit::WasmBindgenAux>()
                .expect("aux section should be present");
            let adapters = module
                .customs
                .delete_typed::<wit::NonstandardWitSection>()
                .unwrap();
            let mut cx = js::Context::new(&mut module, self, &adapters, &aux)?;
            cx.generate()?;
            let (js, ts, start) = cx.finalize(stem)?;
            Generated::Js(JsGenerated {
                snippets: aux.snippets.clone(),
                local_modules: aux.local_modules.clone(),
                mode: self.mode.clone(),
                typescript: self.typescript,
                npm_dependencies: cx.npm_dependencies.clone(),
                js,
                ts,
                start,
            })
        };

        Ok(Output {
            module,
            stem: stem.to_string(),
            generated,
        })
    }

    fn module_from_bytes(&self, bytes: &[u8]) -> Result<Module, Error> {
        let wasm = wit_text::parse_bytes(bytes).context("failed to parse bytes")?;
        wit_validator::validate(&wasm).context("failed to validate")?;
        walrus::ModuleConfig::new()
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
            .on_parse(wit_walrus::on_parse)
            .parse(&wasm)
            .context("failed to parse input as wasm")
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
        // Ignore { inside of comments and if it's an exported enum
        if line.ends_with('{') && !line.starts_with('*') && !line.ends_with("Object.freeze({") {
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
            }
            | OutputMode::Deno => true,
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

    fn web(&self) -> bool {
        match self {
            OutputMode::Web => true,
            _ => false,
        }
    }

    fn esm_integration(&self) -> bool {
        match self {
            OutputMode::Bundler { .. }
            | OutputMode::Node {
                experimental_modules: true,
            } => true,
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
        &self.gen().js
    }

    pub fn ts(&self) -> Option<&str> {
        let gen = self.gen();
        if gen.typescript {
            Some(&gen.ts)
        } else {
            None
        }
    }

    pub fn start(&self) -> Option<&String> {
        self.gen().start.as_ref()
    }

    pub fn snippets(&self) -> &HashMap<String, Vec<String>> {
        &self.gen().snippets
    }

    pub fn local_modules(&self) -> &HashMap<String, String> {
        &self.gen().local_modules
    }

    pub fn npm_dependencies(&self) -> &HashMap<String, (PathBuf, String)> {
        &self.gen().npm_dependencies
    }

    fn gen(&self) -> &JsGenerated {
        match &self.generated {
            Generated::InterfaceTypes => panic!("no js with interface types output"),
            Generated::Js(gen) => &gen,
        }
    }

    pub fn wasm(&self) -> &walrus::Module {
        &self.module
    }

    pub fn wasm_mut(&mut self) -> &mut walrus::Module {
        &mut self.module
    }

    pub fn emit(&mut self, out_dir: impl AsRef<Path>) -> Result<(), Error> {
        self._emit(out_dir.as_ref())
    }

    fn _emit(&mut self, out_dir: &Path) -> Result<(), Error> {
        let wasm_name = match &self.generated {
            Generated::InterfaceTypes => self.stem.clone(),
            Generated::Js(_) => format!("{}_bg", self.stem),
        };
        let wasm_path = out_dir.join(&wasm_name).with_extension("wasm");
        fs::create_dir_all(out_dir)?;
        let wasm_bytes = self.module.emit_wasm();
        fs::write(&wasm_path, wasm_bytes)
            .with_context(|| format!("failed to write `{}`", wasm_path.display()))?;

        let gen = match &self.generated {
            Generated::InterfaceTypes => return Ok(()),
            Generated::Js(gen) => gen,
        };

        // Write out all local JS snippets to the final destination now that
        // we've collected them from all the programs.
        for (identifier, list) in gen.snippets.iter() {
            for (i, js) in list.iter().enumerate() {
                let name = format!("inline{}.js", i);
                let path = out_dir.join("snippets").join(identifier).join(name);
                fs::create_dir_all(path.parent().unwrap())?;
                fs::write(&path, js)
                    .with_context(|| format!("failed to write `{}`", path.display()))?;
            }
        }

        for (path, contents) in gen.local_modules.iter() {
            let path = out_dir.join("snippets").join(path);
            fs::create_dir_all(path.parent().unwrap())?;
            fs::write(&path, contents)
                .with_context(|| format!("failed to write `{}`", path.display()))?;
        }

        if gen.npm_dependencies.len() > 0 {
            let map = gen
                .npm_dependencies
                .iter()
                .map(|(k, v)| (k, &v.1))
                .collect::<BTreeMap<_, _>>();
            let json = serde_json::to_string_pretty(&map)?;
            fs::write(out_dir.join("package.json"), json)?;
        }

        // And now that we've got all our JS and TypeScript, actually write it
        // out to the filesystem.
        let extension = if gen.mode.nodejs_experimental_modules() {
            "mjs"
        } else {
            "js"
        };

        fn write<P, C>(path: P, contents: C) -> Result<(), anyhow::Error>
        where
            P: AsRef<Path>,
            C: AsRef<[u8]>,
        {
            fs::write(&path, contents)
                .with_context(|| format!("failed to write `{}`", path.as_ref().display()))
        }

        let js_path = out_dir.join(&self.stem).with_extension(extension);

        if gen.mode.esm_integration() {
            let js_name = format!("{}_bg.{}", self.stem, extension);

            let start = gen.start.as_deref().unwrap_or("");

            write(
                &js_path,
                format!(
                    "import * as wasm from \"./{}.wasm\";\nexport * from \"./{}\";{}",
                    wasm_name, js_name, start
                ),
            )?;

            write(&out_dir.join(&js_name), reset_indentation(&gen.js))?;
        } else {
            write(&js_path, reset_indentation(&gen.js))?;
        }

        if gen.typescript {
            let ts_path = js_path.with_extension("d.ts");
            fs::write(&ts_path, &gen.ts)
                .with_context(|| format!("failed to write `{}`", ts_path.display()))?;
        }

        if gen.typescript {
            let ts_path = wasm_path.with_extension("wasm.d.ts");
            let ts = wasm2es6js::typescript(&self.module)?;
            fs::write(&ts_path, ts)
                .with_context(|| format!("failed to write `{}`", ts_path.display()))?;
        }

        Ok(())
    }
}

fn gc_module_and_adapters(module: &mut Module) {
    loop {
        // Fist up, cleanup the native wasm module. Note that roots can come
        // from custom sections, namely our wasm interface types custom section
        // as well as the aux section.
        walrus::passes::gc::run(module);

        // ... and afterwards we can delete any `implements` directives for any
        // imports that have been deleted.
        let imports_remaining = module
            .imports
            .iter()
            .map(|i| i.id())
            .collect::<HashSet<_>>();
        let mut section = module
            .customs
            .delete_typed::<wit::NonstandardWitSection>()
            .unwrap();
        section
            .implements
            .retain(|pair| imports_remaining.contains(&pair.0));

        // ... and after we delete the `implements` directive we try to
        // delete some adapters themselves. If nothing is deleted, then we're
        // good to go. If something is deleted though then we may have free'd up
        // some functions in the main module to get deleted, so go again to gc
        // things.
        let aux = module.customs.get_typed::<wit::WasmBindgenAux>().unwrap();
        let any_removed = section.gc(aux);
        module.customs.add(*section);
        if !any_removed {
            break;
        }
    }
}

/// Returns a sorted iterator over a hash map, sorted based on key.
///
/// The intention of this API is to be used whenever the iteration order of a
/// `HashMap` might affect the generated JS bindings. We want to ensure that the
/// generated output is deterministic and we do so by ensuring that iteration of
/// hash maps is consistently sorted.
fn sorted_iter<K, V>(map: &HashMap<K, V>) -> impl Iterator<Item = (&K, &V)>
where
    K: Ord,
{
    let mut pairs = map.iter().collect::<Vec<_>>();
    pairs.sort_by_key(|(k, _)| *k);
    pairs.into_iter()
}
