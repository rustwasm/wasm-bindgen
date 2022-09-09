use crate::descriptor::VectorKind;
use crate::intrinsic::Intrinsic;
use crate::wit::{
    Adapter, AdapterId, AdapterJsImportKind, AuxExportedMethodKind, AuxReceiverKind, AuxValue,
};
use crate::wit::{AdapterKind, Instruction, InstructionData};
use crate::wit::{AuxEnum, AuxExport, AuxExportKind, AuxImport, AuxStruct};
use crate::wit::{JsImport, JsImportName, NonstandardWitSection, WasmBindgenAux};
use crate::{reset_indentation, Bindgen, EncodeInto, OutputMode, PLACEHOLDER_MODULE};
use anyhow::{anyhow, bail, Context as _, Error};
use std::borrow::Cow;
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::fmt;
use std::fmt::Write;
use std::fs;
use std::path::{Path, PathBuf};
use walrus::{FunctionId, ImportId, MemoryId, Module, TableId, ValType};

mod binding;

pub struct Context<'a> {
    globals: String,
    imports_post: String,
    typescript: String,
    exposed_globals: Option<HashSet<Cow<'static, str>>>,
    next_export_idx: usize,
    config: &'a Bindgen,
    pub module: &'a mut Module,
    aux: &'a WasmBindgenAux,
    wit: &'a NonstandardWitSection,

    /// A map representing the `import` statements we'll be generating in the JS
    /// glue. The key is the module we're importing from and the value is the
    /// list of identifier we're importing from the module, with optional
    /// renames for each identifier.
    js_imports: HashMap<String, Vec<(String, Option<String>)>>,

    /// A map of each wasm import and what JS to hook up to it.
    wasm_import_definitions: HashMap<ImportId, String>,

    /// A map from an import to the name we've locally imported it as.
    imported_names: HashMap<JsImportName, String>,

    /// A set of all defined identifiers through either exports or imports to
    /// the number of times they've been used, used to generate new
    /// identifiers.
    defined_identifiers: HashMap<String, usize>,

    exported_classes: Option<BTreeMap<String, ExportedClass>>,

    /// A map of the name of npm dependencies we've loaded so far to the path
    /// they're defined in as well as their version specification.
    pub npm_dependencies: HashMap<String, (PathBuf, String)>,

    /// A mapping from the memory IDs as we see them to an index for that memory,
    /// used in function names, as well as all the kinds of views we've created
    /// of that memory.
    ///
    /// `BTreeMap` and `BTreeSet` are used to make the ordering deterministic.
    memories: BTreeMap<MemoryId, (usize, BTreeSet<&'static str>)>,
    table_indices: HashMap<TableId, usize>,

    /// A flag to track if the stack pointer setter shim has been injected.
    stack_pointer_shim_injected: bool,
}

#[derive(Default)]
pub struct ExportedClass {
    comments: String,
    contents: String,
    /// The TypeScript for the class's methods.
    typescript: String,
    /// Whether TypeScript for this class should be emitted (i.e., `skip_typescript` wasn't specified).
    generate_typescript: bool,
    has_constructor: bool,
    wrap_needed: bool,
    /// Whether to generate helper methods for inspecting the class
    is_inspectable: bool,
    /// All readable properties of the class
    readable_properties: Vec<String>,
    /// Map from field name to type as a string, docs plus whether it has a setter,
    /// whether it's optional and whether it's static.
    typescript_fields: HashMap<String, (String, String, bool, bool, bool)>,
}

const INITIAL_HEAP_VALUES: &[&str] = &["undefined", "null", "true", "false"];
// Must be kept in sync with `src/lib.rs` of the `wasm-bindgen` crate
const INITIAL_HEAP_OFFSET: usize = 32;

