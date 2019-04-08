use crate::decode;
use crate::descriptor::{Descriptor, VectorKind};
use crate::{Bindgen, EncodeInto, OutputMode};
use failure::{bail, Error, ResultExt};
use std::collections::{BTreeMap, HashMap, HashSet};
use std::env;
use std::fs;
use walrus::{MemoryId, Module};
use wasm_bindgen_wasm_interpreter::Interpreter;

mod js2rust;
use self::js2rust::{ExportedShim, Js2Rust};
mod rust2js;
use self::rust2js::Rust2Js;
mod closures;

pub struct Context<'a> {
    pub globals: String,
    pub imports: String,
    pub imports_post: String,
    pub footer: String,
    pub typescript: String,
    pub exposed_globals: Option<HashSet<&'static str>>,
    pub required_internal_exports: HashSet<&'static str>,
    pub imported_functions: HashSet<&'a str>,
    pub imported_statics: HashSet<&'a str>,
    pub config: &'a Bindgen,
    pub module: &'a mut Module,
    pub start: Option<String>,

    /// A map which maintains a list of what identifiers we've imported and what
    /// they're named locally.
    ///
    /// The `Option<String>` key is the module that identifiers were imported
    /// from, `None` being the global module. The second key is a map of
    /// identifiers we've already imported from the module to what they're
    /// called locally.
    pub imported_names: HashMap<ImportModule<'a>, HashMap<&'a str, String>>,

    /// A set of all imported identifiers to the number of times they've been
    /// imported, used to generate new identifiers.
    pub imported_identifiers: HashMap<String, usize>,

    /// A map of all imported shim functions which can actually be directly
    /// imported from the containing module. The mapping here maps to a tuple,
    /// where the first element is the module to import from and the second
    /// element is the name to import from the module.
    ///
    /// Note that for `direct_imports` no shims are generated in JS that
    /// wasm-bindgen emits.
    pub direct_imports: HashMap<&'a str, (&'a str, &'a str)>,

    pub exported_classes: Option<BTreeMap<String, ExportedClass>>,
    pub function_table_needed: bool,
    pub interpreter: &'a mut Interpreter,
    pub memory: MemoryId,

    /// A map of all local modules we've found, from the identifier they're
    /// known as to their actual JS contents.
    pub local_modules: HashMap<&'a str, &'a str>,

    /// A map of how many snippets we've seen from each unique crate identifier,
    /// used to number snippets correctly when writing them to the filesystem
    /// when there's multiple snippets within one crate that aren't all part of
    /// the same `Program`.
    pub snippet_offsets: HashMap<&'a str, usize>,

    /// All package.json dependencies we've learned about so far
    pub package_json_read: HashSet<&'a str>,

    /// A map of the name of npm dependencies we've loaded so far to the path
    /// they're defined in as well as their version specification.
    pub npm_dependencies: HashMap<String, (&'a str, String)>,

    pub anyref: wasm_bindgen_anyref_xform::Context,
}

#[derive(Default)]
pub struct ExportedClass {
    comments: String,
    contents: String,
    typescript: String,
    has_constructor: bool,
    wrap_needed: bool,
}

pub struct SubContext<'a, 'b: 'a> {
    pub program: &'b decode::Program<'b>,
    pub cx: &'a mut Context<'b>,
    pub vendor_prefixes: HashMap<&'b str, Vec<&'b str>>,
}

pub enum ImportTarget {
    Function(String),
    Method(String),
    Constructor(String),
    StructuralMethod(String),
    StructuralGetter(Option<String>, String),
    StructuralSetter(Option<String>, String),
    StructuralIndexingGetter(Option<String>),
    StructuralIndexingSetter(Option<String>),
    StructuralIndexingDeleter(Option<String>),
}

/// Return value of `determine_import` which is where we look at an imported
/// function AST and figure out where it's actually being imported from
/// (performing some validation checks and whatnot).
enum Import<'a> {
    /// An item is imported from the global scope. The `name` is what's imported
    /// and the optional `field` is the field on that item we're importing.
    Global {
        name: &'a str,
        field: Option<&'a str>,
    },
    /// Same as `Global`, except the `name` is imported via an ESM import from
    /// the specified `module` path.
    Module {
        module: &'a str,
        name: &'a str,
        field: Option<&'a str>,
    },
    /// Same as `Module`, except we're importing from a local module defined in
    /// a local JS snippet.
    LocalModule {
        module: &'a str,
        name: &'a str,
        field: Option<&'a str>,
    },
    /// Same as `Module`, except we're importing from an `inline_js` attribute
    InlineJs {
        unique_crate_identifier: &'a str,
        snippet_idx_in_crate: usize,
        name: &'a str,
        field: Option<&'a str>,
    },
    /// A global import which may have a number of vendor prefixes associated
    /// with it, like `webkitAudioPrefix`. The `name` is the name to test
    /// whether it's prefixed.
    VendorPrefixed {
        name: &'a str,
        prefixes: Vec<&'a str>,
    },
}

const INITIAL_HEAP_VALUES: &[&str] = &["undefined", "null", "true", "false"];
// Must be kept in sync with `src/lib.rs` of the `wasm-bindgen` crate
const INITIAL_HEAP_OFFSET: usize = 32;

impl<'a> Context<'a> {
    fn should_write_global(&mut self, name: &'static str) -> bool {
        self.exposed_globals.as_mut().unwrap().insert(name)
    }

    fn export(&mut self, name: &str, contents: &str, comments: Option<String>) {
        let contents = contents.trim();
        if let Some(ref c) = comments {
            self.globals.push_str(c);
            self.typescript.push_str(c);
        }
        let global = match self.config.mode {
            OutputMode::Node {
                experimental_modules: false,
            } => {
                if contents.starts_with("class") {
                    format!("{1}\nmodule.exports.{0} = {0};\n", name, contents)
                } else {
                    format!("module.exports.{} = {};\n", name, contents)
                }
            }
            OutputMode::NoModules { .. } => {
                if contents.starts_with("class") {
                    format!("{1}\n__exports.{0} = {0};\n", name, contents)
                } else {
                    format!("__exports.{} = {};\n", name, contents)
                }
            }
            OutputMode::Bundler { .. }
            | OutputMode::Node {
                experimental_modules: true,
            } => {
                if contents.starts_with("function") {
                    format!("export function {}{}\n", name, &contents[8..])
                } else if contents.starts_with("class") {
                    format!("export {}\n", contents)
                } else {
                    format!("export const {} = {};\n", name, contents)
                }
            }
            OutputMode::Web => {
                // In web mode there's no need to export the internals of
                // wasm-bindgen as we're not using the module itself as the
                // import object but rather the `__exports` map we'll be
                // initializing below.
                let export = if name.starts_with("__wbindgen")
                    || name.starts_with("__wbg_")
                    || name.starts_with("__widl_")
                {
                    ""
                } else {
                    "export "
                };
                if contents.starts_with("function") {
                    format!("{}function {}{}\n", export, name, &contents[8..])
                } else if contents.starts_with("class") {
                    format!("{}{}\n", export, contents)
                } else {
                    format!("{}const {} = {};\n", export, name, contents)
                }
            }
        };
        self.global(&global);

        if self.config.mode.web() {
            self.global(&format!("__exports.{} = {0};", name));
        }
    }

    fn require_internal_export(&mut self, name: &'static str) -> Result<(), Error> {
        if !self.required_internal_exports.insert(name) {
            return Ok(());
        }

        if self.module.exports.iter().any(|e| e.name == name) {
            return Ok(());
        }

        bail!(
            "the exported function `{}` is required to generate bindings \
             but it was not found in the wasm file, perhaps the `std` feature \
             of the `wasm-bindgen` crate needs to be enabled?",
            name
        );
    }

    pub fn finalize(&mut self, module_name: &str) -> Result<(String, String), Error> {
        // Wire up all default intrinsics, those which don't have any sort of
        // dependency on the clsoure/anyref/etc passes. This is where almost all
        // intrinsics are wired up.
        self.wire_up_initial_intrinsics()?;

        // Next up, perform our closure rewriting pass. This is where we'll
        // update invocations of the closure intrinsics we have to instead call
        // appropriate JS functions which actually create closures.
        closures::rewrite(self).with_context(|_| "failed to generate internal closure shims")?;

        // Finalize all bindings for JS classes. This is where we'll generate JS
        // glue for all classes as well as finish up a few final imports like
        // `__wrap` and such.
        self.write_classes()?;

        // And now that we're almost ready, run the final "anyref" pass. This is
        // where we transform a wasm module which doesn't actually use `anyref`
        // anywhere to using the type internally. The transformation here is
        // based off all the previous data we've collected so far for each
        // import/export listed.
        self.anyref.run(self.module)?;

        // With our transforms finished, we can now wire up the final list of
        // intrinsics which may depend on the passes run above.
        self.wire_up_late_intrinsics()?;

        // We're almost done here, so we can delete any internal exports (like
        // `__wbindgen_malloc`) if none of our JS glue actually needed it.
        self.unexport_unused_internal_exports();

        // Handle the `start` function, if one was specified. If we're in a
        // --test mode (such as wasm-bindgen-test-runner) then we skip this
        // entirely. Otherwise we want to first add a start function to the
        // `start` section if one is specified.
        //
        // Note that once a start function is added, if any, we immediately
        // un-start it. This is done because we require that the JS glue
        // initializes first, so we execute wasm startup manually once the JS
        // glue is all in place.
        let mut needs_manual_start = false;
        if self.config.emit_start {
            self.add_start_function()?;
            needs_manual_start = self.unstart_start_function();
        }

        // If our JS glue needs to access the function table, then do so here.
        // JS closure shim generation may access the function table as an
        // example, but if there's no closures in the module there's no need to
        // export it!
        self.export_table()?;

        // After all we've done, especially
        // `unexport_unused_internal_exports()`, we probably have a bunch of
        // garbage in the module that's no longer necessary, so delete
        // everything that we don't actually need.
        walrus::passes::gc::run(self.module);

        // Almost there, but before we're done make sure to rewrite the `module`
        // field of all imports in the wasm module. The field is currently
        // always `__wbindgen_placeholder__` coming out of rustc, but we need to
        // update that here to the shim file or an actual ES module.
        self.rewrite_imports(module_name);

        // We likely made a ton of modifications, so add ourselves to the
        // producers section!
        self.update_producers_section();

        // Cause any future calls to `should_write_global` to panic, making sure
        // we don't ask for items which we can no longer emit.
        drop(self.exposed_globals.take().unwrap());

        Ok(self.finalize_js(module_name, needs_manual_start))
    }

