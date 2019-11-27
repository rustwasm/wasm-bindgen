use crate::wit::{AdapterKind, Instruction, NonstandardWitSection};
use crate::wit::{AdapterType, InstructionData, StackChange};
use anyhow::Error;
use std::collections::HashMap;
use walrus::Module;
use wasm_bindgen_anyref_xform::Context;

pub fn process(module: &mut Module, wasm_interface_types: bool) -> Result<(), Error> {
    let mut cfg = Context::default();
    cfg.prepare(module)?;
    let section = module
        .customs
        .get_typed_mut::<NonstandardWitSection>()
        .expect("wit custom section should exist");

    let implements = section
        .implements
        .iter()
        .cloned()
        .map(|(core, adapter)| (adapter, core))
        .collect::<HashMap<_, _>>();

    // Transform all exported functions in the module, using the bindings listed
    // for each exported function.
    for (id, adapter) in section.adapters.iter_mut() {
        let instructions = match &mut adapter.kind {
            AdapterKind::Local { instructions } => instructions,
            AdapterKind::Import { .. } => continue,
        };
        if let Some(id) = implements.get(&id) {
            import_xform(
                &mut cfg,
                *id,
                instructions,
                &mut adapter.params,
                &mut adapter.results,
            );
            continue;
        }
        if let Some(id) = find_call_export(instructions) {
            export_xform(&mut cfg, id, instructions);
            continue;
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

    // TODO: still needed?
    // // Clean up now-unused intrinsics and shims and such
    // walrus::passes::gc::run(module);
    //
    // // The GC pass above may end up removing some imported intrinsics. For
    // // example `__wbindgen_object_clone_ref` is no longer needed after the
    // // anyref pass. Make sure to delete the associated metadata for those
    // // intrinsics so we don't try to access stale intrinsics later on.
    // let remaining_imports = module
    //     .imports
    //     .iter()
    //     .map(|i| i.id())
    //     .collect::<HashSet<_>>();
    // module
    //     .customs
    //     .get_typed_mut::<NonstandardWitSection>()
    //     .expect("wit custom section should exist")
    //     .implements
    //     .retain(|(id, _)| remaining_imports.contains(id));
    // module
    //     .customs
    //     .get_typed_mut::<WasmBindgenAux>()
    //     .expect("wasm-bindgen aux section should exist")
    //     .import_map
    //     .retain(|id, _| remaining_imports.contains(id));
    Ok(())
}

fn find_call_export(instrs: &[InstructionData]) -> Option<Export> {
    instrs
        .iter()
        .enumerate()
        .filter_map(|(i, instr)| match instr.instr {
            Instruction::CallExport(e) => Some(Export::Export(e)),
            Instruction::CallTableElement(e) => Some(Export::TableElement {
                idx: e,
                call_idx: i,
            }),
            _ => None,
        })
        .next()
}

enum Export {
    Export(walrus::ExportId),
    TableElement {
        /// Table element that we're calling
        idx: u32,
        /// Index in the instruction stream where the call instruction is found
        call_idx: usize,
    },
}

/// Adapts the `instrs` given which are an implementation of the import of `id`.
///
/// This function will pattern match outgoing arguments and update the
/// instruction stream to remove any anyref-management instructions since
/// we'll be sinking those into the WebAssembly module.
fn import_xform(
    cx: &mut Context,
    id: walrus::ImportId,
    instrs: &mut Vec<InstructionData>,
    params: &mut [AdapterType],
    results: &mut [AdapterType],
) {
    struct Arg {
        idx: usize,
        // Some(false) for a borrowed anyref, Some(true) for an owned one
        anyref: Option<bool>,
    }

    let mut to_delete = Vec::new();
    let mut iter = instrs.iter().enumerate();
    let mut args = Vec::new();
    while let Some((i, instr)) = iter.next() {
        match instr.instr {
            Instruction::CallAdapter(_) => break,
            Instruction::AnyrefLoadOwned | Instruction::TableGet => {
                let owned = match instr.instr {
                    Instruction::TableGet => false,
                    _ => true,
                };
                let mut arg: Arg = match args.pop().unwrap() {
                    Some(arg) => arg,
                    None => panic!("previous instruction must be `arg.get`"),
                };
                arg.anyref = Some(owned);
                match params[arg.idx] {
                    AdapterType::I32 => {}
                    _ => panic!("must be `i32` type"),
                }
                params[arg.idx] = AdapterType::Anyref;
                args.push(Some(arg));
                to_delete.push(i);
            }
            Instruction::Standard(wit_walrus::Instruction::ArgGet(n)) => {
                args.push(Some(Arg {
                    idx: n as usize,
                    anyref: None,
                }));
            }
            _ => match instr.stack_change {
                StackChange::Modified { pushed, popped } => {
                    for _ in 0..popped {
                        args.pop();
                    }
                    for _ in 0..pushed {
                        args.push(None);
                    }
                }
                StackChange::Unknown => {
                    panic!("must have stack change data");
                }
            },
        }
    }

    let mut ret_anyref = false;
    while let Some((i, instr)) = iter.next() {
        match instr.instr {
            Instruction::I32FromAnyrefOwned => {
                assert_eq!(results.len(), 1);
                match results[0] {
                    AdapterType::I32 => {}
                    _ => panic!("must be `i32` type"),
                }
                results[0] = AdapterType::Anyref;
                ret_anyref = true;
                to_delete.push(i);
            }
            _ => {}
        }
    }

    // Delete all unnecessary anyref management insructions
    for idx in to_delete.into_iter().rev() {
        instrs.remove(idx);
    }

    // Filter down our list of arguments to just the ones that are anyref
    // values.
    let args = args
        .iter()
        .filter_map(|arg| arg.as_ref())
        .filter_map(|arg| arg.anyref.map(|owned| (arg.idx, owned)))
        .collect::<Vec<_>>();

    // ... and register this entire transformation with the anyref
    // transformation pass.
    cx.import_xform(id, &args, ret_anyref);
}

/// Adapts the `instrs` of an adapter function that calls an export.
///
/// The `instrus` must be generated by wasm-bindgen itself and follow the
/// pattern matched below to pass off to the anyref transformation pass. The
/// signature of the adapter doesn't change (it remains as anyref-aware) but the
/// signature of the export we're calling will change during the transformation.
fn export_xform(cx: &mut Context, export: Export, instrs: &mut Vec<InstructionData>) {
    let mut to_delete = Vec::new();
    let mut iter = instrs.iter().enumerate();
    let mut args = Vec::new();

    // Mutate instructions leading up to the `CallExport` instruction. We
    // maintain a stack of indicators whether the element at that stack slot is
    // unknown (`None`) or whether it's an owned/borrowed anyref
    // (`Some(owned)`).
    //
    // Note that we're going to delete the `I32FromAnyref*` instructions, so we
    // also maintain indices of the instructions to delete.
    while let Some((i, instr)) = iter.next() {
        match instr.instr {
            Instruction::CallExport(_) | Instruction::CallTableElement(_) => break,
            Instruction::I32FromAnyrefOwned => {
                args.pop();
                args.push(Some(true));
                to_delete.push(i);
            }
            Instruction::I32FromAnyrefBorrow => {
                args.pop();
                args.push(Some(false));
                to_delete.push(i);
            }
            _ => match instr.stack_change {
                StackChange::Modified { pushed, popped } => {
                    for _ in 0..popped {
                        args.pop();
                    }
                    for _ in 0..pushed {
                        args.push(None);
                    }
                }
                StackChange::Unknown => {
                    panic!("must have stack change data");
                }
            },
        }
    }

    // If one of the instructions after the call is an `AnyrefLoadOwned` then we
    // know that the function returned an anyref. Currently `&'static Anyref`
    // can't be done as a return value, so this is the only case we handle here.
    let mut ret_anyref = false;
    while let Some((i, instr)) = iter.next() {
        match instr.instr {
            Instruction::AnyrefLoadOwned => {
                ret_anyref = true;
                to_delete.push(i);
            }
            _ => {}
        }
    }

    // Filter down our list of arguments to just the ones that are anyref
    // values.
    let args = args
        .iter()
        .enumerate()
        .filter_map(|(i, owned)| owned.map(|owned| (i, owned)))
        .collect::<Vec<_>>();

    // ... and register this entire transformation with the anyref
    // transformation pass.
    match export {
        Export::Export(id) => {
            cx.export_xform(id, &args, ret_anyref);
        }
        Export::TableElement { idx, call_idx } => {
            if let Some(new_idx) = cx.table_element_xform(idx, &args, ret_anyref) {
                instrs[call_idx].instr = Instruction::CallTableElement(new_idx);
            }
        }
    }

    // Delete all unnecessary anyref management instructions. We're going to
    // sink these instructions into the wasm module itself.
    for idx in to_delete.into_iter().rev() {
        instrs.remove(idx);
    }
}