impl<'a> Context<'a> {
    pub fn new(
        module: &'a mut Module,
        config: &'a Bindgen,
        wit: &'a NonstandardWitSection,
        aux: &'a WasmBindgenAux,
    ) -> Result<Context<'a>, Error> {
        Ok(Context {
            globals: String::new(),
            imports_post: String::new(),
            typescript: "/* tslint:disable */\n/* eslint-disable */\n".to_string(),
            exposed_globals: Some(Default::default()),
            imported_names: Default::default(),
            js_imports: Default::default(),
            defined_identifiers: Default::default(),
            wasm_import_definitions: Default::default(),
            exported_classes: Some(Default::default()),
            config,
            module,
            npm_dependencies: Default::default(),
            next_export_idx: 0,
            wit,
            aux,
            memories: Default::default(),
            table_indices: Default::default(),
            stack_pointer_shim_injected: false,
        })
    }

    fn should_write_global(&mut self, name: impl Into<Cow<'static, str>>) -> bool {
        self.exposed_globals.as_mut().unwrap().insert(name.into())
    }

    fn export(
        &mut self,
        export_name: &str,
        contents: &str,
        comments: Option<&str>,
    ) -> Result<(), Error> {
        let definition_name = self.generate_identifier(export_name);
        if contents.starts_with("class") && definition_name != export_name {
            bail!("cannot shadow already defined class `{}`", export_name);
        }

        let contents = contents.trim();
        if let Some(c) = comments {
            self.globals.push_str(c);
        }
        let global = match self.config.mode {
            OutputMode::Node {
                experimental_modules: false,
            } => {
                if contents.starts_with("class") {
                    format!("{}\nmodule.exports.{1} = {1};\n", contents, export_name)
                } else {
                    format!("module.exports.{} = {};\n", export_name, contents)
                }
            }
            OutputMode::NoModules { .. } => {
                if contents.starts_with("class") {
                    format!("{}\n__exports.{1} = {1};\n", contents, export_name)
                } else {
                    format!("__exports.{} = {};\n", export_name, contents)
                }
            }
            OutputMode::Bundler { .. }
            | OutputMode::Node {
                experimental_modules: true,
            }
            | OutputMode::Web
            | OutputMode::Deno => {
                if contents.starts_with("function") {
                    let body = &contents[8..];
                    if export_name == definition_name {
                        format!("export function {}{}\n", export_name, body)
                    } else {
                        format!(
                            "function {}{}\nexport {{ {} as {} }};\n",
                            definition_name, body, definition_name, export_name,
                        )
                    }
                } else if contents.starts_with("class") {
                    assert_eq!(export_name, definition_name);
                    format!("export {}\n", contents)
                } else {
                    assert_eq!(export_name, definition_name);
                    format!("export const {} = {};\n", export_name, contents)
                }
            }
        };
        self.global(&global);
        Ok(())
    }

    pub fn finalize(
        &mut self,
        module_name: &str,
    ) -> Result<(String, String, Option<String>), Error> {
        // Finalize all bindings for JS classes. This is where we'll generate JS
        // glue for all classes as well as finish up a few final imports like
        // `__wrap` and such.
        self.write_classes()?;

        // Initialization is just flat out tricky and not something we
        // understand super well. To try to handle various issues that have come
        // up we always remove the `start` function if one is present. The JS
        // bindings glue then manually calls the start function (if it was
        // previously present).
        let needs_manual_start = self.unstart_start_function();

        // Cause any future calls to `should_write_global` to panic, making sure
        // we don't ask for items which we can no longer emit.
        drop(self.exposed_globals.take().unwrap());

        self.finalize_js(module_name, needs_manual_start)
    }

    fn generate_node_imports(&self) -> String {
        let mut imports = BTreeSet::new();
        for import in self.module.imports.iter() {
            imports.insert(&import.module);
        }

        let mut shim = String::new();

        shim.push_str("let imports = {};\n");

        if self.config.mode.nodejs_experimental_modules() {
            for (i, module) in imports.iter().enumerate() {
                if module.as_str() != PLACEHOLDER_MODULE {
                    shim.push_str(&format!("import * as import{} from '{}';\n", i, module));
                }
            }
        }

        for (i, module) in imports.iter().enumerate() {
            if module.as_str() == PLACEHOLDER_MODULE {
                shim.push_str(&format!(
                    "imports['{0}'] = module.exports;\n",
                    PLACEHOLDER_MODULE
                ));
            } else {
                if self.config.mode.nodejs_experimental_modules() {
                    shim.push_str(&format!("imports['{}'] = import{};\n", module, i));
                } else {
                    shim.push_str(&format!("imports['{0}'] = require('{0}');\n", module));
                }
            }
        }

        reset_indentation(&shim)
    }

    fn generate_node_wasm_loading(&self, path: &Path) -> String {
        let mut shim = String::new();

        if self.config.mode.nodejs_experimental_modules() {
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

        shim.push_str(
            "
            const wasmModule = new WebAssembly.Module(bytes);
            const wasmInstance = new WebAssembly.Instance(wasmModule, imports);
            wasm = wasmInstance.exports;
            module.exports.__wasm = wasm;
        ",
        );

        reset_indentation(&shim)
    }

    // generates something like
    // ```js
    // import * as import0 from './snippets/.../inline1.js';
    // ```,
    //
    // ```js
    // const imports = {
    //   __wbindgen_placeholder__: {
    //     __wbindgen_throw: function(..) { .. },
    //     ..
    //   },
    //   './snippets/deno-65e2634a84cc3c14/inline1.js': import0,
    // }
    // ```
    fn generate_deno_imports(&self) -> (String, String) {
        let mut imports = String::new();
        let mut wasm_import_object = "const imports = {\n".to_string();

        wasm_import_object.push_str(&format!("  {}: {{\n", crate::PLACEHOLDER_MODULE));

        for (id, js) in crate::sorted_iter(&self.wasm_import_definitions) {
            let import = self.module.imports.get(*id);
            wasm_import_object.push_str(&format!("{}: {},\n", &import.name, js.trim()));
        }

        wasm_import_object.push_str("\t},\n");

        // e.g. snippets without parameters
        let import_modules = self
            .module
            .imports
            .iter()
            .map(|import| &import.module)
            .filter(|module| module.as_str() != PLACEHOLDER_MODULE);
        for (i, module) in import_modules.enumerate() {
            imports.push_str(&format!("import * as import{} from '{}'\n", i, module));
            wasm_import_object.push_str(&format!("  '{}': import{},", module, i))
        }

        wasm_import_object.push_str("\n};\n\n");

        (imports, wasm_import_object)
    }

    fn generate_deno_wasm_loading(&self, module_name: &str) -> String {
        // Deno removed support for .wasm imports in https://github.com/denoland/deno/pull/5135
        // the issue for bringing it back is https://github.com/denoland/deno/issues/5609.
        format!(
            "const wasm_url = new URL('{module_name}_bg.wasm', import.meta.url);
            let wasmCode = '';
            switch (wasm_url.protocol) {{
                case 'file:':
                    wasmCode = await Deno.readFile(wasm_url);
                    break
                case 'https:':
                case 'http:':
                    wasmCode = await (await fetch(wasm_url)).arrayBuffer();
                    break
                default:
                    throw new Error(`Unsupported protocol: ${{wasm_url.protocol}}`);
            }}

            const wasmInstance = (await WebAssembly.instantiate(wasmCode, imports)).instance;
            const wasm = wasmInstance.exports;",
            module_name = module_name
        )
    }

    /// Performs the task of actually generating the final JS module, be it
    /// `--target no-modules`, `--target web`, or for bundlers. This is the very
    /// last step performed in `finalize`.
    fn finalize_js(
        &mut self,
        module_name: &str,
        needs_manual_start: bool,
    ) -> Result<(String, String, Option<String>), Error> {
        let mut ts;
        let mut js = String::new();
        let mut start = None;

        if let OutputMode::NoModules { global } = &self.config.mode {
            js.push_str(&format!("let {};\n(function() {{\n", global));
        }

        // Depending on the output mode, generate necessary glue to actually
        // import the wasm file in one way or another.
        let mut init = (String::new(), String::new());
        let mut footer = String::new();
        let mut imports = self.js_import_header()?;
        match &self.config.mode {
            // In `--target no-modules` mode we need to both expose a name on
            // the global object as well as generate our own custom start
            // function.
            OutputMode::NoModules { global } => {
                js.push_str("const __exports = {};\n");
                js.push_str("let wasm;\n");
                init = self.gen_init(needs_manual_start, None)?;
                footer.push_str(&format!(
                    "{} = Object.assign(init, {{ initSync }}, __exports);\n",
                    global
                ));
            }

            // With normal CommonJS node we need to defer requiring the wasm
            // until the end so most of our own exports are hooked up
            OutputMode::Node {
                experimental_modules: false,
            } => {
                js.push_str(&self.generate_node_imports());

                js.push_str("let wasm;\n");

                for (id, js) in crate::sorted_iter(&self.wasm_import_definitions) {
                    let import = self.module.imports.get_mut(*id);
                    footer.push_str("\nmodule.exports.");
                    footer.push_str(&import.name);
                    footer.push_str(" = ");
                    footer.push_str(js.trim());
                    footer.push_str(";\n");
                }

                footer.push_str(
                    &self.generate_node_wasm_loading(&Path::new(&format!(
                        "./{}_bg.wasm",
                        module_name
                    ))),
                );

                if needs_manual_start {
                    footer.push_str("\nwasm.__wbindgen_start();\n");
                }
            }

            OutputMode::Deno => {
                let (js_imports, wasm_import_object) = self.generate_deno_imports();
                imports.push_str(&js_imports);
                footer.push_str(&wasm_import_object);

                footer.push_str(&self.generate_deno_wasm_loading(module_name));

                footer.push_str("\n\n");

                if needs_manual_start {
                    footer.push_str("\nwasm.__wbindgen_start();\n");
                }
            }

            // With Bundlers and modern ES6 support in Node we can simply import
            // the wasm file as if it were an ES module and let the
            // bundler/runtime take care of it.
            OutputMode::Bundler { .. }
            | OutputMode::Node {
                experimental_modules: true,
            } => {
                imports.push_str(&format!(
                    "import * as wasm from './{}_bg.wasm';\n",
                    module_name
                ));
                for (id, js) in crate::sorted_iter(&self.wasm_import_definitions) {
                    let import = self.module.imports.get_mut(*id);
                    import.module = format!("./{}_bg.js", module_name);
                    if js.starts_with("function") {
                        let body = &js[8..];
                        footer.push_str("\nexport function ");
                        footer.push_str(&import.name);
                        footer.push_str(body.trim());
                        footer.push_str(";\n");
                    } else {
                        footer.push_str("\nexport const ");
                        footer.push_str(&import.name);
                        footer.push_str(" = ");
                        footer.push_str(js.trim());
                        footer.push_str(";\n");
                    }
                }

                if needs_manual_start {
                    start = Some("\nwasm.__wbindgen_start();\n".to_string());
                }
            }

            // With a browser-native output we're generating an ES module, but
            // browsers don't support natively importing wasm right now so we
            // expose the same initialization function as `--target no-modules`
            // as the default export of the module.
            OutputMode::Web => {
                self.imports_post.push_str("let wasm;\n");
                init = self.gen_init(needs_manual_start, Some(&mut imports))?;
                footer.push_str("export { initSync }\n");
                footer.push_str("export default init;");
            }
        }

        // Before putting the static init code declaration info, put all existing typescript into a `wasm_bindgen` namespace declaration.
        // Not sure if this should happen in all cases, so just adding it to NoModules for now...
        if self.config.mode.no_modules() {
            ts = String::from("declare namespace wasm_bindgen {\n\t");
            ts.push_str(&self.typescript.replace("\n", "\n\t"));
            ts.push_str("\n}\n");
        } else {
            ts = self.typescript.clone();
        }

        let (init_js, init_ts) = init;

        ts.push_str(&init_ts);

        // Emit all the JS for importing all our functionality
        assert!(
            !self.config.mode.uses_es_modules() || js.is_empty(),
            "ES modules require imports to be at the start of the file"
        );
        js.push_str(&imports);
        js.push_str("\n");
        js.push_str(&self.imports_post);
        js.push_str("\n");

        // Emit all our exports from this module
        js.push_str(&self.globals);
        js.push_str("\n");

        // Generate the initialization glue, if there was any
        js.push_str(&init_js);
        js.push_str("\n");
        js.push_str(&footer);
        js.push_str("\n");
        if self.config.mode.no_modules() {
            js.push_str("})();\n");
        }

        while js.contains("\n\n\n") {
            js = js.replace("\n\n\n", "\n\n");
        }

        Ok((js, ts, start))
    }

    fn js_import_header(&self) -> Result<String, Error> {
        let mut imports = String::new();

        if self.config.omit_imports {
            return Ok(imports);
        }

        match &self.config.mode {
            OutputMode::NoModules { .. } => {
                for (module, _items) in self.js_imports.iter() {
                    bail!(
                        "importing from `{}` isn't supported with `--target no-modules`",
                        module
                    );
                }
            }

            OutputMode::Node {
                experimental_modules: false,
            } => {
                for (module, items) in crate::sorted_iter(&self.js_imports) {
                    imports.push_str("const { ");
                    for (i, (item, rename)) in items.iter().enumerate() {
                        if i > 0 {
                            imports.push_str(", ");
                        }
                        imports.push_str(item);
                        if let Some(other) = rename {
                            imports.push_str(": ");
                            imports.push_str(other)
                        }
                    }
                    if module.starts_with(".") || PathBuf::from(module).is_absolute() {
                        imports.push_str(" } = require(String.raw`");
                    } else {
                        imports.push_str(" } = require(`");
                    }
                    imports.push_str(module);
                    imports.push_str("`);\n");
                }
            }

            OutputMode::Bundler { .. }
            | OutputMode::Node {
                experimental_modules: true,
            }
            | OutputMode::Web
            | OutputMode::Deno => {
                for (module, items) in crate::sorted_iter(&self.js_imports) {
                    imports.push_str("import { ");
                    for (i, (item, rename)) in items.iter().enumerate() {
                        if i > 0 {
                            imports.push_str(", ");
                        }
                        imports.push_str(item);
                        if let Some(other) = rename {
                            imports.push_str(" as ");
                            imports.push_str(other)
                        }
                    }
                    imports.push_str(" } from '");
                    imports.push_str(module);
                    imports.push_str("';\n");
                }
            }
        }
        Ok(imports)
    }

    fn ts_for_init_fn(
        &self,
        has_memory: bool,
        has_module_or_path_optional: bool,
    ) -> Result<String, Error> {
        let output = crate::wasm2es6js::interface(&self.module)?;

        let (memory_doc, memory_param) = if has_memory {
            (
                "* @param {WebAssembly.Memory} maybe_memory\n",
                ", maybe_memory?: WebAssembly.Memory",
            )
        } else {
            ("", "")
        };
        let arg_optional = if has_module_or_path_optional { "?" } else { "" };
        // With TypeScript 3.8.3, I'm seeing that any "export"s at the root level cause TypeScript to ignore all "declare" statements.
        // So using "declare" everywhere for at least the NoModules option.
        // Also in (at least) the NoModules, the `init()` method is renamed to `wasm_bindgen()`.
        let setup_function_declaration;
        let mut sync_init_function = String::new();
        let declare_or_export;
        if self.config.mode.no_modules() {
            declare_or_export = "declare";
            setup_function_declaration = "declare function wasm_bindgen";
        } else {
            declare_or_export = "export";

            sync_init_function.push_str(&format!(
                "\
                {declare_or_export} type SyncInitInput = BufferSource | WebAssembly.Module;\n\
                /**\n\
                * Instantiates the given `module`, which can either be bytes or\n\
                * a precompiled `WebAssembly.Module`.\n\
                *\n\
                * @param {{SyncInitInput}} module\n\
                {memory_doc}\
                *\n\
                * @returns {{InitOutput}}\n\
                */\n\
                export function initSync(module: SyncInitInput{memory_param}): InitOutput;\n\n\
                ",
                memory_doc = memory_doc,
                memory_param = memory_param
            ));

            setup_function_declaration = "export default function init";
        }
        Ok(format!(
            "\n\
            {declare_or_export} type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;\n\
            \n\
            {declare_or_export} interface InitOutput {{\n\
            {output}}}\n\
            \n\
            {sync_init_function}\
            /**\n\
            * If `module_or_path` is {{RequestInfo}} or {{URL}}, makes a request and\n\
            * for everything else, calls `WebAssembly.instantiate` directly.\n\
            *\n\
            * @param {{InitInput | Promise<InitInput>}} module_or_path\n\
            {}\
            *\n\
            * @returns {{Promise<InitOutput>}}\n\
            */\n\
            {setup_function_declaration} \
                (module_or_path{}: InitInput | Promise<InitInput>{}): Promise<InitOutput>;\n",
            memory_doc, arg_optional, memory_param,
            output = output,
            sync_init_function = sync_init_function,
            declare_or_export = declare_or_export,
            setup_function_declaration = setup_function_declaration,
        ))
    }

    fn gen_init(
        &mut self,
        needs_manual_start: bool,
        mut imports: Option<&mut String>,
    ) -> Result<(String, String), Error> {
        let module_name = "wbg";
        let mut init_memory_arg = "";
        let mut init_memory = String::new();
        let mut has_memory = false;
        if let Some(mem) = self.module.memories.iter().next() {
            if let Some(id) = mem.import {
                self.module.imports.get_mut(id).module = module_name.to_string();
                init_memory = format!(
                    "imports.{}.memory = maybe_memory || new WebAssembly.Memory({{",
                    module_name
                );
                init_memory.push_str(&format!("initial:{}", mem.initial));
                if let Some(max) = mem.maximum {
                    init_memory.push_str(&format!(",maximum:{}", max));
                }
                if mem.shared {
                    init_memory.push_str(",shared:true");
                }
                init_memory.push_str("});");
                init_memory_arg = ", maybe_memory";
                has_memory = true;
            }
        }

        let default_module_path = if !self.config.omit_default_module_path {
            match self.config.mode {
                OutputMode::Web => format!(
                    "\
                    if (typeof input === 'undefined') {{
                        input = new URL('{stem}_bg.wasm', import.meta.url);
                    }}",
                    stem = self.config.stem()?
                ),
                OutputMode::NoModules { .. } => "\
                    if (typeof input === 'undefined') {
                        let src;
                        if (typeof document === 'undefined') {
                            src = location.href;
                        } else {
                            src = document.currentScript.src;
                        }
                        input = src.replace(/\\.js$/, '_bg.wasm');
                    }"
                .to_string(),
                _ => "".to_string(),
            }
        } else {
            String::from("")
        };

        let ts = self.ts_for_init_fn(
            has_memory,
            !self.config.omit_default_module_path && !default_module_path.is_empty(),
        )?;

        // Initialize the `imports` object for all import definitions that we're
        // directed to wire up.
        let mut imports_init = String::new();

        imports_init.push_str("imports.");
        imports_init.push_str(module_name);
        imports_init.push_str(" = {};\n");

        for (id, js) in crate::sorted_iter(&self.wasm_import_definitions) {
            let import = self.module.imports.get_mut(*id);
            import.module = module_name.to_string();
            imports_init.push_str("imports.");
            imports_init.push_str(module_name);
            imports_init.push_str(".");
            imports_init.push_str(&import.name);
            imports_init.push_str(" = ");
            imports_init.push_str(js.trim());
            imports_init.push_str(";\n");
        }

        let extra_modules = self
            .module
            .imports
            .iter()
            .filter(|i| !self.wasm_import_definitions.contains_key(&i.id()))
            .filter(|i| {
                // Importing memory is handled specially in this area, so don't
                // consider this a candidate for importing from extra modules.
                match i.kind {
                    walrus::ImportKind::Memory(_) => false,
                    _ => true,
                }
            })
            .map(|i| &i.module)
            .collect::<BTreeSet<_>>();
        for (i, extra) in extra_modules.iter().enumerate() {
            let imports = match &mut imports {
                Some(list) => list,
                None => bail!(
                    "cannot import from modules (`{}`) with `--no-modules`",
                    extra
                ),
            };
            imports.push_str(&format!("import * as __wbg_star{} from '{}';\n", i, extra));
            imports_init.push_str(&format!("imports['{}'] = __wbg_star{};\n", extra, i));
        }

        let mut init_memviews = String::new();
        for &(num, ref views) in self.memories.values() {
            for kind in views {
                writeln!(
                    init_memviews,
                    // Reset the memory views to empty in case `init` gets called multiple times.
                    // Without this, the `length = 0` check would never detect that the view was
                    // outdated.
                    "cached{kind}Memory{num} = new {kind}Array();",
                    kind = kind,
                    num = num,
                )
                .unwrap()
            }
        }

        let js = format!(
            "\
                async function load(module, imports) {{
                    if (typeof Response === 'function' && module instanceof Response) {{
                        if (typeof WebAssembly.instantiateStreaming === 'function') {{
                            try {{
                                return await WebAssembly.instantiateStreaming(module, imports);

                            }} catch (e) {{
                                if (module.headers.get('Content-Type') != 'application/wasm') {{
                                    console.warn(\"`WebAssembly.instantiateStreaming` failed \
                                                    because your server does not serve wasm with \
                                                    `application/wasm` MIME type. Falling back to \
                                                    `WebAssembly.instantiate` which is slower. Original \
                                                    error:\\n\", e);

                                }} else {{
                                    throw e;
                                }}
                            }}
                        }}

                        const bytes = await module.arrayBuffer();
                        return await WebAssembly.instantiate(bytes, imports);

                    }} else {{
                        const instance = await WebAssembly.instantiate(module, imports);

                        if (instance instanceof WebAssembly.Instance) {{
                            return {{ instance, module }};

                        }} else {{
                            return instance;
                        }}
                    }}
                }}

                function getImports() {{
                    const imports = {{}};
                    {imports_init}
                    return imports;
                }}

                function initMemory(imports, maybe_memory) {{
                    {init_memory}
                }}

                function finalizeInit(instance, module) {{
                    wasm = instance.exports;
                    init.__wbindgen_wasm_module = module;
                    {init_memviews}
                    {start}
                    return wasm;
                }}

                function initSync(module{init_memory_arg}) {{
                    const imports = getImports();

                    initMemory(imports{init_memory_arg});

                    if (!(module instanceof WebAssembly.Module)) {{
                        module = new WebAssembly.Module(module);
                    }}

                    const instance = new WebAssembly.Instance(module, imports);

                    return finalizeInit(instance, module);
                }}

                async function init(input{init_memory_arg}) {{
                    {default_module_path}
                    const imports = getImports();

                    if (typeof input === 'string' || (typeof Request === 'function' && input instanceof Request) || (typeof URL === 'function' && input instanceof URL)) {{
                        input = fetch(input);
                    }}

                    initMemory(imports{init_memory_arg});

                    const {{ instance, module }} = await load(await input, imports);

                    return finalizeInit(instance, module);
                }}
            ",
            init_memory_arg = init_memory_arg,
            default_module_path = default_module_path,
            init_memory = init_memory,
            init_memviews = init_memviews,
            start = if needs_manual_start {
                "wasm.__wbindgen_start();"
            } else {
                ""
            },
            imports_init = imports_init,
        );

        Ok((js, ts))
    }

    fn write_classes(&mut self) -> Result<(), Error> {
        for (class, exports) in self.exported_classes.take().unwrap() {
            self.write_class(&class, &exports)?;
        }
        Ok(())
    }

    fn write_class(&mut self, name: &str, class: &ExportedClass) -> Result<(), Error> {
        let mut dst = format!("class {} {{\n", name);
        let mut ts_dst = format!("export {}", dst);

        if self.config.debug && !class.has_constructor {
            dst.push_str(
                "
                    constructor() {
                        throw new Error('cannot invoke `new` directly');
                    }
                ",
            );
        }

        if class.wrap_needed {
            dst.push_str(&format!(
                "
                static __wrap(ptr) {{
                    const obj = Object.create({}.prototype);
                    obj.ptr = ptr;
                    {}
                    return obj;
                }}
                ",
                name,
                if self.config.weak_refs {
                    format!("{}Finalization.register(obj, obj.ptr, obj);", name)
                } else {
                    String::new()
                },
            ));
        }

        if self.config.weak_refs {
            self.global(&format!(
                "const {}Finalization = new FinalizationRegistry(ptr => wasm.{}(ptr));",
                name,
                wasm_bindgen_shared::free_function(&name),
            ));
        }

        // If the class is inspectable, generate `toJSON` and `toString`
        // to expose all readable properties of the class. Otherwise,
        // the class shows only the "ptr" property when logged or serialized
        if class.is_inspectable {
            // Creates a `toJSON` method which returns an object of all readable properties
            // This object looks like { a: this.a, b: this.b }
            dst.push_str(&format!(
                "
                toJSON() {{
                    return {{{}}};
                }}

                toString() {{
                    return JSON.stringify(this);
                }}
                ",
                class
                    .readable_properties
                    .iter()
                    .fold(String::from("\n"), |fields, field_name| {
                        format!("{}{name}: this.{name},\n", fields, name = field_name)
                    })
            ));
            // Also add definitions to the .d.ts file.
            ts_dst.push_str(
                "\
            /**\n*\
            * Return copy of self without private attributes.\n\
            */\n  toJSON(): Object;\n\
            /**\n\
            * Return stringified version of self.\n\
            */\n  toString(): string;\n",
            );

            if self.config.mode.nodejs() {
                // `util.inspect` must be imported in Node.js to define [inspect.custom]
                let module_name = self.import_name(&JsImport {
                    name: JsImportName::Module {
                        module: "util".to_string(),
                        name: "inspect".to_string(),
                    },
                    fields: Vec::new(),
                })?;

                // Node.js supports a custom inspect function to control the
                // output of `console.log` and friends. The constructor is set
                // to display the class name as a typical JavaScript class would
                dst.push_str(&format!(
                    "
                    [{}.custom]() {{
                        return Object.assign(Object.create({{constructor: this.constructor}}), this.toJSON());
                    }}
                    ",
                    module_name
                ));
            }
        }

        dst.push_str(&format!(
            "
            __destroy_into_raw() {{
                const ptr = this.ptr;
                this.ptr = 0;
                {}
                return ptr;
            }}

            free() {{
                const ptr = this.__destroy_into_raw();
                wasm.{}(ptr);
            }}
            ",
            if self.config.weak_refs {
                format!("{}Finalization.unregister(this);", name)
            } else {
                String::new()
            },
            wasm_bindgen_shared::free_function(&name),
        ));
        ts_dst.push_str("  free(): void;\n");
        dst.push_str(&class.contents);
        ts_dst.push_str(&class.typescript);

        let mut fields = class.typescript_fields.keys().collect::<Vec<_>>();
        fields.sort(); // make sure we have deterministic output
        for name in fields {
            let (ty, docs, has_setter, is_optional, is_static) = &class.typescript_fields[name];
            ts_dst.push_str(docs);
            ts_dst.push_str("  ");
            if *is_static {
                ts_dst.push_str("static ");
            }
            if !has_setter {
                ts_dst.push_str("readonly ");
            }
            ts_dst.push_str(name);
            if *is_optional {
                ts_dst.push_str("?: ");
            } else {
                ts_dst.push_str(": ");
            }
            ts_dst.push_str(&ty);
            ts_dst.push_str(";\n");
        }
        dst.push_str("}\n");
        ts_dst.push_str("}\n");

        self.export(&name, &dst, Some(&class.comments))?;

        if class.generate_typescript {
            self.typescript.push_str(&class.comments);
            self.typescript.push_str(&ts_dst);
        }

        Ok(())
    }

    fn expose_drop_ref(&mut self) {
        if !self.should_write_global("drop_ref") {
            return;
        }
        self.expose_global_heap();
        self.expose_global_heap_next();

        // Note that here we check if `idx` shouldn't actually be dropped. This
        // is due to the fact that `JsValue::null()` and friends can be passed
        // by value to JS where we'll automatically call this method. Those
        // constants, however, cannot be dropped. See #1054 for removing this
        // branch.
        //
        // Otherwise the free operation here is pretty simple, just appending to
        // the linked list of heap slots that are free.
        self.global(&format!(
            "
            function dropObject(idx) {{
                if (idx < {}) return;
                heap[idx] = heap_next;
                heap_next = idx;
            }}
            ",
            INITIAL_HEAP_OFFSET + INITIAL_HEAP_VALUES.len(),
        ));
    }

    fn expose_global_heap(&mut self) {
        if !self.should_write_global("heap") {
            return;
        }
        assert!(!self.config.externref);
        self.global(&format!(
            "const heap = new Array({}).fill(undefined);",
            INITIAL_HEAP_OFFSET
        ));
        self.global(&format!("heap.push({});", INITIAL_HEAP_VALUES.join(", ")));
    }

    fn expose_global_heap_next(&mut self) {
        if !self.should_write_global("heap_next") {
            return;
        }
        self.expose_global_heap();
        self.global("let heap_next = heap.length;");
    }

    fn expose_get_object(&mut self) {
        if !self.should_write_global("get_object") {
            return;
        }
        self.expose_global_heap();

        // Accessing a heap object is just a simple index operation due to how
        // the stack/heap are laid out.
        self.global("function getObject(idx) { return heap[idx]; }");
    }

    fn expose_not_defined(&mut self) {
        if !self.should_write_global("not_defined") {
            return;
        }
        self.global("function notDefined(what) { return () => { throw new Error(`${what} is not defined`); }; }");
    }

    fn expose_assert_num(&mut self) {
        if !self.should_write_global("assert_num") {
            return;
        }
        self.global(&format!(
            "
            function _assertNum(n) {{
                if (typeof(n) !== 'number') throw new Error('expected a number argument');
            }}
            "
        ));
    }

    fn expose_assert_bigint(&mut self) {
        if !self.should_write_global("assert_bigint") {
            return;
        }
        self.global(&format!(
            "
            function _assertBigInt(n) {{
                if (typeof(n) !== 'bigint') throw new Error('expected a bigint argument');
            }}
            "
        ));
    }

    fn expose_assert_bool(&mut self) {
        if !self.should_write_global("assert_bool") {
            return;
        }
        self.global(&format!(
            "
            function _assertBoolean(n) {{
                if (typeof(n) !== 'boolean') {{
                    throw new Error('expected a boolean argument');
                }}
            }}
            "
        ));
    }

    fn expose_wasm_vector_len(&mut self) {
        if !self.should_write_global("wasm_vector_len") {
            return;
        }
        self.global("let WASM_VECTOR_LEN = 0;");
    }

    fn expose_pass_string_to_wasm(&mut self, memory: MemoryId) -> Result<MemView, Error> {
        self.expose_wasm_vector_len();

        let debug = if self.config.debug {
            "
                if (typeof(arg) !== 'string') throw new Error('expected a string argument');
            "
        } else {
            ""
        };

        let mem = self.expose_uint8_memory(memory);
        let ret = MemView {
            name: "passStringToWasm".into(),
            num: mem.num,
        };
        if !self.should_write_global(ret.to_string()) {
            return Ok(ret);
        }
        self.expose_text_encoder()?;

        // The first implementation we have for this is to use
        // `TextEncoder#encode` which has been around for quite some time.
        let encode = "function (arg, view) {
            const buf = cachedTextEncoder.encode(arg);
            view.set(buf);
            return {
                read: arg.length,
                written: buf.length
            };
        }";

        // Another possibility is to use `TextEncoder#encodeInto` which is much
        // newer and isn't implemented everywhere yet. It's more efficient,
        // however, becaues it allows us to elide an intermediate allocation.
        let encode_into = "function (arg, view) {
            return cachedTextEncoder.encodeInto(arg, view);
        }";

        // Looks like `encodeInto` doesn't currently work when the memory passed
        // in is backed by a `SharedArrayBuffer`, so force usage of `encode` if
        // a `SharedArrayBuffer` is in use.
        let shared = self.module.memories.get(memory).shared;

        match self.config.encode_into {
            EncodeInto::Always if !shared => {
                self.global(&format!(
                    "
                    const encodeString = {};
                ",
                    encode_into
                ));
            }
            EncodeInto::Test if !shared => {
                self.global(&format!(
                    "
                    const encodeString = (typeof cachedTextEncoder.encodeInto === 'function'
                        ? {}
                        : {});
                ",
                    encode_into, encode
                ));
            }
            _ => {
                self.global(&format!(
                    "
                    const encodeString = {};
                ",
                    encode
                ));
            }
        }

        // A fast path that directly writes char codes into WASM memory as long
        // as it finds only ASCII characters.
        //
        // This is much faster for common ASCII strings because it can avoid
        // calling out into C++ TextEncoder code.
        //
        // This might be not very intuitive, but such calls are usually more
        // expensive in mainstream engines than staying in the JS, and
        // charCodeAt on ASCII strings is usually optimised to raw bytes.
        let encode_as_ascii = format!(
            "\
                if (realloc === undefined) {{
                    const buf = cachedTextEncoder.encode(arg);
                    const ptr = malloc(buf.length);
                    {mem}().subarray(ptr, ptr + buf.length).set(buf);
                    WASM_VECTOR_LEN = buf.length;
                    return ptr;
                }}

                let len = arg.length;
                let ptr = malloc(len);

                const mem = {mem}();

                let offset = 0;

                for (; offset < len; offset++) {{
                    const code = arg.charCodeAt(offset);
                    if (code > 0x7F) break;
                    mem[ptr + offset] = code;
                }}
            ",
            mem = mem,
        );

        // TODO:
        // When converting a JS string to UTF-8, the maximum size is `arg.length * 3`,
        // so we just allocate that. This wastes memory, so we should investigate
        // looping over the string to calculate the precise size, or perhaps using
        // `shrink_to_fit` on the Rust side.
        self.global(&format!(
            "function {name}(arg, malloc, realloc) {{
                {debug}
                {ascii}
                if (offset !== len) {{
                    if (offset !== 0) {{
                        arg = arg.slice(offset);
                    }}
                    ptr = realloc(ptr, len, len = offset + arg.length * 3);
                    const view = {mem}().subarray(ptr + offset, ptr + len);
                    const ret = encodeString(arg, view);
                    {debug_end}
                    offset += ret.written;
                }}

                WASM_VECTOR_LEN = offset;
                return ptr;
            }}",
            name = ret,
            debug = debug,
            ascii = encode_as_ascii,
            mem = mem,
            debug_end = if self.config.debug {
                "if (ret.read !== arg.length) throw new Error('failed to pass whole string');"
            } else {
                ""
            },
        ));

        Ok(ret)
    }

    fn expose_pass_array8_to_wasm(&mut self, memory: MemoryId) -> Result<MemView, Error> {
        let view = self.expose_uint8_memory(memory);
        self.pass_array_to_wasm("passArray8ToWasm", view, 1)
    }

    fn expose_pass_array16_to_wasm(&mut self, memory: MemoryId) -> Result<MemView, Error> {
        let view = self.expose_uint16_memory(memory);
        self.pass_array_to_wasm("passArray16ToWasm", view, 2)
    }

    fn expose_pass_array32_to_wasm(&mut self, memory: MemoryId) -> Result<MemView, Error> {
        let view = self.expose_uint32_memory(memory);
        self.pass_array_to_wasm("passArray32ToWasm", view, 4)
    }

    fn expose_pass_array64_to_wasm(&mut self, memory: MemoryId) -> Result<MemView, Error> {
        let view = self.expose_uint64_memory(memory);
        self.pass_array_to_wasm("passArray64ToWasm", view, 8)
    }

    fn expose_pass_array_f32_to_wasm(&mut self, memory: MemoryId) -> Result<MemView, Error> {
        let view = self.expose_f32_memory(memory);
        self.pass_array_to_wasm("passArrayF32ToWasm", view, 4)
    }

    fn expose_pass_array_f64_to_wasm(&mut self, memory: MemoryId) -> Result<MemView, Error> {
        let view = self.expose_f64_memory(memory);
        self.pass_array_to_wasm("passArrayF64ToWasm", view, 8)
    }

    fn expose_pass_array_jsvalue_to_wasm(&mut self, memory: MemoryId) -> Result<MemView, Error> {
        let mem = self.expose_uint32_memory(memory);
        let ret = MemView {
            name: "passArrayJsValueToWasm".into(),
            num: mem.num,
        };
        if !self.should_write_global(ret.to_string()) {
            return Ok(ret);
        }
        self.expose_wasm_vector_len();
        match (self.aux.externref_table, self.aux.externref_alloc) {
            (Some(table), Some(alloc)) => {
                // TODO: using `addToExternrefTable` goes back and forth between wasm
                // and JS a lot, we should have a bulk operation for this.
                let add = self.expose_add_to_externref_table(table, alloc)?;
                self.global(&format!(
                    "
                        function {}(array, malloc) {{
                            const ptr = malloc(array.length * 4);
                            const mem = {}();
                            for (let i = 0; i < array.length; i++) {{
                                mem[ptr / 4 + i] = {}(array[i]);
                            }}
                            WASM_VECTOR_LEN = array.length;
                            return ptr;
                        }}
                    ",
                    ret, mem, add,
                ));
            }
            _ => {
                self.expose_add_heap_object();
                self.global(&format!(
                    "
                        function {}(array, malloc) {{
                            const ptr = malloc(array.length * 4);
                            const mem = {}();
                            for (let i = 0; i < array.length; i++) {{
                                mem[ptr / 4 + i] = addHeapObject(array[i]);
                            }}
                            WASM_VECTOR_LEN = array.length;
                            return ptr;
                        }}
                    ",
                    ret, mem,
                ));
            }
        }
        Ok(ret)
    }

    fn pass_array_to_wasm(
        &mut self,
        name: &'static str,
        view: MemView,
        size: usize,
    ) -> Result<MemView, Error> {
        let ret = MemView {
            name: name.into(),
            num: view.num,
        };
        if !self.should_write_global(ret.to_string()) {
            return Ok(ret);
        }
        self.expose_wasm_vector_len();
        self.global(&format!(
            "
            function {}(arg, malloc) {{
                const ptr = malloc(arg.length * {size});
                {}().set(arg, ptr / {size});
                WASM_VECTOR_LEN = arg.length;
                return ptr;
            }}
            ",
            ret,
            view,
            size = size
        ));
        Ok(ret)
    }

    fn expose_text_encoder(&mut self) -> Result<(), Error> {
        if !self.should_write_global("text_encoder") {
            return Ok(());
        }
        self.expose_text_processor("TextEncoder", "('utf-8')")
    }

    fn expose_text_decoder(&mut self) -> Result<(), Error> {
        if !self.should_write_global("text_decoder") {
            return Ok(());
        }

        // `ignoreBOM` is needed so that the BOM will be preserved when sending a string from Rust to JS
        // `fatal` is needed to catch any weird encoding bugs when sending a string from Rust to JS
        self.expose_text_processor("TextDecoder", "('utf-8', { ignoreBOM: true, fatal: true })")?;

        // This is needed to workaround a bug in Safari
        // See: https://github.com/rustwasm/wasm-bindgen/issues/1825
        self.global("cachedTextDecoder.decode();");

        Ok(())
    }

    fn expose_text_processor(&mut self, s: &str, args: &str) -> Result<(), Error> {
        match &self.config.mode {
            OutputMode::Node { .. } => {
                let name = self.import_name(&JsImport {
                    name: JsImportName::Module {
                        module: "util".to_string(),
                        name: s.to_string(),
                    },
                    fields: Vec::new(),
                })?;
                self.global(&format!("let cached{} = new {}{};", s, name, args));
            }
            OutputMode::Bundler {
                browser_only: false,
            } => {
                self.global(&format!(
                    "
                    const l{0} = typeof {0} === 'undefined' ? \
                        (0, module.require)('util').{0} : {0};\
                ",
                    s
                ));
                self.global(&format!("let cached{0} = new l{0}{1};", s, args));
            }
            OutputMode::Deno
            | OutputMode::Web
            | OutputMode::NoModules { .. }
            | OutputMode::Bundler { browser_only: true } => {
                self.global(&format!("const cached{0} = new {0}{1};", s, args))
            }
        };

        Ok(())
    }

    fn expose_get_string_from_wasm(&mut self, memory: MemoryId) -> Result<MemView, Error> {
        self.expose_text_decoder()?;
        let mem = self.expose_uint8_memory(memory);
        let ret = MemView {
            name: "getStringFromWasm".into(),
            num: mem.num,
        };

        if !self.should_write_global(ret.to_string()) {
            return Ok(ret);
        }

        // Typically we try to give a raw view of memory out to `TextDecoder` to
        // avoid copying too much data. If, however, a `SharedArrayBuffer` is
        // being used it looks like that is rejected by `TextDecoder` or
        // otherwise doesn't work with it. When we detect a shared situation we
        // use `slice` which creates a new array instead of `subarray` which
        // creates just a view. That way in shared mode we copy more data but in
        // non-shared mode there's no need to copy the data except for the
        // string itself.
        let is_shared = self.module.memories.get(memory).shared;
        let method = if is_shared { "slice" } else { "subarray" };

        self.global(&format!(
            "
            function {}(ptr, len) {{
                return cachedTextDecoder.decode({}().{}(ptr, ptr + len));
            }}
            ",
            ret, mem, method
        ));
        Ok(ret)
    }

    fn expose_get_cached_string_from_wasm(
        &mut self,
        memory: MemoryId,
        table: Option<TableId>,
    ) -> Result<MemView, Error> {
        let get_object = if let Some(table) = table {
            self.expose_get_from_externref_table(table)?.to_string()
        } else {
            self.expose_get_object();
            "getObject".to_string()
        };
        let get_string = self.expose_get_string_from_wasm(memory)?;
        let ret = MemView {
            name: "getCachedStringFromWasm".into(),
            num: get_string.num,
        };

        if !self.should_write_global(ret.to_string()) {
            return Ok(ret);
        }

        // This has support for both `&str` and `Option<&str>`.
        //
        // If `ptr` is not `0` then we know that it's a `&str` or `Some(&str)`, so we just decode it.
        //
        // If `ptr` is `0` then the `len` is a pointer to the cached `JsValue`, so we return that.
        //
        // If `ptr` and `len` are both `0` then that means it's `None`, in that case we rely upon
        // the fact that `getObject(0)` is guaranteed to be `undefined`.
        self.global(&format!(
            "
            function {name}(ptr, len) {{
                if (ptr === 0) {{
                    return {get_object}(len);
                }} else {{
                    return {get_string}(ptr, len);
                }}
            }}
            ",
            name = ret,
            get_string = get_string,
            get_object = get_object
        ));
        Ok(ret)
    }

    fn expose_get_array_js_value_from_wasm(&mut self, memory: MemoryId) -> Result<MemView, Error> {
        let mem = self.expose_uint32_memory(memory);
        let ret = MemView {
            name: "getArrayJsValueFromWasm".into(),
            num: mem.num,
        };
        if !self.should_write_global(ret.to_string()) {
            return Ok(ret);
        }
        match (self.aux.externref_table, self.aux.externref_drop_slice) {
            (Some(table), Some(drop)) => {
                let table = self.export_name_of(table);
                let drop = self.export_name_of(drop);
                self.global(&format!(
                    "
                    function {}(ptr, len) {{
                        const mem = {}();
                        const slice = mem.subarray(ptr / 4, ptr / 4 + len);
                        const result = [];
                        for (let i = 0; i < slice.length; i++) {{
                            result.push(wasm.{}.get(slice[i]));
                        }}
                        wasm.{}(ptr, len);
                        return result;
                    }}
                    ",
                    ret, mem, table, drop,
                ));
            }
            _ => {
                self.expose_take_object();
                self.global(&format!(
                    "
                    function {}(ptr, len) {{
                        const mem = {}();
                        const slice = mem.subarray(ptr / 4, ptr / 4 + len);
                        const result = [];
                        for (let i = 0; i < slice.length; i++) {{
                            result.push(takeObject(slice[i]));
                        }}
                        return result;
                    }}
                    ",
                    ret, mem,
                ));
            }
        }
        Ok(ret)
    }

    fn expose_get_array_i8_from_wasm(&mut self, memory: MemoryId) -> MemView {
        let view = self.expose_int8_memory(memory);
        self.arrayget("getArrayI8FromWasm", view, 1)
    }

    fn expose_get_array_u8_from_wasm(&mut self, memory: MemoryId) -> MemView {
        let view = self.expose_uint8_memory(memory);
        self.arrayget("getArrayU8FromWasm", view, 1)
    }

    fn expose_get_clamped_array_u8_from_wasm(&mut self, memory: MemoryId) -> MemView {
        let view = self.expose_clamped_uint8_memory(memory);
        self.arrayget("getClampedArrayU8FromWasm", view, 1)
    }

    fn expose_get_array_i16_from_wasm(&mut self, memory: MemoryId) -> MemView {
        let view = self.expose_int16_memory(memory);
        self.arrayget("getArrayI16FromWasm", view, 2)
    }

    fn expose_get_array_u16_from_wasm(&mut self, memory: MemoryId) -> MemView {
        let view = self.expose_uint16_memory(memory);
        self.arrayget("getArrayU16FromWasm", view, 2)
    }

    fn expose_get_array_i32_from_wasm(&mut self, memory: MemoryId) -> MemView {
        let view = self.expose_int32_memory(memory);
        self.arrayget("getArrayI32FromWasm", view, 4)
    }

    fn expose_get_array_u32_from_wasm(&mut self, memory: MemoryId) -> MemView {
        let view = self.expose_uint32_memory(memory);
        self.arrayget("getArrayU32FromWasm", view, 4)
    }

    fn expose_get_array_i64_from_wasm(&mut self, memory: MemoryId) -> MemView {
        let view = self.expose_int64_memory(memory);
        self.arrayget("getArrayI64FromWasm", view, 8)
    }

    fn expose_get_array_u64_from_wasm(&mut self, memory: MemoryId) -> MemView {
        let view = self.expose_uint64_memory(memory);
        self.arrayget("getArrayU64FromWasm", view, 8)
    }

    fn expose_get_array_f32_from_wasm(&mut self, memory: MemoryId) -> MemView {
        let view = self.expose_f32_memory(memory);
        self.arrayget("getArrayF32FromWasm", view, 4)
    }

    fn expose_get_array_f64_from_wasm(&mut self, memory: MemoryId) -> MemView {
        let view = self.expose_f64_memory(memory);
        self.arrayget("getArrayF64FromWasm", view, 8)
    }

    fn arrayget(&mut self, name: &'static str, view: MemView, size: usize) -> MemView {
        let ret = MemView {
            name: name.into(),
            num: view.num,
        };
        if !self.should_write_global(name) {
            return ret;
        }
        self.global(&format!(
            "
            function {name}(ptr, len) {{
                return {mem}().subarray(ptr / {size}, ptr / {size} + len);
            }}
            ",
            name = ret,
            mem = view,
            size = size,
        ));
        return ret;
    }

    fn expose_int8_memory(&mut self, memory: MemoryId) -> MemView {
        self.memview("Int8", memory)
    }

    fn expose_uint8_memory(&mut self, memory: MemoryId) -> MemView {
        self.memview("Uint8", memory)
    }

    fn expose_clamped_uint8_memory(&mut self, memory: MemoryId) -> MemView {
        self.memview("Uint8Clamped", memory)
    }

    fn expose_int16_memory(&mut self, memory: MemoryId) -> MemView {
        self.memview("Int16", memory)
    }

    fn expose_uint16_memory(&mut self, memory: MemoryId) -> MemView {
        self.memview("Uint16", memory)
    }

    fn expose_int32_memory(&mut self, memory: MemoryId) -> MemView {
        self.memview("Int32", memory)
    }

    fn expose_uint32_memory(&mut self, memory: MemoryId) -> MemView {
        self.memview("Uint32", memory)
    }

    fn expose_int64_memory(&mut self, memory: MemoryId) -> MemView {
        self.memview("BigInt64", memory)
    }

    fn expose_uint64_memory(&mut self, memory: MemoryId) -> MemView {
        self.memview("BigUint64", memory)
    }

    fn expose_f32_memory(&mut self, memory: MemoryId) -> MemView {
        self.memview("Float32", memory)
    }

    fn expose_f64_memory(&mut self, memory: MemoryId) -> MemView {
        self.memview("Float64", memory)
    }

    fn memview_function(&mut self, t: VectorKind, memory: MemoryId) -> MemView {
        match t {
            VectorKind::String => self.expose_uint8_memory(memory),
            VectorKind::I8 => self.expose_int8_memory(memory),
            VectorKind::U8 => self.expose_uint8_memory(memory),
            VectorKind::ClampedU8 => self.expose_clamped_uint8_memory(memory),
            VectorKind::I16 => self.expose_int16_memory(memory),
            VectorKind::U16 => self.expose_uint16_memory(memory),
            VectorKind::I32 => self.expose_int32_memory(memory),
            VectorKind::U32 => self.expose_uint32_memory(memory),
            VectorKind::I64 => self.expose_int64_memory(memory),
            VectorKind::U64 => self.expose_uint64_memory(memory),
            VectorKind::F32 => self.expose_f32_memory(memory),
            VectorKind::F64 => self.expose_f64_memory(memory),
            VectorKind::Externref => self.expose_uint32_memory(memory),
            VectorKind::NamedExternref(_) => self.expose_uint32_memory(memory),
        }
    }

    fn memview(&mut self, kind: &'static str, memory: walrus::MemoryId) -> MemView {
        let view = self.memview_memory(kind, memory);
        if !self.should_write_global(view.name.clone()) {
            return view;
        }
        let mem = self.export_name_of(memory);

        let cache = format!("cached{}Memory{}", kind, view.num);
        let resized_check = if self.module.memories.get(memory).shared {
            // When it's backed by a `SharedArrayBuffer`, growing the wasm module's memory
            // doesn't detach old references; instead, it just leaves them pointing to a
            // slice of the up-to-date memory. So in order to check if it's been grown, we
            // have to compare it to the up-to-date buffer.
            format!(
                "{cache}.buffer !== wasm.{mem}.buffer",
                cache = cache,
                mem = mem
            )
        } else {
            // Otherwise, we can do a quicker check of whether the buffer's been detached,
            // which is indicated by a length of 0.
            format!("{cache}.byteLength === 0", cache = cache)
        };

        // Initialize the cache to an empty array, which will trigger the resized check
        // on the first call and initialise the view.
        self.global(&format!("let {cache} = new {kind}Array();\n"));

        self.global(&format!(
            "
            function {name}() {{
                if ({resized_check}) {{
                    {cache} = new {kind}Array(wasm.{mem}.buffer);
                }}
                return {cache};
            }}
            ",
            name = view,
            cache = cache,
            resized_check = resized_check,
            kind = kind,
            mem = mem,
        ));
        return view;
    }

    fn memview_memory(&mut self, kind: &'static str, memory: walrus::MemoryId) -> MemView {
        let next = self.memories.len();
        let &mut (num, ref mut kinds) = self
            .memories
            .entry(memory)
            .or_insert((next, Default::default()));
        kinds.insert(kind);
        MemView {
            name: format!("get{}Memory", kind).into(),
            num,
        }
    }

    fn memview_table(&mut self, name: &'static str, table: walrus::TableId) -> MemView {
        let next = self.table_indices.len();
        let num = *self.table_indices.entry(table).or_insert(next);
        MemView {
            name: name.into(),
            num,
        }
    }

    fn expose_assert_class(&mut self) {
        if !self.should_write_global("assert_class") {
            return;
        }
        self.global(
            "
            function _assertClass(instance, klass) {
                if (!(instance instanceof klass)) {
                    throw new Error(`expected instance of ${klass.name}`);
                }
                return instance.ptr;
            }
            ",
        );
    }

    fn expose_global_stack_pointer(&mut self) {
        if !self.should_write_global("stack_pointer") {
            return;
        }
        self.global(&format!("let stack_pointer = {};", INITIAL_HEAP_OFFSET));
    }

    fn expose_borrowed_objects(&mut self) {
        if !self.should_write_global("borrowed_objects") {
            return;
        }
        self.expose_global_heap();
        self.expose_global_stack_pointer();
        // Our `stack_pointer` points to where we should start writing stack
        // objects, and the `stack_pointer` is incremented in a `finally` block
        // after executing this. Once we've reserved stack space we write the
        // value. Eventually underflow will throw an exception, but JS sort of
        // just handles it today...
        self.global(
            "
            function addBorrowedObject(obj) {
                if (stack_pointer == 1) throw new Error('out of js stack');
                heap[--stack_pointer] = obj;
                return stack_pointer;
            }
            ",
        );
    }

    fn expose_take_object(&mut self) {
        if !self.should_write_global("take_object") {
            return;
        }
        self.expose_get_object();
        self.expose_drop_ref();
        self.global(
            "
            function takeObject(idx) {
                const ret = getObject(idx);
                dropObject(idx);
                return ret;
            }
            ",
        );
    }

    fn expose_add_heap_object(&mut self) {
        if !self.should_write_global("add_heap_object") {
            return;
        }
        self.expose_global_heap();
        self.expose_global_heap_next();
        let set_heap_next = if self.config.debug {
            String::from(
                "
                if (typeof(heap_next) !== 'number') throw new Error('corrupt heap');
                ",
            )
        } else {
            String::new()
        };

        // Allocating a slot on the heap first goes through the linked list
        // (starting at `heap_next`). Once that linked list is exhausted we'll
        // be pointing beyond the end of the array, at which point we'll reserve
        // one more slot and use that.
        self.global(&format!(
            "
            function addHeapObject(obj) {{
                if (heap_next === heap.length) heap.push(heap.length + 1);
                const idx = heap_next;
                heap_next = heap[idx];
                {}
                heap[idx] = obj;
                return idx;
            }}
            ",
            set_heap_next
        ));
    }

    fn expose_handle_error(&mut self) -> Result<(), Error> {
        if !self.should_write_global("handle_error") {
            return Ok(());
        }
        let store = self
            .aux
            .exn_store
            .ok_or_else(|| anyhow!("failed to find `__wbindgen_exn_store` intrinsic"))?;
        let store = self.export_name_of(store);
        match (self.aux.externref_table, self.aux.externref_alloc) {
            (Some(table), Some(alloc)) => {
                let add = self.expose_add_to_externref_table(table, alloc)?;
                self.global(&format!(
                    "\
                    function handleError(f, args) {{
                        try {{
                            return f.apply(this, args);
                        }} catch (e) {{
                            const idx = {}(e);
                            wasm.{}(idx);
                        }}
                    }}
                    ",
                    add, store,
                ));
            }
            _ => {
                self.expose_add_heap_object();
                self.global(&format!(
                    "\
                    function handleError(f, args) {{
                        try {{
                            return f.apply(this, args);
                        }} catch (e) {{
                            wasm.{}(addHeapObject(e));
                        }}
                    }}
                    ",
                    store,
                ));
            }
        }
        Ok(())
    }

    fn expose_log_error(&mut self) {
        if !self.should_write_global("log_error") {
            return;
        }
        self.global(
            "\
            function logError(f, args) {
                try {
                    return f.apply(this, args);
                } catch (e) {
                    let error = (function () {
                        try {
                            return e instanceof Error \
                                ? `${e.message}\\n\\nStack:\\n${e.stack}` \
                                : e.toString();
                        } catch(_) {
                            return \"<failed to stringify thrown value>\";
                        }
                    }());
                    console.error(\"wasm-bindgen: imported JS function that \
                                    was not marked as `catch` threw an error:\", \
                                    error);
                    throw e;
                }
            }
            ",
        );
    }

    fn pass_to_wasm_function(&mut self, t: VectorKind, memory: MemoryId) -> Result<MemView, Error> {
        match t {
            VectorKind::String => self.expose_pass_string_to_wasm(memory),
            VectorKind::I8 | VectorKind::U8 | VectorKind::ClampedU8 => {
                self.expose_pass_array8_to_wasm(memory)
            }
            VectorKind::U16 | VectorKind::I16 => self.expose_pass_array16_to_wasm(memory),
            VectorKind::I32 | VectorKind::U32 => self.expose_pass_array32_to_wasm(memory),
            VectorKind::I64 | VectorKind::U64 => self.expose_pass_array64_to_wasm(memory),
            VectorKind::F32 => self.expose_pass_array_f32_to_wasm(memory),
            VectorKind::F64 => self.expose_pass_array_f64_to_wasm(memory),
            VectorKind::Externref => self.expose_pass_array_jsvalue_to_wasm(memory),
            VectorKind::NamedExternref(_) => self.expose_pass_array_jsvalue_to_wasm(memory),
        }
    }

    fn expose_get_vector_from_wasm(
        &mut self,
        ty: VectorKind,
        memory: MemoryId,
    ) -> Result<MemView, Error> {
        Ok(match ty {
            VectorKind::String => self.expose_get_string_from_wasm(memory)?,
            VectorKind::I8 => self.expose_get_array_i8_from_wasm(memory),
            VectorKind::U8 => self.expose_get_array_u8_from_wasm(memory),
            VectorKind::ClampedU8 => self.expose_get_clamped_array_u8_from_wasm(memory),
            VectorKind::I16 => self.expose_get_array_i16_from_wasm(memory),
            VectorKind::U16 => self.expose_get_array_u16_from_wasm(memory),
            VectorKind::I32 => self.expose_get_array_i32_from_wasm(memory),
            VectorKind::U32 => self.expose_get_array_u32_from_wasm(memory),
            VectorKind::I64 => self.expose_get_array_i64_from_wasm(memory),
            VectorKind::U64 => self.expose_get_array_u64_from_wasm(memory),
            VectorKind::F32 => self.expose_get_array_f32_from_wasm(memory),
            VectorKind::F64 => self.expose_get_array_f64_from_wasm(memory),
            VectorKind::Externref => self.expose_get_array_js_value_from_wasm(memory)?,
            VectorKind::NamedExternref(_) => self.expose_get_array_js_value_from_wasm(memory)?,
        })
    }

    fn expose_get_inherited_descriptor(&mut self) {
        if !self.should_write_global("get_inherited_descriptor") {
            return;
        }
        // It looks like while rare some browsers will move descriptors up the
        // property chain which runs the risk of breaking wasm-bindgen-generated
        // code because we're looking for precise descriptor functions rather
        // than relying on the prototype chain like most "normal JS" projects
        // do.
        //
        // As a result we have a small helper here which will walk the prototype
        // chain looking for a descriptor. For some more information on this see
        // #109
        self.global(
            "
            function GetOwnOrInheritedPropertyDescriptor(obj, id) {
              while (obj) {
                let desc = Object.getOwnPropertyDescriptor(obj, id);
                if (desc) return desc;
                obj = Object.getPrototypeOf(obj);
              }
              return {};
            }
            ",
        );
    }

    fn expose_is_like_none(&mut self) {
        if !self.should_write_global("is_like_none") {
            return;
        }
        self.global(
            "
            function isLikeNone(x) {
                return x === undefined || x === null;
            }
        ",
        );
    }

    fn expose_make_mut_closure(&mut self) -> Result<(), Error> {
        if !self.should_write_global("make_mut_closure") {
            return Ok(());
        }

        let table = self.export_function_table()?;

        let (register, unregister) = if self.config.weak_refs {
            self.expose_closure_finalization()?;
            (
                "CLOSURE_DTORS.register(real, state, state);",
                "CLOSURE_DTORS.unregister(state)",
            )
        } else {
            ("", "")
        };

        // For mutable closures they can't be invoked recursively.
        // To handle that we swap out the `this.a` pointer with zero
        // while we invoke it. If we finish and the closure wasn't
        // destroyed, then we put back the pointer so a future
        // invocation can succeed.
        self.global(&format!(
            "
            function makeMutClosure(arg0, arg1, dtor, f) {{
                const state = {{ a: arg0, b: arg1, cnt: 1, dtor }};
                const real = (...args) => {{
                    // First up with a closure we increment the internal reference
                    // count. This ensures that the Rust closure environment won't
                    // be deallocated while we're invoking it.
                    state.cnt++;
                    const a = state.a;
                    state.a = 0;
                    try {{
                        return f(a, state.b, ...args);
                    }} finally {{
                        if (--state.cnt === 0) {{
                            wasm.{table}.get(state.dtor)(a, state.b);
                            {unregister}
                        }} else {{
                            state.a = a;
                        }}
                    }}
                }};
                real.original = state;
                {register}
                return real;
            }}
            ",
            table = table,
            register = register,
            unregister = unregister,
        ));

        Ok(())
    }

    fn expose_make_closure(&mut self) -> Result<(), Error> {
        if !self.should_write_global("make_closure") {
            return Ok(());
        }

        let table = self.export_function_table()?;

        let (register, unregister) = if self.config.weak_refs {
            self.expose_closure_finalization()?;
            (
                "CLOSURE_DTORS.register(real, state, state);",
                "CLOSURE_DTORS.unregister(state)",
            )
        } else {
            ("", "")
        };

        // For shared closures they can be invoked recursively so we
        // just immediately pass through `this.a`. If we end up
        // executing the destructor, however, we clear out the
        // `this.a` pointer to prevent it being used again the
        // future.
        self.global(&format!(
            "
            function makeClosure(arg0, arg1, dtor, f) {{
                const state = {{ a: arg0, b: arg1, cnt: 1, dtor }};
                const real = (...args) => {{
                    // First up with a closure we increment the internal reference
                    // count. This ensures that the Rust closure environment won't
                    // be deallocated while we're invoking it.
                    state.cnt++;
                    try {{
                        return f(state.a, state.b, ...args);
                    }} finally {{
                        if (--state.cnt === 0) {{
                            wasm.{table}.get(state.dtor)(state.a, state.b);
                            state.a = 0;
                            {unregister}
                        }}
                    }}
                }};
                real.original = state;
                {register}
                return real;
            }}
            ",
            table = table,
            register = register,
            unregister = unregister,
        ));

        Ok(())
    }

    fn expose_closure_finalization(&mut self) -> Result<(), Error> {
        if !self.should_write_global("closure_finalization") {
            return Ok(());
        }
        assert!(self.config.weak_refs);
        let table = self.export_function_table()?;
        self.global(&format!(
            "
            const CLOSURE_DTORS = new FinalizationRegistry(state => {{
                wasm.{}.get(state.dtor)(state.a, state.b)
            }});
            ",
            table
        ));

        Ok(())
    }
    fn global(&mut self, s: &str) {
        let s = s.trim();

        // Ensure a blank line between adjacent items, and ensure everything is
        // terminated with a newline.
        while !self.globals.ends_with("\n\n\n") && !self.globals.ends_with("*/\n") {
            self.globals.push_str("\n");
        }
        self.globals.push_str(s);
        self.globals.push_str("\n");
    }

    fn require_class_wrap(&mut self, name: &str) {
        require_class(&mut self.exported_classes, name).wrap_needed = true;
    }

    fn add_module_import(&mut self, module: String, name: &str, actual: &str) {
        let rename = if name == actual {
            None
        } else {
            Some(actual.to_string())
        };
        self.js_imports
            .entry(module)
            .or_insert(Vec::new())
            .push((name.to_string(), rename));
    }

    fn import_name(&mut self, import: &JsImport) -> Result<String, Error> {
        if let Some(name) = self.imported_names.get(&import.name) {
            let mut name = name.clone();
            for field in import.fields.iter() {
                name.push_str(".");
                name.push_str(field);
            }
            return Ok(name.clone());
        }

        let mut name = match &import.name {
            JsImportName::Module { module, name } => {
                let unique_name = self.generate_identifier(name);
                self.add_module_import(module.clone(), name, &unique_name);
                unique_name
            }

            JsImportName::LocalModule { module, name } => {
                let unique_name = self.generate_identifier(name);
                let module = self.config.local_module_name(module);
                self.add_module_import(module, name, &unique_name);
                unique_name
            }

            JsImportName::InlineJs {
                unique_crate_identifier,
                snippet_idx_in_crate,
                name,
            } => {
                let module = self
                    .config
                    .inline_js_module_name(unique_crate_identifier, *snippet_idx_in_crate);
                let unique_name = self.generate_identifier(name);
                self.add_module_import(module, name, &unique_name);
                unique_name
            }

            JsImportName::VendorPrefixed { name, prefixes } => {
                self.imports_post.push_str("const l");
                self.imports_post.push_str(name);
                self.imports_post.push_str(" = ");
                switch(&mut self.imports_post, name, "", prefixes);
                self.imports_post.push_str(";\n");

                fn switch(dst: &mut String, name: &str, prefix: &str, left: &[String]) {
                    dst.push_str("(typeof ");
                    dst.push_str(prefix);
                    dst.push_str(name);
                    dst.push_str(" !== 'undefined' ? ");
                    dst.push_str(prefix);
                    dst.push_str(name);
                    dst.push_str(" : ");
                    if left.is_empty() {
                        dst.push_str("undefined");
                    } else {
                        switch(dst, name, &left[0], &left[1..]);
                    }
                    dst.push_str(")");
                }
                format!("l{}", name)
            }

            JsImportName::Global { name } => {
                let unique_name = self.generate_identifier(name);
                if unique_name != *name {
                    bail!("cannot import `{}` from two locations", name);
                }
                unique_name
            }
        };
        self.imported_names
            .insert(import.name.clone(), name.clone());

        // After we've got an actual name handle field projections
        for field in import.fields.iter() {
            name.push_str(".");
            name.push_str(field);
        }
        Ok(name)
    }

    /// If a start function is present, it removes it from the `start` section
    /// of the wasm module and then moves it to an exported function, named
    /// `__wbindgen_start`.
    fn unstart_start_function(&mut self) -> bool {
        let start = match self.module.start.take() {
            Some(id) => id,
            None => return false,
        };
        self.module.exports.add("__wbindgen_start", start);
        true
    }

    fn expose_get_from_externref_table(&mut self, table: TableId) -> Result<MemView, Error> {
        let view = self.memview_table("getFromExternrefTable", table);
        assert!(self.config.externref);
        if !self.should_write_global(view.to_string()) {
            return Ok(view);
        }
        let table = self.export_name_of(table);
        self.global(&format!(
            "function {view}(idx) {{ return wasm.{table}.get(idx); }}",
            view = view,
            table = table,
        ));

        Ok(view)
    }

    fn expose_take_from_externref_table(
        &mut self,
        table: TableId,
        drop: FunctionId,
    ) -> Result<MemView, Error> {
        let view = self.memview_table("takeFromExternrefTable", table);
        assert!(self.config.externref);
        if !self.should_write_global(view.to_string()) {
            return Ok(view);
        }
        let drop = self.export_name_of(drop);
        let table = self.export_name_of(table);
        self.global(&format!(
            "
                function {view}(idx) {{
                    const value = wasm.{table}.get(idx);
                    wasm.{drop}(idx);
                    return value;
                }}
            ",
            view = view,
            table = table,
            drop = drop,
        ));

        Ok(view)
    }

    fn expose_add_to_externref_table(
        &mut self,
        table: TableId,
        alloc: FunctionId,
    ) -> Result<MemView, Error> {
        let view = self.memview_table("addToExternrefTable", table);
        assert!(self.config.externref);
        if !self.should_write_global(view.to_string()) {
            return Ok(view);
        }
        let alloc = self.export_name_of(alloc);
        let table = self.export_name_of(table);
        self.global(&format!(
            "
                function {}(obj) {{
                    const idx = wasm.{}();
                    wasm.{}.set(idx, obj);
                    return idx;
                }}
            ",
            view, alloc, table,
        ));

        Ok(view)
    }

    pub fn generate(&mut self) -> Result<(), Error> {
        self.prestore_global_import_identifiers()?;
        for (id, adapter) in crate::sorted_iter(&self.wit.adapters) {
            let instrs = match &adapter.kind {
                AdapterKind::Import { .. } => continue,
                AdapterKind::Local { instructions } => instructions,
            };
            self.generate_adapter(*id, adapter, instrs)?;
        }

        let mut pairs = self.aux.export_map.iter().collect::<Vec<_>>();
        pairs.sort_by_key(|(k, _)| *k);
        check_duplicated_getter_and_setter_names(&pairs)?;

        for e in self.aux.enums.iter() {
            self.generate_enum(e)?;
        }

        for s in self.aux.structs.iter() {
            self.generate_struct(s)?;
        }

        self.typescript.push_str(&self.aux.extra_typescript);

        for path in self.aux.package_jsons.iter() {
            self.process_package_json(path)?;
        }

        self.export_destructor();

        Ok(())
    }

    fn export_destructor(&mut self) {
        let thread_destroy = match self.aux.thread_destroy {
            Some(id) => id,
            None => return,
        };

        self.export_name_of(thread_destroy);
    }

    /// Registers import names for all `Global` imports first before we actually
    /// process any adapters.
    ///
    /// `Global` names must be imported as their exact name, so if the same name
    /// from a global is also imported from a module we have to be sure to
    /// import the global first to ensure we don't shadow the actual global
    /// value. Otherwise we have no way of accessing the global value!
    ///
    /// This function will iterate through the import map up-front and generate
    /// a cache entry for each import name which is a `Global`.
    fn prestore_global_import_identifiers(&mut self) -> Result<(), Error> {
        for import in self.aux.import_map.values() {
            let js = match import {
                AuxImport::Value(AuxValue::Bare(js))
                | AuxImport::Value(AuxValue::ClassGetter(js, ..))
                | AuxImport::Value(AuxValue::Getter(js, ..))
                | AuxImport::Value(AuxValue::ClassSetter(js, ..))
                | AuxImport::Value(AuxValue::Setter(js, ..))
                | AuxImport::ValueWithThis(js, ..)
                | AuxImport::Instanceof(js)
                | AuxImport::Static(js)
                | AuxImport::StructuralClassGetter(js, ..)
                | AuxImport::StructuralClassSetter(js, ..)
                | AuxImport::IndexingGetterOfClass(js)
                | AuxImport::IndexingSetterOfClass(js)
                | AuxImport::IndexingDeleterOfClass(js) => js,
                _ => continue,
            };
            if let JsImportName::Global { .. } = js.name {
                self.import_name(js)?;
            }
        }
        Ok(())
    }

    fn generate_adapter(
        &mut self,
        id: AdapterId,
        adapter: &Adapter,
        instrs: &[InstructionData],
    ) -> Result<(), Error> {
        enum Kind<'a> {
            Export(&'a AuxExport),
            Import(walrus::ImportId),
            Adapter,
        }

        let kind = match self.aux.export_map.get(&id) {
            Some(export) => Kind::Export(export),
            None => {
                let core = self.wit.implements.iter().find(|pair| pair.2 == id);
                match core {
                    Some((core, _, _)) => Kind::Import(*core),
                    None => Kind::Adapter,
                }
            }
        };

        let catch = self.aux.imports_with_catch.contains(&id);
        if let Kind::Import(core) = kind {
            if !catch && self.attempt_direct_import(core, instrs)? {
                return Ok(());
            }
        }

        // Construct a JS shim builder, and configure it based on the kind of
        // export that we're generating.
        let mut builder = binding::Builder::new(self);
        builder.log_error(match kind {
            Kind::Export(_) | Kind::Adapter => false,
            Kind::Import(_) => builder.cx.config.debug,
        });
        builder.catch(catch);
        let mut arg_names = &None;
        let mut asyncness = false;
        let mut variadic = false;
        match kind {
            Kind::Export(export) => {
                arg_names = &export.arg_names;
                asyncness = export.asyncness;
                variadic = export.variadic;
                match &export.kind {
                    AuxExportKind::Function(_) => {}
                    AuxExportKind::Constructor(class) => builder.constructor(class),
                    AuxExportKind::Method {
                        receiver: reciever, ..
                    } => match reciever {
                        AuxReceiverKind::None => {}
                        AuxReceiverKind::Borrowed => builder.method(false),
                        AuxReceiverKind::Owned => builder.method(true),
                    },
                }
            }
            Kind::Import(_) => {}
            Kind::Adapter => {}
        }

        // Process the `binding` and generate a bunch of JS/TypeScript/etc.
        let binding::JsFunction {
            ts_sig,
            ts_arg_tys,
            ts_ret_ty,
            js_doc,
            code,
            might_be_optional_field,
            catch,
            log_error,
        } = builder
            .process(&adapter, instrs, arg_names, asyncness, variadic)
            .with_context(|| match kind {
                Kind::Export(e) => format!("failed to generate bindings for `{}`", e.debug_name),
                Kind::Import(i) => {
                    let i = builder.cx.module.imports.get(i);
                    format!(
                        "failed to generate bindings for import of `{}::{}`",
                        i.module, i.name
                    )
                }
                Kind::Adapter => format!("failed to generates bindings for adapter"),
            })?;

        // Once we've got all the JS then put it in the right location depending
        // on what's being exported.
        match kind {
            Kind::Export(export) => {
                assert_eq!(catch, false);
                assert_eq!(log_error, false);

                let ts_sig = match export.generate_typescript {
                    true => Some(ts_sig.as_str()),
                    false => None,
                };

                let js_docs = format_doc_comments(&export.comments, Some(js_doc));
                let ts_docs = format_doc_comments(&export.comments, None);

                match &export.kind {
                    AuxExportKind::Function(name) => {
                        if let Some(ts_sig) = ts_sig {
                            self.typescript.push_str(&js_docs);
                            self.typescript.push_str("export function ");
                            self.typescript.push_str(&name);
                            self.typescript.push_str(ts_sig);
                            self.typescript.push_str(";\n");
                        }

                        self.export(&name, &format!("function{}", code), Some(&js_docs))?;
                        self.globals.push_str("\n");
                    }
                    AuxExportKind::Constructor(class) => {
                        let exported = require_class(&mut self.exported_classes, class);

                        if exported.has_constructor {
                            bail!("found duplicate constructor for class `{}`", class);
                        }

                        exported.has_constructor = true;
                        exported.push(&js_docs, "constructor", "", &code, ts_sig);
                    }
                    AuxExportKind::Method {
                        class,
                        name,
                        receiver,
                        kind,
                    } => {
                        let exported = require_class(&mut self.exported_classes, class);

                        let mut prefix = String::new();
                        if receiver.is_static() {
                            prefix += "static ";
                        }
                        let ts = match kind {
                            AuxExportedMethodKind::Method => ts_sig,
                            AuxExportedMethodKind::Getter => {
                                prefix += "get ";
                                // For getters and setters, we generate a separate TypeScript definition.
                                if export.generate_typescript {
                                    exported.push_accessor_ts(
                                        &ts_docs,
                                        name,
                                        // This is only set to `None` when generating a constructor.
                                        ts_ret_ty
                                            .as_deref()
                                            .expect("missing return type for getter"),
                                        false,
                                        receiver.is_static(),
                                    );
                                }
                                // Add the getter to the list of readable fields (used to generate `toJSON`)
                                exported.readable_properties.push(name.clone());
                                // Ignore the raw signature.
                                None
                            }
                            AuxExportedMethodKind::Setter => {
                                prefix += "set ";
                                if export.generate_typescript {
                                    let is_optional = exported.push_accessor_ts(
                                        &ts_docs,
                                        name,
                                        &ts_arg_tys[0],
                                        true,
                                        receiver.is_static(),
                                    );
                                    // Set whether the field is optional.
                                    *is_optional = might_be_optional_field;
                                }
                                None
                            }
                        };

                        exported.push(&js_docs, name, &prefix, &code, ts);
                    }
                }
            }
            Kind::Import(core) => {
                let code = if catch {
                    format!(
                        "function() {{ return handleError(function {}, arguments) }}",
                        code
                    )
                } else if log_error {
                    format!(
                        "function() {{ return logError(function {}, arguments) }}",
                        code
                    )
                } else {
                    format!("function{}", code)
                };

                self.wasm_import_definitions.insert(core, code);
            }
            Kind::Adapter => {
                assert_eq!(catch, false);
                assert_eq!(log_error, false);

                self.globals.push_str("function ");
                self.globals.push_str(&self.adapter_name(id));
                self.globals.push_str(&code);
                self.globals.push_str("\n\n");
            }
        }
        return Ok(());
    }

    /// Returns whether we should disable the logic, in debug mode, to catch an
    /// error, log it, and rethrow it. This is only intended for user-defined
    /// imports, not all imports of everything.
    fn import_never_log_error(&self, import: &AuxImport) -> bool {
        match import {
            // Some intrinsics are intended to exactly throw errors, and in
            // general we shouldn't have exceptions in our intrinsics to debug,
            // so skip these.
            AuxImport::Intrinsic(_) => true,

            // Otherwise assume everything else gets a debug log of errors
            // thrown in debug mode.
            _ => false,
        }
    }

    /// Attempts to directly hook up the `id` import in the wasm module with
    /// the `instrs` specified.
    ///
    /// If this succeeds it returns `Ok(true)`, otherwise if it cannot be
    /// directly imported then `Ok(false)` is returned.
    fn attempt_direct_import(
        &mut self,
        id: ImportId,
        instrs: &[InstructionData],
    ) -> Result<bool, Error> {
        // First up extract the ID of the single called adapter, if any.
        let mut call = None;
        for instr in instrs {
            match instr.instr {
                Instruction::CallAdapter(id) => {
                    if call.is_some() {
                        return Ok(false);
                    } else {
                        call = Some(id);
                    }
                }
                Instruction::CallExport(_)
                | Instruction::CallTableElement(_)
                | Instruction::Standard(wit_walrus::Instruction::CallCore(_))
                | Instruction::Standard(wit_walrus::Instruction::CallAdapter(_)) => {
                    return Ok(false)
                }
                _ => {}
            }
        }
        let adapter = match call {
            Some(id) => id,
            None => return Ok(false),
        };
        match &self.wit.adapters[&adapter].kind {
            AdapterKind::Import { kind, .. } => match kind {
                AdapterJsImportKind::Normal => {}
                // method/constructors need glue because we either need to
                // invoke them as `new` or we need to invoke them with
                // method-call syntax to get the `this` parameter right.
                AdapterJsImportKind::Method | AdapterJsImportKind::Constructor => return Ok(false),
            },
            // This is an adapter-to-adapter call, so it needs a shim.
            AdapterKind::Local { .. } => return Ok(false),
        }

        // Next up check to make sure that this import is to a bare JS value
        // itself, no extra fluff intended.
        let js = match &self.aux.import_map[&adapter] {
            AuxImport::Value(AuxValue::Bare(js)) => js,
            _ => return Ok(false),
        };

        // Make sure this isn't variadic in any way which means we need some
        // sort of adapter glue.
        if self.aux.imports_with_variadic.contains(&adapter) {
            return Ok(false);
        }

        // Ensure that every single instruction can be represented without JS
        // glue being generated, aka it's covered by the JS ECMAScript bindings
        // for wasm.
        if !self.representable_without_js_glue(instrs) {
            return Ok(false);
        }

        // If there's no field projection happening here and this is a direct
        // import from an ES-looking module, then we can actually just hook this
        // up directly in the wasm file itself. Note that this is covered in the
        // various output formats as well:
        //
        // * `bundler` - they think wasm is an ES module anyway
        // * `web` - we're sure to emit more `import` directives during
        //   `gen_init` and we update the import object accordingly.
        // * `nodejs` - the polyfill we have for requiring a wasm file as a node
        //   module will naturally emit `require` directives for the module
        //   listed on each wasm import.
        // * `no-modules` - imports aren't allowed here anyway from other
        //   modules and an error is generated.
        if js.fields.len() == 0 {
            match &js.name {
                JsImportName::Module { module, name } => {
                    let import = self.module.imports.get_mut(id);
                    import.module = module.clone();
                    import.name = name.clone();
                    return Ok(true);
                }
                JsImportName::LocalModule { module, name } => {
                    let module = self.config.local_module_name(module);
                    let import = self.module.imports.get_mut(id);
                    import.module = module;
                    import.name = name.clone();
                    return Ok(true);
                }
                JsImportName::InlineJs {
                    unique_crate_identifier,
                    snippet_idx_in_crate,
                    name,
                } => {
                    let module = self
                        .config
                        .inline_js_module_name(unique_crate_identifier, *snippet_idx_in_crate);
                    let import = self.module.imports.get_mut(id);
                    import.module = module;
                    import.name = name.clone();
                    return Ok(true);
                }

                // Fall through below to requiring a JS shim to create an item
                // that we can import. These are plucked from the global
                // environment so there's no way right now to describe these
                // imports in an ES module-like fashion.
                JsImportName::Global { .. } | JsImportName::VendorPrefixed { .. } => {}
            }
        }

        self.expose_not_defined();
        let name = self.import_name(js)?;
        let js = format!(
            "typeof {name} == 'function' ? {name} : notDefined('{name}')",
            name = name,
        );
        self.wasm_import_definitions.insert(id, js);
        Ok(true)
    }

    fn representable_without_js_glue(&self, instrs: &[InstructionData]) -> bool {
        use Instruction::*;
        let standard_enabled = self.config.wasm_interface_types;

        let mut last_arg = None;
        let mut saw_call = false;
        for instr in instrs {
            match instr.instr {
                // Is an adapter section getting emitted? If so, then every
                // standard operation is natively supported!
                Standard(_) if standard_enabled => {}

                // Fetching arguments is just that, a fetch, so no need for
                // glue. Note though that the arguments must be fetched in order
                // for this to actually work,
                Standard(wit_walrus::Instruction::ArgGet(i)) => {
                    if saw_call {
                        return false;
                    }
                    match (i, last_arg) {
                        (0, None) => last_arg = Some(0),
                        (n, Some(i)) if n == i + 1 => last_arg = Some(n),
                        _ => return false,
                    }
                }

                // Similarly calling a function is the same as in JS, no glue
                // needed.
                CallAdapter(_) => saw_call = true,

                // Conversions to wasm integers are always supported since
                // they're coerced into i32/f32/f64 appropriately.
                Standard(wit_walrus::Instruction::IntToWasm { .. }) => {}

                // Converts from wasm to JS, however, only supports most
                // integers. Converting into a u32 isn't supported because we
                // need to generate glue to change the sign.
                Standard(wit_walrus::Instruction::WasmToInt {
                    output: wit_walrus::ValType::U32,
                    ..
                }) => return false,
                Standard(wit_walrus::Instruction::WasmToInt { .. }) => {}

                // JS spec automatically coerces boolean values to i32 of 0 or 1
                // depending on true/false
                I32FromBool => {}

                _ => return false,
            }
        }

        return true;
    }

    /// Generates a JS snippet appropriate for invoking `import`.
    ///
    /// This is generating code for `binding` where `bindings` has more type
    /// infomation. The `args` array is the list of JS expressions representing
    /// the arguments to pass to JS. Finally `variadic` indicates whether the
    /// last argument is a list to be splatted in a variadic way, and `prelude`
    /// is a location to push some more initialization JS if necessary.
    ///
    /// The returned value here is a JS expression which evaluates to the
    /// purpose of `AuxImport`, which depends on the kind of import.
    fn invoke_import(
        &mut self,
        import: &AuxImport,
        kind: AdapterJsImportKind,
        args: &[String],
        variadic: bool,
        prelude: &mut String,
    ) -> Result<String, Error> {
        let variadic_args = |js_arguments: &[String]| {
            Ok(if !variadic {
                format!("{}", js_arguments.join(", "))
            } else {
                let (last_arg, args) = match js_arguments.split_last() {
                    Some(pair) => pair,
                    None => bail!("a function with no arguments cannot be variadic"),
                };
                if args.len() > 0 {
                    format!("{}, ...{}", args.join(", "), last_arg)
                } else {
                    format!("...{}", last_arg)
                }
            })
        };
        match import {
            AuxImport::Value(val) => match kind {
                AdapterJsImportKind::Constructor => {
                    let js = match val {
                        AuxValue::Bare(js) => self.import_name(js)?,
                        _ => bail!("invalid import set for constructor"),
                    };
                    Ok(format!("new {}({})", js, variadic_args(&args)?))
                }
                AdapterJsImportKind::Method => {
                    let descriptor = |anchor: &str, extra: &str, field: &str, which: &str| {
                        format!(
                            "GetOwnOrInheritedPropertyDescriptor({}{}, '{}').{}",
                            anchor, extra, field, which
                        )
                    };
                    let js = match val {
                        AuxValue::Bare(js) => self.import_name(js)?,
                        AuxValue::Getter(class, field) => {
                            self.expose_get_inherited_descriptor();
                            let class = self.import_name(class)?;
                            descriptor(&class, ".prototype", field, "get")
                        }
                        AuxValue::ClassGetter(class, field) => {
                            self.expose_get_inherited_descriptor();
                            let class = self.import_name(class)?;
                            descriptor(&class, "", field, "get")
                        }
                        AuxValue::Setter(class, field) => {
                            self.expose_get_inherited_descriptor();
                            let class = self.import_name(class)?;
                            descriptor(&class, ".prototype", field, "set")
                        }
                        AuxValue::ClassSetter(class, field) => {
                            self.expose_get_inherited_descriptor();
                            let class = self.import_name(class)?;
                            descriptor(&class, "", field, "set")
                        }
                    };
                    Ok(format!("{}.call({})", js, variadic_args(&args)?))
                }
                AdapterJsImportKind::Normal => {
                    let js = match val {
                        AuxValue::Bare(js) => self.import_name(js)?,
                        _ => bail!("invalid import set for free function"),
                    };
                    Ok(format!("{}({})", js, variadic_args(&args)?))
                }
            },

            AuxImport::ValueWithThis(class, name) => {
                let class = self.import_name(class)?;
                Ok(format!("{}.{}({})", class, name, variadic_args(&args)?))
            }

            AuxImport::Instanceof(js) => {
                assert!(kind == AdapterJsImportKind::Normal);
                assert!(!variadic);
                assert_eq!(args.len(), 1);
                let js = self.import_name(js)?;
                write!(
                    prelude,
                    "\
                    let result;
                    try {{
                        result = {} instanceof {};
                    }} catch {{
                        result = false;
                    }}
                    ",
                    args[0], js,
                )
                .unwrap();
                Ok("result".to_owned())
            }

            AuxImport::Static(js) => {
                assert!(kind == AdapterJsImportKind::Normal);
                assert!(!variadic);
                assert_eq!(args.len(), 0);
                self.import_name(js)
            }

            AuxImport::Closure {
                dtor,
                mutable,
                adapter,
                nargs: _,
            } => {
                assert!(kind == AdapterJsImportKind::Normal);
                assert!(!variadic);
                assert_eq!(args.len(), 3);

                let call = self.adapter_name(*adapter);

                if *mutable {
                    self.expose_make_mut_closure()?;

                    Ok(format!(
                        "makeMutClosure({arg0}, {arg1}, {dtor}, {call})",
                        arg0 = &args[0],
                        arg1 = &args[1],
                        dtor = dtor,
                        call = call,
                    ))
                } else {
                    self.expose_make_closure()?;

                    Ok(format!(
                        "makeClosure({arg0}, {arg1}, {dtor}, {call})",
                        arg0 = &args[0],
                        arg1 = &args[1],
                        dtor = dtor,
                        call = call,
                    ))
                }
            }

            AuxImport::StructuralMethod(name) => {
                assert!(kind == AdapterJsImportKind::Normal);
                let (receiver, args) = match args.split_first() {
                    Some(pair) => pair,
                    None => bail!("structural method calls must have at least one argument"),
                };
                Ok(format!("{}.{}({})", receiver, name, variadic_args(args)?))
            }

            AuxImport::StructuralGetter(field) => {
                assert!(kind == AdapterJsImportKind::Normal);
                assert!(!variadic);
                assert_eq!(args.len(), 1);
                Ok(format!("{}.{}", args[0], field))
            }

            AuxImport::StructuralClassGetter(class, field) => {
                assert!(kind == AdapterJsImportKind::Normal);
                assert!(!variadic);
                assert_eq!(args.len(), 0);
                let class = self.import_name(class)?;
                Ok(format!("{}.{}", class, field))
            }

            AuxImport::StructuralSetter(field) => {
                assert!(kind == AdapterJsImportKind::Normal);
                assert!(!variadic);
                assert_eq!(args.len(), 2);
                Ok(format!("{}.{} = {}", args[0], field, args[1]))
            }

            AuxImport::StructuralClassSetter(class, field) => {
                assert!(kind == AdapterJsImportKind::Normal);
                assert!(!variadic);
                assert_eq!(args.len(), 1);
                let class = self.import_name(class)?;
                Ok(format!("{}.{} = {}", class, field, args[0]))
            }

            AuxImport::IndexingGetterOfClass(class) => {
                assert!(kind == AdapterJsImportKind::Normal);
                assert!(!variadic);
                assert_eq!(args.len(), 1);
                let class = self.import_name(class)?;
                Ok(format!("{}[{}]", class, args[0]))
            }

            AuxImport::IndexingGetterOfObject => {
                assert!(kind == AdapterJsImportKind::Normal);
                assert!(!variadic);
                assert_eq!(args.len(), 2);
                Ok(format!("{}[{}]", args[0], args[1]))
            }

            AuxImport::IndexingSetterOfClass(class) => {
                assert!(kind == AdapterJsImportKind::Normal);
                assert!(!variadic);
                assert_eq!(args.len(), 2);
                let class = self.import_name(class)?;
                Ok(format!("{}[{}] = {}", class, args[0], args[1]))
            }

            AuxImport::IndexingSetterOfObject => {
                assert!(kind == AdapterJsImportKind::Normal);
                assert!(!variadic);
                assert_eq!(args.len(), 3);
                Ok(format!("{}[{}] = {}", args[0], args[1], args[2]))
            }

            AuxImport::IndexingDeleterOfClass(class) => {
                assert!(kind == AdapterJsImportKind::Normal);
                assert!(!variadic);
                assert_eq!(args.len(), 1);
                let class = self.import_name(class)?;
                Ok(format!("delete {}[{}]", class, args[0]))
            }

            AuxImport::IndexingDeleterOfObject => {
                assert!(kind == AdapterJsImportKind::Normal);
                assert!(!variadic);
                assert_eq!(args.len(), 2);
                Ok(format!("delete {}[{}]", args[0], args[1]))
            }

            AuxImport::WrapInExportedClass(class) => {
                assert!(kind == AdapterJsImportKind::Normal);
                assert!(!variadic);
                assert_eq!(args.len(), 1);
                self.require_class_wrap(class);
                Ok(format!("{}.__wrap({})", class, args[0]))
            }

            AuxImport::Intrinsic(intrinsic) => {
                assert!(kind == AdapterJsImportKind::Normal);
                assert!(!variadic);
                self.invoke_intrinsic(intrinsic, args, prelude)
            }
        }
    }

    /// Same as `invoke_import` above, except more specialized and only used for
    /// generating the JS expression needed to implement a particular intrinsic.
    fn invoke_intrinsic(
        &mut self,
        intrinsic: &Intrinsic,
        args: &[String],
        prelude: &mut String,
    ) -> Result<String, Error> {
        let expr = match intrinsic {
            Intrinsic::JsvalEq => {
                assert_eq!(args.len(), 2);
                format!("{} === {}", args[0], args[1])
            }

            Intrinsic::JsvalLooseEq => {
                assert_eq!(args.len(), 2);
                format!("{} == {}", args[0], args[1])
            }

            Intrinsic::IsFunction => {
                assert_eq!(args.len(), 1);
                format!("typeof({}) === 'function'", args[0])
            }

            Intrinsic::IsUndefined => {
                assert_eq!(args.len(), 1);
                format!("{} === undefined", args[0])
            }

            Intrinsic::IsNull => {
                assert_eq!(args.len(), 1);
                format!("{} === null", args[0])
            }

            Intrinsic::IsObject => {
                assert_eq!(args.len(), 1);
                prelude.push_str(&format!("const val = {};\n", args[0]));
                format!("typeof(val) === 'object' && val !== null")
            }

            Intrinsic::IsSymbol => {
                assert_eq!(args.len(), 1);
                format!("typeof({}) === 'symbol'", args[0])
            }

            Intrinsic::IsString => {
                assert_eq!(args.len(), 1);
                format!("typeof({}) === 'string'", args[0])
            }

            Intrinsic::IsBigInt => {
                assert_eq!(args.len(), 1);
                format!("typeof({}) === 'bigint'", args[0])
            }

            Intrinsic::Typeof => {
                assert_eq!(args.len(), 1);
                format!("typeof {}", args[0])
            }

            Intrinsic::In => {
                assert_eq!(args.len(), 2);
                format!("{} in {}", args[0], args[1])
            }

            Intrinsic::IsFalsy => {
                assert_eq!(args.len(), 1);
                format!("!{}", args[0])
            }

            Intrinsic::AsNumber => {
                assert_eq!(args.len(), 1);
                format!("+{}", args[0])
            }

            Intrinsic::TryIntoNumber => {
                assert_eq!(args.len(), 1);
                prelude.push_str("let result;\n");
                writeln!(
                    prelude,
                    "try {{ result = +{} }} catch (e) {{ result = e }}",
                    args[0]
                )
                .unwrap();
                "result".to_owned()
            }

            Intrinsic::Neg => {
                assert_eq!(args.len(), 1);
                format!("-{}", args[0])
            }

            Intrinsic::BitAnd => {
                assert_eq!(args.len(), 2);
                format!("{} & {}", args[0], args[1])
            }

            Intrinsic::BitOr => {
                assert_eq!(args.len(), 2);
                format!("{} | {}", args[0], args[1])
            }

            Intrinsic::BitXor => {
                assert_eq!(args.len(), 2);
                format!("{} ^ {}", args[0], args[1])
            }

            Intrinsic::BitNot => {
                assert_eq!(args.len(), 1);
                format!("~{}", args[0])
            }

            Intrinsic::Shl => {
                assert_eq!(args.len(), 2);
                format!("{} << {}", args[0], args[1])
            }

            Intrinsic::Shr => {
                assert_eq!(args.len(), 2);
                format!("{} >> {}", args[0], args[1])
            }

            Intrinsic::UnsignedShr => {
                assert_eq!(args.len(), 2);
                format!("{} >>> {}", args[0], args[1])
            }

            Intrinsic::Add => {
                assert_eq!(args.len(), 2);
                format!("{} + {}", args[0], args[1])
            }

            Intrinsic::Sub => {
                assert_eq!(args.len(), 2);
                format!("{} - {}", args[0], args[1])
            }

            Intrinsic::Div => {
                assert_eq!(args.len(), 2);
                format!("{} / {}", args[0], args[1])
            }

            Intrinsic::CheckedDiv => {
                assert_eq!(args.len(), 2);
                prelude.push_str("let result;\n");
                writeln!(
                    prelude,
                    "try {{
                        result = {} / {};
                    }} catch (e) {{
                        if (e instanceof RangeError) {{
                            result = e;
                        }} else {{
                            throw e;
                        }}
                    }}",
                    args[0], args[1]
                )
                .unwrap();
                "result".to_owned()
            }

            Intrinsic::Mul => {
                assert_eq!(args.len(), 2);
                format!("{} * {}", args[0], args[1])
            }

            Intrinsic::Rem => {
                assert_eq!(args.len(), 2);
                format!("{} % {}", args[0], args[1])
            }

            Intrinsic::Pow => {
                assert_eq!(args.len(), 2);
                format!("{} ** {}", args[0], args[1])
            }

            Intrinsic::LT => {
                assert_eq!(args.len(), 2);
                format!("{} < {}", args[0], args[1])
            }

            Intrinsic::LE => {
                assert_eq!(args.len(), 2);
                format!("{} <= {}", args[0], args[1])
            }

            Intrinsic::GE => {
                assert_eq!(args.len(), 2);
                format!("{} >= {}", args[0], args[1])
            }

            Intrinsic::GT => {
                assert_eq!(args.len(), 2);
                format!("{} > {}", args[0], args[1])
            }

            Intrinsic::ObjectCloneRef => {
                assert_eq!(args.len(), 1);
                args[0].clone()
            }

            Intrinsic::ObjectDropRef => {
                assert_eq!(args.len(), 1);
                args[0].clone()
            }

            Intrinsic::CallbackDrop => {
                assert_eq!(args.len(), 1);
                prelude.push_str(&format!("const obj = {}.original;\n", args[0]));
                prelude.push_str("if (obj.cnt-- == 1) {\n");
                prelude.push_str("obj.a = 0;\n");
                prelude.push_str("return true;\n");
                prelude.push_str("}\n");
                "false".to_string()
            }

            Intrinsic::NumberNew => {
                assert_eq!(args.len(), 1);
                args[0].clone()
            }

            Intrinsic::BigIntFromStr => {
                assert_eq!(args.len(), 1);
                format!("BigInt({})", args[0])
            }

            Intrinsic::BigIntFromI64 | Intrinsic::BigIntFromU64 => {
                assert_eq!(args.len(), 1);
                args[0].clone()
            }

            Intrinsic::BigIntFromI128 | Intrinsic::BigIntFromU128 => {
                assert_eq!(args.len(), 2);
                format!("{} << BigInt(64) | {}", args[0], args[1])
            }

            Intrinsic::StringNew => {
                assert_eq!(args.len(), 1);
                args[0].clone()
            }

            Intrinsic::SymbolNamedNew => {
                assert_eq!(args.len(), 1);
                format!("Symbol({})", args[0])
            }

            Intrinsic::SymbolAnonymousNew => {
                assert_eq!(args.len(), 0);
                "Symbol()".to_string()
            }

            Intrinsic::NumberGet => {
                assert_eq!(args.len(), 1);
                prelude.push_str(&format!("const obj = {};\n", args[0]));
                format!("typeof(obj) === 'number' ? obj : undefined")
            }

            Intrinsic::StringGet => {
                assert_eq!(args.len(), 1);
                prelude.push_str(&format!("const obj = {};\n", args[0]));
                format!("typeof(obj) === 'string' ? obj : undefined")
            }

            Intrinsic::BooleanGet => {
                assert_eq!(args.len(), 1);
                prelude.push_str(&format!("const v = {};\n", args[0]));
                format!("typeof(v) === 'boolean' ? (v ? 1 : 0) : 2")
            }

            Intrinsic::BigIntGetAsI64 => {
                assert_eq!(args.len(), 1);
                prelude.push_str(&format!("const v = {};\n", args[0]));
                format!("typeof(v) === 'bigint' ? v : undefined")
            }

            Intrinsic::Throw => {
                assert_eq!(args.len(), 1);
                format!("throw new Error({})", args[0])
            }

            Intrinsic::Rethrow => {
                assert_eq!(args.len(), 1);
                format!("throw {}", args[0])
            }

            Intrinsic::ErrorNew => {
                assert_eq!(args.len(), 1);
                format!("new Error({})", args[0])
            }

            Intrinsic::Module => {
                assert_eq!(args.len(), 0);
                if !self.config.mode.no_modules() && !self.config.mode.web() {
                    bail!(
                        "`wasm_bindgen::module` is currently only supported with \
                         `--target no-modules` and `--target web`"
                    );
                }
                format!("init.__wbindgen_wasm_module")
            }

            Intrinsic::Memory => {
                assert_eq!(args.len(), 0);
                let mut memories = self.module.memories.iter();
                let memory = memories
                    .next()
                    .ok_or_else(|| anyhow!("no memory found to return in memory intrinsic"))?
                    .id();
                if memories.next().is_some() {
                    bail!(
                        "multiple memories found, unsure which to return \
                         from memory intrinsic"
                    );
                }
                drop(memories);
                format!("wasm.{}", self.export_name_of(memory))
            }

            Intrinsic::FunctionTable => {
                assert_eq!(args.len(), 0);
                let name = self.export_function_table()?;
                format!("wasm.{}", name)
            }

            Intrinsic::DebugString => {
                assert_eq!(args.len(), 1);
                self.expose_debug_string();
                format!("debugString({})", args[0])
            }

            Intrinsic::JsonParse => {
                assert_eq!(args.len(), 1);
                format!("JSON.parse({})", args[0])
            }

            Intrinsic::JsonSerialize => {
                assert_eq!(args.len(), 1);
                // Turns out `JSON.stringify(undefined) === undefined`, so if
                // we're passed `undefined` reinterpret it as `null` for JSON
                // purposes.
                prelude.push_str(&format!("const obj = {};\n", args[0]));
                "JSON.stringify(obj === undefined ? null : obj)".to_string()
            }

            Intrinsic::ExternrefHeapLiveCount => {
                assert_eq!(args.len(), 0);
                self.expose_global_heap();
                prelude.push_str(
                    "
                        let free_count = 0;
                        let next = heap_next;
                        while (next < heap.length) {
                            free_count += 1;
                            next = heap[next];
                        }
                    ",
                );
                format!(
                    "heap.length - free_count - {} - {}",
                    INITIAL_HEAP_OFFSET,
                    INITIAL_HEAP_VALUES.len(),
                )
            }

            Intrinsic::InitExternrefTable => {
                let table = self
                    .aux
                    .externref_table
                    .ok_or_else(|| anyhow!("must enable externref to use externref intrinsic"))?;
                let name = self.export_name_of(table);
                // Grow the table to insert our initial values, and then also
                // set the 0th slot to `undefined` since that's what we've
                // historically used for our ABI which is that the index of 0
                // returns `undefined` for types like `None` going out.
                let mut base = format!(
                    "
                      const table = wasm.{};
                      const offset = table.grow({});
                      table.set(0, undefined);
                    ",
                    name,
                    INITIAL_HEAP_VALUES.len(),
                );
                for (i, value) in INITIAL_HEAP_VALUES.iter().enumerate() {
                    base.push_str(&format!("table.set(offset + {}, {});\n", i, value));
                }
                base
            }
        };
        Ok(expr)
    }

    fn generate_enum(&mut self, enum_: &AuxEnum) -> Result<(), Error> {
        let docs = format_doc_comments(&enum_.comments, None);
        let mut variants = String::new();

        if enum_.generate_typescript {
            self.typescript.push_str(&docs);
            self.typescript
                .push_str(&format!("export enum {} {{", enum_.name));
        }
        for (name, value, comments) in enum_.variants.iter() {
            let variant_docs = if comments.is_empty() {
                String::new()
            } else {
                format_doc_comments(&comments, None)
            };
            if !variant_docs.is_empty() {
                variants.push_str("\n");
                variants.push_str(&variant_docs);
            }
            variants.push_str(&format!("{}:{},", name, value));
            variants.push_str(&format!("\"{}\":\"{}\",", value, name));
            if enum_.generate_typescript {
                self.typescript.push_str("\n");
                if !variant_docs.is_empty() {
                    self.typescript.push_str(&variant_docs);
                }
                self.typescript.push_str(&format!("  {},", name));
            }
        }
        if enum_.generate_typescript {
            self.typescript.push_str("\n}\n");
        }
        self.export(
            &enum_.name,
            &format!("Object.freeze({{ {} }})", variants),
            Some(&docs),
        )?;

        Ok(())
    }

    fn generate_struct(&mut self, struct_: &AuxStruct) -> Result<(), Error> {
        let class = require_class(&mut self.exported_classes, &struct_.name);
        class.comments = format_doc_comments(&struct_.comments, None);
        class.is_inspectable = struct_.is_inspectable;
        class.generate_typescript = struct_.generate_typescript;
        Ok(())
    }

    fn process_package_json(&mut self, path: &Path) -> Result<(), Error> {
        if self.config.mode.no_modules() {
            bail!(
                "NPM dependencies have been specified in `{}` but \
                 this is incompatible with the `no-modules` target",
                path.display(),
            );
        }

        let contents =
            fs::read_to_string(path).context(format!("failed to read `{}`", path.display()))?;
        let json: serde_json::Value = serde_json::from_str(&contents)?;
        let object = match json.as_object() {
            Some(s) => s,
            None => bail!(
                "expected `package.json` to have an JSON object in `{}`",
                path.display()
            ),
        };
        let mut iter = object.iter();
        let mut value = None;
        while let Some((key, v)) = iter.next() {
            if key == "dependencies" {
                value = Some(v);
                break;
            }
        }
        let value = if let Some(value) = value {
            value
        } else {
            return Ok(());
        };
        let value = match value.as_object() {
            Some(s) => s,
            None => bail!(
                "expected `dependencies` to be a JSON object in `{}`",
                path.display()
            ),
        };

        for (name, value) in value.iter() {
            let value = match value.as_str() {
                Some(s) => s,
                None => bail!(
                    "keys in `dependencies` are expected to be strings in `{}`",
                    path.display()
                ),
            };
            if let Some((prev, _prev_version)) = self.npm_dependencies.get(name) {
                bail!(
                    "dependency on NPM package `{}` specified in two `package.json` files, \
                     which at the time is not allowed:\n  * {}\n  * {}",
                    name,
                    path.display(),
                    prev.display(),
                )
            }

            self.npm_dependencies
                .insert(name.to_string(), (path.to_path_buf(), value.to_string()));
        }

        Ok(())
    }

    fn expose_debug_string(&mut self) {
        if !self.should_write_global("debug_string") {
            return;
        }

        self.global(
            "
           function debugString(val) {
                // primitive types
                const type = typeof val;
                if (type == 'number' || type == 'boolean' || val == null) {
                    return  `${val}`;
                }
                if (type == 'string') {
                    return `\"${val}\"`;
                }
                if (type == 'symbol') {
                    const description = val.description;
                    if (description == null) {
                        return 'Symbol';
                    } else {
                        return `Symbol(${description})`;
                    }
                }
                if (type == 'function') {
                    const name = val.name;
                    if (typeof name == 'string' && name.length > 0) {
                        return `Function(${name})`;
                    } else {
                        return 'Function';
                    }
                }
                // objects
                if (Array.isArray(val)) {
                    const length = val.length;
                    let debug = '[';
                    if (length > 0) {
                        debug += debugString(val[0]);
                    }
                    for(let i = 1; i < length; i++) {
                        debug += ', ' + debugString(val[i]);
                    }
                    debug += ']';
                    return debug;
                }
                // Test for built-in
                const builtInMatches = /\\[object ([^\\]]+)\\]/.exec(toString.call(val));
                let className;
                if (builtInMatches.length > 1) {
                    className = builtInMatches[1];
                } else {
                    // Failed to match the standard '[object ClassName]'
                    return toString.call(val);
                }
                if (className == 'Object') {
                    // we're a user defined class or Object
                    // JSON.stringify avoids problems with cycles, and is generally much
                    // easier than looping through ownProperties of `val`.
                    try {
                        return 'Object(' + JSON.stringify(val) + ')';
                    } catch (_) {
                        return 'Object';
                    }
                }
                // errors
                if (val instanceof Error) {
                    return `${val.name}: ${val.message}\\n${val.stack}`;
                }
                // TODO we could test for more things here, like `Set`s and `Map`s.
                return className;
            }
        ",
        );
    }

    fn export_function_table(&mut self) -> Result<String, Error> {
        match self.aux.function_table {
            Some(id) => Ok(self.export_name_of(id)),
            None => bail!("no function table found in module"),
        }
    }

    fn export_name_of(&mut self, id: impl Into<walrus::ExportItem>) -> String {
        let id = id.into();
        let export = self.module.exports.iter().find(|e| {
            use walrus::ExportItem::*;

            match (e.item, id) {
                (Function(a), Function(b)) => a == b,
                (Table(a), Table(b)) => a == b,
                (Memory(a), Memory(b)) => a == b,
                (Global(a), Global(b)) => a == b,
                _ => false,
            }
        });
        if let Some(export) = export {
            return export.name.clone();
        }
        let default_name = format!("__wbindgen_export_{}", self.next_export_idx);
        self.next_export_idx += 1;
        let name = match id {
            walrus::ExportItem::Memory(_) if self.module.memories.iter().count() == 1 => {
                "memory".to_owned()
            }
            walrus::ExportItem::Function(f) => match &self.module.funcs.get(f).name {
                Some(s) => to_js_identifier(s),
                None => default_name,
            },
            _ => default_name,
        };
        self.module.exports.add(&name, id);
        return name;

        // Not really an exhaustive list, but works for our purposes.
        fn to_js_identifier(name: &str) -> String {
            name.chars()
                .map(|c| {
                    if c.is_ascii() && (c.is_alphabetic() || c.is_numeric()) {
                        c
                    } else {
                        '_'
                    }
                })
                .collect()
        }
    }

    fn adapter_name(&self, id: AdapterId) -> String {
        format!("__wbg_adapter_{}", id.0)
    }

    fn generate_identifier(&mut self, name: &str) -> String {
        let cnt = self
            .defined_identifiers
            .entry(name.to_string())
            .or_insert(0);
        *cnt += 1;
        // We want to mangle `default` at once, so we can support default exports and don't generate
        // invalid glue code like this: `import { default } from './module';`.
        if *cnt == 1 && name != "default" {
            name.to_string()
        } else {
            format!("{}{}", name, cnt)
        }
    }

    fn inject_stack_pointer_shim(&mut self) -> Result<(), Error> {
        if self.stack_pointer_shim_injected {
            return Ok(());
        }
        let stack_pointer = match self.aux.shadow_stack_pointer {
            Some(s) => s,
            // In theory this shouldn't happen since malloc is included in
            // most wasm binaries (and may be gc'd out) and that almost
            // always pulls in a stack pointer. We can try to synthesize
            // something here later if necessary.
            None => bail!("failed to find shadow stack pointer"),
        };

        use walrus::ir::*;

        let mut builder =
            walrus::FunctionBuilder::new(&mut self.module.types, &[ValType::I32], &[ValType::I32]);
        builder.name("__wbindgen_add_to_stack_pointer".to_string());

        let mut body = builder.func_body();
        let arg = self.module.locals.add(ValType::I32);

        // Create a shim function that mutate the stack pointer
        // to avoid exporting a mutable global.
        body.local_get(arg)
            .global_get(stack_pointer)
            .binop(BinaryOp::I32Add)
            .global_set(stack_pointer)
            .global_get(stack_pointer);

        let add_to_stack_pointer_func = builder.finish(vec![arg], &mut self.module.funcs);

        self.module
            .exports
            .add("__wbindgen_add_to_stack_pointer", add_to_stack_pointer_func);

        self.stack_pointer_shim_injected = true;

        Ok(())
    }
}