    /// Performs the task of actually generating the final JS module, be it
    /// `--target no-modules`, `--target web`, or for bundlers. This is the very
    /// last step performed in `finalize`.
    fn finalize_js(&mut self, module_name: &str, needs_manual_start: bool) -> (String, String) {
        let mut ts = self.typescript.clone();
        let mut js = String::new();
        if self.config.mode.no_modules() {
            js.push_str("(function() {\n");
        }

        // Depending on the output mode, generate necessary glue to actually
        // import the wasm file in one way or another.
        let mut init = (String::new(), String::new());
        match &self.config.mode {
            // In `--target no-modules` mode we need to both expose a name on
            // the global object as well as generate our own custom start
            // function.
            OutputMode::NoModules { global } => {
                js.push_str("const __exports = {};\n");
                js.push_str("let wasm;\n");
                init = self.gen_init(&module_name, needs_manual_start);
                self.footer.push_str(&format!(
                    "self.{} = Object.assign(init, __exports);\n",
                    global
                ));
            }

            // With normal CommonJS node we need to defer requiring the wasm
            // until the end so most of our own exports are hooked up
            OutputMode::Node {
                experimental_modules: false,
            } => {
                self.footer
                    .push_str(&format!("wasm = require('./{}_bg');\n", module_name));
                if needs_manual_start {
                    self.footer.push_str("wasm.__wbindgen_start();\n");
                }
                js.push_str("var wasm;\n");
            }

            // With Bundlers and modern ES6 support in Node we can simply import
            // the wasm file as if it were an ES module and let the
            // bundler/runtime take care of it.
            OutputMode::Bundler { .. }
            | OutputMode::Node {
                experimental_modules: true,
            } => {
                self.imports
                    .push_str(&format!("import * as wasm from './{}_bg';\n", module_name));
                if needs_manual_start {
                    self.footer.push_str("wasm.__wbindgen_start();\n");
                }
            }

            // With a browser-native output we're generating an ES module, but
            // browsers don't support natively importing wasm right now so we
            // expose the same initialization function as `--target no-modules`
            // as the default export of the module.
            OutputMode::Web => {
                self.imports_post.push_str("const __exports = {};\n");
                self.imports_post.push_str("let wasm;\n");
                init = self.gen_init(&module_name, needs_manual_start);
                self.footer.push_str("export default init;\n");
            }
        }

        let (init_js, init_ts) = init;

        ts.push_str(&init_ts);

        // Emit all the JS for importing all our functionality
        assert!(
            !self.config.mode.uses_es_modules() || js.is_empty(),
            "ES modules require imports to be at the start of the file"
        );
        js.push_str(&self.imports);
        js.push_str("\n");
        js.push_str(&self.imports_post);
        js.push_str("\n");

        // Emit all our exports from this module
        js.push_str(&self.globals);
        js.push_str("\n");

        // Generate the initialization glue, if there was any
        js.push_str(&init_js);
        js.push_str("\n");
        js.push_str(&self.footer);
        js.push_str("\n");
        if self.config.mode.no_modules() {
            js.push_str("})();\n");
        }

        while js.contains("\n\n\n") {
            js = js.replace("\n\n\n", "\n\n");
        }

        (js, ts)
    }

    fn wire_up_initial_intrinsics(&mut self) -> Result<(), Error> {
        self.bind("__wbindgen_string_new", &|me| {
            me.anyref.import_xform(
                "__wbindgen_placeholder__",
                "__wbindgen_string_new",
                &[],
                true,
            );
            me.expose_get_string_from_wasm();
            Ok(format!(
                "function(p, l) {{ return {}; }}",
                me.add_heap_object("getStringFromWasm(p, l)")
            ))
        })?;

        self.bind("__wbindgen_number_new", &|me| {
            me.anyref.import_xform(
                "__wbindgen_placeholder__",
                "__wbindgen_number_new",
                &[],
                true,
            );
            Ok(format!(
                "function(i) {{ return {}; }}",
                me.add_heap_object("i")
            ))
        })?;

        self.bind("__wbindgen_number_get", &|me| {
            me.anyref.import_xform(
                "__wbindgen_placeholder__",
                "__wbindgen_number_get",
                &[(0, false)],
                false,
            );
            me.expose_uint8_memory();
            Ok(format!(
                "
                function(n, invalid) {{
                    let obj = {};
                    if (typeof(obj) === 'number') return obj;
                    getUint8Memory()[invalid] = 1;
                    return 0;
                }}
                ",
                me.get_object("n"),
            ))
        })?;

        self.bind("__wbindgen_is_null", &|me| {
            me.anyref.import_xform(
                "__wbindgen_placeholder__",
                "__wbindgen_is_null",
                &[(0, false)],
                false,
            );
            Ok(format!(
                "function(i) {{ return {} === null ? 1 : 0; }}",
                me.get_object("i")
            ))
        })?;

        self.bind("__wbindgen_is_undefined", &|me| {
            me.anyref.import_xform(
                "__wbindgen_placeholder__",
                "__wbindgen_is_undefined",
                &[(0, false)],
                false,
            );
            Ok(format!(
                "function(i) {{ return {} === undefined ? 1 : 0; }}",
                me.get_object("i")
            ))
        })?;

        self.bind("__wbindgen_boolean_get", &|me| {
            me.anyref.import_xform(
                "__wbindgen_placeholder__",
                "__wbindgen_boolean_get",
                &[(0, false)],
                false,
            );
            Ok(format!(
                "
                function(i) {{
                    let v = {};
                    return typeof(v) === 'boolean' ? (v ? 1 : 0) : 2;
                }}
                ",
                me.get_object("i"),
            ))
        })?;

        self.bind("__wbindgen_symbol_new", &|me| {
            me.anyref.import_xform(
                "__wbindgen_placeholder__",
                "__wbindgen_symbol_new",
                &[],
                true,
            );
            me.expose_get_string_from_wasm();
            let expr = "ptr === 0 ? Symbol() : Symbol(getStringFromWasm(ptr, len))";
            Ok(format!(
                "function(ptr, len) {{ return {}; }}",
                me.add_heap_object(expr)
            ))
        })?;

        self.bind("__wbindgen_is_symbol", &|me| {
            me.anyref.import_xform(
                "__wbindgen_placeholder__",
                "__wbindgen_is_symbol",
                &[(0, false)],
                false,
            );
            Ok(format!(
                "function(i) {{ return typeof({}) === 'symbol' ? 1 : 0; }}",
                me.get_object("i")
            ))
        })?;

        self.bind("__wbindgen_is_object", &|me| {
            me.anyref.import_xform(
                "__wbindgen_placeholder__",
                "__wbindgen_is_object",
                &[(0, false)],
                false,
            );
            Ok(format!(
                "
                function(i) {{
                    const val = {};
                    return typeof(val) === 'object' && val !== null ? 1 : 0;
                }}",
                me.get_object("i"),
            ))
        })?;

        self.bind("__wbindgen_is_function", &|me| {
            me.anyref.import_xform(
                "__wbindgen_placeholder__",
                "__wbindgen_is_function",
                &[(0, false)],
                false,
            );
            Ok(format!(
                "function(i) {{ return typeof({}) === 'function' ? 1 : 0; }}",
                me.get_object("i")
            ))
        })?;

        self.bind("__wbindgen_is_string", &|me| {
            me.anyref.import_xform(
                "__wbindgen_placeholder__",
                "__wbindgen_is_string",
                &[(0, false)],
                false,
            );
            Ok(format!(
                "function(i) {{ return typeof({}) === 'string' ? 1 : 0; }}",
                me.get_object("i")
            ))
        })?;

        self.bind("__wbindgen_string_get", &|me| {
            me.expose_pass_string_to_wasm()?;
            me.expose_uint32_memory();
            me.anyref.import_xform(
                "__wbindgen_placeholder__",
                "__wbindgen_string_get",
                &[(0, false)],
                false,
            );
            Ok(format!(
                "
                function(i, len_ptr) {{
                    let obj = {};
                    if (typeof(obj) !== 'string') return 0;
                    const ptr = passStringToWasm(obj);
                    getUint32Memory()[len_ptr / 4] = WASM_VECTOR_LEN;
                    return ptr;
                }}
                ",
                me.get_object("i"),
            ))
        })?;

        self.bind("__wbindgen_debug_string", &|me| {
            me.expose_pass_string_to_wasm()?;
            me.expose_uint32_memory();

            let debug_str = "
                val => {
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
                            debug += debug_str(val[0]);
                        }
                        for(let i = 1; i < length; i++) {
                            debug += ', ' + debug_str(val[i]);
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
                        return `${val.name}: ${val.message}\n${val.stack}`;
                    }
                    // TODO we could test for more things here, like `Set`s and `Map`s.
                    return className;
                }
            ";
            Ok(format!(
                "
                function(i, len_ptr) {{
                    const debug_str = {};
                    const toString = Object.prototype.toString;
                    const val = {};
                    const debug = debug_str(val);
                    const ptr = passStringToWasm(debug);
                    getUint32Memory()[len_ptr / 4] = WASM_VECTOR_LEN;
                    return ptr;
                }}
                ",
                debug_str,
                me.get_object("i"),
            ))
        })?;

        self.bind("__wbindgen_cb_drop", &|me| {
            me.anyref.import_xform(
                "__wbindgen_placeholder__",
                "__wbindgen_cb_drop",
                &[(0, true)],
                false,
            );
            Ok(format!(
                "
                function(i) {{
                    const obj = {}.original;
                    if (obj.cnt-- == 1) {{
                        obj.a = 0;
                        return 1;
                    }}
                    return 0;
                }}
                ",
                me.take_object("i"),
            ))
        })?;

        self.bind("__wbindgen_cb_forget", &|me| {
            Ok(if me.config.anyref {
                // TODO: we should rewrite this in the anyref xform to not even
                // call into JS
                me.anyref.import_xform(
                    "__wbindgen_placeholder__",
                    "__wbindgen_cb_drop",
                    &[(0, true)],
                    false,
                );
                String::from("function(obj) {}")
            } else {
                me.expose_drop_ref();
                "dropObject".to_string()
            })
        })?;

        self.bind("__wbindgen_json_parse", &|me| {
            me.expose_get_string_from_wasm();
            me.anyref.import_xform(
                "__wbindgen_placeholder__",
                "__wbindgen_json_parse",
                &[],
                true,
            );
            let expr = "JSON.parse(getStringFromWasm(ptr, len))";
            let expr = me.add_heap_object(expr);
            Ok(format!("function(ptr, len) {{ return {}; }}", expr))
        })?;

        self.bind("__wbindgen_json_serialize", &|me| {
            me.anyref.import_xform(
                "__wbindgen_placeholder__",
                "__wbindgen_json_serialize",
                &[(0, false)],
                false,
            );
            me.expose_pass_string_to_wasm()?;
            me.expose_uint32_memory();
            Ok(format!(
                "
                function(idx, ptrptr) {{
                    const ptr = passStringToWasm(JSON.stringify({}));
                    getUint32Memory()[ptrptr / 4] = ptr;
                    return WASM_VECTOR_LEN;
                }}
                ",
                me.get_object("idx"),
            ))
        })?;

        self.bind("__wbindgen_jsval_eq", &|me| {
            Ok(format!(
                "function(a, b) {{ return {} === {} ? 1 : 0; }}",
                me.get_object("a"),
                me.get_object("b")
            ))
        })?;

        self.bind("__wbindgen_memory", &|me| {
            let mem = me.memory();
            Ok(format!(
                "function() {{ return {}; }}",
                me.add_heap_object(mem)
            ))
        })?;

