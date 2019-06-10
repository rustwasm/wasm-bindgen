//! Location where `Binding` structures are actually created.
//!
//! This module is tasked with converting `Descriptor::Function` instances to
//! `Binding`s. It uses the incoming/outgoing modules/builders to do most of the
//! heavy lifting, and then this is the glue around the edges to make sure
//! everything is processed, hooked up in the second, and then inserted into the
//! right map.
//!
//! This module is called from `src/webidl/mod.rs` exclusively to populate the
//! imports/exports/elements of the bindings section. Most of this module is
//! largely just connecting the dots!

use crate::descriptor::Function;
use crate::webidl::incoming::IncomingBuilder;
use crate::webidl::outgoing::OutgoingBuilder;
use crate::webidl::{Binding, NonstandardWebidlSection};
use failure::{format_err, Error};
use walrus::{FunctionId, Module, ValType};
use wasm_webidl_bindings::ast;

/// Adds an element to the `bindings.imports` map for the `import` specified
/// that is supposed to have the signature specified in `binding`. This also
/// expects that the imported item is called as `kind`.
pub fn register_import(
    module: &mut Module,
    bindings: &mut NonstandardWebidlSection,
    import: walrus::ImportId,
    binding: Function,
    kind: ast::WebidlFunctionKind,
) -> Result<(), Error> {
    let import = module.imports.get(import);
    let id = match import.kind {
        walrus::ImportKind::Function(f) => f,
        _ => unreachable!(),
    };
    let import_id = import.id();

    // Process the return value first to determine if we need a return pointer
    // since that is always the first argument.
    let mut incoming = IncomingBuilder::default();
    incoming.process(&binding.ret)?;

    // Next process all outgoing arguments, and configure the module/bindings
    // section to be available to the builder so we can recursively register
    // stack closures.
    let mut outgoing = OutgoingBuilder::default();
    outgoing.module = Some(module);
    outgoing.bindings_section = Some(bindings);
    if incoming.wasm.len() > 1 {
        outgoing.process_retptr();
    }
    for arg in binding.arguments.iter() {
        outgoing.process(arg)?;
    }

    // A bit of destructuring to kill the borrow that the outgoing builder has
    // on the module/bindings.
    let OutgoingBuilder {
        wasm: outgoing_wasm,
        webidl: outgoing_webidl,
        bindings: outgoing_bindings,
        ..
    } = outgoing;

    // Boilerplate to assemble the `webidl_ty` and `wasm_ty` values.
    let webidl_ty = webidl_ty(
        &mut bindings.types,
        kind,
        &outgoing_webidl,
        &incoming.webidl,
    );
    let (wasm_ty, return_via_outptr) =
        assert_signature_match(module, id, &outgoing_wasm, &incoming.wasm);

    // ... and finally insert it into our map!
    bindings.imports.insert(
        import_id,
        Binding {
            return_via_outptr,
            wasm_ty,
            incoming: incoming.bindings,
            outgoing: outgoing_bindings,
            webidl_ty,
        },
    );
    Ok(())
}

/// Adds an element to `bindings.exports` for the `export` specified to have the
/// `binding` given.
pub fn register_export(
    module: &mut Module,
    bindings: &mut NonstandardWebidlSection,
    export: walrus::ExportId,
    binding: Function,
) -> Result<(), Error> {
    let export = module.exports.get(export);
    let id = match export.item {
        walrus::ExportItem::Function(f) => f,
        _ => unreachable!(),
    };
    let export_id = export.id();
    // Do the actual heavy lifting elsewhere to generate the `binding`.
    let binding = register_wasm_export(module, bindings, id, binding)?;
    bindings.exports.insert(export_id, binding);
    Ok(())
}

/// Like `register_export` except registers a binding for a table element. In
/// this case ensures that the table element `idx` is specified to have the
/// `binding` signature specified, eventually updating `bindings.elems` list.
///
/// Returns the index of the item added in the `bindings.elems` list.
pub fn register_table_element(
    module: &mut Module,
    bindings: &mut NonstandardWebidlSection,
    idx: u32,
    binding: Function,
) -> Result<u32, Error> {
    let table = module
        .tables
        .main_function_table()?
        .ok_or_else(|| format_err!("no function table found"))?;
    let table = module.tables.get(table);
    let functions = match &table.kind {
        walrus::TableKind::Function(f) => f,
        _ => unreachable!(),
    };
    let id = functions.elements[idx as usize].unwrap();
    let ret = bindings.elems.len() as u32;
    // like above, largely just defer the work elsewhere
    let binding = register_wasm_export(module, bindings, id, binding)?;
    bindings.elems.push((idx, binding));
    Ok(ret)
}

