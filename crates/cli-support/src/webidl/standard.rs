//! Support for generating a standard WebIDL custom section
//!
//! This module has all the necessary support for generating a full-fledged
//! standard WebIDL custom section as defined by the `wasm-webidl-bindings`
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

// NB: Returning strings is weird
//
// This module has what is currently a pretty gross hack for dealing with
// returning strings. One of the banner features of WebIDL bindings is not
// requiring any language-specific glue to use wasm files and you get all sorts
// of types like integers and strings by default. Note that strings are a huge
// thing here.
//
// Dealing with *incoming* strings is easy enough in that the binding expression
// has an allocator function to call and it's filled in and passed as two
// pointers. Outgoing strings are a little harder, however, for two reasons:
//
// * One is that we need to return two values, which requires multi-value
// * Another is that someone's got to free the string at some point
//
// Rust/wasm-bindgen don't support multi-value, and WebIDL bindings as literally
// spec'd right this red hot second don't support freeing strings that we pass
// out. These both have obvious fixes (supporting multi-value and extending the
// bindings spec to support freeing) but we also want something working right
// this red-hot second.
//
// To get things working we employ a terrible hack where the first bindign
// expression of the result a function may indicate "use a thing that's way off
// in the ether" and that's actually a sentinel for "oh ignore everything else
// and the string is returned through an out-ptr as the first argument". This
// manifests in all sorts of special hacks all over the place to handle this,
// and it's a real bummer.
//
// This is in general just an explainer for the current state of things and
// also a preemptive apology for writing the code to handle this in so many
// places. I, like you, look forward to actually fixing this for real as the
// spec continues to evolve and we implement more in wasm-bindgen.

use crate::descriptor::VectorKind;
use crate::webidl::{AuxExportKind, AuxImport, AuxValue, JsImport, JsImportName};
use crate::webidl::{NonstandardIncoming, NonstandardOutgoing};
use crate::webidl::{NonstandardWebidlSection, WasmBindgenAux};
use failure::{bail, Error, ResultExt};
use walrus::Module;
use wasm_bindgen_multi_value_xform as multi_value_xform;
use wasm_bindgen_wasm_conventions as wasm_conventions;
use wasm_webidl_bindings::ast;

pub fn add_multi_value(
    module: &mut Module,
    bindings: &mut NonstandardWebidlSection,
) -> Result<(), Error> {
    let mut to_xform = vec![];
    for (id, binding) in &bindings.exports {
        if let Some(ref results) = binding.return_via_outptr {
            // LLVM currently always uses the first parameter for the return
            // pointer. We hard code that here, since we have no better option.
            let return_pointer_index = 0;
            to_xform.push((*id, return_pointer_index, &results[..]));
        }
    }

    if to_xform.is_empty() {
        // Early exit to avoid failing if we don't have a memory or shadow stack
        // pointer because this is a minimal module that doesn't use linear
        // memory.
        return Ok(());
    }

    let shadow_stack_pointer = wasm_conventions::get_shadow_stack_pointer(module)?;
    let memory = wasm_conventions::get_memory(module)?;
    multi_value_xform::run(module, memory, shadow_stack_pointer, &to_xform)?;

    // Finally, unset `return_via_outptr`, fix up its incoming bindings'
    // argument numberings, and update its function type.
    for (id, binding) in &mut bindings.exports {
        if binding.return_via_outptr.take().is_some() {
            if binding.incoming.is_empty() {
                bail!("missing incoming binding expression for return pointer parameter");
            }
            if !is_ret_ptr_bindings(binding.incoming.remove(0)) {
                bail!("unexpected incoming binding expression for return pointer parameter");
            }

            fixup_binding_argument_gets(&mut binding.incoming)?;

            let func = match module.exports.get(*id).item {
                walrus::ExportItem::Function(f) => f,
                _ => unreachable!(),
            };
            binding.wasm_ty = module.funcs.get(func).ty();
        }
    }

    Ok(())
}

fn is_ret_ptr_bindings(b: NonstandardIncoming) -> bool {
    match b {
        NonstandardIncoming::Standard(ast::IncomingBindingExpression::As(
            ast::IncomingBindingExpressionAs {
                ty: walrus::ValType::I32,
                expr,
            },
        )) => match *expr {
            ast::IncomingBindingExpression::Get(ast::IncomingBindingExpressionGet { idx: 0 }) => {
                true
            }
            _ => false,
        },
        _ => false,
    }
}

