//! Support for closures in wasm-bindgen
//!
//! This module contains the bulk of the support necessary to support closures
//! in `wasm-bindgen`. The main "support" here is that `Closure::wrap` creates
//! a `JsValue` through... well... unconventional mechanisms.
//!
//! This module contains one public function, `rewrite`. The function will
//! rewrite the wasm module to correctly call closure factories and thread
//! through values into the final `Closure` object. More details about how all
//! this works can be found in the code below.

use std::collections::{BTreeMap, HashMap, HashSet};
use std::mem;

use failure::Error;
use parity_wasm::elements::*;

use descriptor::Descriptor;
use js::js2rust::Js2Rust;
use js::Context;

pub fn rewrite(input: &mut Context) -> Result<(), Error> {
    let info = ClosureDescriptors::new(input);

    // Sanity check to make sure things look ok and skip everything below if
    // there's not calls to `Closure::new`.
    assert_eq!(
        info.element_removal_list.len(),
        info.code_idx_to_descriptor.len(),
    );
    if info.element_removal_list.len() == 0 {
        return Ok(());
    }

    // Make sure the names section is available in the wasm module because we'll
    // want to remap those function indices, and then actually remap all
    // function indices. We're going to be injecting a few imported functions
    // below which will shift the index space for all defined functions.
    input.parse_wasm_names();
    Remap {
        code_idx_to_descriptor: &info.code_idx_to_descriptor,
        old_num_imports: input
            .module
            .import_section()
            .map(|s| s.functions())
            .unwrap_or(0) as u32,
    }.remap_module(input.module);

    info.delete_function_table_entries(input);
    info.inject_imports(input)?;
    info.rewrite_calls(input);
    Ok(())
}

#[derive(Default)]
struct ClosureDescriptors {
    /// A list of elements to remove from the function table. The first element
    /// of the pair is the index of the entry in the element section, and the
    /// second element of the pair is the index within that entry to remove.
    element_removal_list: Vec<(usize, usize)>,

    /// A map from indexes in the code section which contain calls to
    /// `__wbindgen_describe_closure` to the new function the whole function is
    /// replaced with as well as the descriptor that the function describes.
    ///
    /// This map is later used to replace all calls to the keys of this map with
    /// calls to the value of the map.
    code_idx_to_descriptor: BTreeMap<u32, DescribeInstruction>,
}

struct DescribeInstruction {
    new_idx: u32,
    instr_idx: usize,
    descriptor: Descriptor,
}

impl ClosureDescriptors {
    /// Find all invocations of `__wbindgen_describe_closure`.
    ///
    /// We'll be rewriting all calls to functions who call this import. Here we
    /// iterate over all code found in the module, and anything which calls our
    /// special imported function is interpreted.  The result of interpretation will
    /// inform of us of an entry to remove from the function table (as the describe
    /// function is never needed at runtime) as well as a `Descriptor` which
    /// describes the type of closure needed.
    ///
    /// All this information is then returned in the `ClosureDescriptors` return
    /// value.
    fn new(input: &mut Context) -> ClosureDescriptors {
        let wbindgen_describe_closure = match input.interpreter.describe_closure_idx() {
            Some(i) => i,
            None => return Default::default(),
        };
        let imports = input
            .module
            .import_section()
            .map(|s| s.functions())
            .unwrap_or(0);
        let mut ret = ClosureDescriptors::default();

        let code = match input.module.code_section() {
            Some(code) => code,
            None => return Default::default(),
        };
        for (i, function) in code.bodies().iter().enumerate() {
            let call_pos = function.code().elements().iter().position(|i| match i {
                Instruction::Call(i) => *i == wbindgen_describe_closure,
                _ => false,
            });
            let call_pos = match call_pos {
                Some(i) => i,
                None => continue,
            };
            let descriptor = input
                .interpreter
                .interpret_closure_descriptor(i, input.module, &mut ret.element_removal_list)
                .unwrap();
            // `new_idx` is the function-space index of the function that we'll
            // be injecting. Calls to the code function `i` will instead be
            // rewritten to calls to `new_idx`, which is an import that we'll
            // inject based on `descriptor`.
            let new_idx = (ret.code_idx_to_descriptor.len() + imports) as u32;
            ret.code_idx_to_descriptor.insert(
                i as u32,
                DescribeInstruction {
                    new_idx,
                    instr_idx: call_pos,
                    descriptor: Descriptor::decode(descriptor),
                },
            );
        }
        return ret;
    }

