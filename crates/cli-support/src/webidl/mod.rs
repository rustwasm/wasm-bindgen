//! The polyfill for the WebIDL bindings proposal in wasm-bindgen.
//!
//! This module contains the polyfill (or at least the current state of and as
//! closely as we can match) the WebIDL bindings proposal. The module exports
//! one main function, `process`, which takes a `walrus::Module`. This module is
//! expected to have two items:
//!
//! * First it contains all of the raw wasm-bindgen modules emitted by the Rust
//!   compiler. These raw custom sections are extracted, removed, decoded, and
//!   handled here. They contain information such as what's exported where,
//!   what's imported, comments, etc.
//! * Second, the `descriptors.rs` pass must have run previously to execute all
//!   the descriptor functions in the wasm module. Through the synthesized
//!   custom section there we learn the type information of all
//!   functions/imports/exports in the module.
//!
//! The output of this function is then a new `walrus::Module` with the previous
//! custom sections all removed and two new ones inserted. One is the webidl
//! bindings custom section (or at least a close approximate) and the second is
//! an auxiliary section for wasm-bindgen itself. The goal is for this auxiliary
//! section to eventually be empty or inconsequential, allowing us to emit
//! something that doesn't even need a JS shim one day. For now we're still
//! pretty far away from that, so we'll settle for using webidl bindings as
//! aggressively as possible!

use crate::decode;
use crate::descriptor::{Descriptor, Function};
use crate::descriptors::WasmBindgenDescriptorsSection;
use crate::intrinsic::Intrinsic;
use failure::{bail, format_err, Error};
use std::borrow::Cow;
use std::collections::{HashMap, HashSet};
use std::path::PathBuf;
use std::str;
use walrus::{ExportId, FunctionId, ImportId, Module, TypedCustomSectionId};
use wasm_bindgen_shared::struct_function_export_name;
use wasm_webidl_bindings::ast;

const PLACEHOLDER_MODULE: &str = "__wbindgen_placeholder__";

mod bindings;
mod incoming;
mod outgoing;
pub mod standard;

pub use self::incoming::NonstandardIncoming;
pub use self::outgoing::NonstandardOutgoing;

/// A nonstandard wasm-bindgen-specific WebIDL custom section.
///
/// This nonstandard section is intended to convey all information that
/// wasm-bindgen itself needs to know about binding functions. This means that
/// it basically uses `NonstandardIncoming` instead of
/// `IncomingBindingExpression` and such. It's also in a bit easier to work with
/// format than the official WebIDL bindings custom section.
///
/// Note that this is intended to be consumed during generation of JS shims and
/// bindings. There it can be transformed, however, into an actual WebIDL
/// binding section using all of the values it has internally.
#[derive(Default, Debug)]
pub struct NonstandardWebidlSection {
    /// Store of all WebIDL types. Used currently to store all function types
    /// specified in `Bindings`. This is intended to be passed through verbatim
    /// to a final WebIDL bindings section.
    pub types: ast::WebidlTypes,

    /// A mapping from all bound exported functions to the binding that we have
    /// listed for them. This is the master list of every binding that will be
    /// bound and have a shim generated for it in the wasm module.
    pub exports: HashMap<ExportId, Binding>,

    /// Similar to `exports` above, except for imports. This will describe all
    /// imports from the wasm module to indicate what the JS shim is expected to
    /// do.
    pub imports: HashMap<ImportId, Binding>,

    /// For closures and such we'll be calling entries in the function table
    /// with rich arguments (just like we call exports) so to do that we
    /// describe all the elem indices that we need to modify here as well.
    ///
    /// This is a list of pairs where the first element in the list is the
    /// element index in the function table being described and the `Binding`
    /// describes the signature that it's supposed to have.
    ///
    /// The index within this table itself is then used to call actually
    /// transformed functions.
    pub elems: Vec<(u32, Binding)>,
}

pub type NonstandardWebidlSectionId = TypedCustomSectionId<NonstandardWebidlSection>;

/// A non-standard wasm-bindgen-specifi WebIDL binding. This is meant to vaguely
/// resemble a `FuctionBinding` in the official WebIDL bindings proposal, or at
/// least make it very easy to manufacture an official value from this one.
#[derive(Debug, Clone)]
pub struct Binding {
    /// The WebAssembly type that the function is expected to have. Note that
    /// this may not match the actual bound function's type! That's because this
    /// type includes `anyref` but the Rust compiler never emits anyref. This
    /// is, however, used for the `anyref` pass to know what to transform to
    /// `anyref`.
    pub wasm_ty: walrus::TypeId,

    /// The WebIDL type of this binding, which is an index into the webidl
    /// binding section's `types` field.
    pub webidl_ty: ast::WebidlFunctionId,

    /// A list of incoming bindings. For exports this is the list of arguments,
    /// and for imports this is the return value.
    pub incoming: Vec<NonstandardIncoming>,

    /// A list of outgoing bindings. For exports this is the return value and
    /// for imports this is the list of arguments.
    pub outgoing: Vec<NonstandardOutgoing>,

    /// An unfortunate necessity of today's implementation. Ideally WebIDL
    /// bindings are used with multi-value support in wasm everywhere, but today
    /// few engines support multi-value and LLVM certainly doesn't. Aggregates
    /// are then always returned through an out-ptr, so this indicates that if
    /// an out-ptr is present what wasm types are being transmitted through it.
    pub return_via_outptr: Option<Vec<walrus::ValType>>,
}

impl Binding {
    /// Does this binding's wasm function signature have any `anyref`s?
    pub fn contains_anyref(&self, module: &walrus::Module) -> bool {
        let ty = module.types.get(self.wasm_ty);
        ty.params()
            .iter()
            .chain(ty.results())
            .any(|ty| *ty == walrus::ValType::Anyref)
    }
}

/// A synthetic custom section which is not standardized, never will be, and
/// cannot be serialized or parsed. This is synthesized from all of the
/// compiler-emitted wasm-bindgen sections and then immediately removed to be
/// processed in the JS generation pass.
#[derive(Default, Debug)]
pub struct WasmBindgenAux {
    /// Extra typescript annotations that should be appended to the generated
    /// TypeScript file. This is provided via a custom attribute in Rust code.
    pub extra_typescript: String,

    /// A map from identifier to the contents of each local module defined via
    /// the `#[wasm_bindgen(module = "/foo.js")]` import options.
    pub local_modules: HashMap<String, String>,

    /// A map from unique crate identifier to the list of inline JS snippets for
    /// that crate identifier.
    pub snippets: HashMap<String, Vec<String>>,

    /// A list of all `package.json` files that are intended to be included in
    /// the final build.
    pub package_jsons: HashSet<PathBuf>,

    /// A map from exported function id to where it's expected to be exported
    /// to.
    pub export_map: HashMap<ExportId, AuxExport>,