/// Common routine to create a `Binding` for an exported wasm function, using
/// incoming arguments and an outgoing return value.
fn register_wasm_export(
    module: &mut Module,
    bindings: &mut NonstandardWebidlSection,
    id: walrus::FunctionId,
    binding: Function,
) -> Result<Binding, Error> {
    // Like imports, process the return value first to determine if we need a
    // return pointer
    let mut outgoing = OutgoingBuilder::default();
    outgoing.process(&binding.ret)?;

    // Afterwards process all arguments...
    let mut incoming = IncomingBuilder::default();
    if outgoing.wasm.len() > 1 {
        incoming.process_retptr();
    }
    for arg in binding.arguments.iter() {
        incoming.process(arg)?;
    }

    // ... do similar boilerplate to imports (but with incoming/outgoing
    // swapped) to produce some types ...
    let webidl_ty = webidl_ty(
        &mut bindings.types,
        ast::WebidlFunctionKind::Static,
        &incoming.webidl,
        &outgoing.webidl,
    );
    let (wasm_ty, return_via_outptr) =
        assert_signature_match(module, id, &incoming.wasm, &outgoing.wasm);

    // ... and there's our `Binding`!
    Ok(Binding {
        wasm_ty,
        incoming: incoming.bindings,
        outgoing: outgoing.bindings,
        webidl_ty,
        return_via_outptr,
    })
}

/// Asserts that the `params` and `results` we've determined from an
/// incoming/outgoing builder actually matches the signature of `id` in the
/// `module` provided. This is a somewhat loose comparison since `anyref` in the
/// expected lists will be present as `i32` in the actual module due to rustc
/// limitations.
///
/// This at the end manufactures an actual `walrus::Type` that will be used to
/// describe a WebIDL value. This manufactured value actually has `anyref` types
/// in it and also respects the out ptr ABI that we currently use to handle
/// multiple-value returns.
fn assert_signature_match(
    module: &mut Module,
    id: FunctionId,
    params: &[ValType],
    mut results: &[ValType],
) -> (walrus::TypeId, Option<Vec<walrus::ValType>>) {
    let ty = module.funcs.get(id).ty();
    let ty = module.types.get(ty);

    fn assert_eq(expected: ValType, actual: ValType) {
        match expected {
            ValType::Anyref => assert_eq!(actual, ValType::I32),
            _ => assert_eq!(expected, actual),
        }
    }
    let mut ret_outptr = None;

    match results.len() {
        0 => assert_eq!(ty.results().len(), 0),
        1 => assert_eq(results[0], ty.results()[0]),

        // multi value isn't supported yet so all aggregate returns are done
        // through an outptr as the first argument. This means that our
        // signature should have no results. The new signature we create will
        // also have no results.
        _ => {
            assert_eq!(ty.results().len(), 0);
            ret_outptr = Some(results.to_vec());
            results = &[];
        }
    }

    let mut iter = params.iter();
    for actual in ty.params().iter() {
        let expected = iter.next().unwrap();
        assert_eq(*expected, *actual);
    }
    assert!(iter.next().is_none());

    (module.types.add(params, results), ret_outptr)
}

// boilerplate to convert arguments to a `WebidlFunctionId`.
fn webidl_ty(
    types: &mut ast::WebidlTypes,
    kind: ast::WebidlFunctionKind,
    params: &[ast::WebidlScalarType],
    results: &[ast::WebidlScalarType],
) -> ast::WebidlFunctionId {
    let result = match results.len() {
        0 => None,
        1 => Some(results[0].into()),
        _ => panic!("too many results in a webidl return value"),
    };
    let func = ast::WebidlFunction {
        kind,
        params: params.iter().cloned().map(|x| x.into()).collect(),
        result,
    };
    types.insert(func)
}
