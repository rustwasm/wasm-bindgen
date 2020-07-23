use crate::wit::{Adapter, NonstandardWitSection};
use crate::wit::{AdapterKind, Instruction, WasmBindgenAux};
use anyhow::{anyhow, Error};
use walrus::Module;
use wasm_bindgen_multi_value_xform as multi_value_xform;
use wasm_bindgen_wasm_conventions as wasm_conventions;

pub fn run(module: &mut Module) -> Result<(), Error> {
    let mut adapters = module
        .customs
        .delete_typed::<NonstandardWitSection>()
        .unwrap();
    let mut to_xform = Vec::new();
    let mut slots = Vec::new();

    for (_, adapter) in adapters.adapters.iter_mut() {
        extract_xform(module, adapter, &mut to_xform, &mut slots);
    }
    if to_xform.is_empty() {
        // Early exit to avoid failing if we don't have a memory or shadow stack
        // pointer because this is a minimal module that doesn't use linear
        // memory.
        module.customs.add(*adapters);
        return Ok(());
    }

    let shadow_stack_pointer = module
        .customs
        .get_typed::<WasmBindgenAux>()
        .expect("aux section should be present")
        .shadow_stack_pointer
        .ok_or_else(|| anyhow!("failed to find shadow stack pointer in wasm module"))?;
    let memory = wasm_conventions::get_memory(module)?;
    let wrappers = multi_value_xform::run(module, memory, shadow_stack_pointer, &to_xform)?;

    for (slot, id) in slots.into_iter().zip(wrappers) {
        match slot {
            Slot::Id(s) => *s = id,
            Slot::Export(e) => module.exports.get_mut(e).item = id.into(),
        }
    }

    module.customs.add(*adapters);

    Ok(())
}

enum Slot<'a> {
    Id(&'a mut walrus::FunctionId),
    Export(walrus::ExportId),
}

fn extract_xform<'a>(
    module: &Module,
    adapter: &'a mut Adapter,
    to_xform: &mut Vec<(walrus::FunctionId, usize, Vec<walrus::ValType>)>,
    slots: &mut Vec<Slot<'a>>,
) {
    let instructions = match &mut adapter.kind {
        AdapterKind::Local { instructions } => instructions,
        AdapterKind::Import { .. } => return,
    };

    // If the first instruction is a `Retptr`, then this must be an exported
    // adapter which calls a wasm-defined function. Something we'd like to
    // adapt to multi-value!
    if let Some(Instruction::Retptr { .. }) = instructions.first().map(|e| &e.instr) {
        instructions.remove(0);
        let mut types = Vec::new();
        instructions.retain(|instruction| match &instruction.instr {
            Instruction::LoadRetptr { ty, .. } => {
                types.push(ty.to_wasm().unwrap());
                false
            }
            _ => true,
        });
        let slot = instructions
            .iter_mut()
            .filter_map(|i| match &mut i.instr {
                Instruction::Standard(wit_walrus::Instruction::CallCore(f)) => Some(Slot::Id(f)),
                Instruction::CallExport(e) => Some(Slot::Export(*e)),
                _ => None,
            })
            .next()
            .expect("should have found call-core");

        // LLVM currently always uses the first parameter for the return
        // pointer. We hard code that here, since we have no better option.
        let id = match &slot {
            Slot::Id(i) => **i,
            Slot::Export(e) => match module.exports.get(*e).item {
                walrus::ExportItem::Function(f) => f,
                _ => panic!("found call to non-function export"),
            },
        };
        to_xform.push((id, 0, types));
        slots.push(slot);
        return;
    }

    // If the last instruction is a `StoreRetptr`, then this must be an adapter
    // which calls an imported function.
    //
    // FIXME(#1872) handle this
    // if let Some(Instruction::StoreRetptr { .. }) = instructions.last() {}
}
