use crate::descriptor::{Descriptor, Function};
use crate::descriptors::WasmBindgenDescriptorsSection;
use crate::intrinsic::Intrinsic;
use crate::{decode, PLACEHOLDER_MODULE};
use anyhow::{anyhow, bail, Error};
use std::collections::{HashMap, HashSet};
use std::str;
use walrus::MemoryId;
use walrus::{ExportId, FunctionId, ImportId, Module};
use wasm_bindgen_shared::struct_function_export_name;

mod incoming;
mod nonstandard;
mod outgoing;
pub mod section;
mod standard;
pub use self::nonstandard::*;
pub use self::standard::*;

struct Context<'a> {
    start_found: bool,
    module: &'a mut Module,
    adapters: NonstandardWitSection,
    aux: WasmBindgenAux,
    function_exports: HashMap<String, (ExportId, FunctionId)>,
    function_imports: HashMap<String, (ImportId, FunctionId)>,
    memory: Option<MemoryId>,
    vendor_prefixes: HashMap<String, Vec<String>>,
    unique_crate_identifier: &'a str,
    descriptors: HashMap<String, Descriptor>,
    anyref_enabled: bool,
    wasm_interface_types: bool,
    support_start: bool,
}

struct InstructionBuilder<'a, 'b> {
    input: Vec<AdapterType>,
    output: Vec<AdapterType>,
    instructions: Vec<InstructionData>,
    cx: &'a mut Context<'b>,
    return_position: bool,
}