// Since we removed the first parameter (which was the return pointer) now all
// of the `Get` binding expression's are off by one. This function fixes these
// `Get`s.
fn fixup_binding_argument_gets(incoming: &mut [NonstandardIncoming]) -> Result<(), Error> {
    for inc in incoming {
        fixup_nonstandard_incoming(inc)?;
    }
    return Ok(());

    fn fixup_nonstandard_incoming(inc: &mut NonstandardIncoming) -> Result<(), Error> {
        match inc {
            NonstandardIncoming::Standard(s) => fixup_standard_incoming(s),
            _ => bail!("found usage of non-standard bindings when in standard-bindings-only mode"),
        }
    }

    fn fixup_standard_incoming(s: &mut ast::IncomingBindingExpression) -> Result<(), Error> {
        match s {
            ast::IncomingBindingExpression::Get(e) => {
                if e.idx == 0 {
                    bail!(
                        "found usage of removed return pointer parameter in \
                         non-return pointer bindings"
                    );
                } else {
                    e.idx -= 1;
                    Ok(())
                }
            }
            ast::IncomingBindingExpression::As(e) => fixup_standard_incoming(&mut e.expr),
            ast::IncomingBindingExpression::AllocUtf8Str(e) => fixup_standard_incoming(&mut e.expr),
            ast::IncomingBindingExpression::AllocCopy(e) => fixup_standard_incoming(&mut e.expr),
            ast::IncomingBindingExpression::EnumToI32(e) => fixup_standard_incoming(&mut e.expr),
            ast::IncomingBindingExpression::Field(e) => fixup_standard_incoming(&mut e.expr),
            ast::IncomingBindingExpression::BindImport(e) => fixup_standard_incoming(&mut e.expr),
        }
    }
}

