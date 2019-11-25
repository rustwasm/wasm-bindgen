use crate::descriptor::VectorKind;
use crate::wit::{Adapter, NonstandardWitSection, WasmBindgenAux};
use crate::wit::{AdapterKind, Instruction};
use crate::wit::{AuxExportKind, AuxImport, AuxValue, JsImport, JsImportName};
use anyhow::{bail, Context, Error};
use walrus::Module;
use wasm_bindgen_multi_value_xform as multi_value_xform;
use wasm_bindgen_wasm_conventions as wasm_conventions;

pub fn run(module: &mut Module, adapters: &mut NonstandardWitSection) -> Result<(), Error> {
    let mut to_xform = Vec::new();
    let mut slots = Vec::new();

    for (_, adapter) in adapters.adapters.iter_mut() {
        extract_xform(adapter, &mut to_xform, &mut slots);
    }
    if to_xform.is_empty() {
        // Early exit to avoid failing if we don't have a memory or shadow stack
        // pointer because this is a minimal module that doesn't use linear
        // memory.
        return Ok(());
    }

    let shadow_stack_pointer = wasm_conventions::get_shadow_stack_pointer(module)?;
    let memory = wasm_conventions::get_memory(module)?;
    let wrappers = multi_value_xform::run(module, memory, shadow_stack_pointer, &to_xform)?;

    for (slot, id) in slots.into_iter().zip(wrappers) {
        *slot = id;
    }

    Ok(())
}

fn extract_xform<'a>(
    adapter: &'a mut Adapter,
    to_xform: &mut Vec<(walrus::FunctionId, usize, Vec<walrus::ValType>)>,
    slots: &mut Vec<&'a mut walrus::FunctionId>,
) {
    let instructions = match &mut adapter.kind {
        AdapterKind::Local { instructions } => instructions,
        AdapterKind::Import { .. } => return,
    };

    // If the first instruction is a `Retptr`, then this must be an exported
    // adapter which calls a wasm-defined function. Something we'd like to
    // adapt to multi-value!
    if let Some(Instruction::Retptr) = instructions.first().map(|e| &e.instr) {
        instructions.remove(0);
        let mut types = Vec::new();
        instructions.retain(|instruction| match instruction.instr {
            Instruction::LoadRetptr { ty, .. } => {
                types.push(ty.to_wasm().unwrap());
                false
            }
            _ => true,
        });
        let id = instructions
            .iter_mut()
            .filter_map(|i| match &mut i.instr {
                Instruction::Standard(wit_walrus::Instruction::CallCore(f)) => Some(f),
                _ => None,
            })
            .next()
            .expect("should have found call-core");

        // LLVM currently always uses the first parameter for the return
        // pointer. We hard code that here, since we have no better option.
        to_xform.push((*id, 0, types));
        slots.push(id);
        return;
    }

    // If the last instruction is a `StoreRetptr`, then this must be an adapter
    // which calls an imported function.
    //
    // FIXME(#1872) handle this
    // if let Some(Instruction::StoreRetptr { .. }) = instructions.last() {}
}
