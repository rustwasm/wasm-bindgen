//! Support for generating a standard wasm interface types
//!
//! This module has all the necessary support for generating a full-fledged
//! standard wasm interface types section as defined by the `wit_walrus`
//! crate. This module also critically assumes that the WebAssembly module
//! being generated **must be standalone**. In this mode all sorts of features
//! supported by `#[wasm_bindgen]` aren't actually supported, such as closures,
//! imports of global js names, js getters/setters, exporting structs, etc.
//! These features may all eventually come to the standard bindings proposal,
//! but it will likely take some time. In the meantime this module simply focuses
//! on taking what's already a valid wasm module and letting it through with a
//! standard WebIDL custom section. All other modules generate an error during
//! this binding process.
//!
//! Note that when this function is called and used we're also not actually
//! generating any JS glue. Any JS glue currently generated is also invalid if
//! the module contains the wasm bindings section and it's actually respected.

use crate::wit::{AdapterId, AdapterJsImportKind, AdapterType, AuxExportedMethodKind, Instruction};
use crate::wit::{AdapterKind, NonstandardWitSection, WasmBindgenAux};
use crate::wit::{AuxExport, InstructionData};
use crate::wit::{AuxExportKind, AuxImport, AuxValue, JsImport, JsImportName};
use anyhow::{anyhow, bail, Context, Error};
use std::collections::HashMap;
use walrus::Module;

pub fn add(module: &mut Module) -> Result<(), Error> {
    let nonstandard = module
        .customs
        .delete_typed::<NonstandardWitSection>()
        .unwrap();
    let aux = module.customs.delete_typed::<WasmBindgenAux>().unwrap();
    let mut section = wit_walrus::WasmInterfaceTypes::default();
    let WasmBindgenAux {
        extra_typescript: _, // ignore this even if it's specified
        local_modules,
        snippets,
        package_jsons,
        export_map,
        import_map,
        imports_with_catch,
        imports_with_variadic,
        imports_with_assert_no_shim: _, // not relevant for this purpose
        enums,
        structs,

        // irrelevant ids used to track various internal intrinsics and such
        externref_table: _,
        externref_alloc: _,
        externref_drop: _,
        externref_drop_slice: _,
        exn_store: _,
        shadow_stack_pointer: _,
        function_table: _,
        thread_destroy: _,
    } = *aux;

    let adapter_context = |id: AdapterId| {
        if let Some((name, _)) = nonstandard.exports.iter().find(|p| p.1 == id) {
            return format!("in function export `{}`", name);
        }
        if let Some((core, _, _)) = nonstandard.implements.iter().find(|p| p.2 == id) {
            let import = module.imports.get(*core);
            return format!(
                "in function import from `{}::{}`",
                import.module, import.name
            );
        }
        format!("in adapter function")
    };

    let mut us2walrus = HashMap::new();
    for (us, func) in crate::sorted_iter(&nonstandard.adapters) {
        if let Some(export) = export_map.get(us) {
            check_standard_export(export).context(adapter_context(*us))?;
        }
        if let Some(import) = import_map.get(us) {
            check_standard_import(import).context(adapter_context(*us))?;
        }
        let params = translate_tys(&func.params).context(adapter_context(*us))?;
        let results = translate_tys(&func.results).context(adapter_context(*us))?;
        let ty = section.types.add(params, results);
        let walrus = match &func.kind {
            AdapterKind::Local { .. } => section.funcs.add_local(ty, Vec::new()),
            AdapterKind::Import {
                module,
                name,
                kind: AdapterJsImportKind::Normal,
            } => section.add_import_func(module, name, ty).0,
            AdapterKind::Import {
                module,
                name,
                kind: AdapterJsImportKind::Constructor,
            } => {
                bail!(
                    "interfaces types doesn't support import of `{}::{}` \
                     as a constructor",
                    module,
                    name
                );
            }
            AdapterKind::Import {
                module,
                name,
                kind: AdapterJsImportKind::Method,
            } => {
                bail!(
                    "interfaces types doesn't support import of `{}::{}` \
                     as a method",
                    module,
                    name
                );
            }
        };
        us2walrus.insert(*us, walrus);
    }

    for (_, core, adapter) in nonstandard.implements.iter() {
        section.implements.add(us2walrus[adapter], *core);
    }

    for (name, adapter) in nonstandard.exports.iter() {
        section.exports.add(name, us2walrus[adapter]);
    }

    for (id, func) in nonstandard.adapters.iter() {
        let instructions = match &func.kind {
            AdapterKind::Local { instructions } => instructions,
            AdapterKind::Import { .. } => continue,
        };
        let result = match &mut section.funcs.get_mut(us2walrus[id]).kind {
            wit_walrus::FuncKind::Local(i) => i,
            _ => unreachable!(),
        };

        for instruction in instructions {
            result.push(
                translate_instruction(instruction, &us2walrus, module)
                    .with_context(|| adapter_context(*id))?,
            );
        }
    }

    if let Some((name, _)) = local_modules.iter().next() {
        bail!(
            "generating a bindings section is currently incompatible with \
             local JS modules being specified as well, `{}` cannot be used \
             since a standalone wasm file is being generated",
            name,
        );
    }

    if let Some((name, _)) = snippets.iter().filter(|(_, v)| !v.is_empty()).next() {
        bail!(
            "generating a bindings section is currently incompatible with \
             local JS snippets being specified as well, `{}` cannot be used \
             since a standalone wasm file is being generated",
            name,
        );
    }

    if let Some(path) = package_jsons.iter().next() {
        bail!(
            "generating a bindings section is currently incompatible with \
             package.json being consumed as well, `{}` cannot be used \
             since a standalone wasm file is being generated",
            path.display(),
        );
    }

    if let Some(id) = imports_with_catch.iter().next() {
        bail!(
            "{}\ngenerating a bindings section is currently incompatible with \
             `#[wasm_bindgen(catch)]`",
            adapter_context(*id),
        );
    }

    if let Some(id) = imports_with_variadic.iter().next() {
        bail!(
            "{}\ngenerating a bindings section is currently incompatible with \
             `#[wasm_bindgen(variadic)]`",
            adapter_context(*id),
        );
    }

    if let Some(enum_) = enums.iter().next() {
        bail!(
            "generating a bindings section is currently incompatible with \
             exporting an `enum` from the wasm file, cannot export `{}`",
            enum_.name,
        );
    }

    if let Some(struct_) = structs.iter().next() {
        bail!(
            "generating a bindings section is currently incompatible with \
             exporting a `struct` from the wasm file, cannot export `{}`",
            struct_.name,
        );
    }

    module.customs.add(section);
    Ok(())
}

