use std::collections::{HashMap, HashSet};
use std::mem;

use decode;
use failure::{Error, ResultExt};
use gc;
use parity_wasm::elements::Error as ParityError;
use parity_wasm::elements::*;
use shared;

use super::Bindgen;
use descriptor::{Descriptor, VectorKind};
use wasm_interpreter::Interpreter;
use wasm_utils::Remap;

mod js2rust;
use self::js2rust::Js2Rust;
mod rust2js;
use self::rust2js::Rust2Js;
mod closures;

pub struct Context<'a> {
    pub globals: String,
    pub imports: String,
    pub imports_post: String,
    pub footer: String,
    pub typescript: String,
    pub exposed_globals: HashSet<&'static str>,
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
    pub imported_names: HashMap<Option<&'a str>, HashMap<&'a str, String>>,

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

    pub exported_classes: HashMap<String, ExportedClass>,
    pub function_table_needed: bool,
    pub interpreter: &'a mut Interpreter,
    pub memory_init: Option<ResizableLimits>,
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
    fn export(&mut self, name: &str, contents: &str, comments: Option<String>) {
        let contents = contents.trim();
        if let Some(ref c) = comments {
            self.globals.push_str(c);
        }
        let global = if self.use_node_require() {
            if contents.starts_with("class") {
                format!("{1}\nmodule.exports.{0} = {0};\n", name, contents)
            } else {
                format!("module.exports.{} = {};\n", name, contents)
            }
        } else if self.config.no_modules {
            if contents.starts_with("class") {
                format!("{1}\n__exports.{0} = {0};\n", name, contents)
            } else {
                format!("__exports.{} = {};\n", name, contents)
            }
        } else {
            if contents.starts_with("function") {
                format!("export function {}{}\n", name, &contents[8..])
            } else if contents.starts_with("class") {
                format!("export {}\n", contents)
            } else {
                format!("export const {} = {};\n", name, contents)
            }
        };
        self.global(&global);
    }

    fn require_internal_export(&mut self, name: &'static str) -> Result<(), Error> {
        if !self.required_internal_exports.insert(name) {
            return Ok(());
        }
        if let Some(s) = self.module.export_section() {
            if s.entries().iter().any(|e| e.field() == name) {
                return Ok(());
            }
        }

        bail!(
            "the exported function `{}` is required to generate bindings \
             but it was not found in the wasm file, perhaps the `std` feature \
             of the `wasm-bindgen` crate needs to be enabled?",
            name
        );
    }

    pub fn finalize(&mut self, module_name: &str) -> Result<(String, String), Error> {
        self.write_classes()?;

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

        self.bind("__wbindgen_object_drop_ref", &|me| {
            me.expose_drop_ref();
            Ok(String::from("function(i) { dropObject(i); }"))
        })?;

        self.bind("__wbindgen_string_new", &|me| {
            me.expose_add_heap_object();
            me.expose_get_string_from_wasm();
            Ok(String::from(
                "
                function(p, l) {
                    return addHeapObject(getStringFromWasm(p, l));
                }
                ",
            ))
        })?;

        self.bind("__wbindgen_number_new", &|me| {
            me.expose_add_heap_object();
            Ok(String::from("function(i) { return addHeapObject(i); }"))
        })?;

        self.bind("__wbindgen_number_get", &|me| {
            me.expose_get_object();
            me.expose_uint8_memory();
            Ok(String::from(
                "
                function(n, invalid) {
                    let obj = getObject(n);
                    if (typeof(obj) === 'number') return obj;
                    getUint8Memory()[invalid] = 1;
                    return 0;
                }
                ",
            ))
        })?;

        self.bind("__wbindgen_is_null", &|me| {
            me.expose_get_object();
            Ok(String::from(
                "
                function(idx) {
                    return getObject(idx) === null ? 1 : 0;
                }
                ",
            ))
        })?;

        self.bind("__wbindgen_is_undefined", &|me| {
            me.expose_get_object();
            Ok(String::from(
                "
                function(idx) {
                    return getObject(idx) === undefined ? 1 : 0;
                }
                ",
            ))
        })?;

        self.bind("__wbindgen_boolean_get", &|me| {
            me.expose_get_object();
            Ok(String::from(
                "
                function(i) {
                    let v = getObject(i);
                    if (typeof(v) === 'boolean') {
                        return v ? 1 : 0;
                    } else {
                        return 2;
                    }
                }
                ",
            ))
        })?;

        self.bind("__wbindgen_symbol_new", &|me| {
            me.expose_get_string_from_wasm();
            me.expose_add_heap_object();
            Ok(String::from(
                "
                function(ptr, len) {
                    let a;
                    if (ptr === 0) {
                        a = Symbol();
                    } else {
                        a = Symbol(getStringFromWasm(ptr, len));
                    }
                    return addHeapObject(a);
                }
                ",
            ))
        })?;

        self.bind("__wbindgen_is_symbol", &|me| {
            me.expose_get_object();
            Ok(String::from(
                "
                function(i) {
                    return typeof(getObject(i)) === 'symbol' ? 1 : 0;
                }
                ",
            ))
        })?;

        self.bind("__wbindgen_is_object", &|me| {
            me.expose_get_object();
            Ok(String::from(
                "
                function(i) {
                    const val = getObject(i);
                    return typeof(val) === 'object' && val !== null ? 1 : 0;
                }
                ",
            ))
        })?;

        self.bind("__wbindgen_is_function", &|me| {
            me.expose_get_object();
            Ok(String::from(
                "
                function(i) {
                    return typeof(getObject(i)) === 'function' ? 1 : 0;
                }
                ",
            ))
        })?;

        self.bind("__wbindgen_is_string", &|me| {
            me.expose_get_object();
            Ok(String::from(
                "
                function(i) {
                    return typeof(getObject(i)) === 'string' ? 1 : 0;
                }
                ",
            ))
        })?;

        self.bind("__wbindgen_string_get", &|me| {
            me.expose_pass_string_to_wasm()?;
            me.expose_get_object();
            me.expose_uint32_memory();
            Ok(String::from(
                "
                function(i, len_ptr) {
                    let obj = getObject(i);
                    if (typeof(obj) !== 'string') return 0;
                    const ptr = passStringToWasm(obj);
                    getUint32Memory()[len_ptr / 4] = WASM_VECTOR_LEN;
                    return ptr;
                }
                ",
            ))
        })?;

        self.bind("__wbindgen_cb_drop", &|me| {
            me.expose_drop_ref();
            Ok(String::from(
                "
                function(i) {
                    const obj = getObject(i).original;
                    dropObject(i);
                    if (obj.cnt-- == 1) {
                        obj.a = 0;
                        return 1;
                    }
                    return 0;
                }
                ",
            ))
        })?;

        self.bind("__wbindgen_cb_forget", &|me| {
            me.expose_drop_ref();
            Ok("dropObject".to_string())
        })?;

        self.bind("__wbindgen_json_parse", &|me| {
            me.expose_add_heap_object();
            me.expose_get_string_from_wasm();
            Ok(String::from(
                "
                function(ptr, len) {
                    return addHeapObject(JSON.parse(getStringFromWasm(ptr, len)));
                }
                ",
            ))
        })?;

        self.bind("__wbindgen_json_serialize", &|me| {
            me.expose_get_object();
            me.expose_pass_string_to_wasm()?;
            me.expose_uint32_memory();
            Ok(String::from(
                "
                function(idx, ptrptr) {
                    const ptr = passStringToWasm(JSON.stringify(getObject(idx)));
                    getUint32Memory()[ptrptr / 4] = ptr;
                    return WASM_VECTOR_LEN;
                }
                ",
            ))
        })?;

        self.bind("__wbindgen_jsval_eq", &|me| {
            me.expose_get_object();
            Ok(String::from(
                "
                function(a, b) {
                    return getObject(a) === getObject(b) ? 1 : 0;
                }
                ",
            ))
        })?;

        self.bind("__wbindgen_memory", &|me| {
            me.expose_add_heap_object();
            let mem = me.memory();
            Ok(format!("function() {{ return addHeapObject({}); }}", mem))
        })?;

        self.bind("__wbindgen_module", &|me| {
            if !me.config.no_modules {
                bail!(
                    "`wasm_bindgen::module` is currently only supported with \
                     --no-modules"
                );
            }
            me.expose_add_heap_object();
            Ok(format!(
                "
                function() {{
                    return addHeapObject(init.__wbindgen_wasm_module);
                }}
                ",
            ))
        })?;

        self.bind("__wbindgen_rethrow", &|me| {
            me.expose_take_object();
            Ok(String::from("function(idx) { throw takeObject(idx); }"))
        })?;

        closures::rewrite(self).with_context(|_| "failed to generate internal closure shims")?;
        self.unexport_unused_internal_exports();

        // Handle the `start` function, if one was specified. If we're in a
        // --test mode (such as wasm-bindgen-test-runner) then we skip this
        // entirely. Otherwise we want to first add a start function to the
        // `start` section if one is specified.
        //
        // Afterwards, we need to perform what's a bit of a hack. Right after we
        // added the start function, we remove it again because no current
        // strategy for bundlers and deployment works well enough with it. For
        // `--no-modules` output we need to be sure to call the start function
        // after our exports are wired up (or most imported functions won't
        // work).
        //
        // For ESM outputs bundlers like webpack also don't work because
        // currently they run the wasm initialization before the JS glue
        // initialization, meaning that if the wasm start function calls
        // imported functions the JS glue isn't ready to go just yet.
        //
        // To handle `--no-modules` we just unstart the start function and call
        // it manually. To handle the ESM use case we switch the start function
        // to calling an imported function which defers the start function via
        // `Promise.resolve().then(...)` to execute on the next microtask tick.
        let mut has_start_function = false;
        if self.config.emit_start {
            self.add_start_function()?;
            has_start_function = self.unstart_start_function();
            if has_start_function && !self.config.no_modules {
                self.inject_start_shim();
            }
        }

        self.gc();

        // Note that it's important `throw` comes last *after* we gc. The
        // `__wbindgen_malloc` function may call this but we only want to
        // generate code for this if it's actually live (and __wbindgen_malloc
        // isn't gc'd).
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

        self.rewrite_imports(module_name);
        self.update_producers_section();

        let mut js = if self.config.threads.is_some() {
            // TODO: It's not clear right now how to best use threads with
            // bundlers like webpack. We need a way to get the existing
            // module/memory into web workers for now and we don't quite know
            // idiomatically how to do that! In the meantime, always require
            // `--no-modules`
            if !self.config.no_modules {
                bail!("most use `--no-modules` with threads for now")
            }
            self.memory(); // set `memory_limit` if it's not already set
            let limits = match &self.memory_init {
                Some(l) if l.shared() => l.clone(),
                _ => bail!("must impot a shared memory with threads"),
            };

            let mut memory = String::from("new WebAssembly.Memory({");
            memory.push_str(&format!("initial:{}", limits.initial()));
            if let Some(max) = limits.maximum() {
                memory.push_str(&format!(",maximum:{}", max));
            }
            if limits.shared() {
                memory.push_str(",shared:true");
            }
            memory.push_str("})");

            format!(
                "\
(function() {{
    var wasm;
    var memory;
    const __exports = {{}};
    {globals}
    function init(module_or_path, maybe_memory) {{
        let result;
        const imports = {{ './{module}': __exports }};
        if (module_or_path instanceof WebAssembly.Module) {{
            memory = __exports.memory = maybe_memory;
            result = WebAssembly.instantiate(module_or_path, imports)
                .then(instance => {{
                    return {{ instance, module: module_or_path }}
                }});
        }} else {{
            memory = __exports.memory = {init_memory};
            const response = fetch(module_or_path);
            if (typeof WebAssembly.instantiateStreaming === 'function') {{
                result = WebAssembly.instantiateStreaming(response, imports);
            }} else {{
                result = response
                    .then(r => r.arrayBuffer())
                    .then(bytes => WebAssembly.instantiate(bytes, imports));
            }}
        }}
        return result.then(({{instance, module}}) => {{
            wasm = init.wasm = instance.exports;
            init.__wbindgen_wasm_instance = instance;
            init.__wbindgen_wasm_module = module;
            init.__wbindgen_wasm_memory = __exports.memory;
            {start}
        }});
    }};
    self.{global_name} = Object.assign(init, __exports);
}})();",
                globals = self.globals,
                module = module_name,
                global_name = self
                    .config
                    .no_modules_global
                    .as_ref()
                    .map(|s| &**s)
                    .unwrap_or("wasm_bindgen"),
                init_memory = memory,
                start = if has_start_function {
                    "wasm.__wbindgen_start();"
                } else {
                    ""
                },
            )
        } else if self.config.no_modules {
            format!(
                "\
(function() {{
    var wasm;
    const __exports = {{}};
    {globals}
    function init(path_or_module) {{
        let instantiation;
        const imports = {{ './{module}': __exports }};
        if (path_or_module instanceof WebAssembly.Module) {{
            instantiation = WebAssembly.instantiate(path_or_module, imports)
                .then(instance => {{
                    return {{ instance, module: path_or_module }}
                }});
        }} else {{
            const data = fetch(path_or_module);
            if (typeof WebAssembly.instantiateStreaming === 'function') {{
                instantiation = WebAssembly.instantiateStreaming(data, imports);
            }} else {{
                instantiation = data
                    .then(response => response.arrayBuffer())
                    .then(buffer => WebAssembly.instantiate(buffer, imports));
            }}
        }}
        return instantiation.then(({{instance}}) => {{
            wasm = init.wasm = instance.exports;
            {start}
        }});
    }};
    self.{global_name} = Object.assign(init, __exports);
}})();",
                globals = self.globals,
                module = module_name,
                global_name = self
                    .config
                    .no_modules_global
                    .as_ref()
                    .map(|s| &**s)
                    .unwrap_or("wasm_bindgen"),
                start = if has_start_function {
                    "wasm.__wbindgen_start();"
                } else {
                    ""
                },
            )
        } else {
            let import_wasm = if self.globals.len() == 0 {
                String::new()
            } else if self.use_node_require() {
                self.footer
                    .push_str(&format!("wasm = require('./{}_bg');", module_name));
                format!("var wasm;")
            } else {
                format!("import * as wasm from './{}_bg';", module_name)
            };

            format!(
                "\
                /* tslint:disable */\n\
                {import_wasm}\n\
                {imports}\n\
                {imports_post}\n\

                {globals}\n\
                {footer}",
                import_wasm = import_wasm,
                globals = self.globals,
                imports = self.imports,
                imports_post = self.imports_post,
                footer = self.footer,
            )
        };

        self.export_table();
        self.gc();

        while js.contains("\n\n\n") {
            js = js.replace("\n\n\n", "\n\n");
        }

        Ok((js, self.typescript.clone()))
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
        let classes = mem::replace(&mut self.exported_classes, Default::default());
        for (class, exports) in classes {
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
        let new_name = shared::new_function(&name);
        if self.wasm_import_needed(&new_name) {
            self.expose_add_heap_object();
            wrap_needed = true;

            self.export(
                &new_name,
                &format!(
                    "
                    function(ptr) {{
                        return addHeapObject({}.__wrap(ptr));
                    }}
                    ",
                    name
                ),
                None,
            );
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
            shared::free_function(&name)
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
        ts_dst.push_str("free(): void;\n");
        dst.push_str(&class.contents);
        ts_dst.push_str(&class.typescript);
        dst.push_str("}\n");
        ts_dst.push_str("}\n");

        self.export(&name, &dst, Some(class.comments.clone()));
        self.typescript.push_str(&ts_dst);

        Ok(())
    }

    fn export_table(&mut self) {
        if !self.function_table_needed {
            return;
        }
        for section in self.module.sections_mut() {
            let exports = match *section {
                Section::Export(ref mut s) => s,
                _ => continue,
            };
            let entry = ExportEntry::new("__wbg_function_table".to_string(), Internal::Table(0));
            exports.entries_mut().push(entry);
            break;
        }
    }

    fn rewrite_imports(&mut self, module_name: &str) {
        for (name, contents) in self._rewrite_imports(module_name) {
            self.export(&name, &contents, None);
        }
    }

    fn _rewrite_imports(&mut self, module_name: &str) -> Vec<(String, String)> {
        let mut math_imports = Vec::new();
        let imports = self
            .module
            .sections_mut()
            .iter_mut()
            .filter_map(|s| match *s {
                Section::Import(ref mut s) => Some(s),
                _ => None,
            })
            .flat_map(|s| s.entries_mut());

        for import in imports {
            if import.module() == "__wbindgen_placeholder__" {
                import.module_mut().truncate(0);
                if let Some((module, name)) = self.direct_imports.get(import.field()) {
                    import.field_mut().truncate(0);
                    import.module_mut().push_str(module);
                    import.field_mut().push_str(name);
                } else {
                    import.module_mut().push_str("./");
                    import.module_mut().push_str(module_name);
                }
                continue;
            }

            if import.module() != "env" {
                continue;
            }

            // If memory is imported we'll have exported it from the shim module
            // so let's import it from there.
            if import.field() == "memory" {
                import.module_mut().truncate(0);
                import.module_mut().push_str("./");
                import.module_mut().push_str(module_name);
                continue;
            }

            let renamed_import = format!("__wbindgen_{}", import.field());
            let mut bind_math = |expr: &str| {
                math_imports.push((renamed_import.clone(), format!("function{}", expr)));
            };

            // FIXME(#32): try to not use function shims
            match import.field() {
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

            import.module_mut().truncate(0);
            import.module_mut().push_str("./");
            import.module_mut().push_str(module_name);
            *import.field_mut() = renamed_import.clone();
        }

        math_imports
    }

    fn unexport_unused_internal_exports(&mut self) {
        let required = &self.required_internal_exports;
        for section in self.module.sections_mut() {
            let exports = match *section {
                Section::Export(ref mut s) => s,
                _ => continue,
            };
            exports.entries_mut().retain(|export| {
                !export.field().starts_with("__wbindgen") || required.contains(export.field())
            });
        }
    }

    fn expose_drop_ref(&mut self) {
        if !self.exposed_globals.insert("drop_ref") {
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
        if !self.exposed_globals.insert("heap") {
            return;
        }
        self.global(&format!("const heap = new Array({});", INITIAL_HEAP_OFFSET));
        self.global("heap.fill(undefined);");
        self.global(&format!("heap.push({});", INITIAL_HEAP_VALUES.join(", ")));
    }

    fn expose_global_heap_next(&mut self) {
        if !self.exposed_globals.insert("heap_next") {
            return;
        }
        self.expose_global_heap();
        self.global("let heap_next = heap.length;");
    }

    fn expose_get_object(&mut self) {
        if !self.exposed_globals.insert("get_object") {
            return;
        }
        self.expose_global_heap();

        // Accessing a heap object is just a simple index operation due to how
        // the stack/heap are laid out.
        self.global("function getObject(idx) { return heap[idx]; }");
    }

    fn expose_assert_num(&mut self) {
        if !self.exposed_globals.insert("assert_num") {
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
        if !self.exposed_globals.insert("assert_bool") {
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
        if !self.exposed_globals.insert("wasm_vector_len") {
            return;
        }
        self.global("let WASM_VECTOR_LEN = 0;");
    }

    fn expose_pass_string_to_wasm(&mut self) -> Result<(), Error> {
        if !self.exposed_globals.insert("pass_string_to_wasm") {
            return Ok(());
        }
        self.require_internal_export("__wbindgen_malloc")?;
        self.expose_text_encoder();
        self.expose_uint8_memory();
        self.expose_wasm_vector_len();
        let debug = if self.config.debug {
            "
                if (typeof(arg) !== 'string') throw new Error('expected a string argument');
            "
        } else {
            ""
        };
        self.global(&format!(
            "
            function passStringToWasm(arg) {{
                {}
                const buf = cachedTextEncoder.encode(arg);
                const ptr = wasm.__wbindgen_malloc(buf.length);
                getUint8Memory().set(buf, ptr);
                WASM_VECTOR_LEN = buf.length;
                return ptr;
            }}
            ",
            debug
        ));
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
        if !self.exposed_globals.insert("pass_array_jsvalue") {
            return Ok(());
        }
        self.require_internal_export("__wbindgen_malloc")?;
        self.expose_uint32_memory();
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
        Ok(())
    }

    fn pass_array_to_wasm(
        &mut self,
        name: &'static str,
        delegate: &str,
        size: usize,
    ) -> Result<(), Error> {
        if !self.exposed_globals.insert(name) {
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
        if !self.exposed_globals.insert("text_encoder") {
            return;
        }
        self.expose_text_processor("TextEncoder");
    }

    fn expose_text_decoder(&mut self) {
        if !self.exposed_globals.insert("text_decoder") {
            return;
        }
        self.expose_text_processor("TextDecoder");
    }

    fn expose_text_processor(&mut self, s: &str) {
        if self.config.nodejs_experimental_modules {
            self.imports
                .push_str(&format!("import {{ {} }} from 'util';\n", s));
            self.global(&format!("let cached{0} = new {0}('utf-8');", s));
        } else if self.config.nodejs {
            self.global(&format!("const {0} = require('util').{0};", s));
            self.global(&format!("let cached{0} = new {0}('utf-8');", s));
        } else if !(self.config.browser || self.config.no_modules) {
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
        if !self.exposed_globals.insert("get_string_from_wasm") {
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
        self.memory(); // set self.memory_init
        let is_shared = self
            .module
            .memory_section()
            .map(|s| s.entries()[0].limits().shared())
            .unwrap_or(match &self.memory_init {
                Some(limits) => limits.shared(),
                None => false,
            });
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

    fn expose_get_array_js_value_from_wasm(&mut self) {
        if !self.exposed_globals.insert("get_array_js_value_from_wasm") {
            return;
        }
        self.expose_uint32_memory();
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
        if !self.exposed_globals.insert(name) {
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

    fn expose_int8_memory(&mut self) {
        self.memview("getInt8Memory", "Int8Array");
    }

    fn expose_uint8_memory(&mut self) {
        self.memview("getUint8Memory", "Uint8Array");
    }

    fn expose_clamped_uint8_memory(&mut self) {
        self.memview("getUint8ClampedMemory", "Uint8ClampedArray");
    }

    fn expose_int16_memory(&mut self) {
        self.memview("getInt16Memory", "Int16Array");
    }

    fn expose_uint16_memory(&mut self) {
        self.memview("getUint16Memory", "Uint16Array");
    }

    fn expose_int32_memory(&mut self) {
        self.memview("getInt32Memory", "Int32Array");
    }

    fn expose_uint32_memory(&mut self) {
        self.memview("getUint32Memory", "Uint32Array");
    }

    fn expose_int64_memory(&mut self) {
        self.memview("getInt64Memory", "BigInt64Array");
    }

    fn expose_uint64_memory(&mut self) {
        self.memview("getUint64Memory", "BigUint64Array");
    }

    fn expose_f32_memory(&mut self) {
        self.memview("getFloat32Memory", "Float32Array");
    }

    fn expose_f64_memory(&mut self) {
        self.memview("getFloat64Memory", "Float64Array");
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
        if !self.exposed_globals.insert(name) {
            return;
        }
        let mem = self.memory();
        self.global(&format!(
            "
            let cache{name} = null;
            function {name}() {{
                if (cache{name} === null || cache{name}.buffer !== {mem}.buffer) {{
                    cache{name} = new {js}({mem}.buffer);
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
        if !self.exposed_globals.insert("assert_class") {
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
        if !self.exposed_globals.insert("stack_pointer") {
            return;
        }
        self.global(&format!("let stack_pointer = {};", INITIAL_HEAP_OFFSET));
    }

    fn expose_borrowed_objects(&mut self) {
        if !self.exposed_globals.insert("borrowed_objects") {
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
        if !self.exposed_globals.insert("take_object") {
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
        if !self.exposed_globals.insert("add_heap_object") {
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

    fn wasm_import_needed(&self, name: &str) -> bool {
        let imports = match self.module.import_section() {
            Some(s) => s,
            None => return false,
        };

        imports
            .entries()
            .iter()
            .any(|i| i.module() == "__wbindgen_placeholder__" && i.field() == name)
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

    fn expose_get_vector_from_wasm(&mut self, ty: VectorKind) -> &'static str {
        match ty {
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
                self.expose_get_array_js_value_from_wasm();
                "getArrayJsValueFromWasm"
            }
        }
    }

    fn expose_global_argument_ptr(&mut self) -> Result<(), Error> {
        if !self.exposed_globals.insert("global_argument_ptr") {
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
        if !self.exposed_globals.insert("get_inherited_descriptor") {
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
        if !self.exposed_globals.insert(name) {
            return name;
        }
        self.global(&format!("const {} = new Uint32Array(2);", name));
        name
    }

    fn expose_int64_cvt_shim(&mut self) -> &'static str {
        let name = "int64CvtShim";
        if !self.exposed_globals.insert(name) {
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
        if !self.exposed_globals.insert(name) {
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
        if !self.exposed_globals.insert("is_like_none") {
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
        if !self.exposed_globals.insert("cleanup_groups") {
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

    fn gc(&mut self) {
        gc::Config::new()
            .demangle(self.config.demangle)
            .keep_debug(self.config.keep_debug || self.config.debug)
            .run(&mut self.module);
    }

    pub fn parse_wasm_names(&mut self) {
        let module = mem::replace(self.module, Module::default());
        let module = module.parse_names().unwrap_or_else(|p| p.1);
        *self.module = module;
        if self.config.remove_name_section {
            self.module.sections_mut().retain(|s| match s {
                Section::Name(_) => false,
                _ => true,
            });
        }
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
        self.config.nodejs && !self.config.nodejs_experimental_modules
    }

    fn memory(&mut self) -> &'static str {
        if self.module.memory_section().is_some() {
            return "wasm.memory";
        }

        let (entry, mem) = self
            .module
            .import_section()
            .expect("must import memory")
            .entries()
            .iter()
            .filter_map(|i| match i.external() {
                External::Memory(m) => Some((i, m)),
                _ => None,
            })
            .next()
            .expect("must import memory");
        assert_eq!(entry.field(), "memory");
        self.memory_init = Some(mem.limits().clone());
        "memory"
    }

    fn require_class_wrap(&mut self, class: &str) {
        self.exported_classes
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
                    Import::Module { module, .. } => {
                        if use_node_require {
                            imports.push_str(&format!(
                                "const {} = require(String.raw`{}`).{};\n",
                                name,
                                module,
                                import.name()
                            ));
                        } else if import.name() == name {
                            imports
                                .push_str(&format!("import {{ {} }} from '{}';\n", name, module));
                        } else {
                            imports.push_str(&format!(
                                "import {{ {} as {} }} from '{}';\n",
                                import.name(),
                                name,
                                module
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
        for section in self.module.sections_mut() {
            let section = match section {
                Section::Custom(s) => s,
                _ => continue,
            };
            if section.name() != "producers" {
                return;
            }
            drop(update(section));
            return;
        }

        // `CustomSection::new` added in paritytech/parity-wasm#244 which isn't
        // merged just yet
        let data = [
            ("producers".len() + 2) as u8,
            "producers".len() as u8,
            b'p',
            b'r',
            b'o',
            b'd',
            b'u',
            b'c',
            b'e',
            b'r',
            b's',
            0,
        ];
        let mut section = CustomSection::deserialize(&mut &data[..]).unwrap();
        assert_eq!(section.name(), "producers");
        assert_eq!(section.payload(), [0]);
        drop(update(&mut section));
        self.module.sections_mut().push(Section::Custom(section));

        fn update(section: &mut CustomSection) -> Result<(), ParityError> {
            struct Field {
                name: String,
                values: Vec<FieldValue>,
            }
            struct FieldValue {
                name: String,
                version: String,
            }

            let wasm_bindgen = || FieldValue {
                name: "wasm-bindgen".to_string(),
                version: shared::version(),
            };
            let mut fields = Vec::new();

            // Deserialize the fields, appending the wasm-bidngen field/value
            // where applicable
            {
                let mut data = section.payload();
                let amt: u32 = VarUint32::deserialize(&mut data)?.into();
                let mut found_processed_by = false;
                for _ in 0..amt {
                    let name = String::deserialize(&mut data)?;
                    let cnt: u32 = VarUint32::deserialize(&mut data)?.into();
                    let mut values = Vec::with_capacity(cnt as usize);
                    for _ in 0..cnt {
                        let name = String::deserialize(&mut data)?;
                        let version = String::deserialize(&mut data)?;
                        values.push(FieldValue { name, version });
                    }

                    if name == "processed-by" {
                        found_processed_by = true;
                        values.push(wasm_bindgen());
                    }

                    fields.push(Field { name, values });
                }
                if data.len() != 0 {
                    return Err(ParityError::InconsistentCode);
                }

                if !found_processed_by {
                    fields.push(Field {
                        name: "processed-by".to_string(),
                        values: vec![wasm_bindgen()],
                    });
                }
            }

            // re-serialize these fields back into the custom section
            let dst = section.payload_mut();
            dst.truncate(0);
            VarUint32::from(fields.len() as u32).serialize(dst)?;
            for field in fields.iter() {
                field.name.clone().serialize(dst)?;
                VarUint32::from(field.values.len() as u32).serialize(dst)?;
                for value in field.values.iter() {
                    value.name.clone().serialize(dst)?;
                    value.version.clone().serialize(dst)?;
                }
            }

            Ok(())
        }
    }

    fn add_start_function(&mut self) -> Result<(), Error> {
        let start = match &self.start {
            Some(name) => name.clone(),
            None => return Ok(()),
        };
        let idx = {
            let exports = self
                .module
                .export_section()
                .ok_or_else(|| format_err!("no export section found"))?;
            let entry = exports
                .entries()
                .iter()
                .find(|e| e.field() == start)
                .ok_or_else(|| format_err!("export `{}` not found", start))?;
            match entry.internal() {
                Internal::Function(i) => *i,
                _ => bail!("export `{}` wasn't a function", start),
            }
        };
        if let Some(prev_start) = self.module.start_section() {
            if let Some(NameSection::Function(n)) = self.module.names_section() {
                if let Some(prev) = n.names().get(prev_start) {
                    bail!(
                        "cannot flag `{}` as start function as `{}` is \
                         already the start function",
                        start,
                        prev
                    );
                }
            }

            bail!(
                "cannot flag `{}` as start function as another \
                 function is already the start function",
                start
            );
        }

        self.set_start_section(idx);
        Ok(())
    }

    fn set_start_section(&mut self, start: u32) {
        let mut pos = None;
        // See http://webassembly.github.io/spec/core/binary/modules.html#binary-module
        // for section ordering
        for (i, section) in self.module.sections().iter().enumerate() {
            match section {
                Section::Type(_)
                | Section::Import(_)
                | Section::Function(_)
                | Section::Table(_)
                | Section::Memory(_)
                | Section::Global(_)
                | Section::Export(_) => continue,
                _ => {
                    pos = Some(i);
                    break;
                }
            }
        }
        let pos = pos.unwrap_or(self.module.sections().len() - 1);
        self.module
            .sections_mut()
            .insert(pos, Section::Start(start));
    }

    /// If a start function is present, it removes it from the `start` section
    /// of the wasm module and then moves it to an exported function, named
    /// `__wbindgen_start`.
    fn unstart_start_function(&mut self) -> bool {
        let mut pos = None;
        let mut start = 0;
        for (i, section) in self.module.sections().iter().enumerate() {
            if let Section::Start(idx) = section {
                start = *idx;
                pos = Some(i);
                break;
            }
        }
        match pos {
            Some(i) => {
                self.module.sections_mut().remove(i);
                let entry =
                    ExportEntry::new("__wbindgen_start".to_string(), Internal::Function(start));
                self.module
                    .export_section_mut()
                    .unwrap()
                    .entries_mut()
                    .push(entry);
                true
            }
            None => false,
        }
    }

    /// Injects a `start` function into the wasm module. This start function
    /// calls a shim in the generated JS which defers the actual start function
    /// to the next microtask tick of the event queue.
    ///
    /// See docs above at callsite for why this happens.
    fn inject_start_shim(&mut self) {
        let body = "function() {
            Promise.resolve().then(() => wasm.__wbindgen_start());
        }";
        self.export("__wbindgen_defer_start", body, None);

        let imports = self
            .module
            .import_section()
            .map(|s| s.functions() as u32)
            .unwrap_or(0);
        Remap(|idx| if idx < imports { idx } else { idx + 1 }).remap_module(self.module);

        let type_idx = {
            let types = self.module.type_section_mut().unwrap();
            let ty = Type::Function(FunctionType::new(Vec::new(), None));
            types.types_mut().push(ty);
            (types.types_mut().len() - 1) as u32
        };

        let entry = ImportEntry::new(
            "__wbindgen_placeholder__".to_string(),
            "__wbindgen_defer_start".to_string(),
            External::Function(type_idx),
        );
        self.module
            .import_section_mut()
            .unwrap()
            .entries_mut()
            .push(entry);
        self.set_start_section(imports);
    }
}

impl<'a, 'b> SubContext<'a, 'b> {
    pub fn generate(&mut self) -> Result<(), Error> {
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
            .process(descriptor.unwrap_function())?
            .finish("function", &format!("wasm.{}", export.function.name));
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
        let wasm_name = shared::struct_function_export_name(class_name, &export.function.name);

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
            .process(descriptor.unwrap_function())?
            .finish("", &format!("wasm.{}", wasm_name));

        let class = self
            .cx
            .exported_classes
            .entry(class_name.to_string())
            .or_insert(ExportedClass::default());
        class
            .contents
            .push_str(&format_doc_comments(&export.comments, Some(js_doc)));

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
        self.cx.expose_add_heap_object();
        self.cx.export(
            &import.shim,
            &format!(
                "
                function() {{
                    return addHeapObject({});
                }}
                ",
                obj
            ),
            None,
        );
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
                return Ok(());
            }
        }

        // If the above optimization fails then we actually generate the import
        // here (possibly emitting some glue in our JS module) and then emit the
        // shim as the wasm will be importing the shim.
        let target = shim.cx.generated_import_target(name, import)?;
        let js = shim.finish(&target)?;
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
        self.cx.expose_get_object();
        let body = format!(
            "
                function(idx) {{
                    return getObject(idx) instanceof {} ? 1 : 0;
                }}
            ",
            name,
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

        variants.clear();
        for variant in enum_.variants.iter() {
            variants.push_str(&format!("{},", variant.name));
        }
        self.cx.typescript.push_str(&variants);
        self.cx.typescript.push_str("}\n");
    }

    fn generate_struct(&mut self, struct_: &decode::Struct) -> Result<(), Error> {
        let mut dst = String::new();
        let mut ts_dst = String::new();
        for field in struct_.fields.iter() {
            let wasm_getter = shared::struct_field_get(&struct_.name, &field.name);
            let wasm_setter = shared::struct_field_set(&struct_.name, &field.name);
            let descriptor = match self.cx.describe(&wasm_getter) {
                None => continue,
                Some(d) => d,
            };

            let set = {
                let mut cx = Js2Rust::new(&field.name, self.cx);
                cx.method(true, false)
                    .argument(&descriptor)?
                    .ret(&Descriptor::Unit)?;
                ts_dst.push_str(&format!(
                    "{}{}: {}\n",
                    if field.readonly { "readonly " } else { "" },
                    field.name,
                    &cx.js_arguments[0].1
                ));
                cx.finish("", &format!("wasm.{}", wasm_setter)).0
            };
            let (get, _ts, js_doc) = Js2Rust::new(&field.name, self.cx)
                .method(true, false)
                .ret(&descriptor)?
                .finish("", &format!("wasm.{}", wasm_getter));
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
        // First up, imports don't work at all in `--no-modules` mode as we're
        // not sure how to import them.
        if self.cx.config.no_modules {
            if let Some(module) = &import.module {
                bail!(
                    "import from `{}` module not allowed with `--no-modules`; \
                     use `--nodejs` or `--browser` instead",
                    module
                );
            }
        }

        // Similar to `--no-modules`, only allow vendor prefixes basically for web
        // apis, shouldn't be necessary for things like npm packages or other
        // imported items.
        let vendor_prefixes = self.vendor_prefixes.get(item);
        if let Some(vendor_prefixes) = vendor_prefixes {
            assert!(vendor_prefixes.len() > 0);

            if let Some(module) = &import.module {
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

        let name = import.js_namespace.as_ref().map(|s| &**s).unwrap_or(item);
        let field = if import.js_namespace.is_some() {
            Some(item)
        } else {
            None
        };

        Ok(match import.module {
            Some(module) => Import::Module {
                module,
                name,
                field,
            },
            None => Import::Global { name, field },
        })
    }

    fn import_name(&mut self, import: &decode::Import<'b>, item: &'b str) -> Result<String, Error> {
        let import = self.determine_import(import, item)?;
        Ok(self.cx.import_identifier(import))
    }
}

impl<'a> Import<'a> {
    fn module(&self) -> Option<&'a str> {
        match self {
            Import::Module { module, .. } => Some(module),
            _ => None,
        }
    }

    fn field(&self) -> Option<&'a str> {
        match self {
            Import::Module { field, .. } | Import::Global { field, .. } => *field,
            Import::VendorPrefixed { .. } => None,
        }
    }

    fn name(&self) -> &'a str {
        match self {
            Import::Module { name, .. }
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
