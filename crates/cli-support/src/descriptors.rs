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
use std::fmt::Write;
use walrus::ImportId;
use walrus::{CustomSection, FunctionId, Module, TypedCustomSectionId};
use wasm_bindgen_wasm_interpreter::{Interpreter, Skip};

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

    Ok(module.customs.add(section))
}

/// Returns an error for an invalid descriptor.
///
/// `skipped` is the list of functions that were skipped while retrieving the
/// descriptor from the wasm module, which are likely the cause of the error if
/// present.
fn descriptor_error(descriptor: &[u32], skipped: &[Skip]) -> anyhow::Error {
    let mut msg = format!("invalid descriptor: {descriptor:?}");

    if !skipped.is_empty() {
        writeln!(
            msg,
            "\n\n\
            Some functions containing unsupported instructions were skipped while\n\
            intepreting the wasm module, which may have caused this:"
        )
        .unwrap();

        for skip in skipped {
            writeln!(msg, "    {skip}").unwrap();
        }
    }

    anyhow::Error::msg(msg)
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
            if let Some((d, skipped)) = interpreter.interpret_descriptor(id, module) {
                let name = &export.name[prefix.len()..];
                let descriptor =
                    Descriptor::decode(d).ok_or_else(|| descriptor_error(d, &skipped))?;
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
                let (descriptor, skipped) = interpreter
                    .interpret_closure_descriptor(id, module, &mut element_removal_list)
                    .unwrap();
                func_to_descriptor.insert(
                    id,
                    Descriptor::decode(descriptor)
                        .ok_or_else(|| descriptor_error(descriptor, &skipped))?,
                );
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
        // sort to ensure ids and caches are consistent across runs
        let mut items = func_to_descriptor.into_iter().collect::<Vec<_>>();
        items.sort_by_key(|i| i.0);
        for (func, descriptor) in items {
            let import_name = format!("__wbindgen_closure_wrapper{}", func.index());
            let (id, import_id) =
                module.add_import_func("__wbindgen_placeholder__", &import_name, ty);
            module.funcs.get_mut(id).name = Some(import_name);
            self.closure_imports
                .insert(import_id, descriptor.clone().unwrap_closure());

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

#[cfg(test)]
mod tests {
    use super::WasmBindgenDescriptorsSectionId;

    fn execute(wat: &str) -> anyhow::Result<WasmBindgenDescriptorsSectionId> {
        let wasm = wat::parse_str(wat).unwrap();
        let mut module = walrus::Module::from_buffer(&wasm).unwrap();
        super::execute(&mut module)
    }

    #[test]
    fn unsupported_instruction() {
        let result = execute(
            r#"
            (module
                (import "__wbindgen_placeholder__" "__wbindgen_describe"
                (func $__wbindgen_describe (param i32)))

                (func $bar
                    ;; We don't support `block`, so this will cause an error.
                    block
                    end

                    ;; Which means this descriptor won't go through.
                    i32.const 0
                    call $__wbindgen_describe
                )

                (func $foo
                    ;; Call into a nested function `bar`, since an unsupported instruction in the
                    ;; root function we're calling panics immediately.
                    call $bar
                )

                (export "__wbindgen_describe_foo" (func $foo))
            )
            "#,
        );
        let err = result.unwrap_err();
        assert!(err.to_string().contains("invalid descriptor: []"));
        assert!(err.to_string().contains("Block"));
    }
}