    /// Here we remove elements from the function table. All our descriptor
    /// functions are entries in this function table and can be removed once we
    /// use them as they're not actually needed at runtime.
    ///
    /// One option for removal is to replace the function table entry with an
    /// index to a dummy function, but for now we simply remove the table entry
    /// altogether by splitting the section and having multiple `elem` sections
    /// with holes in them.
    fn delete_function_table_entries(&self, input: &mut Context) {
        let elements = input.module.elements_section_mut().unwrap();
        let mut remove = HashMap::new();
        for (entry, idx) in self.element_removal_list.iter().cloned() {
            remove.entry(entry).or_insert(HashSet::new()).insert(idx);
        }

        let entries = mem::replace(elements.entries_mut(), Vec::new());
        let empty = HashSet::new();
        for (i, entry) in entries.into_iter().enumerate() {
            let to_remove = remove.get(&i).unwrap_or(&empty);

            let mut current = Vec::new();
            let offset = entry.offset().as_ref().unwrap();
            assert_eq!(offset.code().len(), 2);
            let mut offset = match offset.code()[0] {
                Instruction::I32Const(x) => x,
                _ => unreachable!(),
            };
            for (j, idx) in entry.members().iter().enumerate() {
                // If we keep this entry, then keep going
                if !to_remove.contains(&j) {
                    current.push(*idx);
                    continue;
                }

                // If we have members of `current` then we save off a section
                // of the function table, then update `offset` and keep going.
                let next_offset = offset + (current.len() as i32) + 1;
                if current.len() > 0 {
                    let members = mem::replace(&mut current, Vec::new());
                    let offset =
                        InitExpr::new(vec![Instruction::I32Const(offset), Instruction::End]);
                    let new_entry = ElementSegment::new(0, Some(offset), members, false);
                    elements.entries_mut().push(new_entry);
                }
                offset = next_offset;
            }
            // Any remaining function table entries get pushed at the end.
            if current.len() > 0 {
                let offset = InitExpr::new(vec![Instruction::I32Const(offset), Instruction::End]);
                let new_entry = ElementSegment::new(0, Some(offset), current, false);
                elements.entries_mut().push(new_entry);
            }
        }
    }

    /// Inject new imports into the module.
    ///
    /// This function will inject new imported functions into the `input` module
    /// described by the fields internally. These new imports will be closure
    /// factories and are freshly generated shim in JS.
    fn inject_imports(&self, input: &mut Context) -> Result<(), Error> {
        let wbindgen_describe_closure = match input.interpreter.describe_closure_idx() {
            Some(i) => i,
            None => return Ok(()),
        };

        // We'll be injecting new imports and we'll need to give them all a
        // type.  The signature is all `(i32, i32) -> i32` currently and we know
        // that this signature already exists in the module as it's the
        // signature of our `#[inline(never)]` functions. Find the type
        // signature index so we can assign it below.
        let type_idx = {
            let kind = input.module.import_section().unwrap().entries()
                [wbindgen_describe_closure as usize]
                .external();
            match kind {
                External::Function(i) => *i,
                _ => unreachable!(),
            }
        };

        // The last piece of the magic. For all our descriptors we found we
        // inject a JS shim for the descriptor. This JS shim will manufacture a
        // JS `function`, and prepare it to be invoked.
        //
        // Once all that's said and done we inject a new import into the wasm module
        // of our new wrapper, and the `Remap` step above already wrote calls to
        // this function within the module.
        for (i, instr) in self.code_idx_to_descriptor.iter() {
            let import_name = format!("__wbindgen_closure_wrapper{}", i);

            let closure = instr.descriptor.closure().unwrap();

            let (js, _ts, _js_doc) = {
                let mut builder = Js2Rust::new("", input);
                builder.prelude("this.cnt++;");
                if closure.mutable {
                    builder
                        .prelude("let a = this.a;\n")
                        .prelude("this.a = 0;\n")
                        .rust_argument("a")
                        .rust_argument("b")
                        .finally("this.a = a;\n");
                } else {
                    builder.rust_argument("this.a")
                        .rust_argument("b");
                }
                builder.finally("if (this.cnt-- == 1) d(this.a, b);");
                builder
                    .process(&closure.function)?
                    .finish("function", "f")
            };
            input.expose_add_heap_object();
            input.function_table_needed = true;
            let body = format!(
                "function(a, b, fi, di, _ignored) {{
                    const f = wasm.__wbg_function_table.get(fi);
                    const d = wasm.__wbg_function_table.get(di);
                    const cb = {};
                    cb.a = a;
                    cb.cnt = 1;
                    let real = cb.bind(cb);
                    real.original = cb;
                    return addHeapObject(real);
                }}",
                js,
            );
            input.export(&import_name, &body, None);

