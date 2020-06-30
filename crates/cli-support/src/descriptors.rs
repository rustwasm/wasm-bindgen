//! Management of wasm-bindgen descriptor functions.
//!
//! The purpose of this module is to basically execute a pass on a raw wasm
//! module that just came out of the compiler. The pass will execute all
//! relevant descriptor functions contained in the module which wasm-bindgen
//! uses to convey type infomation here, to the CLI.
//!
//! All descriptor functions are removed after this pass runs and in their stead
//! a new custom section, defined in this module, is inserted into the
//! `walrus::Module` which contains all the results of all the descriptor
//! functions.

use crate::descriptor::{Closure, Descriptor};
use anyhow::Error;
use std::borrow::Cow;
use std::collections::{HashMap, HashSet};
use walrus::ImportId;
use walrus::{CustomSection, FunctionId, Module, TypedCustomSectionId};
use wasm_bindgen_wasm_interpreter::Interpreter;

#[derive(Default, Debug)]
pub struct WasmBindgenDescriptorsSection {
    pub descriptors: HashMap<String, Descriptor>,
    pub closure_imports: HashMap<ImportId, Closure>,
    cached_closures: HashMap<Descriptor, FunctionId>,
}

pub type WasmBindgenDescriptorsSectionId = TypedCustomSectionId<WasmBindgenDescriptorsSection>;

/// Execute all `__wbindgen_describe_*` functions in a module, inserting a
/// custom section which represents the executed value of each descriptor.
///
/// Afterwards this will delete all descriptor functions from the module.
pub fn execute(module: &mut Module) -> Result<WasmBindgenDescriptorsSectionId, Error> {
    let mut section = WasmBindgenDescriptorsSection::default();
    let mut interpreter = Interpreter::new(module)?;

    section.execute_exports(module, &mut interpreter)?;
    section.execute_closures(module, &mut interpreter)?;

    Ok(module.customs.add(section))
}

impl WasmBindgenDescriptorsSection {
    fn execute_exports(
        &mut self,
        module: &mut Module,
        interpreter: &mut Interpreter,
    ) -> Result<(), Error> {
        let mut to_remove = Vec::new();
        for export in module.exports.iter() {
            let prefix = "__wbindgen_describe_";
            if !export.name.starts_with(prefix) {
                continue;
            }
            let id = match export.item {
                walrus::ExportItem::Function(id) => id,
                _ => panic!("{} export not a function", export.name),
            };
            if let Some(d) = interpreter.interpret_descriptor(id, module) {
                let name = &export.name[prefix.len()..];
                let descriptor = Descriptor::decode(d);
                self.descriptors.insert(name.to_string(), descriptor);
            }
            to_remove.push(export.id());
        }

        for id in to_remove {
            module.exports.delete(id);
        }
        Ok(())
    }

    fn execute_closures(
        &mut self,
        module: &mut Module,
        interpreter: &mut Interpreter,
    ) -> Result<(), Error> {
        use walrus::ir::*;

        // If our describe closure intrinsic isn't present or wasn't linked
        // then there's no closures, so nothing to do!
        let wbindgen_describe_closure = match interpreter.describe_closure_id() {
            Some(i) => i,
            None => return Ok(()),
        };

        // Find all functions which call `wbindgen_describe_closure`. These are
        // specially codegen'd so we know the rough structure of them. For each
        // one we delegate to the interpreter to figure out the actual result.
        let mut element_removal_list = HashSet::new();
        let mut func_to_descriptor = HashMap::new();
        for (id, local) in module.funcs.iter_local() {
            let mut find = FindDescribeClosure {
                wbindgen_describe_closure,
                found: false,
            };
            dfs_in_order(&mut find, local, local.entry_block());
            if find.found {
                let descriptor = interpreter
                    .interpret_closure_descriptor(id, module, &mut element_removal_list)
                    .unwrap();
                func_to_descriptor.insert(id, Descriptor::decode(descriptor));
            }
        }

        // For all indirect functions that were closure descriptors, delete them
        // from the function table since we've executed them and they're not
        // necessary in the final binary.
        for (segment, idx) in element_removal_list {
            log::trace!("delete element {}", idx);
            module.elements.get_mut(segment).members[idx] = None;
        }

        // And finally replace all calls of `wbindgen_describe_closure` with a
        // freshly manufactured import. Save off the type of this import in
        // ourselves, and then we're good to go.
        let ty = module.funcs.get(wbindgen_describe_closure).ty();
        for (func, descriptor) in func_to_descriptor {
            // This uses a cache so that if the same closure exists multiple times it will
            // deduplicate it so it only exists once.
            let id = match self.cached_closures.get(&descriptor) {
                Some(id) => *id,
                None => {
                    let import_name = format!("__wbindgen_closure_wrapper{}", func.index());
                    let (id, import_id) =
                        module.add_import_func("__wbindgen_placeholder__", &import_name, ty);
                    module.funcs.get_mut(id).name = Some(import_name);
                    self.closure_imports
                        .insert(import_id, descriptor.clone().unwrap_closure());
                    self.cached_closures.insert(descriptor, id);
                    id
                }
            };

            let local = match &mut module.funcs.get_mut(func).kind {
                walrus::FunctionKind::Local(l) => l,
                _ => unreachable!(),
            };
            let entry = local.entry_block();
            dfs_pre_order_mut(
                &mut UpdateDescribeClosure {
                    wbindgen_describe_closure,
                    replacement: id,
                },
                local,
                entry,
            );
        }
        return Ok(());

        struct FindDescribeClosure {
            wbindgen_describe_closure: FunctionId,
            found: bool,
        }

        impl<'a> Visitor<'a> for FindDescribeClosure {
            fn visit_call(&mut self, call: &Call) {
                if call.func == self.wbindgen_describe_closure {
                    self.found = true;
                }
            }
        }

        struct UpdateDescribeClosure {
            wbindgen_describe_closure: FunctionId,
            replacement: FunctionId,
        }

        impl<'a> VisitorMut for UpdateDescribeClosure {
            fn visit_call_mut(&mut self, call: &mut Call) {
                if call.func == self.wbindgen_describe_closure {
                    call.func = self.replacement;
                }
            }
        }
    }
}

impl CustomSection for WasmBindgenDescriptorsSection {
    fn name(&self) -> &str {
        "wasm-bindgen descriptors"
    }

    fn data(&self, _: &walrus::IdsToIndices) -> Cow<[u8]> {
        panic!("shouldn't emit custom sections just yet");
    }
}
