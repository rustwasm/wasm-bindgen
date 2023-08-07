use crate::wit::{Adapter, NonstandardWitSection};
use crate::wit::{AdapterKind, Instruction, WasmBindgenAux};
use anyhow::{anyhow, Error};
use walrus::ir::Value;
use walrus::{FunctionId, InitExpr, Module};
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
            Slot::TableElement(index) => set_table_entry(module, index, id),
        }
    }

    module.customs.add(*adapters);

    Ok(())
}

enum Slot<'a> {
    Id(&'a mut walrus::FunctionId),
    Export(walrus::ExportId),
    TableElement(u32),
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
            .find_map(|i| match &mut i.instr {
                Instruction::CallCore(f) => Some(Slot::Id(f)),
                Instruction::CallExport(e) => Some(Slot::Export(*e)),
                Instruction::CallTableElement(index) => Some(Slot::TableElement(*index)),
                _ => None,
            })
            .expect("adapter never calls the underlying function");

        // LLVM currently always uses the first parameter for the return
        // pointer. We hard code that here, since we have no better option.
        let id = match &slot {
            Slot::Id(i) => **i,
            Slot::Export(e) => match module.exports.get(*e).item {
                walrus::ExportItem::Function(f) => f,
                _ => panic!("found call to non-function export"),
            },
            Slot::TableElement(func_index) => resolve_table_entry(module, *func_index),
        };
        to_xform.push((id, 0, types));
        slots.push(slot);
    }

    // If the last instruction is a `StoreRetptr`, then this must be an adapter
    // which calls an imported function.
    //
    // FIXME(#1872) handle this
    // if let Some(Instruction::StoreRetptr { .. }) = instructions.last() {}
}

/// Resolves an index in the function table to a function ID.
fn resolve_table_entry(module: &Module, func_index: u32) -> FunctionId {
    let table_id = module
        .tables
        .main_function_table()
        .ok()
        .flatten()
        .expect("there should only be one function table");
    module
        .tables
        .get(table_id)
        .elem_segments
        .iter()
        .find_map(|&id| {
            let elem = module.elements.get(id);
            let offset = match elem.kind {
                walrus::ElementKind::Active { offset, .. } => match offset {
                    InitExpr::Value(Value::I32(value)) => value as u32,
                    _ => panic!("table offset was not an i32 value"),
                },
                _ => panic!("found non-active element section for function table"),
            };
            elem.members.iter().enumerate().find_map(|(i, &func_id)| {
                let table_index = i as u32 + offset;
                if table_index == func_index {
                    func_id
                } else {
                    None
                }
            })
        })
        .expect("function in function table is not initialized")
}

/// Changes the function ID at an index in the function table.
fn set_table_entry(module: &mut Module, func_index: u32, new_id: FunctionId) {
    let table_id = module
        .tables
        .main_function_table()
        .ok()
        .flatten()
        .expect("there should only be one function table");
    for &id in module.tables.get(table_id).elem_segments.iter() {
        let elem = module.elements.get_mut(id);
        let offset = match elem.kind {
            walrus::ElementKind::Active { offset, .. } => match offset {
                InitExpr::Value(Value::I32(value)) => value as u32,
                _ => panic!("table offset was not an i32 value"),
            },
            _ => panic!("found non-active element section for function table"),
        };
        for (i, func_id) in elem.members.iter_mut().enumerate() {
            let table_index = i as u32 + offset;
            if table_index == func_index {
                *func_id = Some(new_id);
            }
        }
    }
}
