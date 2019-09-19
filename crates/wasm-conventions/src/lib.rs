//! A tiny crate of utilities for working with implicit Wasm codegen conventions
//! (often established by LLVM and lld).
//!
//! Examples conventions include:
//!
//! * The shadow stack pointer
//! * The canonical linear memory that contains the shadow stack

#![deny(missing_docs, missing_debug_implementations)]

use failure::{bail, format_err, Error};
use walrus::{GlobalId, GlobalKind, MemoryId, Module, ValType};

/// Get a Wasm module's canonical linear memory.
pub fn get_memory(module: &Module) -> Result<MemoryId, Error> {
    let mut memories = module.memories.iter().map(|m| m.id());
    let memory = memories.next();
    if memories.next().is_some() {
        bail!(
            "expected a single memory, found multiple; multiple memories \
             currently not supported"
        );
    }
    memory.ok_or_else(|| {
        format_err!(
            "module does not have a memory; must have a memory \
             to transform return pointers into Wasm multi-value"
        )
    })
}

/// Discover the shadow stack pointer and add it to the module's exports as
/// `__shadow_stack_pointer`.
///
/// Adding it to the exports is useful for making sure it doesn't get GC'd.
pub fn export_shadow_stack_pointer(module: &mut Module) -> Result<(), Error> {
    let candidates = module
        .globals
        .iter()
        .filter(|g| g.ty == ValType::I32)
        .filter(|g| g.mutable)
        .filter(|g| match g.kind {
            GlobalKind::Local(_) => true,
            GlobalKind::Import(_) => false,
        })
        .collect::<Vec<_>>();

    let ssp = match candidates.len() {
        0 => bail!("could not find the shadow stack pointer for the module"),
        // If we've got two mutable globals then we're in a pretty standard
        // situation for threaded code where one is the stack pointer and one is the
        // TLS base offset. We need to figure out which is which, and we basically
        // assume LLVM's current codegen where the first is the stack pointer.
        //
        // TODO: have an actual check here.
        1 | 2 => candidates[0].id(),
        _ => bail!("too many mutable globals to infer which is the shadow stack pointer"),
    };

    module.exports.add("__shadow_stack_pointer", ssp);
    Ok(())
}

/// Unexport the shadow stack pointer that was previously added to the module's
/// exports as `__shadow_stack_pointer`.
pub fn unexport_shadow_stack_pointer(module: &mut Module) -> Result<(), Error> {
    let e = module
        .exports
        .iter()
        .find(|e| e.name == "__shadow_stack_pointer")
        .map(|e| e.id())
        .ok_or_else(|| {
            format_err!("did not find the `__shadow_stack_pointer` export in the module")
        })?;
    module.exports.delete(e);
    Ok(())
}

/// Get the `__shadow_stack_pointer`.
///
/// It must have been previously added to the module's exports via
/// `export_shadow_stack_pointer`.
pub fn get_shadow_stack_pointer(module: &Module) -> Result<GlobalId, Error> {
    module
        .exports
        .iter()
        .find(|e| e.name == "__shadow_stack_pointer")
        .ok_or_else(|| {
            format_err!("did not find the `__shadow_stack_pointer` export in the module")
        })
        .and_then(|e| match e.item {
            walrus::ExportItem::Global(g) => Ok(g),
            _ => bail!("`__shadow_stack_pointer` export is wrong kind"),
        })
}