pub fn add_section(
    module: &mut Module,
    aux: &WasmBindgenAux,
    nonstandard: &NonstandardWebidlSection,
) -> Result<(), Error> {
    let mut section = ast::WebidlBindings::default();
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
    } = aux;

    for (export, binding) in nonstandard.exports.iter() {
        // First up make sure this is something that's actually valid to export
        // form a vanilla WebAssembly module with WebIDL bindings.
        match &export_map[export].kind {
            AuxExportKind::Function(_) => {}
            AuxExportKind::Constructor(name) => {
                bail!(
                    "cannot export `{}` constructor function when generating \
                     a standalone WebAssembly module with no JS glue",
                    name,
                );
            }
            AuxExportKind::Getter { class, field } => {
                bail!(
                    "cannot export `{}::{}` getter function when generating \
                     a standalone WebAssembly module with no JS glue",
                    class,
                    field,
                );
            }
            AuxExportKind::Setter { class, field } => {
                bail!(
                    "cannot export `{}::{}` setter function when generating \
                     a standalone WebAssembly module with no JS glue",
                    class,
                    field,
                );
            }
            AuxExportKind::StaticFunction { class, name } => {
                bail!(
                    "cannot export `{}::{}` static function when \
                     generating a standalone WebAssembly module with no \
                     JS glue",
                    class,
                    name
                );
            }
            AuxExportKind::Method { class, name, .. } => {
                bail!(
                    "cannot export `{}::{}` method when \
                     generating a standalone WebAssembly module with no \
                     JS glue",
                    class,
                    name
                );
            }
        }

        let name = &module.exports.get(*export).name;
        let params = extract_incoming(&binding.incoming).with_context(|_| {
            format!(
                "failed to map arguments for export `{}` to standard \
                 binding expressions",
                name
            )
        })?;
        let mut result = extract_outgoing(&binding.outgoing).with_context(|_| {
            format!(
                "failed to map return value for export `{}` to standard \
                 binding expressions",
                name
            )
        })?;

        // see comment at top of this module about returning strings for what
        // this is doing and why it's weird
        if binding.return_via_outptr.is_some() {
            result.insert(
                0,
                ast::OutgoingBindingExpressionAs {
                    idx: u32::max_value(),
                    ty: ast::WebidlScalarType::Long.into(),
                }
                .into(),
            );
        }
        let binding = section.bindings.insert(ast::ExportBinding {
            wasm_ty: binding.wasm_ty,
            webidl_ty: copy_ty(
                &mut section.types,
                binding.webidl_ty.into(),
                &nonstandard.types,
            ),
            params: ast::IncomingBindingMap { bindings: params },
            result: ast::OutgoingBindingMap { bindings: result },
        });
        let func = match module.exports.get(*export).item {
            walrus::ExportItem::Function(f) => f,
            _ => unreachable!(),
        };
        section.binds.insert(ast::Bind {
            func,
            binding: binding.into(),
        });
    }

    for (import, binding) in nonstandard.imports.iter() {
        check_standard_import(&import_map[import])?;
        let (module_name, name) = {
            let import = module.imports.get(*import);
            (&import.module, &import.name)
        };
        let params = extract_outgoing(&binding.outgoing).with_context(|_| {
            format!(
                "failed to map arguments of import `{}::{}` to standard \
                 binding expressions",
                module_name, name,
            )
        })?;
        let mut result = extract_incoming(&binding.incoming).with_context(|_| {
            format!(
                "failed to map return value of import `{}::{}` to standard \
                 binding expressions",
                module_name, name,
            )
        })?;
        // see comment at top of this module about returning strings for what
        // this is doing and why it's weird
        if binding.return_via_outptr.is_some() {
            result.insert(
                0,
                ast::IncomingBindingExpressionGet {
                    idx: u32::max_value(),
                }
                .into(),
            );
        }
        let binding = section.bindings.insert(ast::ImportBinding {
            wasm_ty: binding.wasm_ty,
            webidl_ty: copy_ty(
                &mut section.types,
                binding.webidl_ty.into(),
                &nonstandard.types,
            ),
            params: ast::OutgoingBindingMap { bindings: params },
            result: ast::IncomingBindingMap { bindings: result },
        });
        let func = match module.imports.get(*import).kind {
            walrus::ImportKind::Function(f) => f,
            _ => unreachable!(),
        };
        section.binds.insert(ast::Bind {
            func,
            binding: binding.into(),
        });
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

    if let Some(import) = imports_with_catch.iter().next() {
        let import = module.imports.get(*import);
        bail!(
            "generating a bindings section is currently incompatible with \
             `#[wasm_bindgen(catch)]` on the `{}::{}` import because a \
             a standalone wasm file is being generated",
            import.module,
            import.name,
        );
    }

    if let Some(import) = imports_with_variadic.iter().next() {
        let import = module.imports.get(*import);
        bail!(
            "generating a bindings section is currently incompatible with \
             `#[wasm_bindgen(variadic)]` on the `{}::{}` import because a \
             a standalone wasm file is being generated",
            import.module,
            import.name,
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

    if nonstandard.elems.len() > 0 {
        // Note that this is a pretty cryptic error message, but we in theory
        // shouldn't ever hit this since closures always show up as some form
        // of nonstandard binding which was previously checked.
        bail!("generating a standalone wasm file requires no table element bindings");
    }

    module.customs.add(section);
    Ok(())
}

fn extract_incoming(
    nonstandard: &[NonstandardIncoming],
) -> Result<Vec<ast::IncomingBindingExpression>, Error> {
    let mut exprs = Vec::new();
    for expr in nonstandard {
        let desc = match expr {
            NonstandardIncoming::Standard(e) => {
                exprs.push(e.clone());
                continue;
            }
            NonstandardIncoming::Int64 { .. } => "64-bit integer",
            NonstandardIncoming::AllocCopyInt64 { .. } => "64-bit integer array",
            NonstandardIncoming::AllocCopyAnyrefArray { .. } => "array of JsValue",
            NonstandardIncoming::MutableSlice { .. } => "mutable slice",
            NonstandardIncoming::OptionSlice { .. } => "optional slice",
            NonstandardIncoming::OptionVector { .. } => "optional vector",
            NonstandardIncoming::OptionAnyref { .. } => "optional anyref",
            NonstandardIncoming::OptionNative { .. } => "optional integer",
            NonstandardIncoming::OptionU32Sentinel { .. } => "optional integer",
            NonstandardIncoming::OptionBool { .. } => "optional bool",
            NonstandardIncoming::OptionChar { .. } => "optional char",
            NonstandardIncoming::OptionIntegerEnum { .. } => "optional enum",
            NonstandardIncoming::OptionInt64 { .. } => "optional integer",
            NonstandardIncoming::RustType { .. } => "native Rust type",
            NonstandardIncoming::RustTypeRef { .. } => "reference to Rust type",
            NonstandardIncoming::OptionRustType { .. } => "optional Rust type",
            NonstandardIncoming::Char { .. } => "character",
            NonstandardIncoming::BorrowedAnyref { .. } => "borrowed anyref",
        };
        bail!(
            "cannot represent {} with a standard bindings expression",
            desc
        );
    }
    Ok(exprs)
}

fn extract_outgoing(
    nonstandard: &[NonstandardOutgoing],
) -> Result<Vec<ast::OutgoingBindingExpression>, Error> {
    let mut exprs = Vec::new();
    for expr in nonstandard {
        let desc = match expr {
            NonstandardOutgoing::Standard(e) => {
                exprs.push(e.clone());
                continue;
            }
            // ... yeah ... let's just leak strings
            // see comment at top of this module about returning strings for
            // what this is doing and why it's weird
            NonstandardOutgoing::Vector {
                offset,
                length,
                kind: VectorKind::String,
            } => {
                exprs.push(
                    ast::OutgoingBindingExpressionUtf8Str {
                        offset: *offset,
                        length: *length,
                        ty: ast::WebidlScalarType::DomString.into(),
                    }
                    .into(),
                );
                continue;
            }

            NonstandardOutgoing::RustType { .. } => "rust type",
            NonstandardOutgoing::Char { .. } => "character",
            NonstandardOutgoing::Number64 { .. } => "64-bit integer",
            NonstandardOutgoing::BorrowedAnyref { .. } => "borrowed anyref",
            NonstandardOutgoing::Vector { .. } => "vector",
            NonstandardOutgoing::CachedString { .. } => "cached string",
            NonstandardOutgoing::View64 { .. } => "64-bit slice",
            NonstandardOutgoing::ViewAnyref { .. } => "anyref slice",
            NonstandardOutgoing::OptionVector { .. } => "optional vector",
            NonstandardOutgoing::OptionSlice { .. } => "optional slice",
            NonstandardOutgoing::OptionNative { .. } => "optional integer",
            NonstandardOutgoing::OptionU32Sentinel { .. } => "optional integer",
            NonstandardOutgoing::OptionBool { .. } => "optional boolean",
            NonstandardOutgoing::OptionChar { .. } => "optional character",
            NonstandardOutgoing::OptionIntegerEnum { .. } => "optional enum",
            NonstandardOutgoing::OptionInt64 { .. } => "optional 64-bit integer",
            NonstandardOutgoing::OptionRustType { .. } => "optional rust type",
            NonstandardOutgoing::StackClosure { .. } => "closures",
        };
        bail!(
            "cannot represent {} with a standard bindings expression",
            desc
        );
    }
    Ok(exprs)
}

/// Recursively clones `ty` into` dst` where it originally indexes values in
/// `src`, returning a new type ref which indexes inside of `dst`.
pub fn copy_ty(
    dst: &mut ast::WebidlTypes,
    ty: ast::WebidlTypeRef,
    src: &ast::WebidlTypes,
) -> ast::WebidlTypeRef {
    let id = match ty {
        ast::WebidlTypeRef::Id(id) => id,
        ast::WebidlTypeRef::Scalar(_) => return ty,
    };
    let ty: &ast::WebidlCompoundType = src.get(id).unwrap();
    match ty {
        ast::WebidlCompoundType::Function(f) => {
            let params = f
                .params
                .iter()
                .map(|param| copy_ty(dst, *param, src))
                .collect();
            let result = f.result.map(|ty| copy_ty(dst, ty, src));
            dst.insert(ast::WebidlFunction {
                kind: f.kind.clone(),
                params,
                result,
            })
            .into()
        }
        _ => unimplemented!(),
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
    bail!(
        "cannot generate a standalone WebAssembly module which \
         contains an import of {} since it requires JS glue",
        item
    );
}
