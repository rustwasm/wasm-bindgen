use crate::descriptor::{Closure, Descriptor, Function};
use crate::webidl::{AuxImport, ImportBinding, WasmBindgenAux, WebidlCustomSection};
use failure::Error;
use std::collections::HashSet;
use walrus::Module;
use wasm_bindgen_anyref_xform::Context;

pub fn process(module: &mut Module) -> Result<(), Error> {
    let mut cfg = Context::default();
    cfg.prepare(module)?;
    let bindings = module
        .customs
        .get_typed_mut::<WebidlCustomSection>()
        .expect("webidl custom section should exist");

    for (export, binding) in bindings.exports.iter_mut() {
        let (args, ret) = extract_anyrefs(binding, 0);
        cfg.export_xform(*export, &args, ret);
        process_closure_arguments(&mut cfg, binding);
    }

    for (import, kind) in bindings.imports.iter_mut() {
        let binding = match kind {
            ImportBinding::Function(f) => f,
            ImportBinding::Constructor(f) => f,
            ImportBinding::Method(f) => f,
        };
        let (args, ret) = extract_anyrefs(binding, 0);
        cfg.import_xform(*import, &args, ret);
        process_closure_arguments(&mut cfg, binding);
    }

    let aux = module
        .customs
        .get_typed_mut::<WasmBindgenAux>()
        .expect("webidl custom section should exist");
    for import in aux.import_map.values_mut() {
        match import {
            AuxImport::Closure(f) => process_closure(&mut cfg, f),
            _ => {}
        }
    }

    cfg.run(module)?;
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
        .get_typed_mut::<WebidlCustomSection>()
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

/// Process the `function` provided to ensure that all references to `Closure`
/// descriptors are processed below.
fn process_closure_arguments(cfg: &mut Context, function: &mut Function) {
    for arg in function.arguments.iter_mut() {
        process_descriptor(cfg, arg);
    }
    process_descriptor(cfg, &mut function.ret);

    fn process_descriptor(cfg: &mut Context, descriptor: &mut Descriptor) {
        match descriptor {
            Descriptor::Ref(d)
            | Descriptor::RefMut(d)
            | Descriptor::Option(d)
            | Descriptor::Slice(d)
            | Descriptor::Vector(d) => process_descriptor(cfg, d),
            Descriptor::Closure(c) => process_closure(cfg, c),
            Descriptor::Function(c) => process_function(cfg, c),
            _ => {}
        }
    }

    fn process_function(cfg: &mut Context, function: &mut Function) {
        let (args, ret) = extract_anyrefs(&function, 2);
        if let Some(new) = cfg.table_element_xform(function.shim_idx, &args, ret) {
            function.shim_idx = new;
        }
        process_closure_arguments(cfg, function);
    }
}

/// Ensure that the `Closure` is processed in case any of its arguments
/// recursively contain `anyref` and such.
fn process_closure(cfg: &mut Context, closure: &mut Closure) {
    let (args, ret) = extract_anyrefs(&closure.function, 2);
    if let Some(new) = cfg.table_element_xform(closure.shim_idx, &args, ret) {
        closure.shim_idx = new;
    }
    process_closure_arguments(cfg, &mut closure.function);
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
fn extract_anyrefs(f: &Function, offset: usize) -> (Vec<(usize, bool)>, bool) {
    let mut args = Vec::new();
    let mut cur = offset;
    if f.ret.abi_returned_through_pointer() {
        cur += 1;
    }
    for arg in f.arguments.iter() {
        if arg.is_anyref() {
            args.push((cur, true));
        } else if arg.is_ref_anyref() {
            args.push((cur, false));
        }
        cur += arg.abi_arg_count();
    }
    (args, f.ret.is_anyref())
}