    /// A map from imported function id to what it's expected to import.
    pub import_map: HashMap<ImportId, AuxImport>,

    /// Small bits of metadata about imports.
    pub imports_with_catch: HashSet<ImportId>,
    pub imports_with_variadic: HashSet<ImportId>,
    pub imports_with_assert_no_shim: HashSet<ImportId>,

    /// Auxiliary information to go into JS/TypeScript bindings describing the
    /// exported enums from Rust.
    pub enums: Vec<AuxEnum>,

    /// Auxiliary information to go into JS/TypeScript bindings describing the
    /// exported structs from Rust and their fields they've got exported.
    pub structs: Vec<AuxStruct>,
}

pub type WasmBindgenAuxId = TypedCustomSectionId<WasmBindgenAux>;

#[derive(Debug)]
pub struct AuxExport {
    /// When generating errors about this export, a helpful name to remember it
    /// by.
    pub debug_name: String,
    /// Comments parsed in Rust and forwarded here to show up in JS bindings.
    pub comments: String,
    /// Argument names in Rust forwarded here to configure the names that show
    /// up in TypeScript bindings.
    pub arg_names: Option<Vec<String>>,
    /// What kind of function this is and where it shows up
    pub kind: AuxExportKind,
}

/// All possible kinds of exports from a wasm module.
///
/// This `enum` says where to place an exported wasm function. For example it
/// may want to get hooked up to a JS class, or it may want to be exported as a
/// free function (etc).
///
/// TODO: it feels like this should not really be here per se. We probably want
/// to either construct the JS object itself from within wasm or somehow move
/// more of this information into some other section. Really what this is is
/// sort of an "export map" saying how to wire up all the free functions from
/// the wasm module into the output expected JS module. All our functions here
/// currently take integer parameters and require a JS wrapper, but ideally
/// we'd change them one day to taking/receiving `anyref` which then use some
/// sort of webidl import to customize behavior or something like that. In any
/// case this doesn't feel quite right in terms of priviledge separation, so
/// we'll want to work on this. For now though it works.
#[derive(Debug)]
pub enum AuxExportKind {
    /// A free function that's just listed on the exported module
    Function(String),

    /// A function that's used to create an instane of a class. The function
    /// actually return just an integer which is put on an JS object currently.
    Constructor(String),

    /// This function is intended to be a getter for a field on a class. The
    /// first argument is the internal pointer and the returned value is
    /// expected to be the field.
    Getter { class: String, field: String },

    /// This function is intended to be a setter for a field on a class. The
    /// first argument is the internal pointer and the second argument is
    /// expected to be the field's new value.
    Setter { class: String, field: String },

    /// This is a free function (ish) but scoped inside of a class name.
    StaticFunction { class: String, name: String },

    /// This is a member function of a class where the first parameter is the
    /// implicit integer stored in the class instance.
    Method {
        class: String,
        name: String,
        /// Whether or not this is calling a by-value method in Rust and should
        /// clear the internal pointer in JS automatically.
        consumed: bool,
    },
}

#[derive(Debug)]
pub struct AuxEnum {
    /// The name of this enum
    pub name: String,
    /// The copied Rust comments to forward to JS
    pub comments: String,
    /// A list of variants with their name and value
    pub variants: Vec<(String, u32)>,
}

#[derive(Debug)]
pub struct AuxStruct {
    /// The name of this struct
    pub name: String,
    /// The copied Rust comments to forward to JS
    pub comments: String,
}

/// All possible types of imports that can be imported by a wasm module.
///
/// This `enum` is intended to map out what an imported value is. For example
/// this contains a ton of shims and various ways you can call a function. The
/// base variant here is `Value` which simply means "hook this up to the import"
/// and the signatures will match up.
///
/// Note that this is *not* the same as the webidl bindings section. This is
/// intended to be coupled with that to map out what actually gets hooked up to
/// an import in the wasm module. The two work in tandem.
///
/// Some of these items here are native to JS (like `Value`, indexing
/// operations, etc). Others are shims generated by wasm-bindgen (like `Closure`
/// or `Instanceof`).
#[derive(Debug)]
pub enum AuxImport {
    /// This import is expected to simply be whatever is the value that's
    /// imported
    Value(AuxValue),

    /// This import is expected to be a function that takes an `anyref` and
    /// returns a `bool`. It's expected that it tests if the argument is an
    /// instance of (using `instanceof`) the name specified.
    ///
    /// TODO: can we use `Reflect` or something like that to avoid an extra kind
    /// of import here?
    Instanceof(JsImport),

    /// This import is expected to be a shim that returns the JS value named by
    /// `JsImport`.
    Static(JsImport),

    /// This import is intended to manufacture a JS closure with the given
    /// signature and then return that back to Rust.
    Closure {
        mutable: bool, // whether or not this was a `FnMut` closure
        dtor: u32,     // table element index of the destructor function
        binding_idx: u32,
        nargs: usize,
    },

    /// This import is expected to be a shim that simply calls the `foo` method
    /// on the first object, passing along all other parameters and returning
    /// the resulting value.
    StructuralMethod(String),

    /// This import is a "structural getter" which simply returns the `.field`
    /// value of the first argument as an object.
    ///
    /// e.g. `function(x) { return x.foo; }`
    StructuralGetter(String),

    /// This import is a "structural getter" which simply returns the `.field`
    /// value of the specified class
    ///
    /// e.g. `function() { return TheClass.foo; }`
    StructuralClassGetter(JsImport, String),

    /// This import is a "structural setter" which simply sets the `.field`
    /// value of the first argument to the second argument.
    ///
    /// e.g. `function(x, y) { x.foo = y; }`
    StructuralSetter(String),

    /// This import is a "structural setter" which simply sets the `.field`
    /// value of the specified class to the first argument of the function.
    ///
    /// e.g. `function(x) { TheClass.foo = x; }`
    StructuralClassSetter(JsImport, String),

    /// This import is expected to be a shim that is an indexing getter of the
    /// JS class here, where the first argument of the function is the field to
    /// look up. The return value is the value of the field.
    ///
    /// e.g. `function(x) { return TheClass[x]; }`
    ///
    /// TODO: can we use `Reflect` or something like that to avoid an extra kind
    /// of import here?
    IndexingGetterOfClass(JsImport),

    /// This import is expected to be a shim that is an indexing getter of the
    /// first argument interpreted as an object where the field to look up is
    /// the second argument.
    ///
    /// e.g. `function(x, y) { return x[y]; }`
    ///
    /// TODO: can we use `Reflect` or something like that to avoid an extra kind
    /// of import here?
    IndexingGetterOfObject,

    /// This import is expected to be a shim that is an indexing setter of the
    /// JS class here, where the first argument of the function is the field to
    /// set and the second is the value to set it to.
    ///
    /// e.g. `function(x, y) { TheClass[x] = y; }`
    ///
    /// TODO: can we use `Reflect` or something like that to avoid an extra kind
    /// of import here?
    IndexingSetterOfClass(JsImport),

