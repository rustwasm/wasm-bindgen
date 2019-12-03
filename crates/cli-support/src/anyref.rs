use crate::descriptor::VectorKind;
use crate::intrinsic::Intrinsic;
use crate::wit::AuxImport;
use crate::wit::{AdapterKind, Instruction, NonstandardWitSection};
use crate::wit::{AdapterType, InstructionData, StackChange, WasmBindgenAux};
use anyhow::Error;
use std::collections::HashMap;
use walrus::Module;
use wasm_bindgen_anyref_xform::Context;

pub fn process(module: &mut Module) -> Result<(), Error> {
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
        .map(|(core, _, adapter)| (adapter, core))
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

    let meta = cfg.run(module)?;

    let mut aux = module
        .customs
        .delete_typed::<WasmBindgenAux>()
        .expect("wit custom section should exist");
    let section = module
        .customs
        .get_typed_mut::<NonstandardWitSection>()
        .expect("wit custom section should exist");

    // If the module looks like it's going to use some of these exports, store
    // them in the aux section to get used.
    //
    // FIXME: this is not great, we should ideally have precise tracking of what
    // requires what. These are used by catch clauses and anyref slices going
    // in/out of wasm. The catch clauses are a bit weird but anyref slices
    // should ideally track in their own instructions what table/functions
    // they're referencing. This doesn't fit well in today's model of
    // slice-related instructions, though, so let's just cop out and only enable
    // these coarsely.
    aux.anyref_table = Some(meta.table);
    if module_needs_anyref_metadata(&aux, section) {
        aux.anyref_alloc = meta.alloc;
        aux.anyref_drop_slice = meta.drop_slice;
    }

    // Additonally we may need to update some adapter instructions other than
    // those found for the anyref pass. These are some general "fringe support"
    // things necessary to get absolutely everything working.
    for (_, adapter) in section.adapters.iter_mut() {
        let instrs = match &mut adapter.kind {
            AdapterKind::Local { instructions } => instructions,
            AdapterKind::Import { .. } => continue,
        };
        for instr in instrs {
            match instr.instr {
                // Calls to the heap live count intrinsic are now routed to the
                // actual wasm function which keeps track of this.
                Instruction::CallAdapter(adapter) => {
                    let id = match meta.live_count {
                        Some(id) => id,
                        None => continue,
                    };
                    let import = match aux.import_map.get(&adapter) {
                        Some(import) => import,
                        None => continue,
                    };
                    match import {
                        AuxImport::Intrinsic(Intrinsic::AnyrefHeapLiveCount) => {}
                        _ => continue,
                    }
                    instr.instr = Instruction::Standard(wit_walrus::Instruction::CallCore(id));
                }

                // Optional anyref values are now managed in the wasm module, so
                // we need to store where they're managed.
                Instruction::I32FromOptionAnyref {
                    ref mut table_and_alloc,
                } => {
                    *table_and_alloc = meta.alloc.map(|id| (meta.table, id));
                }
                _ => continue,
            };
        }
    }

    module.customs.add(*aux);

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
/// The `instrs` must be generated by wasm-bindgen itself and follow the
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

/// This function shouldn't need to exist, see the fixme at the call-site.
fn module_needs_anyref_metadata(aux: &WasmBindgenAux, section: &NonstandardWitSection) -> bool {
    use Instruction::*;

    // our `handleError` intrinsic uses a few pieces of metadata to store
    // indices directly into the wasm module.
    if aux.imports_with_catch.len() > 0 {
        return true;
    }

    // Look for any instructions which may use `VectorKind::Anyref`. If there
    // are any then we'll need our intrinsics/tables/etc, otherwise we shouldn't
    // ever need them.
    section.adapters.iter().any(|(_, adapter)| {
        let instructions = match &adapter.kind {
            AdapterKind::Local { instructions } => instructions,
            AdapterKind::Import { .. } => return false,
        };
        instructions.iter().any(|instr| match instr.instr {
            VectorToMemory {
                kind: VectorKind::Anyref,
                ..
            }
            | MutableSliceToMemory {
                kind: VectorKind::Anyref,
                ..
            }
            | OptionVector {
                kind: VectorKind::Anyref,
                ..
            }
            | VectorLoad {
                kind: VectorKind::Anyref,
                ..
            }
            | OptionVectorLoad {
                kind: VectorKind::Anyref,
                ..
            }
            | View {
                kind: VectorKind::Anyref,
                ..
            }
            | OptionView {
                kind: VectorKind::Anyref,
                ..
            } => true,
            _ => false,
        })
    })
}