fn translate_instruction(
    instr: &InstructionData,
    us2walrus: &HashMap<AdapterId, wit_walrus::FuncId>,
    module: &Module,
) -> Result<wit_walrus::Instruction, Error> {
    use Instruction::*;

    match &instr.instr {
        Standard(s) => Ok(s.clone()),
        CallAdapter(id) => {
            let id = us2walrus[id];
            Ok(wit_walrus::Instruction::CallAdapter(id))
        }
        CallExport(e) => match module.exports.get(*e).item {
            walrus::ExportItem::Function(f) => Ok(wit_walrus::Instruction::CallCore(f)),
            _ => bail!("can only call exported functions"),
        },
        CallTableElement(e) => {
            let entry = wasm_bindgen_wasm_conventions::get_function_table_entry(module, *e)?;
            let id = entry
                .func
                .ok_or_else(|| anyhow!("function table wasn't filled in a {}", e))?;
            Ok(wit_walrus::Instruction::CallCore(id))
        }
        StringToMemory {
            mem,
            malloc,
            realloc: _,
        } => Ok(wit_walrus::Instruction::StringToMemory {
            mem: *mem,
            malloc: *malloc,
        }),
        StoreRetptr { .. } | LoadRetptr { .. } | Retptr { .. } => {
            bail!("return pointers aren't supported in wasm interface types");
        }
        I32FromBool | BoolFromI32 => {
            bail!("booleans aren't supported in wasm interface types");
        }
        I32FromStringFirstChar | StringFromChar => {
            bail!("chars aren't supported in wasm interface types");
        }
        // Note: if `ExternrefLoadOwned` contained `Some`, this error message wouldn't make sense,
        // but that can only occur when returning `Result`,
        // in which case there'll be an earlier `UnwrapResult` instruction and we'll bail before reaching this point.
        I32FromExternrefOwned | I32FromExternrefBorrow | ExternrefLoadOwned { .. } | TableGet => {
            bail!("externref pass failed to sink into wasm module");
        }
        I32FromExternrefRustOwned { .. }
        | I32FromExternrefRustBorrow { .. }
        | RustFromI32 { .. } => {
            bail!("rust types aren't supported in wasm interface types");
        }
        I32FromOptionExternref { .. }
        | I32FromOptionU32Sentinel
        | I32FromOptionRust { .. }
        | I32FromOptionBool
        | I32FromOptionChar
        | I32FromOptionEnum { .. }
        | FromOptionNative { .. }
        | OptionVector { .. }
        | OptionString { .. }
        | OptionRustFromI32 { .. }
        | OptionVectorLoad { .. }
        | OptionView { .. }
        | OptionU32Sentinel
        | ToOptionNative { .. }
        | OptionBoolFromI32
        | OptionCharFromI32
        | OptionEnumFromI32 { .. } => {
            bail!("optional types aren't supported in wasm bindgen");
        }
        UnwrapResult { .. } | UnwrapResultString { .. } => {
            bail!("self-unwrapping result types aren't supported in wasm bindgen");
        }
        MutableSliceToMemory { .. } | VectorToMemory { .. } | VectorLoad { .. } | View { .. } => {
            bail!("vector slices aren't supported in wasm interface types yet");
        }
        CachedStringLoad { .. } => {
            bail!("cached strings aren't supported in wasm interface types");
        }
        StackClosure { .. } => {
            bail!("closures aren't supported in wasm interface types");
        }
    }
}