    /// This import is expected to be a shim that is an indexing setter of the
    /// first argument interpreted as an object where the next two fields are
    /// the field to set and the value to set it to.
    ///
    /// e.g. `function(x, y, z) { x[y] = z; }`
    ///
    /// TODO: can we use `Reflect` or something like that to avoid an extra kind
    /// of import here?
    IndexingSetterOfObject,

    /// This import is expected to be a shim that is an indexing deleter of the
    /// JS class here, where the first argument of the function is the field to
    /// delete.
    ///
    /// e.g. `function(x) { delete TheClass[x]; }`
    ///
    /// TODO: can we use `Reflect` or something like that to avoid an extra kind
    /// of import here?
    IndexingDeleterOfClass(JsImport),

    /// This import is expected to be a shim that is an indexing deleter of the
    /// first argument interpreted as an object where the second argument is
    /// the field to delete.
    ///
    /// e.g. `function(x, y) { delete x[y]; }`
    ///
    /// TODO: can we use `Reflect` or something like that to avoid an extra kind
    /// of import here?
    IndexingDeleterOfObject,

    /// This import is a generated shim which will wrap the provided pointer in
    /// a JS object corresponding to the Class name given here. The class name
    /// is one that is exported from the Rust/wasm.
    ///
    /// TODO: sort of like the export map below we should ideally create the
    /// `anyref` from within Rust itself and then return it directly rather than
    /// requiring an intrinsic here to do so.
    WrapInExportedClass(String),

    /// This is an intrinsic function expected to be implemented with a JS glue
    /// shim. Each intrinsic has its own expected signature and implementation.
    Intrinsic(Intrinsic),
}

/// Values that can be imported verbatim to hook up to an import.
#[derive(Debug)]
pub enum AuxValue {
    /// A bare JS value, no transformations, just put it in the slot.
    Bare(JsImport),

    /// A getter function for the class listed for the field, acquired using
    /// `getOwnPropertyDescriptor`.
    Getter(JsImport, String),

    /// Like `Getter`, but accesses a field of a class instead of an instance
    /// of the class.
    ClassGetter(JsImport, String),

    /// Like `Getter`, except the `set` property.
    Setter(JsImport, String),

    /// Like `Setter`, but for class fields instead of instance fields.
    ClassSetter(JsImport, String),
}

/// What can actually be imported and typically a value in each of the variants
/// above of `AuxImport`
///
/// A `JsImport` is intended to indicate what exactly is being imported for a
/// particular operation.
#[derive(Debug, Hash, Eq, PartialEq, Clone)]
pub struct JsImport {
    /// The base of whatever is being imported, either from a module, the global
    /// namespace, or similar.
    pub name: JsImportName,
    /// Various field accesses (like `.foo.bar.baz`) to hang off the `name`
    /// above.
    pub fields: Vec<String>,
}

/// Return value of `determine_import` which is where we look at an imported
/// function AST and figure out where it's actually being imported from
/// (performing some validation checks and whatnot).
#[derive(Debug, Hash, Eq, PartialEq, Clone)]
pub enum JsImportName {
    /// An item is imported from the global scope. The `name` is what's
    /// imported.
    Global { name: String },
    /// Same as `Global`, except the `name` is imported via an ESM import from
    /// the specified `module` path.
    Module { module: String, name: String },
    /// Same as `Module`, except we're importing from a local module defined in
    /// a local JS snippet.
    LocalModule { module: String, name: String },
    /// Same as `Module`, except we're importing from an `inline_js` attribute
    InlineJs {
        unique_crate_identifier: String,
        snippet_idx_in_crate: usize,
        name: String,
    },
    /// A global import which may have a number of vendor prefixes associated
    /// with it, like `webkitAudioPrefix`. The `name` is the name to test
    /// whether it's prefixed.
    VendorPrefixed { name: String, prefixes: Vec<String> },
}

struct Context<'a> {
    start_found: bool,
    module: &'a mut Module,
    bindings: NonstandardWebidlSection,
    aux: WasmBindgenAux,
    function_exports: HashMap<String, (ExportId, FunctionId)>,
    function_imports: HashMap<String, (ImportId, FunctionId)>,
    vendor_prefixes: HashMap<String, Vec<String>>,
    unique_crate_identifier: &'a str,
    descriptors: HashMap<String, Descriptor>,
    anyref_enabled: bool,
    wasm_interface_types: bool,
    support_start: bool,
}

pub fn process(
    module: &mut Module,
    anyref_enabled: bool,
    wasm_interface_types: bool,
    support_start: bool,
) -> Result<(NonstandardWebidlSectionId, WasmBindgenAuxId), Error> {
    let mut storage = Vec::new();
    let programs = extract_programs(module, &mut storage)?;

    let mut cx = Context {
        bindings: Default::default(),
        aux: Default::default(),
        function_exports: Default::default(),
        function_imports: Default::default(),
        vendor_prefixes: Default::default(),
        descriptors: Default::default(),
        unique_crate_identifier: "",
        module,
        start_found: false,
        anyref_enabled,
        wasm_interface_types,
        support_start,
    };
    cx.init()?;

    for program in programs {
        cx.program(program)?;
    }

    if let Some(standard) = cx.module.customs.delete_typed::<ast::WebidlBindings>() {
        cx.standard(&standard)?;
    }

    cx.verify()?;

    let bindings = cx.module.customs.add(cx.bindings);
    let aux = cx.module.customs.add(cx.aux);
    Ok((bindings, aux))
}