            let new_import = ImportEntry::new(
                "__wbindgen_placeholder__".to_string(),
                import_name,
                External::Function(type_idx as u32),
            );
            input
                .module
                .import_section_mut()
                .unwrap()
                .entries_mut()
                .push(new_import);
        }
        Ok(())
    }

    /// The final step, rewriting calls to `__wbindgen_describe_closure` to the
    /// imported functions
    fn rewrite_calls(&self, input: &mut Context) {
        // FIXME: Ok so this is a bit sketchy in that it introduces overhead.
        // What we're doing is taking a our #[inline(never)] shim and *not*
        // removing it, only switching the one function that it calls internally.
        //
        // This isn't great because now we have this non-inlined function which
        // would certainly benefit from getting inlined. It's a tiny function
        // though and surrounded by allocation so it's probably not a huge
        // problem in the long run. Note that `wasm-opt` also implements
        // inlining, so we can likely rely on that too.
        //
        // Still though, it'd be great to not only delete calls to
        // `__wbindgen_describe_closure`, it'd be great to remove all of the
        // `breaks_if_inlined` functions entirely.
        let code = input.module.code_section_mut().unwrap();
        for (i, instr) in self.code_idx_to_descriptor.iter() {
            let func = &mut code.bodies_mut()[*i as usize];
            let new_instr = Instruction::Call(instr.new_idx);
            func.code_mut().elements_mut()[instr.instr_idx] = new_instr;
        }
    }
}

struct Remap<'a> {
    code_idx_to_descriptor: &'a BTreeMap<u32, DescribeInstruction>,
    old_num_imports: u32,
}

impl<'a> Remap<'a> {
    fn remap_module(&self, module: &mut Module) {
        for section in module.sections_mut() {
            match section {
                Section::Export(e) => self.remap_export_section(e),
                Section::Element(e) => self.remap_element_section(e),
                Section::Code(e) => self.remap_code_section(e),
                Section::Start(i) => {
                    self.remap_idx(i);
                }
                Section::Name(n) => self.remap_name_section(n),
                _ => {}
            }
        }
    }

    fn remap_export_section(&self, section: &mut ExportSection) {
        for entry in section.entries_mut() {
            self.remap_export_entry(entry);
        }
    }

    fn remap_export_entry(&self, entry: &mut ExportEntry) {
        match entry.internal_mut() {
            Internal::Function(i) => {
                self.remap_idx(i);
            }
            _ => {}
        }
    }

    fn remap_element_section(&self, section: &mut ElementSection) {
        for entry in section.entries_mut() {
            self.remap_element_entry(entry);
        }
    }

    fn remap_element_entry(&self, entry: &mut ElementSegment) {
        for member in entry.members_mut() {
            self.remap_idx(member);
        }
    }

    fn remap_code_section(&self, section: &mut CodeSection) {
        for body in section.bodies_mut() {
            self.remap_func_body(body);
        }
    }

    fn remap_func_body(&self, body: &mut FuncBody) {
        self.remap_instructions(body.code_mut());
    }

    fn remap_instructions(&self, code: &mut Instructions) {
        for instr in code.elements_mut() {
            self.remap_instruction(instr);
        }
    }

    fn remap_instruction(&self, instr: &mut Instruction) {
        match instr {
            Instruction::Call(i) => {
                self.remap_idx(i);
            }
            _ => {}
        }
    }

    fn remap_name_section(&self, names: &mut NameSection) {
        match names {
            NameSection::Function(f) => self.remap_function_name_section(f),
            NameSection::Local(f) => self.remap_local_name_section(f),
            _ => {}
        }
    }

    fn remap_function_name_section(&self, names: &mut FunctionNameSection) {
        let map = names.names_mut();
        let new = IndexMap::with_capacity(map.len());
        for (mut idx, name) in mem::replace(map, new) {
            if !self.remap_idx(&mut idx) {
                map.insert(idx, name);
            }
        }
    }

    fn remap_local_name_section(&self, names: &mut LocalNameSection) {
        let map = names.local_names_mut();
        let new = IndexMap::with_capacity(map.len());
        for (mut idx, name) in mem::replace(map, new) {
            if !self.remap_idx(&mut idx) {
                map.insert(idx, name);
            }
        }
    }

    /// Returns whether `idx` pointed to a previously known descriptor function
    /// that we're switching to an import
    fn remap_idx(&self, idx: &mut u32) -> bool {
        // If this was an imported function we didn't reorder those, so nothing
        // to do.
        if *idx < self.old_num_imports {
            return false;
        }
        // ... otherwise we're injecting a number of new imports, so offset
        // everything.
        *idx += self.code_idx_to_descriptor.len() as u32;
        false
    }
}