        self.bind("__wbindgen_module", &|me| {
            if !me.config.mode.no_modules() && !me.config.mode.web() {
                bail!(
                    "`wasm_bindgen::module` is currently only supported with \
                     `--target no-modules` and `--target web`"
                );
            }
            Ok(format!(
                "function() {{ return {}; }}",
                me.add_heap_object("init.__wbindgen_wasm_module")
            ))
        })?;

        self.bind("__wbindgen_function_table", &|me| {
            me.function_table_needed = true;
            Ok(format!(
                "function() {{ return {}; }}",
                me.add_heap_object("wasm.__wbg_function_table")
            ))
        })?;

        self.bind("__wbindgen_rethrow", &|me| {
            Ok(format!(
                "function(idx) {{ throw {}; }}",
                me.take_object("idx")
            ))
        })?;

        self.bind("__wbindgen_throw", &|me| {
            me.expose_get_string_from_wasm();
            Ok(String::from(
                "
                function(ptr, len) {
                    throw new Error(getStringFromWasm(ptr, len));
                }
                ",
            ))
        })?;

        Ok(())
    }

    /// Provide implementations of remaining intrinsics after initial passes
    /// have been run on the wasm module.
    ///
    /// The intrinsics implemented here are added very late in the process or
    /// otherwise may be overwritten by passes (such as the anyref pass). As a
    /// result they don't go into the initial list of intrinsics but go just at
    /// the end.
    fn wire_up_late_intrinsics(&mut self) -> Result<(), Error> {
        // After the anyref pass has executed, if this intrinsic is needed then
        // we expose a function which initializes it
        self.bind("__wbindgen_init_anyref_table", &|me| {
            me.expose_anyref_table();
            Ok(String::from(
                "function() {
                const table = wasm.__wbg_anyref_table;
                const offset = table.grow(4);
                table.set(offset + 0, undefined);
                table.set(offset + 1, null);
                table.set(offset + 2, true);
                table.set(offset + 3, false);
            }",
            ))
        })?;

        // make sure that the anyref pass runs before binding this as anyref may
        // remove calls to this import and then gc would remove it
        self.bind("__wbindgen_object_clone_ref", &|me| {
            me.expose_get_object();
            me.expose_add_heap_object();
            Ok(String::from(
                "
                function(idx) {
                    return addHeapObject(getObject(idx));
                }
                ",
            ))
        })?;

        // like above, make sure anyref runs first and the anyref pass may
        // remove usages of this.
        self.bind("__wbindgen_object_drop_ref", &|me| {
            me.expose_drop_ref();
            Ok(String::from("function(i) { dropObject(i); }"))
        })?;

        Ok(())
    }

    fn ts_for_init_fn(has_memory: bool) -> String {
        let (memory_doc, memory_param) = if has_memory {
            (
                "* @param {WebAssembly.Memory} maybe_memory\n",
                ", maybe_memory: WebAssembly.Memory",
            )
        } else {
            ("", "")
        };
        format!(
            "\n\
            /**\n\
            * If `module_or_path` is {{RequestInfo}}, makes a request and\n\
            * for everything else, calls `WebAssembly.instantiate` directly.\n\
            *\n\
            * @param {{RequestInfo | BufferSource | WebAssembly.Module}} module_or_path\n\
            {}\
            *\n\
            * @returns {{Promise<any>}}\n\
            */\n\
            export function init \
                (module_or_path: RequestInfo | BufferSource | WebAssembly.Module{}): Promise<any>;
        ",
            memory_doc, memory_param
        )
    }

    fn gen_init(&mut self, module_name: &str, needs_manual_start: bool) -> (String, String) {
        let mem = self.module.memories.get(self.memory);
        let (init_memory1, init_memory2) = if mem.import.is_some() {
            let mut memory = String::from("new WebAssembly.Memory({");
            memory.push_str(&format!("initial:{}", mem.initial));
            if let Some(max) = mem.maximum {
                memory.push_str(&format!(",maximum:{}", max));
            }
            if mem.shared {
                memory.push_str(",shared:true");
            }
            memory.push_str("})");
            self.imports_post.push_str("let memory;\n");
            (
                format!("memory = __exports.memory = maybe_memory;"),
                format!("memory = __exports.memory = {};", memory),
            )
        } else {
            (String::new(), String::new())
        };
        let init_memory_arg = if mem.import.is_some() {
            ", maybe_memory"
        } else {
            ""
        };

        let ts = Self::ts_for_init_fn(mem.import.is_some());
        let js = format!(
            "\
                function init(module{init_memory_arg}) {{
                    let result;
                    const imports = {{ './{module}': __exports }};
                    if (module instanceof URL || typeof module === 'string' || module instanceof Request) {{
                        {init_memory2}
                        const response = fetch(module);
                        if (typeof WebAssembly.instantiateStreaming === 'function') {{
                            result = WebAssembly.instantiateStreaming(response, imports)
                                .catch(e => {{
                                    console.warn(\"`WebAssembly.instantiateStreaming` failed. Assuming this is \
                                                    because your server does not serve wasm with \
                                                    `application/wasm` MIME type. Falling back to \
                                                    `WebAssembly.instantiate` which is slower. Original \
                                                    error:\\n\", e);
                                    return response
                                        .then(r => r.arrayBuffer())
                                        .then(bytes => WebAssembly.instantiate(bytes, imports));
                                }});
                        }} else {{
                            result = response
                                .then(r => r.arrayBuffer())
                                .then(bytes => WebAssembly.instantiate(bytes, imports));
                        }}
                    }} else {{
                        {init_memory1}
                        result = WebAssembly.instantiate(module, imports)
                            .then(result => {{
                                if (result instanceof WebAssembly.Instance) {{
                                    return {{ instance: result, module }};
                                }} else {{
                                    return result;
                                }}
                            }});
                    }}
                    return result.then(({{instance, module}}) => {{
                        wasm = instance.exports;
                        init.__wbindgen_wasm_module = module;
                        {start}
                        return wasm;
                    }});
                }}
            ",
            init_memory_arg = init_memory_arg,
            module = module_name,
            init_memory1 = init_memory1,
            init_memory2 = init_memory2,
            start = if needs_manual_start {
                "wasm.__wbindgen_start();"
            } else {
                ""
            },
        );

        (js, ts)
    }

    fn bind(
        &mut self,
        name: &str,
        f: &Fn(&mut Self) -> Result<String, Error>,
    ) -> Result<(), Error> {
        if !self.wasm_import_needed(name) {
            return Ok(());
        }
        let contents = f(self)
            .with_context(|_| format!("failed to generate internal JS function `{}`", name))?;
        self.export(name, &contents, None);
        Ok(())
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

        let (mkweakref, freeref) = if self.config.weak_refs {
            // When weak refs are enabled we use them to automatically free the
            // contents of an exported rust class when it's gc'd. Note that a
            // manual `free` function still exists for deterministic
            // destruction.
            //
            // This is implemented by using a `WeakRefGroup` to run finalizers
            // for all `WeakRef` objects that it creates. Upon construction of
            // a new wasm object we use `makeRef` with "holdings" of a thunk to
            // free the wasm instance.  Once the `this` (the instance we're
            // creating) is gc'd then the finalizer will run with the
            // `WeakRef`, and we'll pull out the `holdings`, our pointer.
            //
            // Note, though, that if manual finalization happens we want to
            // cancel the `WeakRef`-generated finalization, so we retain the
            // `WeakRef` in a global map. This global map is then used to
            // `drop()` the `WeakRef` (cancel finalization) whenever it is
            // finalized.
            self.expose_cleanup_groups();
            let mk = format!("addCleanup(this, this.ptr, free{});", name);
            let free = "
                CLEANUPS_MAP.get(ptr).drop();
                CLEANUPS_MAP.delete(ptr);
            ";
            (mk, free)
        } else {
            (String::new(), "")
        };

        if self.config.debug && !class.has_constructor {
            dst.push_str(
                "
                    constructor() {
                        throw new Error('cannot invoke `new` directly');
                    }
                ",
            );
        }

        let mut wrap_needed = class.wrap_needed;
        let new_name = wasm_bindgen_shared::new_function(&name);
        if self.wasm_import_needed(&new_name) {
            wrap_needed = true;
            self.anyref
                .import_xform("__wbindgen_placeholder__", &new_name, &[], true);
            let expr = format!("{}.__wrap(ptr)", name);
            let expr = self.add_heap_object(&expr);
            let body = format!("function(ptr) {{ return {}; }}", expr);
            self.export(&new_name, &body, None);
        }

        if wrap_needed {
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
                mkweakref.replace("this", "obj"),
            ));
        }

        self.global(&format!(
            "
            function free{}(ptr) {{
                {}
                wasm.{}(ptr);
            }}
            ",
            name,
            freeref,
            wasm_bindgen_shared::free_function(&name)
        ));
        dst.push_str(&format!(
            "
            free() {{
                const ptr = this.ptr;
                this.ptr = 0;
                free{}(ptr);
            }}
            ",
            name,
        ));
        ts_dst.push_str("  free(): void;");
        dst.push_str(&class.contents);
        ts_dst.push_str(&class.typescript);
        dst.push_str("}\n");
        ts_dst.push_str("}\n");

        self.export(&name, &dst, Some(class.comments.clone()));
        self.typescript.push_str(&ts_dst);

        Ok(())
    }

    fn export_table(&mut self) -> Result<(), Error> {
        if !self.function_table_needed {
            return Ok(());
        }
        let id = match self.module.tables.main_function_table()? {
            Some(id) => id,
            None => bail!("no function table found in module"),
        };
        self.module.exports.add("__wbg_function_table", id);
        Ok(())
    }

    fn rewrite_imports(&mut self, module_name: &str) {
        for (name, contents) in self._rewrite_imports(module_name) {
            self.export(&name, &contents, None);
        }
    }

    fn _rewrite_imports(&mut self, module_name: &str) -> Vec<(String, String)> {
        let mut math_imports = Vec::new();
        for import in self.module.imports.iter_mut() {
            if import.module == "__wbindgen_placeholder__" {
                import.module.truncate(0);
                if let Some((module, name)) = self.direct_imports.get(import.name.as_str()) {
                    import.name.truncate(0);
                    import.module.push_str(module);
                    import.name.push_str(name);
                } else {
                    import.module.push_str("./");
                    import.module.push_str(module_name);
                }
                continue;
            }

            if import.module != "env" {
                continue;
            }

            // If memory is imported we'll have exported it from the shim module
            // so let's import it from there.
            //
            // TODO: we should track this is in a more first-class fashion
            // rather than just matching on strings.
            if import.name == "memory" {
                import.module.truncate(0);
                import.module.push_str("./");
                import.module.push_str(module_name);
                continue;
            }

            let renamed_import = format!("__wbindgen_{}", import.name);
            let mut bind_math = |expr: &str| {
                math_imports.push((renamed_import.clone(), format!("function{}", expr)));
            };

            // Note that since Rust 1.32.0 this is no longer necessary. Imports
            // of these functions were fixed in rust-lang/rust#54257 and we're
            // just waiting until pre-1.32.0 compilers are basically no longer
            // in use to remove this.
            match import.name.as_str() {
                "Math_acos" => bind_math("(x) { return Math.acos(x); }"),
                "Math_asin" => bind_math("(x) { return Math.asin(x); }"),
                "Math_atan" => bind_math("(x) { return Math.atan(x); }"),
                "Math_atan2" => bind_math("(x, y) { return Math.atan2(x, y); }"),
                "Math_cbrt" => bind_math("(x) { return Math.cbrt(x); }"),
                "Math_cosh" => bind_math("(x) { return Math.cosh(x); }"),
                "Math_expm1" => bind_math("(x) { return Math.expm1(x); }"),
                "Math_hypot" => bind_math("(x, y) { return Math.hypot(x, y); }"),
                "Math_log1p" => bind_math("(x) { return Math.log1p(x); }"),
                "Math_sinh" => bind_math("(x) { return Math.sinh(x); }"),
                "Math_tan" => bind_math("(x) { return Math.tan(x); }"),
                "Math_tanh" => bind_math("(x) { return Math.tanh(x); }"),
                _ => continue,
            }

            import.module.truncate(0);
            import.module.push_str("./");
            import.module.push_str(module_name);
            import.name = renamed_import.clone();
        }

        math_imports
    }

    fn unexport_unused_internal_exports(&mut self) {
        let mut to_remove = Vec::new();
        for export in self.module.exports.iter() {
            match export.name.as_str() {
                // These are some internal imports set by LLD but currently
                // we've got no use case for continuing to export them, so
                // blacklist them.
                "__heap_base" | "__data_end" | "__indirect_function_table" => {
                    to_remove.push(export.id());
                }

                // Otherwise only consider our special exports, which all start
                // with the same prefix which hopefully only we're using.
                n if n.starts_with("__wbindgen") => {
                    if !self.required_internal_exports.contains(n) {
                        to_remove.push(export.id());
                    }
                }
                _ => {}
            }
        }
        for id in to_remove {
            self.module.exports.delete(id);
        }
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
        assert!(!self.config.anyref);
        self.global(&format!("const heap = new Array({});", INITIAL_HEAP_OFFSET));
        self.global("heap.fill(undefined);");
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

    fn expose_pass_string_to_wasm(&mut self) -> Result<(), Error> {
        if !self.should_write_global("pass_string_to_wasm") {
            return Ok(());
        }
        self.require_internal_export("__wbindgen_malloc")?;
        self.expose_wasm_vector_len();
        let debug = if self.config.debug {
            "
                if (typeof(arg) !== 'string') throw new Error('expected a string argument');
            "
        } else {
            ""
        };

        // If we are targeting Node.js, it doesn't have `encodeInto` yet
        // but it does have `Buffer::write` which has similar semantics but
        // doesn't require creating intermediate view using `subarray`
        // and also has `Buffer::byteLength` to calculate size upfront.
        if self.config.mode.nodejs() {
            self.expose_node_buffer_memory();

            self.global(&format!(
                "
                    function passStringToWasm(arg) {{
                        {}
                        const size = Buffer.byteLength(arg);
                        const ptr = wasm.__wbindgen_malloc(size);
                        getNodeBufferMemory().write(arg, ptr, size);
                        WASM_VECTOR_LEN = size;
                        return ptr;
                    }}
                ",
                debug,
            ));

            return Ok(());
        }

        self.expose_text_encoder();
        self.expose_uint8_memory();

        // The first implementation we have for this is to use
        // `TextEncoder#encode` which has been around for quite some time.
        let use_encode = format!(
            "
                {}
                const buf = cachedTextEncoder.encode(arg);
                const ptr = wasm.__wbindgen_malloc(buf.length);
                getUint8Memory().set(buf, ptr);
                WASM_VECTOR_LEN = buf.length;
                return ptr;
            ",
            debug
        );

        // Another possibility is to use `TextEncoder#encodeInto` which is much
        // newer and isn't implemented everywhere yet. It's more efficient,
        // however, becaues it allows us to elide an intermediate allocation.
        let use_encode_into = format!(
            "
                {}
                let size = arg.length;
                let ptr = wasm.__wbindgen_malloc(size);
                let writeOffset = 0;
                while (true) {{
                    const view = getUint8Memory().subarray(ptr + writeOffset, ptr + size);
                    const {{ read, written }} = cachedTextEncoder.encodeInto(arg, view);
                    if (read === arg.length) {{
                        break;
                    }}
                    arg = arg.substring(read);
                    writeOffset += written;
                    ptr = wasm.__wbindgen_realloc(ptr, size, size += arg.length * 3);
                }}
                WASM_VECTOR_LEN = writeOffset;
                return ptr;
            ",
            debug
        );

        // Looks like `encodeInto` doesn't currently work when the memory passed
        // in is backed by a `SharedArrayBuffer`, so force usage of `encode` if
        // a `SharedArrayBuffer` is in use.
        let shared = self.module.memories.get(self.memory).shared;

        match self.config.encode_into {
            EncodeInto::Always if !shared => {
                self.require_internal_export("__wbindgen_realloc")?;
                self.global(&format!(
                    "function passStringToWasm(arg) {{ {} }}",
                    use_encode_into,
                ));
            }
            EncodeInto::Test if !shared => {
                self.require_internal_export("__wbindgen_realloc")?;
                self.global(&format!(
                    "
                        let passStringToWasm;
                        if (typeof cachedTextEncoder.encodeInto === 'function') {{
                            passStringToWasm = function(arg) {{ {} }};
                        }} else {{
                            passStringToWasm = function(arg) {{ {} }};
                        }}
                    ",
                    use_encode_into, use_encode,
                ));
            }
            _ => {
                self.global(&format!(
                    "function passStringToWasm(arg) {{ {} }}",
                    use_encode,
                ));
            }
        }
        Ok(())
    }

    fn expose_pass_array8_to_wasm(&mut self) -> Result<(), Error> {
        self.expose_uint8_memory();
        self.pass_array_to_wasm("passArray8ToWasm", "getUint8Memory", 1)
    }

    fn expose_pass_array16_to_wasm(&mut self) -> Result<(), Error> {
        self.expose_uint16_memory();
        self.pass_array_to_wasm("passArray16ToWasm", "getUint16Memory", 2)
    }

    fn expose_pass_array32_to_wasm(&mut self) -> Result<(), Error> {
        self.expose_uint32_memory();
        self.pass_array_to_wasm("passArray32ToWasm", "getUint32Memory", 4)
    }

    fn expose_pass_array64_to_wasm(&mut self) -> Result<(), Error> {
        self.expose_uint64_memory();
        self.pass_array_to_wasm("passArray64ToWasm", "getUint64Memory", 8)
    }

    fn expose_pass_array_f32_to_wasm(&mut self) -> Result<(), Error> {
        self.expose_f32_memory();
        self.pass_array_to_wasm("passArrayF32ToWasm", "getFloat32Memory", 4)
    }

    fn expose_pass_array_f64_to_wasm(&mut self) -> Result<(), Error> {
        self.expose_f64_memory();
        self.pass_array_to_wasm("passArrayF64ToWasm", "getFloat64Memory", 8)
    }

    fn expose_pass_array_jsvalue_to_wasm(&mut self) -> Result<(), Error> {
        if !self.should_write_global("pass_array_jsvalue") {
            return Ok(());
        }
        self.require_internal_export("__wbindgen_malloc")?;
        self.expose_uint32_memory();
        if self.config.anyref {
            // TODO: using `addToAnyrefTable` goes back and forth between wasm
            // and JS a lot, we should have a bulk operation for this.
            self.expose_add_to_anyref_table()?;
            self.global(
                "
                function passArrayJsValueToWasm(array) {
                    const ptr = wasm.__wbindgen_malloc(array.length * 4);
                    const mem = getUint32Memory();
                    for (let i = 0; i < array.length; i++) {
                        mem[ptr / 4 + i] = addToAnyrefTable(array[i]);
                    }
                    WASM_VECTOR_LEN = array.length;
                    return ptr;
                }
            ",
            );
        } else {
            self.expose_add_heap_object();
            self.global(
                "
                function passArrayJsValueToWasm(array) {
                    const ptr = wasm.__wbindgen_malloc(array.length * 4);
                    const mem = getUint32Memory();
                    for (let i = 0; i < array.length; i++) {
                        mem[ptr / 4 + i] = addHeapObject(array[i]);
                    }
                    WASM_VECTOR_LEN = array.length;
                    return ptr;
                }

            ",
            );
        }
        Ok(())
    }

    fn pass_array_to_wasm(
        &mut self,
        name: &'static str,
        delegate: &str,
        size: usize,
    ) -> Result<(), Error> {
        if !self.should_write_global(name) {
            return Ok(());
        }
        self.require_internal_export("__wbindgen_malloc")?;
        self.expose_wasm_vector_len();
        self.global(&format!(
            "
            function {}(arg) {{
                const ptr = wasm.__wbindgen_malloc(arg.length * {size});
                {}().set(arg, ptr / {size});
                WASM_VECTOR_LEN = arg.length;
                return ptr;
            }}
            ",
            name,
            delegate,
            size = size
        ));
        Ok(())
    }

    fn expose_text_encoder(&mut self) {
        if !self.should_write_global("text_encoder") {
            return;
        }
        self.expose_text_processor("TextEncoder");
    }

    fn expose_text_decoder(&mut self) {
        if !self.should_write_global("text_decoder") {
            return;
        }
        self.expose_text_processor("TextDecoder");
    }

    fn expose_text_processor(&mut self, s: &str) {
        if self.config.mode.nodejs_experimental_modules() {
            self.imports
                .push_str(&format!("import {{ {} }} from 'util';\n", s));
            self.global(&format!("let cached{0} = new {0}('utf-8');", s));
        } else if self.config.mode.nodejs() {
            self.global(&format!("const {0} = require('util').{0};", s));
            self.global(&format!("let cached{0} = new {0}('utf-8');", s));
        } else if !self.config.mode.always_run_in_browser() {
            self.global(&format!(
                "
                    const l{0} = typeof {0} === 'undefined' ? \
                        require('util').{0} : {0};\
                ",
                s
            ));
            self.global(&format!("let cached{0} = new l{0}('utf-8');", s));
        } else {
            self.global(&format!("let cached{0} = new {0}('utf-8');", s));
        }
    }

    fn expose_get_string_from_wasm(&mut self) {
        if !self.should_write_global("get_string_from_wasm") {
            return;
        }
        self.expose_text_decoder();
        self.expose_uint8_memory();

        // Typically we try to give a raw view of memory out to `TextDecoder` to
        // avoid copying too much data. If, however, a `SharedArrayBuffer` is
        // being used it looks like that is rejected by `TextDecoder` or
        // otherwise doesn't work with it. When we detect a shared situation we
        // use `slice` which creates a new array instead of `subarray` which
        // creates just a view. That way in shared mode we copy more data but in
        // non-shared mode there's no need to copy the data except for the
        // string itself.
        let is_shared = self.module.memories.get(self.memory).shared;
        let method = if is_shared { "slice" } else { "subarray" };

        self.global(&format!(
            "
            function getStringFromWasm(ptr, len) {{
                return cachedTextDecoder.decode(getUint8Memory().{}(ptr, ptr + len));
            }}
        ",
            method
        ));
    }

    fn expose_get_array_js_value_from_wasm(&mut self) -> Result<(), Error> {
        if !self.should_write_global("get_array_js_value_from_wasm") {
            return Ok(());
        }
        self.expose_uint32_memory();
        if self.config.anyref {
            self.expose_anyref_table();
            self.global(
                "
                function getArrayJsValueFromWasm(ptr, len) {
                    const mem = getUint32Memory();
                    const slice = mem.subarray(ptr / 4, ptr / 4 + len);
                    const result = [];
                    for (let i = 0; i < slice.length; i++) {
                        result.push(wasm.__wbg_anyref_table.get(slice[i]));
                    }
                    wasm.__wbindgen_drop_anyref_slice(ptr, len);
                    return result;
                }
                ",
            );
            self.require_internal_export("__wbindgen_drop_anyref_slice")?;
        } else {
            self.expose_take_object();
            self.global(
                "
                function getArrayJsValueFromWasm(ptr, len) {
                    const mem = getUint32Memory();
                    const slice = mem.subarray(ptr / 4, ptr / 4 + len);
                    const result = [];
                    for (let i = 0; i < slice.length; i++) {
                        result.push(takeObject(slice[i]));
                    }
                    return result;
                }
                ",
            );
        }
        Ok(())
    }

    fn expose_get_array_i8_from_wasm(&mut self) {
        self.expose_int8_memory();
        self.arrayget("getArrayI8FromWasm", "getInt8Memory", 1);
    }

    fn expose_get_array_u8_from_wasm(&mut self) {
        self.expose_uint8_memory();
        self.arrayget("getArrayU8FromWasm", "getUint8Memory", 1);
    }

    fn expose_get_clamped_array_u8_from_wasm(&mut self) {
        self.expose_clamped_uint8_memory();
        self.arrayget("getClampedArrayU8FromWasm", "getUint8ClampedMemory", 1);
    }

    fn expose_get_array_i16_from_wasm(&mut self) {
        self.expose_int16_memory();
        self.arrayget("getArrayI16FromWasm", "getInt16Memory", 2);
    }

    fn expose_get_array_u16_from_wasm(&mut self) {
        self.expose_uint16_memory();
        self.arrayget("getArrayU16FromWasm", "getUint16Memory", 2);
    }

    fn expose_get_array_i32_from_wasm(&mut self) {
        self.expose_int32_memory();
        self.arrayget("getArrayI32FromWasm", "getInt32Memory", 4);
    }

    fn expose_get_array_u32_from_wasm(&mut self) {
        self.expose_uint32_memory();
        self.arrayget("getArrayU32FromWasm", "getUint32Memory", 4);
    }

    fn expose_get_array_i64_from_wasm(&mut self) {
        self.expose_int64_memory();
        self.arrayget("getArrayI64FromWasm", "getInt64Memory", 8);
    }

    fn expose_get_array_u64_from_wasm(&mut self) {
        self.expose_uint64_memory();
        self.arrayget("getArrayU64FromWasm", "getUint64Memory", 8);
    }

    fn expose_get_array_f32_from_wasm(&mut self) {
        self.expose_f32_memory();
        self.arrayget("getArrayF32FromWasm", "getFloat32Memory", 4);
    }

    fn expose_get_array_f64_from_wasm(&mut self) {
        self.expose_f64_memory();
        self.arrayget("getArrayF64FromWasm", "getFloat64Memory", 8);
    }

    fn arrayget(&mut self, name: &'static str, mem: &'static str, size: usize) {
        if !self.should_write_global(name) {
            return;
        }
        self.global(&format!(
            "
            function {name}(ptr, len) {{
                return {mem}().subarray(ptr / {size}, ptr / {size} + len);
            }}
            ",
            name = name,
            mem = mem,
            size = size,
        ));
    }

    fn expose_node_buffer_memory(&mut self) {
        self.memview("getNodeBufferMemory", "Buffer.from");
    }

    fn expose_int8_memory(&mut self) {
        self.memview("getInt8Memory", "new Int8Array");
    }

    fn expose_uint8_memory(&mut self) {
        self.memview("getUint8Memory", "new Uint8Array");
    }

    fn expose_clamped_uint8_memory(&mut self) {
        self.memview("getUint8ClampedMemory", "new Uint8ClampedArray");
    }

    fn expose_int16_memory(&mut self) {
        self.memview("getInt16Memory", "new Int16Array");
    }

    fn expose_uint16_memory(&mut self) {
        self.memview("getUint16Memory", "new Uint16Array");
    }

    fn expose_int32_memory(&mut self) {
        self.memview("getInt32Memory", "new Int32Array");
    }

    fn expose_uint32_memory(&mut self) {
        self.memview("getUint32Memory", "new Uint32Array");
    }

    fn expose_int64_memory(&mut self) {
        self.memview("getInt64Memory", "new BigInt64Array");
    }

    fn expose_uint64_memory(&mut self) {
        self.memview("getUint64Memory", "new BigUint64Array");
    }

    fn expose_f32_memory(&mut self) {
        self.memview("getFloat32Memory", "new Float32Array");
    }

    fn expose_f64_memory(&mut self) {
        self.memview("getFloat64Memory", "new Float64Array");
    }

    fn memview_function(&mut self, t: VectorKind) -> &'static str {
        match t {
            VectorKind::String => {
                self.expose_uint8_memory();
                "getUint8Memory"
            }
            VectorKind::I8 => {
                self.expose_int8_memory();
                "getInt8Memory"
            }
            VectorKind::U8 => {
                self.expose_uint8_memory();
                "getUint8Memory"
            }
            VectorKind::ClampedU8 => {
                self.expose_clamped_uint8_memory();
                "getUint8ClampedMemory"
            }
            VectorKind::I16 => {
                self.expose_int16_memory();
                "getInt16Memory"
            }
            VectorKind::U16 => {
                self.expose_uint16_memory();
                "getUint16Memory"
            }
            VectorKind::I32 => {
                self.expose_int32_memory();
                "getInt32Memory"
            }
            VectorKind::U32 => {
                self.expose_uint32_memory();
                "getUint32Memory"
            }
            VectorKind::I64 => {
                self.expose_int64_memory();
                "getInt64Memory"
            }
            VectorKind::U64 => {
                self.expose_uint64_memory();
                "getUint64Memory"
            }
            VectorKind::F32 => {
                self.expose_f32_memory();
                "getFloat32Memory"
            }
            VectorKind::F64 => {
                self.expose_f64_memory();
                "getFloat64Memory"
            }
            VectorKind::Anyref => {
                self.expose_uint32_memory();
                "getUint32Memory"
            }
        }
    }

    fn memview(&mut self, name: &'static str, js: &str) {
        if !self.should_write_global(name) {
            return;
        }
        let mem = self.memory();
        self.global(&format!(
            "
            let cache{name} = null;
            function {name}() {{
                if (cache{name} === null || cache{name}.buffer !== {mem}.buffer) {{
                    cache{name} = {js}({mem}.buffer);
                }}
                return cache{name};
            }}
            ",
            name = name,
            js = js,
            mem = mem,
        ));
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
        self.expose_uint32_memory();
        if self.config.anyref {
            self.expose_add_to_anyref_table()?;
            self.global(
                "
                function handleError(exnptr, e) {
                    const idx = addToAnyrefTable(e);
                    const view = getUint32Memory();
                    view[exnptr / 4] = 1;
                    view[exnptr / 4 + 1] = idx;
                }
                ",
            );
        } else {
            self.expose_add_heap_object();
            self.global(
                "
                function handleError(exnptr, e) {
                    const view = getUint32Memory();
                    view[exnptr / 4] = 1;
                    view[exnptr / 4 + 1] = addHeapObject(e);
                }
                ",
            );
        }
        Ok(())
    }

    fn wasm_import_needed(&self, name: &str) -> bool {
        self.module
            .imports
            .iter()
            .any(|i| i.module == "__wbindgen_placeholder__" && i.name == name)
    }

    fn pass_to_wasm_function(&mut self, t: VectorKind) -> Result<&'static str, Error> {
        let s = match t {
            VectorKind::String => {
                self.expose_pass_string_to_wasm()?;
                "passStringToWasm"
            }
            VectorKind::I8 | VectorKind::U8 | VectorKind::ClampedU8 => {
                self.expose_pass_array8_to_wasm()?;
                "passArray8ToWasm"
            }
            VectorKind::U16 | VectorKind::I16 => {
                self.expose_pass_array16_to_wasm()?;
                "passArray16ToWasm"
            }
            VectorKind::I32 | VectorKind::U32 => {
                self.expose_pass_array32_to_wasm()?;
                "passArray32ToWasm"
            }
            VectorKind::I64 | VectorKind::U64 => {
                self.expose_pass_array64_to_wasm()?;
                "passArray64ToWasm"
            }
            VectorKind::F32 => {
                self.expose_pass_array_f32_to_wasm()?;
                "passArrayF32ToWasm"
            }
            VectorKind::F64 => {
                self.expose_pass_array_f64_to_wasm()?;
                "passArrayF64ToWasm"
            }
            VectorKind::Anyref => {
                self.expose_pass_array_jsvalue_to_wasm()?;
                "passArrayJsValueToWasm"
            }
        };
        Ok(s)
    }

    fn expose_get_vector_from_wasm(&mut self, ty: VectorKind) -> Result<&'static str, Error> {
        Ok(match ty {
            VectorKind::String => {
                self.expose_get_string_from_wasm();
                "getStringFromWasm"
            }
            VectorKind::I8 => {
                self.expose_get_array_i8_from_wasm();
                "getArrayI8FromWasm"
            }
            VectorKind::U8 => {
                self.expose_get_array_u8_from_wasm();
                "getArrayU8FromWasm"
            }
            VectorKind::ClampedU8 => {
                self.expose_get_clamped_array_u8_from_wasm();
                "getClampedArrayU8FromWasm"
            }
            VectorKind::I16 => {
                self.expose_get_array_i16_from_wasm();
                "getArrayI16FromWasm"
            }
            VectorKind::U16 => {
                self.expose_get_array_u16_from_wasm();
                "getArrayU16FromWasm"
            }
            VectorKind::I32 => {
                self.expose_get_array_i32_from_wasm();
                "getArrayI32FromWasm"
            }
            VectorKind::U32 => {
                self.expose_get_array_u32_from_wasm();
                "getArrayU32FromWasm"
            }
            VectorKind::I64 => {
                self.expose_get_array_i64_from_wasm();
                "getArrayI64FromWasm"
            }
            VectorKind::U64 => {
                self.expose_get_array_u64_from_wasm();
                "getArrayU64FromWasm"
            }
            VectorKind::F32 => {
                self.expose_get_array_f32_from_wasm();
                "getArrayF32FromWasm"
            }
            VectorKind::F64 => {
                self.expose_get_array_f64_from_wasm();
                "getArrayF64FromWasm"
            }
            VectorKind::Anyref => {
                self.expose_get_array_js_value_from_wasm()?;
                "getArrayJsValueFromWasm"
            }
        })
    }

    fn expose_global_argument_ptr(&mut self) -> Result<(), Error> {
        if !self.should_write_global("global_argument_ptr") {
            return Ok(());
        }
        self.require_internal_export("__wbindgen_global_argument_ptr")?;
        self.global(
            "
            let cachedGlobalArgumentPtr = null;
            function globalArgumentPtr() {
                if (cachedGlobalArgumentPtr === null) {
                    cachedGlobalArgumentPtr = wasm.__wbindgen_global_argument_ptr();
                }
                return cachedGlobalArgumentPtr;
            }
            ",
        );
        Ok(())
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
              return {}
            }
            ",
        );
    }

    fn expose_u32_cvt_shim(&mut self) -> &'static str {
        let name = "u32CvtShim";
        if !self.should_write_global(name) {
            return name;
        }
        self.global(&format!("const {} = new Uint32Array(2);", name));
        name
    }

    fn expose_int64_cvt_shim(&mut self) -> &'static str {
        let name = "int64CvtShim";
        if !self.should_write_global(name) {
            return name;
        }
        let n = self.expose_u32_cvt_shim();
        self.global(&format!(
            "const {} = new BigInt64Array({}.buffer);",
            name, n
        ));
        name
    }

    fn expose_uint64_cvt_shim(&mut self) -> &'static str {
        let name = "uint64CvtShim";
        if !self.should_write_global(name) {
            return name;
        }
        let n = self.expose_u32_cvt_shim();
        self.global(&format!(
            "const {} = new BigUint64Array({}.buffer);",
            name, n
        ));
        name
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

    fn expose_cleanup_groups(&mut self) {
        if !self.should_write_global("cleanup_groups") {
            return;
        }
        self.global(
            "
                const CLEANUPS = new WeakRefGroup(x => x.holdings());
                const CLEANUPS_MAP = new Map();

                function addCleanup(obj, ptr, free) {
                    const ref = CLEANUPS.makeRef(obj, () => free(ptr));
                    CLEANUPS_MAP.set(ptr, ref);
                }
            ",
        );
    }

    fn describe(&mut self, name: &str) -> Option<Descriptor> {
        let name = format!("__wbindgen_describe_{}", name);
        let descriptor = self.interpreter.interpret_descriptor(&name, self.module)?;
        Some(Descriptor::decode(descriptor))
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

    fn use_node_require(&self) -> bool {
        self.config.mode.nodejs() && !self.config.mode.nodejs_experimental_modules()
    }

    fn memory(&mut self) -> &'static str {
        if self.module.memories.get(self.memory).import.is_some() {
            "memory"
        } else {
            "wasm.memory"
        }
    }

    fn require_class_wrap(&mut self, class: &str) {
        self.exported_classes
            .as_mut()
            .expect("classes already written")
            .entry(class.to_string())
            .or_insert_with(ExportedClass::default)
            .wrap_needed = true;
    }

    fn import_identifier(&mut self, import: Import<'a>) -> String {
        // Here's where it's a bit tricky. We need to make sure that importing
        // the same identifier from two different modules works, and they're
        // named uniquely below. Additionally if we've already imported the same
        // identifier from the module in question then we'd like to reuse the
        // one that was previously imported.
        //
        // Our `imported_names` map keeps track of all imported identifiers from
        // modules, mapping the imported names onto names actually available for
        // use in our own module. If our identifier isn't present then we
        // generate a new identifier and are sure to generate the appropriate JS
        // import for our new identifier.
        let use_node_require = self.use_node_require();
        let imported_identifiers = &mut self.imported_identifiers;
        let imports = &mut self.imports;
        let imports_post = &mut self.imports_post;
        let identifier = self
            .imported_names
            .entry(import.module())
            .or_insert_with(Default::default)
            .entry(import.name())
            .or_insert_with(|| {
                let name = generate_identifier(import.name(), imported_identifiers);
                match &import {
                    Import::Module { .. }
                    | Import::LocalModule { .. }
                    | Import::InlineJs { .. } => {
                        // When doing a modular import local snippets (either
                        // inline or not) are routed to a local `./snippets`
                        // directory which the rest of `wasm-bindgen` will fill
                        // in.
                        let path = match import {
                            Import::Module { module, .. } => module.to_string(),
                            Import::LocalModule { module, .. } => format!("./snippets/{}", module),
                            Import::InlineJs {
                                unique_crate_identifier,
                                snippet_idx_in_crate,
                                ..
                            } => format!(
                                "./snippets/{}/inline{}.js",
                                unique_crate_identifier, snippet_idx_in_crate
                            ),
                            _ => unreachable!(),
                        };
                        if use_node_require {
                            imports.push_str(&format!(
                                "const {} = require(String.raw`{}`).{};\n",
                                name,
                                path,
                                import.name()
                            ));
                        } else if import.name() == name {
                            imports.push_str(&format!("import {{ {} }} from '{}';\n", name, path));
                        } else {
                            imports.push_str(&format!(
                                "import {{ {} as {} }} from '{}';\n",
                                import.name(),
                                name,
                                path
                            ));
                        }
                        name
                    }

                    Import::VendorPrefixed { prefixes, .. } => {
                        imports_post.push_str("const l");
                        imports_post.push_str(&name);
                        imports_post.push_str(" = ");
                        switch(imports_post, &name, "", prefixes);
                        imports_post.push_str(";\n");

                        fn switch(dst: &mut String, name: &str, prefix: &str, left: &[&str]) {
                            if left.len() == 0 {
                                dst.push_str(prefix);
                                return dst.push_str(name);
                            }
                            dst.push_str("(typeof ");
                            dst.push_str(prefix);
                            dst.push_str(name);
                            dst.push_str(" == 'undefined' ? ");
                            match left.len() {
                                1 => {
                                    dst.push_str(&left[0]);
                                    dst.push_str(name);
                                }
                                _ => switch(dst, name, &left[0], &left[1..]),
                            }
                            dst.push_str(" : ");
                            dst.push_str(prefix);
                            dst.push_str(name);
                            dst.push_str(")");
                        }
                        format!("l{}", name)
                    }

                    Import::Global { .. } => name,
                }
            });

        // If there's a namespace we didn't actually import `item` but rather
        // the namespace, so access through that.
        match import.field() {
            Some(field) => format!("{}.{}", identifier, field),
            None => identifier.clone(),
        }
    }

    fn generated_import_target(
        &mut self,
        name: Import<'a>,
        import: &decode::ImportFunction<'a>,
    ) -> Result<ImportTarget, Error> {
        let method_data = match &import.method {
            Some(data) => data,
            None => {
                let name = self.import_identifier(name);
                if import.structural || !name.contains(".") {
                    return Ok(ImportTarget::Function(name));
                }
                self.global(&format!("const {}_target = {};", import.shim, name));
                let target = format!("{}_target", import.shim);
                return Ok(ImportTarget::Function(target));
            }
        };

        let class = self.import_identifier(name);
        let op = match &method_data.kind {
            decode::MethodKind::Constructor => {
                return Ok(ImportTarget::Constructor(class.to_string()));
            }
            decode::MethodKind::Operation(op) => op,
        };
        if import.structural {
            let class = if op.is_static {
                Some(class.clone())
            } else {
                None
            };

            return Ok(match &op.kind {
                decode::OperationKind::Regular => {
                    let name = import.function.name.to_string();
                    match class {
                        Some(c) => ImportTarget::Function(format!("{}.{}", c, name)),
                        None => ImportTarget::StructuralMethod(name),
                    }
                }
                decode::OperationKind::Getter(g) => {
                    ImportTarget::StructuralGetter(class, g.to_string())
                }
                decode::OperationKind::Setter(s) => {
                    ImportTarget::StructuralSetter(class, s.to_string())
                }
                decode::OperationKind::IndexingGetter => {
                    ImportTarget::StructuralIndexingGetter(class)
                }
                decode::OperationKind::IndexingSetter => {
                    ImportTarget::StructuralIndexingSetter(class)
                }
                decode::OperationKind::IndexingDeleter => {
                    ImportTarget::StructuralIndexingDeleter(class)
                }
            });
        }

        let target = format!(
            "typeof {0} === 'undefined' ? null : {}{}",
            class,
            if op.is_static { "" } else { ".prototype" }
        );
        let (mut target, name) = match &op.kind {
            decode::OperationKind::Regular => (
                format!("{}.{}", target, import.function.name),
                &import.function.name,
            ),
            decode::OperationKind::Getter(g) => {
                self.expose_get_inherited_descriptor();
                (
                    format!(
                        "GetOwnOrInheritedPropertyDescriptor({}, '{}').get",
                        target, g,
                    ),
                    g,
                )
            }
            decode::OperationKind::Setter(s) => {
                self.expose_get_inherited_descriptor();
                (
                    format!(
                        "GetOwnOrInheritedPropertyDescriptor({}, '{}').set",
                        target, s,
                    ),
                    s,
                )
            }
            decode::OperationKind::IndexingGetter => panic!("indexing getter should be structural"),
            decode::OperationKind::IndexingSetter => panic!("indexing setter should be structural"),
            decode::OperationKind::IndexingDeleter => {
                panic!("indexing deleter should be structural")
            }
        };
        target.push_str(&format!(
            " || function() {{
            throw new Error(`wasm-bindgen: {}.{} does not exist`);
        }}",
            class, name
        ));
        if op.is_static {
            target.insert(0, '(');
            target.push_str(").bind(");
            target.push_str(&class);
            target.push_str(")");
        }

        self.global(&format!("const {}_target = {};", import.shim, target));
        Ok(if op.is_static {
            ImportTarget::Function(format!("{}_target", import.shim))
        } else {
            ImportTarget::Method(format!("{}_target", import.shim))
        })
    }

    /// Update the wasm file's `producers` section to include information about
    /// the wasm-bindgen tool.
    ///
    /// Specified at:
    /// https://github.com/WebAssembly/tool-conventions/blob/master/ProducersSection.md
    fn update_producers_section(&mut self) {
        self.module
            .producers
            .add_processed_by("wasm-bindgen", &wasm_bindgen_shared::version());
    }

    fn add_start_function(&mut self) -> Result<(), Error> {
        let start = match &self.start {
            Some(name) => name.clone(),
            None => return Ok(()),
        };
        let export = match self.module.exports.iter().find(|e| e.name == start) {
            Some(export) => export,
            None => bail!("export `{}` not found", start),
        };
        let id = match export.item {
            walrus::ExportItem::Function(i) => i,
            _ => bail!("export `{}` wasn't a function", start),
        };

        let prev_start = match self.module.start {
            Some(f) => f,
            None => {
                self.module.start = Some(id);
                return Ok(());
            }
        };

        // Note that we call the previous start function, if any, first. This is
        // because the start function currently only shows up when it's injected
        // through thread/anyref transforms. These injected start functions need
        // to happen before user code, so we always schedule them first.
        let mut builder = walrus::FunctionBuilder::new();
        let call1 = builder.call(prev_start, Box::new([]));
        let call2 = builder.call(id, Box::new([]));
        let ty = self.module.funcs.get(id).ty();
        let new_start = builder.finish(ty, Vec::new(), vec![call1, call2], self.module);
        self.module.start = Some(new_start);
        Ok(())
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

    fn expose_anyref_table(&mut self) {
        assert!(self.config.anyref);
        if !self.should_write_global("anyref_table") {
            return;
        }
        self.module
            .exports
            .add("__wbg_anyref_table", self.anyref.anyref_table_id());
    }

    fn expose_add_to_anyref_table(&mut self) -> Result<(), Error> {
        assert!(self.config.anyref);
        if !self.should_write_global("add_to_anyref_table") {
            return Ok(());
        }
        self.expose_anyref_table();
        self.require_internal_export("__wbindgen_anyref_table_alloc")?;
        self.global(
            "
                function addToAnyrefTable(obj) {
                    const idx = wasm.__wbindgen_anyref_table_alloc();
                    wasm.__wbg_anyref_table.set(idx, obj);
                    return idx;
                }
            ",
        );

        Ok(())
    }

    fn add_heap_object(&mut self, expr: &str) -> String {
        if self.config.anyref {
            expr.to_string()
        } else {
            self.expose_add_heap_object();
            format!("addHeapObject({})", expr)
        }
    }

    fn take_object(&mut self, expr: &str) -> String {
        if self.config.anyref {
            expr.to_string()
        } else {
            self.expose_take_object();
            format!("takeObject({})", expr)
        }
    }

    fn get_object(&mut self, expr: &str) -> String {
        if self.config.anyref {
            expr.to_string()
        } else {
            self.expose_get_object();
            format!("getObject({})", expr)
        }
    }
}