impl<'a> Context<'a> {
    fn init(&mut self) -> Result<(), Error> {
        // Make a map from string name to ids of all exports
        for export in self.module.exports.iter() {
            if let walrus::ExportItem::Function(f) = export.item {
                self.function_exports
                    .insert(export.name.clone(), (export.id(), f));
            }
        }

        // Make a map from string name to ids of all imports from our
        // placeholder module name which we'll want to be sure that we've got a
        // location listed of what to import there for each item.
        let mut intrinsics = Vec::new();
        for import in self.module.imports.iter() {
            if import.module != PLACEHOLDER_MODULE {
                continue;
            }
            if let walrus::ImportKind::Function(f) = import.kind {
                self.function_imports
                    .insert(import.name.clone(), (import.id(), f));
                if let Some(intrinsic) = Intrinsic::from_symbol(&import.name) {
                    intrinsics.push((import.id(), intrinsic));
                }
            }
        }
        for (id, intrinsic) in intrinsics {
            self.bind_intrinsic(id, intrinsic)?;
        }

        self.inject_anyref_initialization()?;

        if let Some(custom) = self
            .module
            .customs
            .delete_typed::<WasmBindgenDescriptorsSection>()
        {
            let WasmBindgenDescriptorsSection {
                descriptors,
                closure_imports,
            } = *custom;
            // Store all the executed descriptors in our own field so we have
            // access to them while processing programs.
            self.descriptors.extend(descriptors);

            // Register all the injected closure imports as that they're expected
            // to manufacture a particular type of closure.
            //
            // First we register the imported function shim which returns a
            // `JsValue` for the closure. We manufacture this signature's
            // binding since it's not listed anywhere.
            //
            // Next we register the corresponding table element's binding in
            // the webidl bindings section. This binding will later be used to
            // generate a shim (if necessary) for the table element.
            //
            // Finally we store all this metadata in the import map which we've
            // learned so when a binding for the import is generated we can
            // generate all the appropriate shims.
            for (id, descriptor) in closure_imports {
                let binding = Function {
                    shim_idx: 0,
                    arguments: vec![Descriptor::I32; 3],
                    ret: Descriptor::Anyref,
                };
                bindings::register_import(
                    self.module,
                    &mut self.bindings,
                    id,
                    binding,
                    ast::WebidlFunctionKind::Static,
                )?;
                // Synthesize the two integer pointers we pass through which
                // aren't present in the signature but are present in the wasm
                // signature.
                let mut function = descriptor.function.clone();
                let nargs = function.arguments.len();
                function.arguments.insert(0, Descriptor::I32);
                function.arguments.insert(0, Descriptor::I32);
                let binding_idx = bindings::register_table_element(
                    self.module,
                    &mut self.bindings,
                    descriptor.shim_idx,
                    function,
                )?;
                self.aux.import_map.insert(
                    id,
                    AuxImport::Closure {
                        dtor: descriptor.dtor_idx,
                        mutable: descriptor.mutable,
                        binding_idx,
                        nargs,
                    },
                );
            }
        }

        Ok(())
    }

    // Ensure that the `start` function for this module calls the
    // `__wbindgen_init_anyref_table` function. This'll ensure that all
    // instances of this module have the initial slots of the anyref table
    // initialized correctly.
    //
    // Note that this is disabled if WebAssembly interface types are enabled
    // since that's a slightly different environment for now which doesn't have
    // quite the same initialization.
    fn inject_anyref_initialization(&mut self) -> Result<(), Error> {
        if !self.anyref_enabled || self.wasm_interface_types {
            return Ok(());
        }

        let ty = self.module.types.add(&[], &[]);
        let (import, import_id) =
            self.module
                .add_import_func(PLACEHOLDER_MODULE, "__wbindgen_init_anyref_table", ty);

        self.module.start = Some(match self.module.start {
            Some(prev_start) => {
                let mut builder = walrus::FunctionBuilder::new(&mut self.module.types, &[], &[]);
                builder.func_body().call(import).call(prev_start);
                builder.finish(Vec::new(), &mut self.module.funcs)
            }
            None => import,
        });
        self.bind_intrinsic(import_id, Intrinsic::InitAnyrefTable)?;

        Ok(())
    }

    fn bind_intrinsic(&mut self, id: ImportId, intrinsic: Intrinsic) -> Result<(), Error> {
        bindings::register_import(
            self.module,
            &mut self.bindings,
            id,
            intrinsic.binding(),
            ast::WebidlFunctionKind::Static,
        )?;
        self.aux
            .import_map
            .insert(id, AuxImport::Intrinsic(intrinsic));
        Ok(())
    }

    fn program(&mut self, program: decode::Program<'a>) -> Result<(), Error> {
        self.unique_crate_identifier = program.unique_crate_identifier;
        let decode::Program {
            exports,
            enums,
            imports,
            structs,
            typescript_custom_sections,
            local_modules,
            inline_js,
            unique_crate_identifier,
            package_json,
        } = program;

        for module in local_modules {
            // All local modules we find should be unique, but the same module
            // may have showed up in a few different blocks. If that's the case
            // all the same identifiers should have the same contents.
            if let Some(prev) = self
                .aux
                .local_modules
                .insert(module.identifier.to_string(), module.contents.to_string())
            {
                assert_eq!(prev, module.contents);
            }
        }
        if let Some(s) = package_json {
            self.aux.package_jsons.insert(s.into());
        }
        for export in exports {
            self.export(export)?;
        }

        // Register vendor prefixes for all types before we walk over all the
        // imports to ensure that if a vendor prefix is listed somewhere it'll
        // apply to all the imports.
        for import in imports.iter() {
            if let decode::ImportKind::Type(ty) = &import.kind {
                if ty.vendor_prefixes.len() == 0 {
                    continue;
                }
                self.vendor_prefixes
                    .entry(ty.name.to_string())
                    .or_insert(Vec::new())
                    .extend(ty.vendor_prefixes.iter().map(|s| s.to_string()));
            }
        }
        for import in imports {
            self.import(import)?;
        }

        for enum_ in enums {
            self.enum_(enum_)?;
        }
        for struct_ in structs {
            self.struct_(struct_)?;
        }
        for section in typescript_custom_sections {
            self.aux.extra_typescript.push_str(section);
            self.aux.extra_typescript.push_str("\n\n");
        }
        self.aux
            .snippets
            .entry(unique_crate_identifier.to_string())
            .or_insert(Vec::new())
            .extend(inline_js.iter().map(|s| s.to_string()));
        Ok(())
    }

    fn export(&mut self, export: decode::Export<'_>) -> Result<(), Error> {
        let wasm_name = match &export.class {
            Some(class) => struct_function_export_name(class, export.function.name),
            None => export.function.name.to_string(),
        };
        let mut descriptor = match self.descriptors.remove(&wasm_name) {
            None => return Ok(()),
            Some(d) => d.unwrap_function(),
        };
        let (export_id, id) = self.function_exports[&wasm_name];
        if export.start {
            self.add_start_function(id)?;
        }

        let kind = match export.class {
            Some(class) => {
                let class = class.to_string();
                match export.method_kind {
                    decode::MethodKind::Constructor => AuxExportKind::Constructor(class),
                    decode::MethodKind::Operation(op) => match op.kind {
                        decode::OperationKind::Getter(f) => {
                            descriptor.arguments.insert(0, Descriptor::I32);
                            AuxExportKind::Getter {
                                class,
                                field: f.to_string(),
                            }
                        }
                        decode::OperationKind::Setter(f) => {
                            descriptor.arguments.insert(0, Descriptor::I32);
                            AuxExportKind::Setter {
                                class,
                                field: f.to_string(),
                            }
                        }
                        _ if op.is_static => AuxExportKind::StaticFunction {
                            class,
                            name: export.function.name.to_string(),
                        },
                        _ => {
                            descriptor.arguments.insert(0, Descriptor::I32);
                            AuxExportKind::Method {
                                class,
                                name: export.function.name.to_string(),
                                consumed: export.consumed,
                            }
                        }
                    },
                }
            }
            None => AuxExportKind::Function(export.function.name.to_string()),
        };

        self.aux.export_map.insert(
            export_id,
            AuxExport {
                debug_name: wasm_name,
                comments: concatenate_comments(&export.comments),
                arg_names: Some(export.function.arg_names),
                kind,
            },
        );
        bindings::register_export(self.module, &mut self.bindings, export_id, descriptor)?;
        Ok(())
    }

