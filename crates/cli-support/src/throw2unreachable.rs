use crate::intrinsic::Intrinsic;
use crate::wit::Instruction;
use crate::wit::{AdapterKind, AuxImport, NonstandardWitSection, WasmBindgenAux};
use walrus::ir::*;
use walrus::Module;

/// Runs a small pass over `Module` to replace all calls to the
/// `__wbindgen_throw` intrinsic with an `unreachable` instruction.
///
/// This pass is executed as part of the wasm interface types support. This is
/// done to support debug mode executables with wasm interface types. Debug mode
/// executables will use malloc as well as externref intrinsics. These intrinsics
/// internally, when they fail, abort the instance. This abort is done through
/// the `__wbindgen_throw` intrinsic in debug mode to provide a hopefully
/// useful error message. In release mode it's simply an `unreachable`
/// instruction.
///
/// With wasm interface types we can't rely on intrinsics being available, so we
/// need to do something about this in debug mode. Our answer is to remove calls
/// to `__wbindgen_throw` and replace them with `unreachable`.
///
/// This has the unintended side effect of making the user-visible function
/// `wasm_bindgen::throw_str` "just work", but that's hoped to get fix with a
/// split of crates like described in #1841
pub fn run(module: &mut Module) {
    // Find the adapter ID which is the import for the call to the throw
    // intrinsic.
    let aux = module.customs.get_typed::<WasmBindgenAux>().unwrap();
    let throw_import = aux.import_map.iter().find(|(_, import)| match import {
        AuxImport::Intrinsic(Intrinsic::Throw) => true,
        _ => false,
    });
    let throw_adapter = match throw_import {
        Some((id, _)) => *id,
        None => return,
    };

    // Find the adapter, if any, which calls this intrinsic
    let wit = module.customs.get_typed::<NonstandardWitSection>().unwrap();
    let adapter_calling_throw = wit.adapters.iter().find(|(_, adapter)| {
        let instrs = match &adapter.kind {
            AdapterKind::Local { instructions } => instructions,
            _ => return false,
        };
        instrs.iter().any(|i| match i.instr {
            Instruction::CallAdapter(a) => a == throw_adapter,
            _ => false,
        })
    });
    let adapter_calling_throw = match adapter_calling_throw {
        Some((id, _)) => *id,
        None => return,
    };

    // ... then using the adapter that calls the intrinsic, find which core
    // import in the wasm module it's implementing.
    let import = wit
        .implements
        .iter()
        .find(|(_, _, adapter)| *adapter == adapter_calling_throw);
    let function = match import {
        Some((_, function, _)) => *function,
        None => return,
    };

    // .. and now replace all calls to `function` with `unreachable`
    // instructions
    for (_, func) in module.funcs.iter_local_mut() {
        let entry = func.entry_block();
        dfs_pre_order_mut(&mut Rewrite { function }, func, entry);
    }

    struct Rewrite {
        function: walrus::FunctionId,
    }

    impl VisitorMut for Rewrite {
        fn visit_instr_mut(&mut self, instr: &mut Instr, _: &mut InstrLocId) {
            match instr {
                Instr::Call(c) if c.func == self.function => {
                    *instr = Unreachable {}.into();
                }
                _ => {}
            }
        }
    }
}
