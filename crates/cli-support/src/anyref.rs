use crate::webidl::{NonstandardIncoming, NonstandardOutgoing};
use crate::webidl::{NonstandardWebidlSection, WasmBindgenAux};
use failure::Error;
use std::collections::HashSet;
use walrus::Module;
use wasm_bindgen_anyref_xform::Context;
use wasm_webidl_bindings::ast;

pub fn process(module: &mut Module, wasm_interface_types: bool) -> Result<(), Error> {
    let mut cfg = Context::default();
    cfg.prepare(module)?;
    let bindings = module
        .customs
        .get_typed_mut::<NonstandardWebidlSection>()
        .expect("webidl custom section should exist");

    // Transform all exported functions in the module, using the bindings listed
    // for each exported function.
    for (export, binding) in bindings.exports.iter_mut() {
        let ty = module.types.get(binding.wasm_ty);
        let args = Arguments::Incoming(&mut binding.incoming);
        let (args, ret) = extract_anyrefs(ty, args);
        cfg.export_xform(*export, &args, ret);
    }

    // Transform all imported functions in the module, using the bindings listed
    // for each imported function.
    for (import, binding) in bindings.imports.iter_mut() {
        let ty = module.types.get(binding.wasm_ty);
        let args = Arguments::Outgoing(&mut binding.outgoing);
        let (args, ret) = extract_anyrefs(ty, args);
        cfg.import_xform(*import, &args, ret);
    }

    // And finally transform all table elements that are used as function
    // pointers for closures and such.
    for (idx, binding) in bindings.elems.iter_mut() {
        let ty = module.types.get(binding.wasm_ty);
        let args = Arguments::Incoming(&mut binding.incoming);
        let (args, ret) = extract_anyrefs(ty, args);
        if let Some(new) = cfg.table_element_xform(*idx, &args, ret) {
            *idx = new;
        }
    }

    cfg.run(module)?;

    // If our output is using WebAssembly interface types then our bindings will
    // never use this table, so no need to export it. Otherwise it's highly
    // likely in web/JS embeddings this will be used, so make sure we export it
    // to avoid it getting gc'd accidentally.
    if !wasm_interface_types {
        // Make sure to export the `anyref` table for the JS bindings since it
        // will need to be initialized. If it doesn't exist though then the
        // module must not use it, so we skip it.
        let table = module.tables.iter().find(|t| match t.kind {
            walrus::TableKind::Anyref(_) => true,
            _ => false,
        });
        let table = match table {
            Some(t) => t.id(),
            None => return Ok(()),
        };
        module.exports.add("__wbg_anyref_table", table);
    }

    // Clean up now-unused intrinsics and shims and such
    walrus::passes::gc::run(module);

    // The GC pass above may end up removing some imported intrinsics. For
    // example `__wbindgen_object_clone_ref` is no longer needed after the
    // anyref pass. Make sure to delete the associated metadata for those
    // intrinsics so we don't try to access stale intrinsics later on.
    let remaining_imports = module
        .imports
        .iter()
        .map(|i| i.id())
        .collect::<HashSet<_>>();
    module
        .customs
        .get_typed_mut::<NonstandardWebidlSection>()
        .expect("webidl custom section should exist")
        .imports
        .retain(|id, _| remaining_imports.contains(id));
    module
        .customs
        .get_typed_mut::<WasmBindgenAux>()
        .expect("wasm-bindgen aux section should exist")
        .import_map
        .retain(|id, _| remaining_imports.contains(id));
    Ok(())
}

enum Arguments<'a> {
    Incoming(&'a mut [NonstandardIncoming]),
    Outgoing(&'a mut [NonstandardOutgoing]),
}

/// Extract a description of the anyref arguments from the function signature
/// described by `f`.
///
/// The returned values are expected to be passed to the anyref transformation
/// pass, and indicate which arguments (by index) in the wasm signature should
/// be transformed from `i32` to `anyref` as well as whether the returned value
/// is an `anyref` or not.
///
/// The `offset` argument here is typically 0 and indicates the offset at which
/// the wasm abi arguments described by `f` start at. For closures this is 2
/// because two synthetic arguments are injected into the wasm signature which
/// aren't present in the `Function` signature.
fn extract_anyrefs(ty: &walrus::Type, args: Arguments<'_>) -> (Vec<(usize, bool)>, bool) {
    let mut ret = Vec::new();

    // First find all the `anyref` arguments in the input type, and we'll
    // assume that they're owned anyref arguments for now (the `true`)
    for (i, arg) in ty.params().iter().enumerate() {
        if *arg == walrus::ValType::Anyref {
            ret.push((i, true));
        }
    }

    // Afterwards look through the argument list (specified with various
    // bindings) to find any borrowed anyref values and update our
    // transformation metadata accordingly. if we find one then the binding no
    // longer needs to remember its borrowed but rather it's just a simple cast
    // from wasm anyref to JS any.
    match args {
        Arguments::Incoming(incoming) => {
            for binding in incoming {
                let expr = match binding {
                    NonstandardIncoming::BorrowedAnyref {
                        val: ast::IncomingBindingExpression::Get(expr),
                    } => expr.clone(),
                    _ => continue,
                };
                ret.iter_mut().find(|p| p.0 == expr.idx as usize).unwrap().1 = false;
                let new_binding = ast::IncomingBindingExpressionAs {
                    ty: walrus::ValType::Anyref,
                    expr: Box::new(expr.into()),
                };
                *binding = NonstandardIncoming::Standard(new_binding.into());
            }
        }
        Arguments::Outgoing(outgoing) => {
            for binding in outgoing {
                let idx = match binding {
                    NonstandardOutgoing::BorrowedAnyref { idx } => *idx,
                    _ => continue,
                };
                ret.iter_mut().find(|p| p.0 == idx as usize).unwrap().1 = false;
                let new_binding = ast::OutgoingBindingExpressionAs {
                    idx,
                    ty: ast::WebidlScalarType::Any.into(),
                };
                *binding = NonstandardOutgoing::Standard(new_binding.into());
            }
        }
    }
    (ret, ty.results() == &[walrus::ValType::Anyref])
}