    fn add_start_function(&mut self, id: FunctionId) -> Result<(), Error> {
        if self.start_found {
            bail!("cannot specify two `start` functions");
        }
        self.start_found = true;

        // Skip this when we're generating tests
        if !self.support_start {
            return Ok(());
        }

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
        let mut builder = walrus::FunctionBuilder::new(&mut self.module.types, &[], &[]);
        builder.func_body().call(prev_start).call(id);
        let new_start = builder.finish(Vec::new(), &mut self.module.funcs);
        self.module.start = Some(new_start);
        Ok(())
    }

    fn import(&mut self, import: decode::Import<'_>) -> Result<(), Error> {
        match &import.kind {
            decode::ImportKind::Function(f) => self.import_function(&import, f),
            decode::ImportKind::Static(s) => self.import_static(&import, s),
            decode::ImportKind::Type(t) => self.import_type(&import, t),
            decode::ImportKind::Enum(_) => Ok(()),
        }
    }

    fn import_function(
        &mut self,
        import: &decode::Import<'_>,
        function: &decode::ImportFunction<'_>,
    ) -> Result<(), Error> {
        let decode::ImportFunction {
            shim,
            catch,
            variadic,
            method,
            structural,
            function,
            assert_no_shim,
        } = function;
        let (import_id, _id) = match self.function_imports.get(*shim) {
            Some(pair) => *pair,
            None => return Ok(()),
        };
        let descriptor = match self.descriptors.remove(*shim) {
            None => return Ok(()),
            Some(d) => d.unwrap_function(),
        };

        // Record this for later as it affects JS binding generation, but note
        // that this doesn't affect the WebIDL interface at all.
        if *variadic {
            self.aux.imports_with_variadic.insert(import_id);
        }
        if *catch {
            self.aux.imports_with_catch.insert(import_id);
        }
        if *assert_no_shim {
            self.aux.imports_with_assert_no_shim.insert(import_id);
        }

        // Perform two functions here. First we're saving off our WebIDL
        // bindings signature, indicating what we think our import is going to
        // be. Next we're saving off other metadata indicating where this item
        // is going to be imported from. The `import_map` table will record, for
        // each import, what is getting hooked up to that slot of the import
        // table to the WebAssembly instance.
        let import = match method {
            Some(data) => {
                let class = self.determine_import(import, &data.class)?;
                match &data.kind {
                    // NB: `structural` is ignored for constructors since the
                    // js type isn't expected to change anyway.
                    decode::MethodKind::Constructor => {
                        bindings::register_import(
                            self.module,
                            &mut self.bindings,
                            import_id,
                            descriptor,
                            ast::WebidlFunctionKind::Constructor,
                        )?;
                        AuxImport::Value(AuxValue::Bare(class))
                    }
                    decode::MethodKind::Operation(op) => {
                        let (import, method) =
                            self.determine_import_op(class, function, *structural, op)?;
                        let kind = if method {
                            let kind = ast::WebidlFunctionKindMethod {
                                // TODO: what should this actually be?
                                ty: ast::WebidlScalarType::Any.into(),
                            };
                            ast::WebidlFunctionKind::Method(kind)
                        } else {
                            ast::WebidlFunctionKind::Static
                        };
                        bindings::register_import(
                            self.module,
                            &mut self.bindings,
                            import_id,
                            descriptor,
                            kind,
                        )?;
                        import
                    }
                }
            }

            // NB: `structural` is ignored for free functions since it's
            // expected that the binding isn't changing anyway.
            None => {
                bindings::register_import(
                    self.module,
                    &mut self.bindings,
                    import_id,
                    descriptor,
                    ast::WebidlFunctionKind::Static,
                )?;
                let name = self.determine_import(import, function.name)?;
                AuxImport::Value(AuxValue::Bare(name))
            }
        };

        self.aux.import_map.insert(import_id, import);
        Ok(())
    }

    /// The `bool` returned indicates whether the imported value should be
    /// invoked as a method (first arg is implicitly `this`) or if the imported
    /// value is a simple function-like shim
    fn determine_import_op(
        &mut self,
        mut class: JsImport,
        function: &decode::Function<'_>,
        structural: bool,
        op: &decode::Operation<'_>,
    ) -> Result<(AuxImport, bool), Error> {
        match op.kind {
            decode::OperationKind::Regular => {
                if op.is_static {
                    class.fields.push(function.name.to_string());
                    Ok((AuxImport::Value(AuxValue::Bare(class)), false))
                } else if structural {
                    Ok((
                        AuxImport::StructuralMethod(function.name.to_string()),
                        false,
                    ))
                } else {
                    class.fields.push("prototype".to_string());
                    class.fields.push(function.name.to_string());
                    Ok((AuxImport::Value(AuxValue::Bare(class)), true))
                }
            }

            decode::OperationKind::Getter(field) => {
                if structural {
                    if op.is_static {
                        Ok((
                            AuxImport::StructuralClassGetter(class, field.to_string()),
                            false,
                        ))
                    } else {
                        Ok((AuxImport::StructuralGetter(field.to_string()), false))
                    }
                } else {
                    let val = if op.is_static {
                        AuxValue::ClassGetter(class, field.to_string())
                    } else {
                        AuxValue::Getter(class, field.to_string())
                    };
                    Ok((AuxImport::Value(val), true))
                }
            }

            decode::OperationKind::Setter(field) => {
                if structural {
                    if op.is_static {
                        Ok((
                            AuxImport::StructuralClassSetter(class, field.to_string()),
                            false,
                        ))
                    } else {
                        Ok((AuxImport::StructuralSetter(field.to_string()), false))
                    }
                } else {
                    let val = if op.is_static {
                        AuxValue::ClassSetter(class, field.to_string())
                    } else {
                        AuxValue::Setter(class, field.to_string())
                    };
                    Ok((AuxImport::Value(val), true))
                }
            }

            decode::OperationKind::IndexingGetter => {
                if !structural {
                    bail!("indexing getters must always be structural");
                }
                if op.is_static {
                    Ok((AuxImport::IndexingGetterOfClass(class), false))
                } else {
                    Ok((AuxImport::IndexingGetterOfObject, false))
                }
            }

            decode::OperationKind::IndexingSetter => {
                if !structural {
                    bail!("indexing setters must always be structural");
                }
                if op.is_static {
                    Ok((AuxImport::IndexingSetterOfClass(class), false))
                } else {
                    Ok((AuxImport::IndexingSetterOfObject, false))
                }
            }

            decode::OperationKind::IndexingDeleter => {
                if !structural {
                    bail!("indexing deleters must always be structural");
                }
                if op.is_static {
                    Ok((AuxImport::IndexingDeleterOfClass(class), false))
                } else {
                    Ok((AuxImport::IndexingDeleterOfObject, false))
                }
            }
        }
    }

