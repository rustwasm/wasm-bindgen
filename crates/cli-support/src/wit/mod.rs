use crate::decode;
use crate::descriptor::{Descriptor, Function};
use crate::descriptors::WasmBindgenDescriptorsSection;
use crate::intrinsic::Intrinsic;
use anyhow::{anyhow, bail, Error};
use std::borrow::Cow;
use std::collections::{HashMap, HashSet};
use std::path::PathBuf;
use std::str;
use walrus::{ExportId, FunctionId, ImportId, Module, TypedCustomSectionId};
use wasm_bindgen_shared::struct_function_export_name;

const PLACEHOLDER_MODULE: &str = "__wbindgen_placeholder__";

mod adapters;
mod incoming;
mod nonstandard;
mod outgoing;
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

    let adapters = cx.module.customs.add(cx.adapters);
    let aux = cx.module.customs.add(cx.aux);
    Ok((adapters, aux))
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
                adapters::import(
                    self.module,
                    &mut self.adapters,
                    id,
                    signature,
                    AdapterJsImportKind::Normal,
                )?;
                // Synthesize the two integer pointers we pass through which
                // aren't present in the signature but are present in the wasm
                // signature.
                let mut function = descriptor.function.clone();
                let nargs = function.arguments.len();
                function.arguments.insert(0, Descriptor::I32);
                function.arguments.insert(0, Descriptor::I32);
                let id = adapters::table_element(
                    self.module,
                    &mut self.adapters,
                    descriptor.shim_idx,
                    function,
                )?;
                self.aux.import_map.insert(
                    id,
                    AuxImport::Closure {
                        dtor: descriptor.dtor_idx,
                        mutable: descriptor.mutable,
                        nargs,
                    },
                );
            }
        }

        Ok(())
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
        let id = adapters::import(
            self.module,
            &mut self.adapters,
            id,
            intrinsic.signature(),
            AdapterJsImportKind::Normal,
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

        let id = adapters::export(self.module, &mut self.adapters, export_id, descriptor)?;
        self.aux.export_map.insert(
            id,
            AuxExport {
                debug_name: wasm_name,
                comments: concatenate_comments(&export.comments),
                arg_names: Some(export.function.arg_names),
                kind,
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
                        let id = adapters::import(
                            self.module,
                            &mut self.adapters,
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
                        (
                            adapters::import(
                                self.module,
                                &mut self.adapters,
                                import_id,
                                descriptor,
                                kind,
                            )?,
                            import,
                        )
                    }
                }
            }

            // NB: `structural` is ignored for free functions since it's
            // expected that the binding isn't changing anyway.
            None => {
                let id = adapters::import(
                    self.module,
                    &mut self.adapters,
                    import_id,
                    descriptor,
                    AdapterJsImportKind::Normal,
                )?;
                let name = self.determine_import(import, function.name)?;
                (id, AuxImport::Value(AuxValue::Bare(name)))
            }
        };

        // Record this for later as it affects JS binding generation, but note
        // that this doesn't affect the WebIDL interface at all.
        if *variadic {
            self.aux.imports_with_variadic.insert(id);
        }
        if *catch {
            self.aux.imports_with_catch.insert(id);
        }
        if *assert_no_shim {
            self.aux.imports_with_assert_no_shim.insert(id);
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
        let id = adapters::import(
            self.module,
            &mut self.adapters,
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
        let id = adapters::import(
            self.module,
            &mut self.adapters,
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
            let getter_id = adapters::export(
                self.module,
                &mut self.adapters,
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
            let setter_id = adapters::export(
                self.module,
                &mut self.adapters,
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
            let signature = Function {
                shim_idx: 0,
                arguments: vec![Descriptor::I32],
                ret: Descriptor::Anyref,
            };
            let id = adapters::import(
                self.module,
                &mut self.adapters,
                *import_id,
                signature,
                AdapterJsImportKind::Normal,
            )?;
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
                .push((import_id, walrus2us[&i.adapter_func]));
        }
        Ok(())
    }

    /// Perform a small verification pass over the module to perform some
    /// internal sanity checks.
    fn verify(&self) -> Result<(), Error> {
        let mut imports_counted = 0;
        let mut implemented = HashMap::new();
        for (core, adapter) in self.adapters.implements.iter() {
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
            let adapter = match implemented.remove(&import.id()) {
                Some(id) => id,
                None => {
                    bail!("import of `{}` doesn't have an adapter listed", import.name);
                }
            };

            // Ensure that everything imported from the `__wbindgen_placeholder__`
            // module has a location listed as to where it's expected to be
            // imported from.
            if !self.aux.import_map.contains_key(&adapter) {
                bail!(
                    "import of `{}` doesn't have an import map item listed",
                    import.name
                );
            }

            imports_counted += 1;
        }

        // Make sure there's no extraneous adapters that weren't actually
        // imported in the module.
        if self.aux.import_map.len() != imports_counted {
            bail!("import map is larger than the number of imports");
        }
        if implemented.len() != 0 {
            bail!("more implementations listed than imports");
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

// /// Do we need to generate JS glue shims for these incoming bindings?
// pub fn incoming_do_not_require_glue(
//     exprs: &[NonstandardIncoming],
//     from_webidl_tys: &[ast::WebidlTypeRef],
//     to_wasm_tys: &[walrus::ValType],
//     standard_webidl_enabled: bool,
// ) -> bool {
//     // If anything is nonstandard, then we're unconditionally going to need a JS
//     // shim because, well, it's not standard.
//     if exprs.iter().any(|e| match e {
//         NonstandardIncoming::Standard(_) => false,
//         _ => true,
//     }) {
//         return false;
//     }
//
//     // If everything is `Standard` and we've actually got WebIDL bindings fully
//     // enabled, then we don't require any glue at all!
//     if standard_webidl_enabled {
//         return true;
//     }
//
//     exprs.len() == from_webidl_tys.len()
//         && exprs.len() == to_wasm_tys.len()
//         && exprs
//             .iter()
//             .zip(from_webidl_tys)
//             .zip(to_wasm_tys)
//             .enumerate()
//             .all(|(i, ((expr, from_webidl_ty), to_wasm_ty))| match expr {
//                 NonstandardIncoming::Standard(e) => e.is_expressible_in_js_without_webidl_bindings(
//                     *from_webidl_ty,
//                     *to_wasm_ty,
//                     i as u32,
//                 ),
//                 _ => false,
//             })
// }
//
// /// Do we need to generate JS glue shims for these outgoing bindings?
// pub fn outgoing_do_not_require_glue(
//     exprs: &[NonstandardOutgoing],
//     from_wasm_tys: &[walrus::ValType],
//     to_webidl_tys: &[ast::WebidlTypeRef],
//     standard_webidl_enabled: bool,
// ) -> bool {
//     // Same short-circuits as above.
//     if exprs.iter().any(|e| match e {
//         NonstandardOutgoing::Standard(_) => false,
//         _ => true,
//     }) {
//         return false;
//     }
//     if standard_webidl_enabled {
//         return true;
//     }
//
//     exprs.len() == from_wasm_tys.len()
//         && exprs.len() == to_webidl_tys.len()
//         && exprs
//             .iter()
//             .zip(from_wasm_tys)
//             .zip(to_webidl_tys)
//             .enumerate()
//             .all(|(i, ((expr, from_wasm_ty), to_webidl_ty))| match expr {
//                 NonstandardOutgoing::Standard(e) => e.is_expressible_in_js_without_webidl_bindings(
//                     *from_wasm_ty,
//                     *to_webidl_ty,
//                     i as u32,
//                 ),
//                 _ => false,
//             })
// }
