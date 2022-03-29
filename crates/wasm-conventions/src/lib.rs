//! A tiny crate of utilities for working with implicit Wasm codegen conventions
//! (often established by LLVM and lld).
//!
//! Examples conventions include:
//!
//! * The shadow stack pointer
//! * The canonical linear memory that contains the shadow stack

use anyhow::{anyhow, bail, Result};
use walrus::{
    ir::Value, ElementId, FunctionId, GlobalId, GlobalKind, InitExpr, MemoryId, Module, ValType,
};

/// Get a Wasm module's canonical linear memory.
pub fn get_memory(module: &Module) -> Result<MemoryId> {
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

/// Get the `__tls_base`.
pub fn get_tls_base(module: &Module) -> Option<GlobalId> {
    let candidates = module
        .exports
        .iter()
        .filter(|ex| ex.name == "__tls_base")
        .filter_map(|ex| match ex.item {
            walrus::ExportItem::Global(id) => Some(id),
            _ => None,
        })
        .filter(|id| {
            let global = module.globals.get(*id);

            global.ty == ValType::I32
        })
        .collect::<Vec<_>>();

    match candidates.len() {
        1 => Some(candidates[0]),
        _ => None,
    }
}

pub struct FunctionTableEntry {
    pub element: ElementId,
    pub idx: usize,
    pub func: Option<FunctionId>,
}

/// Looks up a function table entry by index in the main function table.
pub fn get_function_table_entry(module: &Module, idx: u32) -> Result<FunctionTableEntry> {
    let table = module
        .tables
        .main_function_table()?
        .ok_or_else(|| anyhow!("no function table found in module"))?;
    let table = module.tables.get(table);
    for &segment in table.elem_segments.iter() {
        let segment = module.elements.get(segment);
        let offset = match &segment.kind {
            walrus::ElementKind::Active {
                offset: InitExpr::Value(Value::I32(n)),
                ..
            } => *n as u32,
            _ => continue,
        };
        let idx = (idx - offset) as usize;
        match segment.members.get(idx) {
            Some(slot) => {
                return Ok(FunctionTableEntry {
                    element: segment.id(),
                    idx,
                    func: slot.clone(),
                })
            }
            None => continue,
        }
    }
    bail!("failed to find `{}` in function table", idx);
}