impl<'a, 'b> SubContext<'a, 'b> {
    pub fn generate(&mut self) -> Result<(), Error> {
        for m in self.program.local_modules.iter() {
            // All local modules we find should be unique, but the same module
            // may have showed up in a few different blocks. If that's the case
            // all the same identifiers should have the same contents.
            if let Some(prev) = self.cx.local_modules.insert(m.identifier, m.contents) {
                assert_eq!(prev, m.contents);
            }
        }
        for f in self.program.exports.iter() {
            self.generate_export(f).with_context(|_| {
                format!(
                    "failed to generate bindings for Rust export `{}`",
                    f.function.name
                )
            })?;
        }
        for f in self.program.imports.iter() {
            if let decode::ImportKind::Type(ty) = &f.kind {
                self.register_vendor_prefix(ty);
            }
        }
        for f in self.program.imports.iter() {
            self.generate_import(f)?;
        }
        for e in self.program.enums.iter() {
            self.generate_enum(e);
        }
        for s in self.program.structs.iter() {
            self.generate_struct(s).with_context(|_| {
                format!("failed to generate bindings for Rust struct `{}`", s.name,)
            })?;
        }
        for s in self.program.typescript_custom_sections.iter() {
            self.cx.typescript.push_str(s);
            self.cx.typescript.push_str("\n\n");
        }

        if let Some(path) = self.program.package_json {
            self.add_package_json(path)?;
        }

        Ok(())
    }