fn check_duplicated_getter_and_setter_names(
    exports: &[(&AdapterId, &AuxExport)],
) -> Result<(), Error> {
    fn verify_exports(
        first_class: &str,
        first_field: &str,
        first_receiver: &AuxReceiverKind,
        second_class: &str,
        second_field: &str,
        second_receiver: &AuxReceiverKind,
    ) -> Result<(), Error> {
        let both_are_in_the_same_class = first_class == second_class;
        let both_are_referencing_the_same_field = first_field == second_field
            && first_receiver.is_static() == second_receiver.is_static();
        if both_are_in_the_same_class && both_are_referencing_the_same_field {
            bail!(format!(
                "There can be only one getter/setter definition for `{}` in `{}`",
                first_field, first_class
            ));
        }
        Ok(())
    }
    for (idx, (_, first_export)) in exports.iter().enumerate() {
        for (_, second_export) in exports.iter().skip(idx + 1) {
            match (&first_export.kind, &second_export.kind) {
                (
                    AuxExportKind::Method {
                        class: first_class,
                        name: first_name,
                        kind: AuxExportedMethodKind::Getter,
                        receiver: first_receiver,
                    },
                    AuxExportKind::Method {
                        class: second_class,
                        name: second_name,
                        kind: AuxExportedMethodKind::Getter,
                        receiver: second_receiver,
                    },
                ) => verify_exports(
                    first_class,
                    first_name,
                    first_receiver,
                    second_class,
                    second_name,
                    second_receiver,
                )?,
                (
                    AuxExportKind::Method {
                        class: first_class,
                        name: first_name,
                        kind: AuxExportedMethodKind::Setter,
                        receiver: first_receiver,
                    },
                    AuxExportKind::Method {
                        class: second_class,
                        name: second_name,
                        kind: AuxExportedMethodKind::Setter,
                        receiver: second_receiver,
                    },
                ) => verify_exports(
                    first_class,
                    first_name,
                    first_receiver,
                    second_class,
                    second_name,
                    second_receiver,
                )?,
                _ => {}
            }
        }
    }
    Ok(())
}

