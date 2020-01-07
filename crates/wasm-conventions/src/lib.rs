//! A tiny crate of utilities for working with implicit Wasm codegen conventions
//! (often established by LLVM and lld).
//!
//! Examples conventions include:
//!
//! * The shadow stack pointer
//! * The canonical linear memory that contains the shadow stack

#![deny(missing_docs, missing_debug_implementations)]

use anyhow::{anyhow, bail, Error};
use walrus::{ir::Value, GlobalId, GlobalKind, InitExpr, MemoryId, Module, ValType};

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
        anyhow!(
            "module does not have a memory; must have a memory \
             to transform return pointers into Wasm multi-value"
        )
    })
}

/// Get the `__shadow_stack_pointer`.
///
/// It must have been previously added to the module's exports via
/// `export_shadow_stack_pointer`.
pub fn get_shadow_stack_pointer(module: &Module) -> Option<GlobalId> {
    let candidates = module
        .globals
        .iter()
        .filter(|g| g.ty == ValType::I32)
        .filter(|g| g.mutable)
        // The stack pointer is guaranteed to not be initialized to 0, and it's
        // guaranteed to have an i32 initializer, so find globals which are
        // locally defined, are an i32, and have a nonzero initializer
        .filter(|g| match g.kind {
            GlobalKind::Local(InitExpr::Value(Value::I32(n))) => n != 0,
            _ => false,
        })
        .collect::<Vec<_>>();

    match candidates.len() {
        0 => None,
        // TODO: have an actual check here.
        1 => Some(candidates[0].id()),
        _ => None,
    }
}