    fn generate_export(&mut self, export: &decode::Export<'b>) -> Result<(), Error> {
        if let Some(ref class) = export.class {
            assert!(!export.start);
            return self.generate_export_for_class(class, export);
        }

        let descriptor = match self.cx.describe(&export.function.name) {
            None => return Ok(()),
            Some(d) => d,
        };

        if export.start {
            self.set_start_function(export.function.name)?;
        }

        let (js, ts, js_doc) = Js2Rust::new(&export.function.name, self.cx)
            .process(descriptor.unwrap_function(), &export.function.arg_names)?
            .finish(
                "function",
                &format!("wasm.{}", export.function.name),
                ExportedShim::Named(&export.function.name),
            );
        self.cx.export(
            &export.function.name,
            &js,
            Some(format_doc_comments(&export.comments, Some(js_doc))),
        );
        self.cx.globals.push_str("\n");
        self.cx.typescript.push_str("export ");
        self.cx.typescript.push_str(&ts);
        self.cx.typescript.push_str("\n");
        Ok(())
    }

    fn set_start_function(&mut self, start: &str) -> Result<(), Error> {
        if let Some(prev) = &self.cx.start {
            bail!(
                "cannot flag `{}` as start function as `{}` is \
                 already the start function",
                start,
                prev
            );
        }
        self.cx.start = Some(start.to_string());
        Ok(())
    }