fn check_standard_import(import: &AuxImport) -> Result<(), Error> {
    let desc_js = |js: &JsImport| {
        let mut extra = String::new();
        for field in js.fields.iter() {
            extra.push_str(".");
            extra.push_str(field);
        }
        match &js.name {
            JsImportName::Global { name } | JsImportName::VendorPrefixed { name, .. } => {
                format!("global `{}{}`", name, extra)
            }
            JsImportName::Module { module, name } => {
                format!("`{}{}` from '{}'", name, extra, module)
            }
            JsImportName::LocalModule { module, name } => {
                format!("`{}{}` from local module '{}'", name, extra, module)
            }
            JsImportName::InlineJs {
                unique_crate_identifier,
                name,
                ..
            } => format!(
                "`{}{}` from inline js in '{}'",
                name, extra, unique_crate_identifier
            ),
        }
    };

    let item = match import {
        AuxImport::Value(AuxValue::Bare(js)) => {
            if js.fields.len() == 0 {
                if let JsImportName::Module { .. } = js.name {
                    return Ok(());
                }
            }
            desc_js(js)
        }
        AuxImport::Value(AuxValue::Getter(js, name))
        | AuxImport::Value(AuxValue::Setter(js, name))
        | AuxImport::Value(AuxValue::ClassGetter(js, name))
        | AuxImport::Value(AuxValue::ClassSetter(js, name)) => {
            format!("field access of `{}` for {}", name, desc_js(js))
        }
        AuxImport::ValueWithThis(js, method) => format!("method `{}.{}`", desc_js(js), method),
        AuxImport::Instanceof(js) => format!("instance of check of {}", desc_js(js)),
        AuxImport::Static(js) => format!("static js value {}", desc_js(js)),
        AuxImport::StructuralMethod(name) => format!("structural method `{}`", name),
        AuxImport::StructuralGetter(name)
        | AuxImport::StructuralSetter(name)
        | AuxImport::StructuralClassGetter(_, name)
        | AuxImport::StructuralClassSetter(_, name) => {
            format!("structural field access of `{}`", name)
        }
        AuxImport::IndexingDeleterOfClass(_)
        | AuxImport::IndexingDeleterOfObject
        | AuxImport::IndexingGetterOfClass(_)
        | AuxImport::IndexingGetterOfObject
        | AuxImport::IndexingSetterOfClass(_)
        | AuxImport::IndexingSetterOfObject => format!("indexing getters/setters/deleters"),
        AuxImport::WrapInExportedClass(name) => {
            format!("wrapping a pointer in a `{}` js class wrapper", name)
        }
        AuxImport::Intrinsic(intrinsic) => {
            format!("wasm-bindgen specific intrinsic `{}`", intrinsic.name())
        }
        AuxImport::Closure { .. } => format!("creating a `Closure` wrapper"),
    };
    bail!("import of {} requires JS glue", item);
}

fn check_standard_export(export: &AuxExport) -> Result<(), Error> {
    // First up make sure this is something that's actually valid to export
    // form a vanilla WebAssembly module with WebIDL bindings.
    match &export.kind {
        AuxExportKind::Function(_) => Ok(()),
        AuxExportKind::Constructor(name) => {
            bail!(
                "cannot export `{}` constructor function when generating \
                 a standalone WebAssembly module with no JS glue",
                name,
            );
        }
        AuxExportKind::Method {
            class, name, kind, ..
        } => {
            let kind_name = match kind {
                AuxExportedMethodKind::Method => "method",
                AuxExportedMethodKind::Getter => "getter",
                AuxExportedMethodKind::Setter => "setter",
            };

            bail!(
                "cannot export `{}::{}` {} when \
                 generating a standalone WebAssembly module with no \
                 JS glue",
                class,
                name,
                kind_name
            );
        }
    }
}

fn translate_tys(tys: &[AdapterType]) -> Result<Vec<wit_walrus::ValType>, Error> {
    tys.iter()
        .map(|ty| {
            ty.to_wit()
                .ok_or_else(|| anyhow!("type {:?} isn't supported in standard interface types", ty))
        })
        .collect()
}