    fn import_static(
        &mut self,
        import: &decode::Import<'_>,
        static_: &decode::ImportStatic<'_>,
    ) -> Result<(), Error> {
        let (import_id, _id) = match self.function_imports.get(static_.shim) {
            Some(pair) => *pair,
            None => return Ok(()),
        };

        // Register the signature of this imported shim
        bindings::register_import(
            self.module,
            &mut self.bindings,
            import_id,
            Function {
                arguments: Vec::new(),
                shim_idx: 0,
                ret: Descriptor::Anyref,
            },
            ast::WebidlFunctionKind::Static,
        )?;

        // And then save off that this function is is an instanceof shim for an
        // imported item.
        let import = self.determine_import(import, &static_.name)?;
        self.aux
            .import_map
            .insert(import_id, AuxImport::Static(import));
        Ok(())
    }

    fn import_type(
        &mut self,
        import: &decode::Import<'_>,
        type_: &decode::ImportType<'_>,
    ) -> Result<(), Error> {
        let (import_id, _id) = match self.function_imports.get(type_.instanceof_shim) {
            Some(pair) => *pair,
            None => return Ok(()),
        };

        // Register the signature of this imported shim
        bindings::register_import(
            self.module,
            &mut self.bindings,
            import_id,
            Function {
                arguments: vec![Descriptor::Ref(Box::new(Descriptor::Anyref))],
                shim_idx: 0,
                ret: Descriptor::Boolean,
            },
            ast::WebidlFunctionKind::Static,
        )?;

        // And then save off that this function is is an instanceof shim for an
        // imported item.
        let import = self.determine_import(import, &type_.name)?;
        self.aux
            .import_map
            .insert(import_id, AuxImport::Instanceof(import));
        Ok(())
    }

    fn enum_(&mut self, enum_: decode::Enum<'_>) -> Result<(), Error> {
        let aux = AuxEnum {
            name: enum_.name.to_string(),
            comments: concatenate_comments(&enum_.comments),
            variants: enum_
                .variants
                .iter()
                .map(|v| (v.name.to_string(), v.value))
                .collect(),
        };
        self.aux.enums.push(aux);
        Ok(())
    }

    fn struct_(&mut self, struct_: decode::Struct<'_>) -> Result<(), Error> {
        for field in struct_.fields {
            let getter = wasm_bindgen_shared::struct_field_get(&struct_.name, &field.name);
            let setter = wasm_bindgen_shared::struct_field_set(&struct_.name, &field.name);
            let descriptor = match self.descriptors.remove(&getter) {
                None => continue,
                Some(d) => d,
            };

            // Register a webidl transformation for the getter
            let (getter_id, _) = self.function_exports[&getter];
            let getter_descriptor = Function {
                arguments: vec![Descriptor::I32],
                shim_idx: 0,
                ret: descriptor.clone(),
            };
            bindings::register_export(
                self.module,
                &mut self.bindings,
                getter_id,
                getter_descriptor,
            )?;
            self.aux.export_map.insert(
                getter_id,
                AuxExport {
                    debug_name: format!("getter for `{}::{}`", struct_.name, field.name),
                    arg_names: None,
                    comments: concatenate_comments(&field.comments),
                    kind: AuxExportKind::Getter {
                        class: struct_.name.to_string(),
                        field: field.name.to_string(),
                    },
                },
            );

            // If present, register information for the setter as well.
            if field.readonly {
                continue;
            }

            let (setter_id, _) = self.function_exports[&setter];
            let setter_descriptor = Function {
                arguments: vec![Descriptor::I32, descriptor],
                shim_idx: 0,
                ret: Descriptor::Unit,
            };
            bindings::register_export(
                self.module,
                &mut self.bindings,
                setter_id,
                setter_descriptor,
            )?;
            self.aux.export_map.insert(
                setter_id,
                AuxExport {
                    debug_name: format!("setter for `{}::{}`", struct_.name, field.name),
                    arg_names: None,
                    comments: concatenate_comments(&field.comments),
                    kind: AuxExportKind::Setter {
                        class: struct_.name.to_string(),
                        field: field.name.to_string(),
                    },
                },
            );
        }
        let aux = AuxStruct {
            name: struct_.name.to_string(),
            comments: concatenate_comments(&struct_.comments),
        };
        self.aux.structs.push(aux);

        let wrap_constructor = wasm_bindgen_shared::new_function(struct_.name);
        if let Some((import_id, _id)) = self.function_imports.get(&wrap_constructor) {
            self.aux.import_map.insert(
                *import_id,
                AuxImport::WrapInExportedClass(struct_.name.to_string()),
            );
            let binding = Function {
                shim_idx: 0,
                arguments: vec![Descriptor::I32],
                ret: Descriptor::Anyref,
            };
            bindings::register_import(
                self.module,
                &mut self.bindings,
                *import_id,
                binding,
                ast::WebidlFunctionKind::Static,
            )?;
        }

        Ok(())
    }

    fn determine_import(&self, import: &decode::Import<'_>, item: &str) -> Result<JsImport, Error> {
        let is_local_snippet = match import.module {
            decode::ImportModule::Named(s) => self.aux.local_modules.contains_key(s),
            decode::ImportModule::RawNamed(_) => false,
            decode::ImportModule::Inline(_) => true,
            decode::ImportModule::None => false,
        };

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
            return Ok(JsImport {
                name: JsImportName::VendorPrefixed {
                    name: item.to_string(),
                    prefixes: vendor_prefixes.clone(),
                },
                fields: Vec::new(),
            });
        }

        let (name, fields) = match import.js_namespace {
            Some(ns) => (ns, vec![item.to_string()]),
            None => (item, Vec::new()),
        };

