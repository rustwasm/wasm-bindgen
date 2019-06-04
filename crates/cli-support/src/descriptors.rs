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
use failure::Error;
use std::borrow::Cow;
use std::collections::{HashMap, HashSet};
use std::mem;
use walrus::ImportId;
use walrus::{CustomSection, FunctionId, LocalFunction, Module, TypedCustomSectionId};
use wasm_bindgen_wasm_interpreter::Interpreter;

#[derive(Default, Debug)]
pub struct WasmBindgenDescriptorsSection {
    pub descriptors: HashMap<String, Descriptor>,
    pub closure_imports: HashMap<ImportId, Closure>,
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

    // Delete all descriptor functions and imports from the module now that
    // we've executed all of them.
    walrus::passes::gc::run(module);

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
            let entry = local.entry_block();
            let mut find = FindDescribeClosure {
                func: local,
                wbindgen_describe_closure,
                cur: entry.into(),
                call: None,
            };
            find.visit_block_id(&entry);
            if let Some(call) = find.call {
                let descriptor = interpreter
                    .interpret_closure_descriptor(id, module, &mut element_removal_list)
                    .unwrap();
                func_to_descriptor.insert(id, (call, Descriptor::decode(descriptor)));
            }
        }

        // For all indirect functions that were closure descriptors, delete them
        // from the function table since we've executed them and they're not
        // necessary in the final binary.
        let table_id = match interpreter.function_table_id() {
            Some(id) => id,
            None => return Ok(()),
        };
        let table = module.tables.get_mut(table_id);
        let table = match &mut table.kind {
            walrus::TableKind::Function(f) => f,
            _ => unreachable!(),
        };
        for idx in element_removal_list {
            log::trace!("delete element {}", idx);
            assert!(table.elements[idx].is_some());
            table.elements[idx] = None;
        }

        // And finally replace all calls of `wbindgen_describe_closure` with a
        // freshly manufactured import. Save off the type of this import in
        // ourselves, and then we're good to go.
        let ty = module.funcs.get(wbindgen_describe_closure).ty();
        for (func, (call_instr, descriptor)) in func_to_descriptor {
            let import_name = format!("__wbindgen_closure_wrapper{}", func.index());
            let id = module.add_import_func("__wbindgen_placeholder__", &import_name, ty);
            let import_id = module
                .imports
                .iter()
                .find(|i| i.name == import_name)
                .unwrap()
                .id();
            module.funcs.get_mut(id).name = Some(import_name);

            let local = match &mut module.funcs.get_mut(func).kind {
                walrus::FunctionKind::Local(l) => l,
                _ => unreachable!(),
            };
            let call = local.get_mut(call_instr).unwrap_call_mut();
            assert_eq!(call.func, wbindgen_describe_closure);
            call.func = id;
            self.closure_imports
                .insert(import_id, descriptor.unwrap_closure());
        }
        return Ok(());

        struct FindDescribeClosure<'a> {
            func: &'a LocalFunction,
            wbindgen_describe_closure: FunctionId,
            cur: ExprId,
            call: Option<ExprId>,
        }

        impl<'a> Visitor<'a> for FindDescribeClosure<'a> {
            fn local_function(&self) -> &'a LocalFunction {
                self.func
            }

            fn visit_expr_id(&mut self, id: &ExprId) {
                let prev = mem::replace(&mut self.cur, *id);
                id.visit(self);
                self.cur = prev;
            }

            fn visit_call(&mut self, call: &Call) {
                call.visit(self);
                if call.func == self.wbindgen_describe_closure {
                    assert!(self.call.is_none());
                    self.call = Some(self.cur);
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