fn format_doc_comments(comments: &str, js_doc_comments: Option<String>) -> String {
    let body: String = comments.lines().map(|c| format!("*{}\n", c)).collect();
    let doc = if let Some(docs) = js_doc_comments {
        docs.lines().map(|l| format!("* {}\n", l)).collect()
    } else {
        String::new()
    };
    format!("/**\n{}{}*/\n", body, doc)
}

fn require_class<'a>(
    exported_classes: &'a mut Option<BTreeMap<String, ExportedClass>>,
    name: &str,
) -> &'a mut ExportedClass {
    exported_classes
        .as_mut()
        .expect("classes already written")
        .entry(name.to_string())
        .or_insert_with(ExportedClass::default)
}

impl ExportedClass {
    fn push(
        &mut self,
        docs: &str,
        function_name: &str,
        function_prefix: &str,
        js: &str,
        ts: Option<&str>,
    ) {
        self.contents.push_str(docs);
        self.contents.push_str(function_prefix);
        self.contents.push_str(function_name);
        self.contents.push_str(js);
        self.contents.push_str("\n");
        if let Some(ts) = ts {
            self.typescript.push_str(docs);
            self.typescript.push_str("  ");
            self.typescript.push_str(function_prefix);
            self.typescript.push_str(function_name);
            self.typescript.push_str(ts);
            self.typescript.push_str(";\n");
        }
    }

    fn push_accessor_ts(
        &mut self,
        docs: &str,
        field: &str,
        ty: &str,
        is_setter: bool,
        is_static: bool,
    ) -> &mut bool {
        let (ty_dst, accessor_docs, has_setter, is_optional, is_static_dst) = self
            .typescript_fields
            .entry(field.to_string())
            .or_insert_with(Default::default);

        *ty_dst = ty.to_string();
        // Deterministic output: always use the getter's docs if available
        if !docs.is_empty() && (accessor_docs.is_empty() || !is_setter) {
            *accessor_docs = docs.to_owned();
        }
        *has_setter |= is_setter;
        *is_static_dst = is_static;
        is_optional
    }
}

struct MemView {
    name: Cow<'static, str>,
    num: usize,
}

impl fmt::Display for MemView {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.name, self.num)
    }
}