        let name = match import.module {
            decode::ImportModule::Named(module) if is_local_snippet => JsImportName::LocalModule {
                module: module.to_string(),
                name: name.to_string(),
            },
            decode::ImportModule::Named(module) | decode::ImportModule::RawNamed(module) => {
                JsImportName::Module {
                    module: module.to_string(),
                    name: name.to_string(),
                }
            }
            decode::ImportModule::Inline(idx) => {
                let offset = self
                    .aux
                    .snippets
                    .get(self.unique_crate_identifier)
                    .map(|s| s.len())
                    .unwrap_or(0);
                JsImportName::InlineJs {
                    unique_crate_identifier: self.unique_crate_identifier.to_string(),
                    snippet_idx_in_crate: idx as usize + offset,
                    name: name.to_string(),
                }
            }
            decode::ImportModule::None => JsImportName::Global {
                name: name.to_string(),
            },
        };
        Ok(JsImport { name, fields })
    }

    /// Processes bindings from a standard WebIDL bindings custom section.
    ///
    /// No module coming out of the Rust compiler will have one of these, but
    /// eventually there's going to be other producers of the WebIDL bindings
    /// custom section as well. This functionality is intended to allow
    /// `wasm-bindgen`-the-CLI-tool to act as a polyfill for those modules as
    /// well as Rust modules.
    ///
    /// Here a standard `WebidlBindings` custom section is taken and we process
    /// that into our own internal data structures to ensure that we have a
    /// binding listed for all the described bindings.
    ///
    /// In other words, this is a glorified conversion from the "official"
    /// WebIDL bindings custom section into the wasm-bindgen internal
    /// representation.
    fn standard(&mut self, std: &ast::WebidlBindings) -> Result<(), Error> {
        for (_id, bind) in std.binds.iter() {
            let binding = self.standard_binding(std, bind)?;
            let func = self.module.funcs.get(bind.func);
            match &func.kind {
                walrus::FunctionKind::Import(i) => {
                    let id = i.import;
                    self.standard_import(binding, id)?;
                }
                walrus::FunctionKind::Local(_) => {
                    let export = self
                        .module
                        .exports
                        .iter()
                        .find(|e| match e.item {
                            walrus::ExportItem::Function(f) => f == bind.func,
                            _ => false,
                        })
                        .ok_or_else(|| format_err!("missing export function for webidl binding"))?;
                    let id = export.id();
                    self.standard_export(binding, id)?;
                }
                walrus::FunctionKind::Uninitialized(_) => unreachable!(),
            }
        }
        Ok(())
    }

    /// Creates a wasm-bindgen-internal `Binding` from an official `Bind`
    /// structure specified in the upstream binary format.
    ///
    /// This will largely just copy some things into our own arenas but also
    /// processes the list of binding expressions into our own representations.
    fn standard_binding(
        &mut self,
        std: &ast::WebidlBindings,
        bind: &ast::Bind,
    ) -> Result<Binding, Error> {
        let binding: &ast::FunctionBinding = std
            .bindings
            .get(bind.binding)
            .ok_or_else(|| format_err!("bad binding id"))?;
        let mut return_via_outptr = None;
        let (wasm_ty, webidl_ty, incoming, outgoing) = match binding {
            ast::FunctionBinding::Export(e) => {
                // This `match` is weird, see the comment at the top of
                // `standard.rs` for what it's doing.
                let outgoing = match e.result.bindings.get(0) {
                    Some(ast::OutgoingBindingExpression::As(a)) if a.idx == u32::max_value() => {
                        return_via_outptr = Some(vec![walrus::ValType::I32, walrus::ValType::I32]);
                        &e.result.bindings[1..]
                    }
                    _ => &e.result.bindings[..],
                };
                (
                    e.wasm_ty,
                    e.webidl_ty,
                    e.params.bindings.as_slice(),
                    outgoing,
                )
            }
            ast::FunctionBinding::Import(e) => {
                // This `match` is weird, see the comment at the top of
                // `standard.rs` for what it's doing.
                let incoming = match e.result.bindings.get(0) {
                    Some(ast::IncomingBindingExpression::Get(g)) if g.idx == u32::max_value() => {
                        return_via_outptr = Some(vec![walrus::ValType::I32, walrus::ValType::I32]);
                        &e.result.bindings[1..]
                    }
                    _ => &e.result.bindings[..],
                };
                (
                    e.wasm_ty,
                    e.webidl_ty,
                    incoming,
                    e.params.bindings.as_slice(),
                )
            }
        };
        let webidl_ty = standard::copy_ty(&mut self.bindings.types, webidl_ty, &std.types);
        let webidl_ty = match webidl_ty {
            ast::WebidlTypeRef::Id(id) => <ast::WebidlFunction as ast::WebidlTypeId>::wrap(id),
            _ => bail!("invalid webidl type listed"),
        };
        Ok(Binding {
            wasm_ty,
            webidl_ty,
            incoming: incoming
                .iter()
                .cloned()
                .map(NonstandardIncoming::Standard)
                .collect(),
            outgoing: outgoing
                .iter()
                .cloned()
                .map(NonstandardOutgoing::Standard)
                .collect(),
            return_via_outptr,
        })
    }

    /// Registers that `id` has a `binding` which was read from a standard
    /// webidl bindings section, so the source of `id` is its actual module/name
    /// listed in the wasm module.
    fn standard_import(&mut self, binding: Binding, id: walrus::ImportId) -> Result<(), Error> {
        let import = self.module.imports.get(id);
        let js = JsImport {
            name: JsImportName::Module {
                module: import.module.clone(),
                name: import.name.clone(),
            },
            fields: Vec::new(),
        };
        let value = AuxValue::Bare(js);
        assert!(self
            .aux
            .import_map
            .insert(id, AuxImport::Value(value))
            .is_none());
        assert!(self.bindings.imports.insert(id, binding).is_none());

        Ok(())
    }

    /// Registers that `id` has a `binding` and comes from a standard webidl
    /// bindings section so it doesn't have any documentation or debug names we
    /// can work with.
    fn standard_export(&mut self, binding: Binding, id: walrus::ExportId) -> Result<(), Error> {
        let export = self.module.exports.get(id);
        let kind = AuxExportKind::Function(export.name.clone());
        let export = AuxExport {
            debug_name: format!("standard export {:?}", id),
            comments: String::new(),
            arg_names: None,
            kind,
        };
        assert!(self.aux.export_map.insert(id, export).is_none());
        assert!(self.bindings.exports.insert(id, binding).is_none());
        Ok(())
    }

    /// Perform a small verification pass over the module to perform some
    /// internal sanity checks.
    fn verify(&self) -> Result<(), Error> {
        let mut imports_counted = 0;
        for import in self.module.imports.iter() {
            if import.module != PLACEHOLDER_MODULE {
                continue;
            }
            match import.kind {
                walrus::ImportKind::Function(_) => {}
                _ => bail!("import from `{}` was not a function", PLACEHOLDER_MODULE),
            }

            // Ensure that everything imported from the `__wbindgen_placeholder__`
            // module has a location listed as to where it's expected to be
            // imported from.
            if !self.aux.import_map.contains_key(&import.id()) {
                bail!(
                    "import of `{}` doesn't have an import map item listed",
                    import.name
                );
            }

            // Also make sure there's a binding listed for it.
            if !self.bindings.imports.contains_key(&import.id()) {
                bail!("import of `{}` doesn't have a binding listed", import.name);
            }
            imports_counted += 1;
        }

        // Make sure there's no extraneous bindings that weren't actually
        // imported in the module.
        if self.aux.import_map.len() != imports_counted {
            bail!("import map is larger than the number of imports");
        }
        if self.bindings.imports.len() != imports_counted {
            bail!("import binding map is larger than the number of imports");
        }

        // Make sure the export map and export bindings map contain the same
        // number of entries.
        for id in self.bindings.exports.keys() {
            if !self.aux.export_map.contains_key(id) {
                bail!("bindings map has an entry that the export map does not");
            }
        }

        if self.bindings.exports.len() != self.aux.export_map.len() {
            bail!("export map and export bindings map have different sizes");
        }

        Ok(())
    }
}