pub fn process(
    module: &mut Module,
    anyref_enabled: bool,
    wasm_interface_types: bool,
    support_start: bool,
) -> Result<(NonstandardWitSectionId, WasmBindgenAuxId), Error> {
    let mut storage = Vec::new();
    let programs = extract_programs(module, &mut storage)?;

    let mut cx = Context {
        adapters: Default::default(),
        aux: Default::default(),
        function_exports: Default::default(),
        function_imports: Default::default(),
        vendor_prefixes: Default::default(),
        descriptors: Default::default(),
        unique_crate_identifier: "",
        memory: wasm_bindgen_wasm_conventions::get_memory(module).ok(),
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

    if !cx.start_found {
        cx.discover_main()?;
    }

    if let Some(standard) = cx.module.customs.delete_typed() {
        cx.standard(&standard)?;
    }

    cx.verify()?;

    cx.unexport_intrinsics();

    let adapters = cx.module.customs.add(cx.adapters);
    let aux = cx.module.customs.add(cx.aux);
    Ok((adapters, aux))
}

impl<'a> Context<'a> {
    fn init(&mut self) -> Result<(), Error> {
        self.aux.shadow_stack_pointer =
            wasm_bindgen_wasm_conventions::get_shadow_stack_pointer(self.module);

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
        let mut duplicate_import_map = HashMap::new();
        let mut imports_to_delete = HashSet::new();
        for import in self.module.imports.iter() {
            if import.module != PLACEHOLDER_MODULE {
                continue;
            }
            let f = match import.kind {
                walrus::ImportKind::Function(f) => f,
                _ => continue,
            };

            match self.function_imports.get(&import.name) {
                // If this `import.name` already exists in our import map, then
                // we need to delete `import`. We also need to replace any
                // references to it with `prev_func`, so register that here to
                // happen later.
                Some((_, prev_func)) => {
                    imports_to_delete.insert(import.id());
                    duplicate_import_map.insert(f, *prev_func);
                }

                // Otherwise this is brand new, so insert it into the map.
                None => {
                    self.function_imports
                        .insert(import.name.clone(), (import.id(), f));
                }
            }

            // Test to see if this is an intrinsic symbol, in which case we'll
            // process this later.
            if let Some(intrinsic) = Intrinsic::from_symbol(&import.name) {
                intrinsics.push((import.id(), intrinsic));
            }
        }
        for (id, intrinsic) in intrinsics {
            self.bind_intrinsic(id, intrinsic)?;
        }
        for import in imports_to_delete {
            self.module.imports.delete(import);
        }
        self.handle_duplicate_imports(&duplicate_import_map);

        self.inject_anyref_initialization()?;

        if let Some(custom) = self
            .module
            .customs
            .delete_typed::<WasmBindgenDescriptorsSection>()
        {
            let WasmBindgenDescriptorsSection {
                descriptors,
                closure_imports,
                ..
            } = *custom;
            // Store all the executed descriptors in our own field so we have
            // access to them while processing programs.
            self.descriptors.extend(descriptors);

            // If any closures exist we need to prevent the function table from
            // getting gc'd
            if closure_imports.len() > 0 {
                self.aux.function_table = self.module.tables.main_function_table()?;
            }

            // Register all the injected closure imports as that they're expected
            // to manufacture a particular type of closure.
            //
            // First we register the imported function shim which returns a
            // `JsValue` for the closure. We manufacture this signature
            // since it's not listed anywhere.
            //
            // Next we register the corresponding table element's signature in
            // the interface types section. This adapter will later be used to
            // generate a shim (if necessary) for the table element.
            //
            // Finally we store all this metadata in the import map which we've
            // learned so when a binding for the import is generated we can
            // generate all the appropriate shims.
            for (id, descriptor) in closure_imports {
                let signature = Function {
                    shim_idx: 0,
                    arguments: vec![Descriptor::I32; 3],
                    ret: Descriptor::Anyref,
                };
                let id = self.import_adapter(id, signature, AdapterJsImportKind::Normal)?;
                // Synthesize the two integer pointers we pass through which
                // aren't present in the signature but are present in the wasm
                // signature.
                let mut function = descriptor.function.clone();
                let nargs = function.arguments.len();
                function.arguments.insert(0, Descriptor::I32);
                function.arguments.insert(0, Descriptor::I32);
                let adapter = self.table_element_adapter(descriptor.shim_idx, function)?;
                self.aux.import_map.insert(
                    id,
                    AuxImport::Closure {
                        dtor: descriptor.dtor_idx,
                        mutable: descriptor.mutable,
                        nargs,
                        adapter,
                    },
                );
            }
        }

        Ok(())
    }

    /// The same name function from the same module may be imported at different
    /// points in a program. The compiler may synthesize two `import`
    /// statements, both with the same module/name, to match these two function
    /// imports. This is handled here.
    ///
    /// Almost all of our handling of directives and such is string-based (eew)
    /// instead of ID based due to the way the macro works right now. This means
    /// that we don't work well with these duplicate imports. As a result when
    /// we see these duplicate imports we fixup the module to ensure that only
    /// one import is used, deleting all the other imports. This is what's
    /// wanted anyway in terms of semantics.
    ///
    /// The map provided here is a map where the key is a function id to replace
    /// and the value is what to replace it with.
    fn handle_duplicate_imports(&mut self, map: &HashMap<FunctionId, FunctionId>) {
        struct Replace<'a> {
            map: &'a HashMap<FunctionId, FunctionId>,
        }
        impl walrus::ir::VisitorMut for Replace<'_> {
            fn visit_function_id_mut(&mut self, function: &mut FunctionId) {
                if let Some(replacement) = self.map.get(function) {
                    *function = *replacement;
                }
            }
        }
        let mut replace = Replace { map };
        for (_id, func) in self.module.funcs.iter_local_mut() {
            let entry = func.entry_block();
            walrus::ir::dfs_pre_order_mut(&mut replace, func, entry);
        }
    }

    // Discover a function `main(i32, i32) -> i32` and, if it exists, make that function run at module start.
    fn discover_main(&mut self) -> Result<(), Error> {
        // find a `main(i32, i32) -> i32`
        let main_id = self
            .module
            .functions()
            .find(|x| {
                use walrus::ValType::I32;
                // name has to be `main`
                let name_matches = x.name.as_ref().map_or(false, |x| x == "main");
                // type has to be `(i32, i32) -> i32`
                let ty = self.module.types.get(x.ty());
                let type_matches = ty.params() == [I32, I32] && ty.results() == [I32];
                name_matches && type_matches
            })
            .map(|x| x.id());
        let main_id = match main_id {
            Some(x) => x,
            None => return Ok(()),
        };

        // build a wrapper to zero out the arguments and ignore the return value
        let mut wrapper = walrus::FunctionBuilder::new(&mut self.module.types, &[], &[]);
        wrapper
            .func_body()
            .i32_const(0)
            .i32_const(0)
            .call(main_id)
            .drop()
            .return_();
        let wrapper = wrapper.finish(vec![], &mut self.module.funcs);

        // call that wrapper when the module starts
        self.add_start_function(wrapper)?;

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
        if let Intrinsic::FunctionTable = intrinsic {
            self.aux.function_table = self.module.tables.main_function_table()?;
        }
        let id = self.import_adapter(id, intrinsic.signature(), AdapterJsImportKind::Normal)?;
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

        let id = self.export_adapter(export_id, descriptor)?;
        self.aux.export_map.insert(
            id,
            AuxExport {
                debug_name: wasm_name,
                comments: concatenate_comments(&export.comments),
                arg_names: Some(export.function.arg_names),
                kind,
                generate_typescript: export.function.generate_typescript,
            },
        );
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

        // Perform two functions here. First we're saving off our adapter
        // signature, indicating what we think our import is going to be. Next
        // we're saving off other metadata indicating where this item is going
        // to be imported from. The `import_map` table will record, for each
        // import, what is getting hooked up to that slot of the import table
        // to the WebAssembly instance.
        let (id, import) = match method {
            Some(data) => {
                let class = self.determine_import(import, &data.class)?;
                match &data.kind {
                    // NB: `structural` is ignored for constructors since the
                    // js type isn't expected to change anyway.
                    decode::MethodKind::Constructor => {
                        let id = self.import_adapter(
                            import_id,
                            descriptor,
                            AdapterJsImportKind::Constructor,
                        )?;
                        (id, AuxImport::Value(AuxValue::Bare(class)))
                    }
                    decode::MethodKind::Operation(op) => {
                        let (import, method) =
                            self.determine_import_op(class, function, *structural, op)?;
                        let kind = if method {
                            AdapterJsImportKind::Method
                        } else {
                            AdapterJsImportKind::Normal
                        };
                        (self.import_adapter(import_id, descriptor, kind)?, import)
                    }
                }
            }

            // NB: `structural` is ignored for free functions since it's
            // expected that the binding isn't changing anyway.
            None => {
                let id = self.import_adapter(import_id, descriptor, AdapterJsImportKind::Normal)?;
                let name = self.determine_import(import, function.name)?;
                (id, AuxImport::Value(AuxValue::Bare(name)))
            }
        };

        // Record this for later as it affects JS binding generation, but note
        // that this doesn't affect the WebIDL interface at all.
        if *variadic {
            self.aux.imports_with_variadic.insert(id);
        }

        // Note that `catch`/`assert_no_shim` is applied not to the import
        // itself but to the adapter shim we generated, so fetch that shim id
        // and flag it as catch here. This basically just needs to be kept in
        // sync with `js/mod.rs`.
        //
        // For `catch` once we see that we'll need an internal intrinsic later
        // for JS glue generation, so be sure to find that here.
        let adapter = self.adapters.implements.last().unwrap().2;
        if *catch {
            self.aux.imports_with_catch.insert(adapter);
            if self.aux.exn_store.is_none() {
                self.find_exn_store();
            }
        }
        if *assert_no_shim {
            self.aux.imports_with_assert_no_shim.insert(adapter);
        }

        self.aux.import_map.insert(id, import);
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
                    Ok((
                        AuxImport::ValueWithThis(class, function.name.to_string()),
                        false,
                    ))
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

        let descriptor = match self.descriptors.remove(static_.shim) {
            None => return Ok(()),
            Some(d) => d,
        };

        // Register the signature of this imported shim
        let id = self.import_adapter(
            import_id,
            Function {
                arguments: Vec::new(),
                shim_idx: 0,
                ret: descriptor,
            },
            AdapterJsImportKind::Normal,
        )?;

        // And then save off that this function is is an instanceof shim for an
        // imported item.
        let import = self.determine_import(import, &static_.name)?;
        self.aux.import_map.insert(id, AuxImport::Static(import));
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
        let id = self.import_adapter(
            import_id,
            Function {
                arguments: vec![Descriptor::Ref(Box::new(Descriptor::Anyref))],
                shim_idx: 0,
                ret: Descriptor::Boolean,
            },
            AdapterJsImportKind::Normal,
        )?;

        // And then save off that this function is is an instanceof shim for an
        // imported item.
        let import = self.determine_import(import, &type_.name)?;
        self.aux
            .import_map
            .insert(id, AuxImport::Instanceof(import));
        Ok(())
    }

    fn enum_(&mut self, enum_: decode::Enum<'_>) -> Result<(), Error> {
        let aux = AuxEnum {
            name: enum_.name.to_string(),
            comments: concatenate_comments(&enum_.comments),
            variants: enum_
                .variants
                .iter()
                .map(|v| {
                    (
                        v.name.to_string(),
                        v.value,
                        concatenate_comments(&v.comments),
                    )
                })
                .collect(),
            generate_typescript: enum_.generate_typescript,
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
            let getter_id = self.export_adapter(getter_id, getter_descriptor)?;
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
                    generate_typescript: field.generate_typescript,
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
            let setter_id = self.export_adapter(setter_id, setter_descriptor)?;
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
                    generate_typescript: field.generate_typescript,
                },
            );
        }
        let aux = AuxStruct {
            name: struct_.name.to_string(),
            comments: concatenate_comments(&struct_.comments),
            is_inspectable: struct_.is_inspectable,
            generate_typescript: struct_.generate_typescript,
        };
        self.aux.structs.push(aux);

        let wrap_constructor = wasm_bindgen_shared::new_function(struct_.name);
        if let Some((import_id, _id)) = self.function_imports.get(&wrap_constructor).cloned() {
            let signature = Function {
                shim_idx: 0,
                arguments: vec![Descriptor::I32],
                ret: Descriptor::Anyref,
            };
            let id = self.import_adapter(import_id, signature, AdapterJsImportKind::Normal)?;
            self.aux
                .import_map
                .insert(id, AuxImport::WrapInExportedClass(struct_.name.to_string()));
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

    fn standard(&mut self, std: &wit_walrus::WasmInterfaceTypes) -> Result<(), Error> {
        let mut walrus2us = HashMap::new();
        let params_and_results = |id: wit_walrus::TypeId| -> (Vec<_>, Vec<_>) {
            let ty = std.types.get(id);
            let params = ty
                .params()
                .iter()
                .cloned()
                .map(AdapterType::from_wit)
                .collect();
            let results = ty
                .results()
                .iter()
                .cloned()
                .map(AdapterType::from_wit)
                .collect();
            (params, results)
        };

        // Register all imports, allocating our own id for them and configuring
        // where the JS value for the import is coming from.
        for import in std.imports.iter() {
            let func = std.funcs.get(import.func);
            let (params, results) = params_and_results(func.ty);
            let id = self.adapters.append(
                params,
                results,
                AdapterKind::Import {
                    module: import.module.clone(),
                    name: import.name.clone(),
                    kind: AdapterJsImportKind::Normal,
                },
            );
            walrus2us.insert(import.func, id);
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
        }

        // Register all functions, allocating our own id system for each of the
        // functions.
        for func in std.funcs.iter() {
            if let wit_walrus::FuncKind::Import(_) = func.kind {
                continue;
            }
            let (params, results) = params_and_results(func.ty);
            walrus2us.insert(
                func.id(),
                self.adapters.append(
                    params,
                    results,
                    AdapterKind::Local {
                        instructions: Vec::new(),
                    },
                ),
            );
        }

        // .. and then actually translate all functions using our id mapping,
        // now that we're able to remap all the `CallAdapter` instructions.
        for func in std.funcs.iter() {
            let instrs = match &func.kind {
                wit_walrus::FuncKind::Local(instrs) => instrs,
                wit_walrus::FuncKind::Import(_) => continue,
            };
            let instrs = instrs
                .iter()
                .map(|i| match i {
                    wit_walrus::Instruction::CallAdapter(f) => {
                        Instruction::CallAdapter(walrus2us[&f])
                    }
                    other => Instruction::Standard(other.clone()),
                })
                .map(|instr| InstructionData {
                    instr,
                    stack_change: StackChange::Unknown,
                })
                .collect::<Vec<_>>();

            // Store the instrs into the adapter function directly.
            let adapter = self
                .adapters
                .adapters
                .get_mut(&walrus2us[&func.id()])
                .unwrap();
            match &mut adapter.kind {
                AdapterKind::Local { instructions } => *instructions = instrs,
                _ => unreachable!(),
            }
        }

        // next up register all exports, ensuring that our export map says
        // what's happening as well for JS
        for export in std.exports.iter() {
            let id = walrus2us[&export.func];
            self.adapters.exports.push((export.name.clone(), id));

            let kind = AuxExportKind::Function(export.name.clone());
            let export = AuxExport {
                debug_name: format!("standard export {:?}", id),
                comments: String::new(),
                arg_names: None,
                kind,
                generate_typescript: true,
            };
            assert!(self.aux.export_map.insert(id, export).is_none());
        }

        // ... and finally the `implements` section
        for i in std.implements.iter() {
            let import_id = match &self.module.funcs.get(i.core_func).kind {
                walrus::FunctionKind::Import(i) => i.import,
                _ => panic!("malformed wasm interface typess section"),
            };
            self.adapters
                .implements
                .push((import_id, i.core_func, walrus2us[&i.adapter_func]));
        }
        Ok(())
    }

    /// Perform a small verification pass over the module to perform some
    /// internal sanity checks.
    fn verify(&self) -> Result<(), Error> {
        // First up verify that all imports in the wasm module from our
        // `$PLACEHOLDER_MODULE` are connected to an adapter via the
        // `implements` section.
        let mut implemented = HashMap::new();
        for (core, _, adapter) in self.adapters.implements.iter() {
            implemented.insert(core, adapter);
        }
        for import in self.module.imports.iter() {
            if import.module != PLACEHOLDER_MODULE {
                continue;
            }
            match import.kind {
                walrus::ImportKind::Function(_) => {}
                _ => bail!("import from `{}` was not a function", PLACEHOLDER_MODULE),
            }

            // These are special intrinsics which were handled in the descriptor
            // phase, but we don't have an implementation for them. We don't
            // need to error about them in this verification pass though,
            // having them lingering in the module is normal.
            if import.name == "__wbindgen_describe" || import.name == "__wbindgen_describe_closure"
            {
                continue;
            }
            if implemented.remove(&import.id()).is_none() {
                bail!("import of `{}` doesn't have an adapter listed", import.name);
            }
        }
        if implemented.len() != 0 {
            bail!("more implementations listed than imports");
        }

        // Next up verify that all imported adapter functions have a listing of
        // where they're imported from.
        let mut imports_counted = 0;
        for (id, adapter) in self.adapters.adapters.iter() {
            let name = match &adapter.kind {
                AdapterKind::Import { name, .. } => name,
                AdapterKind::Local { .. } => continue,
            };
            if !self.aux.import_map.contains_key(id) {
                bail!(
                    "import of `{}` doesn't have an import map item listed",
                    name
                );
            }

            imports_counted += 1;
        }
        // Make sure there's no extraneous adapters that weren't actually
        // imported in the module.
        if self.aux.import_map.len() != imports_counted {
            bail!("import map is larger than the number of imports");
        }

        // Make sure the export map and export adapters map contain the same
        // number of entries.
        for (_, id) in self.adapters.exports.iter() {
            if !self.aux.export_map.contains_key(id) {
                bail!("adapters map has an entry that the export map does not");
            }
        }

        if self.adapters.exports.len() != self.aux.export_map.len() {
            bail!("export map and export adapters map have different sizes");
        }

        Ok(())
    }

    /// Creates an import adapter for the `import` which will have the given
    /// `signature`.
    ///
    /// Note that the JS function imported will be invoked as `kind`.
    fn import_adapter(
        &mut self,
        import: ImportId,
        signature: Function,
        kind: AdapterJsImportKind,
    ) -> Result<AdapterId, Error> {
        let import = self.module.imports.get(import);
        let (import_module, import_name) = (import.module.clone(), import.name.clone());
        let import_id = import.id();
        let core_id = match import.kind {
            walrus::ImportKind::Function(f) => f,
            _ => bail!("bound import must be assigned to function"),
        };

        // Process the returned type first to see if it needs an out-pointer. This
        // happens if the results of the incoming arguments translated to wasm take
        // up more than one type.
        let mut ret = self.instruction_builder(true);
        ret.incoming(&signature.ret)?;
        let uses_retptr = ret.output.len() > 1;

        // Process the argument next, allocating space of the return value if one
        // was present. Additionally configure the `module` and `adapters` to allow
        // usage of closures going out to the import.
        let mut args = ret.cx.instruction_builder(false);
        if uses_retptr {
            args.input.push(AdapterType::I32);
        }
        for arg in signature.arguments.iter() {
            args.outgoing(arg)?;
        }

        // Build up the list of instructions for our adapter function. We start out
        // with all the outgoing instructions which convert all wasm params to the
        // desired types to call our import...
        let mut instructions = args.instructions;

        // ... and then we actually call our import. We synthesize an adapter
        // definition for it with the appropriate types here on the fly.
        let f = args.cx.adapters.append(
            args.output,
            ret.input,
            AdapterKind::Import {
                module: import_module,
                name: import_name,
                kind,
            },
        );
        instructions.push(InstructionData {
            instr: Instruction::CallAdapter(f),
            stack_change: StackChange::Unknown,
        });

        // ... and then we follow up with a conversion of the incoming type
        // back to wasm.
        instructions.extend(ret.instructions);

        // ... and if a return pointer is in use then we need to store the types on
        // the stack into the wasm return pointer. Note that we iterate in reverse
        // here because the last result is the top value on the stack.
        let results = if uses_retptr {
            let mem = args.cx.memory()?;
            for (i, ty) in ret.output.into_iter().enumerate().rev() {
                instructions.push(InstructionData {
                    instr: Instruction::StoreRetptr { offset: i, ty, mem },
                    stack_change: StackChange::Modified {
                        pushed: 0,
                        popped: 1,
                    },
                });
            }
            Vec::new()
        } else {
            ret.output
        };
        let id = args
            .cx
            .adapters
            .append(args.input, results, AdapterKind::Local { instructions });
        args.cx.adapters.implements.push((import_id, core_id, id));
        Ok(f)
    }

    /// Creates an adapter function for the `export` given to have the
    /// `signature` specified.
    fn export_adapter(
        &mut self,
        export: ExportId,
        signature: Function,
    ) -> Result<AdapterId, Error> {
        let export = self.module.exports.get(export);
        let name = export.name.clone();
        // Do the actual heavy lifting elsewhere to generate the `binding`.
        let call = Instruction::CallExport(export.id());
        let id = self.register_export_adapter(call, signature)?;
        self.adapters.exports.push((name, id));
        Ok(id)
    }

    fn table_element_adapter(&mut self, idx: u32, signature: Function) -> Result<AdapterId, Error> {
        let call = Instruction::CallTableElement(idx);
        // like above, largely just defer the work elsewhere
        Ok(self.register_export_adapter(call, signature)?)
    }

    fn register_export_adapter(
        &mut self,
        call: Instruction,
        signature: Function,
    ) -> Result<AdapterId, Error> {
        // Figure out how to translate all the incoming arguments ...
        let mut args = self.instruction_builder(false);
        for arg in signature.arguments.iter() {
            args.incoming(arg)?;
        }

        // ... then the returned value being translated back
        let mut ret = args.cx.instruction_builder(true);
        ret.outgoing(&signature.ret)?;
        let uses_retptr = ret.input.len() > 1;

        // Our instruction stream starts out with the return pointer as the first
        // argument to the wasm function, if one is in use. Then we convert
        // everything to wasm types.
        //
        // After calling the core wasm function we need to load all the return
        // pointer arguments if there were any, otherwise we simply convert
        // everything into the outgoing arguments.
        let mut instructions = Vec::new();
        if uses_retptr {
            instructions.push(InstructionData {
                instr: Instruction::Retptr,
                stack_change: StackChange::Modified {
                    pushed: 1,
                    popped: 0,
                },
            });
        }
        instructions.extend(args.instructions);
        instructions.push(InstructionData {
            instr: call,
            stack_change: StackChange::Unknown,
        });
        if uses_retptr {
            let mem = ret.cx.memory()?;
            for (i, ty) in ret.input.into_iter().enumerate() {
                instructions.push(InstructionData {
                    instr: Instruction::LoadRetptr { offset: i, ty, mem },
                    stack_change: StackChange::Modified {
                        pushed: 1,
                        popped: 0,
                    },
                });
            }
        }
        instructions.extend(ret.instructions);

        Ok(ret
            .cx
            .adapters
            .append(args.input, ret.output, AdapterKind::Local { instructions }))
    }

    fn instruction_builder<'b>(&'b mut self, return_position: bool) -> InstructionBuilder<'b, 'a> {
        InstructionBuilder {
            cx: self,
            input: Vec::new(),
            output: Vec::new(),
            instructions: Vec::new(),
            return_position,
        }
    }

    fn malloc(&self) -> Result<FunctionId, Error> {
        self.function_exports
            .get("__wbindgen_malloc")
            .cloned()
            .map(|p| p.1)
            .ok_or_else(|| anyhow!("failed to find declaration of `__wbindgen_malloc` in module"))
    }

    fn realloc(&self) -> Option<FunctionId> {
        self.function_exports
            .get("__wbindgen_realloc")
            .cloned()
            .map(|p| p.1)
    }

    fn free(&self) -> Result<FunctionId, Error> {
        self.function_exports
            .get("__wbindgen_free")
            .cloned()
            .map(|p| p.1)
            .ok_or_else(|| anyhow!("failed to find declaration of `__wbindgen_free` in module"))
    }

    fn memory(&self) -> Result<MemoryId, Error> {
        self.memory
            .ok_or_else(|| anyhow!("failed to find memory declaration in module"))
    }

    /// Removes the export item for all `__wbindgen` intrinsics which are
    /// generally only purely internal helpers.
    ///
    /// References to these functions are preserved through adapter instructions
    /// if necessary, otherwise they can all be gc'd out. By the time this
    /// function is called our discovery of these intrinsics has completed and
    /// there's no need to keep around these exports.
    fn unexport_intrinsics(&mut self) {
        let mut to_remove = Vec::new();
        for export in self.module.exports.iter() {
            match export.name.as_str() {
                n if n.starts_with("__wbindgen") => {
                    to_remove.push(export.id());
                }
                _ => {}
            }
        }
        for id in to_remove {
            self.module.exports.delete(id);
        }
    }

    /// Attempts to locate the `__wbindgen_exn_store` intrinsic and stores it in
    /// our auxiliary information.
    ///
    /// This is only invoked if the intrinsic will actually be needed for JS
    /// glue generation somewhere.
    fn find_exn_store(&mut self) {
        self.aux.exn_store = self
            .module
            .exports
            .iter()
            .find(|e| e.name == "__wbindgen_exn_store")
            .and_then(|e| match e.item {
                walrus::ExportItem::Function(f) => Some(f),
                _ => None,
            });
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
    comments.iter().map(|&s| s).collect::<Vec<_>>().join("\n")
}
