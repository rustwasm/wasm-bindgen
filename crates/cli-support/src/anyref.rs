use crate::descriptor::Function;
use crate::webidl::{ImportBinding, WasmBindgenAux, WebidlCustomSection, AuxImport};
use failure::Error;
use std::collections::HashSet;
use walrus::Module;

pub fn process(module: &mut Module) -> Result<(), Error> {
    let mut cfg = wasm_bindgen_anyref_xform::Context::default();
    cfg.prepare(module)?;
    let bindings = module
        .customs
        .get_typed::<WebidlCustomSection>()
        .expect("webidl custom section should exist");

    for (export, binding) in bindings.exports.iter() {
        let (args, ret) = extract_anyrefs(binding);
        cfg.export_xform(*export, &args, ret);
    }

    for (import, kind) in bindings.imports.iter() {
        let binding = match kind {
            ImportBinding::Function(f) => f,
            ImportBinding::Constructor(f) => f,
            ImportBinding::Method(f) => f,
        };
        let (args, ret) = extract_anyrefs(binding);
        cfg.import_xform(*import, &args, ret);
    }

    let aux = module
        .customs
        .get_typed_mut::<WasmBindgenAux>()
        .expect("webidl custom section should exist");
    for import in aux.import_map.values_mut() {
        let closure = match import {
            AuxImport::Closure(f) => f,
            _ => continue,
        };
        let (args, ret) = extract_anyrefs(&closure.function);
        if let Some(new) = cfg.table_element_xform(closure.shim_idx, &args, ret) {
            closure.shim_idx = new;
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

fn extract_anyrefs(f: &Function) -> (Vec<(usize, bool)>, bool) {
    let mut args = Vec::new();
    let mut cur = 0;
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