    fn generate_export_for_class(
        &mut self,
        class_name: &'b str,
        export: &decode::Export,
    ) -> Result<(), Error> {
        let wasm_name =
            wasm_bindgen_shared::struct_function_export_name(class_name, &export.function.name);

        let descriptor = match self.cx.describe(&wasm_name) {
            None => return Ok(()),
            Some(d) => d,
        };

        let function_name = if export.is_constructor {
            "constructor"
        } else {
            &export.function.name
        };
        let (js, ts, js_doc) = Js2Rust::new(function_name, self.cx)
            .method(export.method, export.consumed)
            .constructor(if export.is_constructor {
                Some(class_name)
            } else {
                None
            })
            .process(descriptor.unwrap_function(), &export.function.arg_names)?
            .finish(
                "",
                &format!("wasm.{}", wasm_name),
                ExportedShim::Named(&wasm_name),
            );

        let class = self
            .cx
            .exported_classes
            .as_mut()
            .expect("classes already written")
            .entry(class_name.to_string())
            .or_insert(ExportedClass::default());
        class
            .contents
            .push_str(&format_doc_comments(&export.comments, Some(js_doc)));

        class.typescript.push_str("  "); // Indentation

        if export.is_constructor {
            if class.has_constructor {
                bail!("found duplicate constructor `{}`", export.function.name);
            }
            class.has_constructor = true;
        } else if !export.method {
            class.contents.push_str("static ");
            class.typescript.push_str("static ");
        }

        class.contents.push_str(function_name);
        class.contents.push_str(&js);
        class.contents.push_str("\n");
        class.typescript.push_str(&ts);
        class.typescript.push_str("\n");
        Ok(())
    }