impl walrus::CustomSection for NonstandardWebidlSection {
    fn name(&self) -> &str {
        "webidl custom section"
    }

    fn data(&self, _: &walrus::IdsToIndices) -> Cow<[u8]> {
        panic!("shouldn't emit custom sections just yet");
    }
}

impl walrus::CustomSection for WasmBindgenAux {
    fn name(&self) -> &str {
        "wasm-bindgen custom section"
    }

    fn data(&self, _: &walrus::IdsToIndices) -> Cow<[u8]> {
        panic!("shouldn't emit custom sections just yet");
    }
}

fn extract_programs<'a>(
    module: &mut Module,
    program_storage: &'a mut Vec<Vec<u8>>,
) -> Result<Vec<decode::Program<'a>>, Error> {
    let my_version = wasm_bindgen_shared::version();
    assert!(program_storage.is_empty());

    while let Some(raw) = module.customs.remove_raw("__wasm_bindgen_unstable") {
        log::debug!(
            "custom section '{}' looks like a wasm bindgen section",
            raw.name
        );
        program_storage.push(raw.data);
    }

    let mut ret = Vec::new();
    for program in program_storage.iter() {
        let mut payload = &program[..];
        while let Some(data) = get_remaining(&mut payload) {
            // Historical versions of wasm-bindgen have used JSON as the custom
            // data section format. Newer versions, however, are using a custom
            // serialization protocol that looks much more like the wasm spec.
            //
            // We, however, want a sanity check to ensure that if we're running
            // against the wrong wasm-bindgen we get a nicer error than an
            // internal decode error. To that end we continue to verify a tiny
            // bit of json at the beginning of each blob before moving to the
            // next blob. This should keep us compatible with older wasm-bindgen
            // instances as well as forward-compatible for now.
            //
            // Note, though, that as `wasm-pack` picks up steam it's hoped we
            // can just delete this entirely. The `wasm-pack` project already
            // manages versions for us, so we in theory should need this check
            // less and less over time.
            if let Some(their_version) = verify_schema_matches(data)? {
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
to open an issue at https://github.com/rustwasm/wasm-bindgen/issues!
",
                    their_version,
                    my_version,
                );
            }
            let next = get_remaining(&mut payload).unwrap();
            log::debug!("found a program of length {}", next.len());
            ret.push(<decode::Program as decode::Decode>::decode_all(next));
        }
    }
    Ok(ret)
}

fn get_remaining<'a>(data: &mut &'a [u8]) -> Option<&'a [u8]> {
    if data.len() == 0 {
        return None;
    }
    let len = ((data[0] as usize) << 0)
        | ((data[1] as usize) << 8)
        | ((data[2] as usize) << 16)
        | ((data[3] as usize) << 24);
    let (a, b) = data[4..].split_at(len);
    *data = b;
    Some(a)
}

fn verify_schema_matches<'a>(data: &'a [u8]) -> Result<Option<&'a str>, Error> {
    macro_rules! bad {
        () => {
            bail!("failed to decode what looked like wasm-bindgen data")
        };
    }
    let data = match str::from_utf8(data) {
        Ok(s) => s,
        Err(_) => bad!(),
    };
    log::debug!("found version specifier {}", data);
    if !data.starts_with("{") || !data.ends_with("}") {
        bad!()
    }
    let needle = "\"schema_version\":\"";
    let rest = match data.find(needle) {
        Some(i) => &data[i + needle.len()..],
        None => bad!(),
    };
    let their_schema_version = match rest.find("\"") {
        Some(i) => &rest[..i],
        None => bad!(),
    };
    if their_schema_version == wasm_bindgen_shared::SCHEMA_VERSION {
        return Ok(None);
    }
    let needle = "\"version\":\"";
    let rest = match data.find(needle) {
        Some(i) => &data[i + needle.len()..],
        None => bad!(),
    };
    let their_version = match rest.find("\"") {
        Some(i) => &rest[..i],
        None => bad!(),
    };
    Ok(Some(their_version))
}

fn concatenate_comments(comments: &[&str]) -> String {
    comments
        .iter()
        .map(|s| s.trim_matches('"'))
        .collect::<Vec<_>>()
        .join("\n")
}

/// Do we need to generate JS glue shims for these incoming bindings?
pub fn incoming_do_not_require_glue(
    exprs: &[NonstandardIncoming],
    from_webidl_tys: &[ast::WebidlTypeRef],
    to_wasm_tys: &[walrus::ValType],
    standard_webidl_enabled: bool,
) -> bool {
    // If anything is nonstandard, then we're unconditionally going to need a JS
    // shim because, well, it's not standard.
    if exprs.iter().any(|e| match e {
        NonstandardIncoming::Standard(_) => false,
        _ => true,
    }) {
        return false;
    }

    // If everything is `Standard` and we've actually got WebIDL bindings fully
    // enabled, then we don't require any glue at all!
    if standard_webidl_enabled {
        return true;
    }

    exprs.len() == from_webidl_tys.len()
        && exprs.len() == to_wasm_tys.len()
        && exprs
            .iter()
            .zip(from_webidl_tys)
            .zip(to_wasm_tys)
            .enumerate()
            .all(|(i, ((expr, from_webidl_ty), to_wasm_ty))| match expr {
                NonstandardIncoming::Standard(e) => e.is_expressible_in_js_without_webidl_bindings(
                    *from_webidl_ty,
                    *to_wasm_ty,
                    i as u32,
                ),
                _ => false,
            })
}

/// Do we need to generate JS glue shims for these outgoing bindings?
pub fn outgoing_do_not_require_glue(
    exprs: &[NonstandardOutgoing],
    from_wasm_tys: &[walrus::ValType],
    to_webidl_tys: &[ast::WebidlTypeRef],
    standard_webidl_enabled: bool,
) -> bool {
    // Same short-circuits as above.
    if exprs.iter().any(|e| match e {
        NonstandardOutgoing::Standard(_) => false,
        _ => true,
    }) {
        return false;
    }
    if standard_webidl_enabled {
        return true;
    }

    exprs.len() == from_wasm_tys.len()
        && exprs.len() == to_webidl_tys.len()
        && exprs
            .iter()
            .zip(from_wasm_tys)
            .zip(to_webidl_tys)
            .enumerate()
            .all(|(i, ((expr, from_wasm_ty), to_webidl_ty))| match expr {
                NonstandardOutgoing::Standard(e) => e.is_expressible_in_js_without_webidl_bindings(
                    *from_wasm_ty,
                    *to_webidl_ty,
                    i as u32,
                ),
                _ => false,
            })
}