    fn generate_import(&mut self, import: &decode::Import<'b>) -> Result<(), Error> {
        match import.kind {
            decode::ImportKind::Function(ref f) => {
                self.generate_import_function(import, f).with_context(|_| {
                    format!(
                        "failed to generate bindings for JS import `{}`",
                        f.function.name
                    )
                })?;
            }
            decode::ImportKind::Static(ref s) => {
                self.generate_import_static(import, s).with_context(|_| {
                    format!("failed to generate bindings for JS import `{}`", s.name)
                })?;
            }
            decode::ImportKind::Type(ref ty) => {
                self.generate_import_type(import, ty).with_context(|_| {
                    format!("failed to generate bindings for JS import `{}`", ty.name,)
                })?;
            }
            decode::ImportKind::Enum(_) => {}
        }
        Ok(())
    }

    fn generate_import_static(
        &mut self,
        info: &decode::Import<'b>,
        import: &decode::ImportStatic<'b>,
    ) -> Result<(), Error> {
        // The same static can be imported in multiple locations, so only
        // generate bindings once for it.
        if !self.cx.imported_statics.insert(import.shim) {
            return Ok(());
        }

        // TODO: should support more types to import here
        let obj = self.import_name(info, &import.name)?;
        self.cx
            .anyref
            .import_xform("__wbindgen_placeholder__", &import.shim, &[], true);
        let body = format!("function() {{ return {}; }}", self.cx.add_heap_object(&obj));
        self.cx.export(&import.shim, &body, None);
        Ok(())
    }

    fn generate_import_function(
        &mut self,
        info: &decode::Import<'b>,
        import: &decode::ImportFunction<'b>,
    ) -> Result<(), Error> {
        if !self.cx.wasm_import_needed(&import.shim) {
            return Ok(());
        }

        // It's possible for the same function to be imported in two locations,
        // but we only want to generate one.
        if !self.cx.imported_functions.insert(import.shim) {
            return Ok(());
        }

        let descriptor = match self.cx.describe(&import.shim) {
            None => return Ok(()),
            Some(d) => d,
        };

        // Figure out the name that we're importing to dangle further references
        // off of. This is the function name if there's no method all here, or
        // the class if there's a method call.
        let name = match &import.method {
            Some(data) => self.determine_import(info, &data.class)?,
            None => self.determine_import(info, &import.function.name)?,
        };

        // Build up our shim's state, and we'll use that to guide whether we
        // actually emit an import here or not.
        let mut shim = Rust2Js::new(self.cx);
        if shim.cx.config.debug {
            shim.catch_and_rethrow(true);
        }
        shim.catch(import.catch)
            .variadic(import.variadic)
            .process(descriptor.unwrap_function())?;

        // If this is a bare function import and the shim doesn't actually do
        // anything (all argument/return conversions are noops) then we can wire
        // up the wasm import directly to the destination. We don't actually
        // wire up anything here, but we record it to get wired up later.
        if import.method.is_none() && shim.is_noop() {
            if let Import::Module {
                module,
                name,
                field: None,
            } = name
            {
                shim.cx.direct_imports.insert(import.shim, (module, name));

                if shim.ret_anyref || shim.anyref_args.len() > 0 {
                    shim.cx.anyref.import_xform(
                        "__wbindgen_placeholder__",
                        &import.shim,
                        &shim.anyref_args,
                        shim.ret_anyref,
                    );
                }
                return Ok(());
            }
        }

        // If the above optimization fails then we actually generate the import
        // here (possibly emitting some glue in our JS module) and then emit the
        // shim as the wasm will be importing the shim.
        let target = shim.cx.generated_import_target(name, import)?;
        let js = shim.finish(&target, &import.shim)?;
        shim.cx.export(&import.shim, &js, None);
        Ok(())
    }

    fn generate_import_type(
        &mut self,
        info: &decode::Import<'b>,
        import: &decode::ImportType<'b>,
    ) -> Result<(), Error> {
        if !self.cx.wasm_import_needed(&import.instanceof_shim) {
            return Ok(());
        }
        let name = self.import_name(info, &import.name)?;
        self.cx.anyref.import_xform(
            "__wbindgen_placeholder__",
            &import.instanceof_shim,
            &[(0, false)],
            false,
        );
        let body = format!(
            "function(idx) {{ return {} instanceof {} ? 1 : 0; }}",
            self.cx.get_object("idx"),
            name
        );
        self.cx.export(&import.instanceof_shim, &body, None);
        Ok(())
    }

    fn generate_enum(&mut self, enum_: &decode::Enum) {
        let mut variants = String::new();

        for variant in enum_.variants.iter() {
            variants.push_str(&format!("{}:{},", variant.name, variant.value));
        }
        self.cx.export(
            &enum_.name,
            &format!("Object.freeze({{ {} }})", variants),
            Some(format_doc_comments(&enum_.comments, None)),
        );
        self.cx
            .typescript
            .push_str(&format!("export enum {} {{", enum_.name));

        for variant in enum_.variants.iter() {
            self.cx
                .typescript
                .push_str(&format!("\n  {},", variant.name));
        }
        self.cx.typescript.push_str("\n}\n");
    }

    fn generate_struct(&mut self, struct_: &decode::Struct) -> Result<(), Error> {
        let mut dst = String::new();
        let mut ts_dst = String::new();
        for field in struct_.fields.iter() {
            let wasm_getter = wasm_bindgen_shared::struct_field_get(&struct_.name, &field.name);
            let wasm_setter = wasm_bindgen_shared::struct_field_set(&struct_.name, &field.name);
            let descriptor = match self.cx.describe(&wasm_getter) {
                None => continue,
                Some(d) => d,
            };

            let set = {
                let setter = ExportedShim::Named(&wasm_setter);
                let mut cx = Js2Rust::new(&field.name, self.cx);
                cx.method(true, false)
                    .argument(&descriptor, None)?
                    .ret(&Descriptor::Unit)?;
                ts_dst.push_str(&format!(
                    "\n  {}{}: {};",
                    if field.readonly { "readonly " } else { "" },
                    field.name,
                    &cx.js_arguments[0].1
                ));
                cx.finish("", &format!("wasm.{}", wasm_setter), setter).0
            };
            let getter = ExportedShim::Named(&wasm_getter);
            let (get, _ts, js_doc) = Js2Rust::new(&field.name, self.cx)
                .method(true, false)
                .ret(&descriptor)?
                .finish("", &format!("wasm.{}", wasm_getter), getter);
            if !dst.ends_with("\n") {
                dst.push_str("\n");
            }
            dst.push_str(&format_doc_comments(&field.comments, Some(js_doc)));
            dst.push_str("get ");
            dst.push_str(&field.name);
            dst.push_str(&get);
            dst.push_str("\n");
            if !field.readonly {
                dst.push_str("set ");
                dst.push_str(&field.name);
                dst.push_str(&set);
            }
        }

        let class = self
            .cx
            .exported_classes
            .as_mut()
            .expect("classes already written")
            .entry(struct_.name.to_string())
            .or_insert_with(Default::default);
        class.comments = format_doc_comments(&struct_.comments, None);
        class.contents.push_str(&dst);
        class.contents.push_str("\n");
        class.typescript.push_str(&ts_dst);
        class.typescript.push_str("\n");
        Ok(())
    }

    fn register_vendor_prefix(&mut self, info: &decode::ImportType<'b>) {
        if info.vendor_prefixes.len() == 0 {
            return;
        }
        self.vendor_prefixes
            .entry(info.name)
            .or_insert(Vec::new())
            .extend(info.vendor_prefixes.iter().cloned());
    }

    fn determine_import(
        &self,
        import: &decode::Import<'b>,
        item: &'b str,
    ) -> Result<Import<'b>, Error> {
        // First up, imports don't work at all in `--target no-modules` mode as
        // we're not sure how to import them.
        let is_local_snippet = match import.module {
            decode::ImportModule::Named(s) => self.cx.local_modules.contains_key(s),
            decode::ImportModule::RawNamed(_) => false,
            decode::ImportModule::Inline(_) => true,
            decode::ImportModule::None => false,
        };
        if self.cx.config.mode.no_modules() {
            if is_local_snippet {
                bail!(
                    "local JS snippets are not supported with `--target no-modules`; \
                     use `--target web` or no flag instead",
                );
            }
            if let decode::ImportModule::Named(module) = &import.module {
                bail!(
                    "import from `{}` module not allowed with `--target no-modules`; \
                     use `nodejs`, `web`, or `bundler` target instead",
                    module
                );
            }
        }

        // FIXME: currently we require that local JS snippets are written in ES
        // module syntax for imports/exports, but nodejs uses CommonJS to handle
        // this meaning that local JS snippets are basically guaranteed to be
        // incompatible. We need to implement a pass that translates the ES
        // module syntax in the snippet to a CommonJS module, which is in theory
        // not that hard but is a chunk of work to do.
        if is_local_snippet && self.cx.config.mode.nodejs() {
            // have a small unergonomic escape hatch for our webidl-tests tests
            if env::var("WBINDGEN_I_PROMISE_JS_SYNTAX_WORKS_IN_NODE").is_err() {
                bail!(
                    "local JS snippets are not supported with `--target nodejs`; \
                     see rustwasm/rfcs#6 for more details, but this restriction \
                     will be lifted in the future"
                );
            }
        }

        // Similar to `--target no-modules`, only allow vendor prefixes
        // basically for web apis, shouldn't be necessary for things like npm
        // packages or other imported items.
        let vendor_prefixes = self.vendor_prefixes.get(item);
        if let Some(vendor_prefixes) = vendor_prefixes {
            assert!(vendor_prefixes.len() > 0);

            if is_local_snippet {
                bail!(
                    "local JS snippets do not support vendor prefixes for \
                     the import of `{}` with a polyfill of `{}`",
                    item,
                    &vendor_prefixes[0]
                );
            }
            if let decode::ImportModule::Named(module) = &import.module {
                bail!(
                    "import of `{}` from `{}` has a polyfill of `{}` listed, but
                     vendor prefixes aren't supported when importing from modules",
                    item,
                    module,
                    &vendor_prefixes[0],
                );
            }
            if let Some(ns) = &import.js_namespace {
                bail!(
                    "import of `{}` through js namespace `{}` isn't supported \
                     right now when it lists a polyfill",
                    item,
                    ns
                );
            }
            return Ok(Import::VendorPrefixed {
                name: item,
                prefixes: vendor_prefixes.clone(),
            });
        }

        let (name, field) = match import.js_namespace {
            Some(ns) => (ns, Some(item)),
            None => (item, None),
        };

        Ok(match import.module {
            decode::ImportModule::Named(module) if is_local_snippet => Import::LocalModule {
                module,
                name,
                field,
            },
            decode::ImportModule::Named(module) | decode::ImportModule::RawNamed(module) => {
                Import::Module {
                    module,
                    name,
                    field,
                }
            }
            decode::ImportModule::Inline(idx) => {
                let offset = *self
                    .cx
                    .snippet_offsets
                    .get(self.program.unique_crate_identifier)
                    .unwrap_or(&0);
                Import::InlineJs {
                    unique_crate_identifier: self.program.unique_crate_identifier,
                    snippet_idx_in_crate: idx as usize + offset,
                    name,
                    field,
                }
            }
            decode::ImportModule::None => Import::Global { name, field },
        })
    }

    fn import_name(&mut self, import: &decode::Import<'b>, item: &'b str) -> Result<String, Error> {
        let import = self.determine_import(import, item)?;
        Ok(self.cx.import_identifier(import))
    }

    fn add_package_json(&mut self, path: &'b str) -> Result<(), Error> {
        if !self.cx.package_json_read.insert(path) {
            return Ok(());
        }
        if !self.cx.config.mode.nodejs() && !self.cx.config.mode.bundler() {
            bail!(
                "NPM dependencies have been specified in `{}` but \
                 this is only compatible with the `bundler` and `nodejs` targets"
            );
        }
        let contents = fs::read_to_string(path).context(format!("failed to read `{}`", path))?;
        let json: serde_json::Value = serde_json::from_str(&contents)?;
        let object = match json.as_object() {
            Some(s) => s,
            None => bail!(
                "expected `package.json` to have an JSON object in `{}`",
                path
            ),
        };
        let mut iter = object.iter();
        let (key, value) = match iter.next() {
            Some(pair) => pair,
            None => return Ok(()),
        };
        if key != "dependencies" || iter.next().is_some() {
            bail!(
                "NPM manifest found at `{}` can currently only have one key, \
                 `dependencies`, and no other fields",
                path
            );
        }
        let value = match value.as_object() {
            Some(s) => s,
            None => bail!("expected `dependencies` to be a JSON object in `{}`", path),
        };

        for (name, value) in value.iter() {
            let value = match value.as_str() {
                Some(s) => s,
                None => bail!(
                    "keys in `dependencies` are expected to be strings in `{}`",
                    path
                ),
            };
            if let Some((prev, _prev_version)) = self.cx.npm_dependencies.get(name) {
                bail!(
                    "dependency on NPM package `{}` specified in two `package.json` files, \
                     which at the time is not allowed:\n  * {}\n  * {}",
                    name,
                    path,
                    prev
                )
            }

            self.cx
                .npm_dependencies
                .insert(name.to_string(), (path, value.to_string()));
        }

        Ok(())
    }
}

#[derive(Hash, Eq, PartialEq)]
pub enum ImportModule<'a> {
    Named(&'a str),
    Inline(&'a str, usize),
    None,
}

impl<'a> Import<'a> {
    fn module(&self) -> ImportModule<'a> {
        match self {
            Import::Module { module, .. } | Import::LocalModule { module, .. } => {
                ImportModule::Named(module)
            }
            Import::InlineJs {
                unique_crate_identifier,
                snippet_idx_in_crate,
                ..
            } => ImportModule::Inline(unique_crate_identifier, *snippet_idx_in_crate),
            Import::Global { .. } | Import::VendorPrefixed { .. } => ImportModule::None,
        }
    }

    fn field(&self) -> Option<&'a str> {
        match self {
            Import::Module { field, .. }
            | Import::LocalModule { field, .. }
            | Import::InlineJs { field, .. }
            | Import::Global { field, .. } => *field,
            Import::VendorPrefixed { .. } => None,
        }
    }

    fn name(&self) -> &'a str {
        match self {
            Import::Module { name, .. }
            | Import::LocalModule { name, .. }
            | Import::InlineJs { name, .. }
            | Import::Global { name, .. }
            | Import::VendorPrefixed { name, .. } => *name,
        }
    }
}

fn generate_identifier(name: &str, used_names: &mut HashMap<String, usize>) -> String {
    let cnt = used_names.entry(name.to_string()).or_insert(0);
    *cnt += 1;
    // We want to mangle `default` at once, so we can support default exports and don't generate
    // invalid glue code like this: `import { default } from './module';`.
    if *cnt == 1 && name != "default" {
        name.to_string()
    } else {
        format!("{}{}", name, cnt)
    }
}

fn format_doc_comments(comments: &[&str], js_doc_comments: Option<String>) -> String {
    let body: String = comments
        .iter()
        .map(|c| format!("*{}\n", c.trim_matches('"')))
        .collect();
    let doc = if let Some(docs) = js_doc_comments {
        docs.lines().map(|l| format!("* {} \n", l)).collect()
    } else {
        String::new()
    };
    format!("/**\n{}{}*/\n", body, doc)
}

#[test]
fn test_generate_identifier() {
    let mut used_names: HashMap<String, usize> = HashMap::new();
    assert_eq!(
        generate_identifier("someVar", &mut used_names),
        "someVar".to_string()
    );
    assert_eq!(
        generate_identifier("someVar", &mut used_names),
        "someVar2".to_string()
    );
    assert_eq!(
        generate_identifier("default", &mut used_names),
        "default1".to_string()
    );
    assert_eq!(
        generate_identifier("default", &mut used_names),
        "default2".to_string()
    );
}
