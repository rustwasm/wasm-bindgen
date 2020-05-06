//! Transformation for wasm-bindgen to enable usage of `anyref` in a wasm
//! module.
//!
//! This crate is in charge of enabling code using `wasm-bindgen` to use the
//! `anyref` type inside of the wasm module. This transformation pass primarily
//! wraps exports and imports in shims which use `anyref`, but quickly turn them
//! into `i32` value types. This is all largely a stopgap until Rust has
//! first-class support for the `anyref` type, but that's thought to be in the
//! far future and will take quite some time to implement. In the meantime, we
//! have this!
//!
//! The pass here works by collecting information during binding generation
//! about imports and exports. Afterwards this pass runs in one go against a
//! wasm module, updating exports, imports, calls to these functions, etc. The
//! goal at least is to have valid wasm modules coming in that don't use
//! `anyref` and valid wasm modules going out which use `anyref` at the fringes.

use anyhow::{anyhow, bail, Error};
use std::cmp;
use std::collections::{BTreeMap, HashMap, HashSet};
use std::mem;
use walrus::ir::*;
use walrus::{ElementId, ExportId, ImportId, InstrLocId, TypeId};
use walrus::{FunctionId, GlobalId, InitExpr, Module, TableId, ValType};

// must be kept in sync with src/lib.rs and ANYREF_HEAP_START
const DEFAULT_MIN: u32 = 32;

/// State of the anyref pass, used to collect information while bindings are
/// generated and used eventually to actually execute the entire pass.
#[derive(Default)]
pub struct Context {
    // Functions within the module that we're gonna be wrapping, organized by
    // type. The `Function` contains information about what arguments/return
    // values in the function signature should turn into anyref.
    imports: HashMap<ImportId, Function>,
    exports: HashMap<ExportId, Function>,

    // List of functions we're transforming that are present in the function
    // table. Each index here is an index into the function table, and the
    // `Function` describes how we're transforming it.
    new_elements: Vec<(u32, Function)>,

    // When wrapping closures with new shims, this is the index of the next
    // table entry that we'll be handing out.
    new_element_offset: u32,

    // Map of the existing function table, keyed by offset and contains the
    // final offset plus the element segment used to initialized that range.
    elements: BTreeMap<u32, ElementId>,

    // The anyref table we'll be using, injected after construction
    table: Option<TableId>,
}

pub struct Meta {
    pub table: TableId,
    pub alloc: Option<FunctionId>,
    pub drop_slice: Option<FunctionId>,
    pub live_count: Option<FunctionId>,
}

struct Transform<'a> {
    cx: &'a mut Context,

    // A map of functions to intrinsics that they represent
    intrinsic_map: HashMap<FunctionId, Intrinsic>,
    // A map of old import functions to the new internally-defined shims which
    // call the correct new import functions
    import_map: HashMap<FunctionId, FunctionId>,
    // A set of all shims we've created
    shims: HashSet<FunctionId>,

    // Indices of items that we have injected or found. This state is maintained
    // during the pass execution.
    table: TableId,
    clone_ref: Option<FunctionId>,
    heap_alloc: Option<FunctionId>,
    heap_dealloc: Option<FunctionId>,
    stack_pointer: GlobalId,
}

struct Function {
    // A map of argument index to whether it's an owned or borrowed anyref
    // (owned = true)
    args: HashMap<usize, bool>,
    ret_anyref: bool,
}

enum Intrinsic {
    TableGrow,
    TableSetNull,
    DropRef,
    CloneRef,
}

impl Context {
    /// Executed first very early over a wasm module, used to learn about how
    /// large the function table is so we know what indexes to hand out when
    /// we're appending entries.
    pub fn prepare(&mut self, module: &mut Module) -> Result<(), Error> {
        // Figure out what the maximum index of functions pointers are. We'll
        // be adding new entries to the function table later (maybe) so
        // precalculate this ahead of time.
        if let Some(t) = module.tables.main_function_table()? {
            let t = module.tables.get(t);
            for id in t.elem_segments.iter() {
                let elem = module.elements.get(*id);
                let offset = match &elem.kind {
                    walrus::ElementKind::Active { offset, .. } => offset,
                    _ => continue,
                };
                let offset = match offset {
                    walrus::InitExpr::Value(Value::I32(n)) => *n as u32,
                    other => bail!("invalid offset for segment of function table {:?}", other),
                };
                let max = offset + elem.members.len() as u32;
                self.new_element_offset = cmp::max(self.new_element_offset, max);
                self.elements.insert(offset, *id);
            }
        }

        // Add in an anyref table to the module, which we'll be using for
        // our transform below.
        self.table = Some(module.tables.add_local(DEFAULT_MIN, None, ValType::Anyref));

        Ok(())
    }

    /// Store information about an imported function that needs to be
    /// transformed. The actual transformation happens later during `run`.
    pub fn import_xform(
        &mut self,
        id: ImportId,
        anyref: &[(usize, bool)],
        ret_anyref: bool,
    ) -> &mut Self {
        if let Some(f) = self.function(anyref, ret_anyref) {
            self.imports.insert(id, f);
        }
        self
    }

    /// Store information about an exported function that needs to be
    /// transformed. The actual transformation happens later during `run`.
    pub fn export_xform(
        &mut self,
        id: ExportId,
        anyref: &[(usize, bool)],
        ret_anyref: bool,
    ) -> &mut Self {
        if let Some(f) = self.function(anyref, ret_anyref) {
            self.exports.insert(id, f);
        }
        self
    }

    /// Store information about a function pointer that needs to be transformed.
    /// The actual transformation happens later during `run`. Returns an index
    /// that the new wrapped function pointer will be injected at.
    pub fn table_element_xform(
        &mut self,
        idx: u32,
        anyref: &[(usize, bool)],
        ret_anyref: bool,
    ) -> Option<u32> {
        self.function(anyref, ret_anyref).map(|f| {
            self.new_elements.push((idx, f));
            self.new_elements.len() as u32 + self.new_element_offset - 1
        })
    }

    fn function(&self, anyref: &[(usize, bool)], ret_anyref: bool) -> Option<Function> {
        if !ret_anyref && anyref.len() == 0 {
            return None;
        }
        Some(Function {
            args: anyref.iter().cloned().collect(),
            ret_anyref,
        })
    }

    pub fn run(&mut self, module: &mut Module) -> Result<Meta, Error> {
        let table = self.table.unwrap();

        // Inject a stack pointer global which will be used for managing the
        // stack on the anyref table.
        let init = InitExpr::Value(Value::I32(DEFAULT_MIN as i32));
        let stack_pointer = module.globals.add_local(ValType::I32, true, init);

        let mut heap_alloc = None;
        let mut heap_dealloc = None;
        let mut drop_slice = None;
        let mut live_count = None;

        // Find exports of some intrinsics which we only need for a runtime
        // implementation.
        let mut to_delete = Vec::new();
        for export in module.exports.iter() {
            let f = match export.item {
                walrus::ExportItem::Function(f) => f,
                _ => continue,
            };
            match export.name.as_str() {
                "__anyref_table_alloc" => heap_alloc = Some(f),
                "__anyref_table_dealloc" => heap_dealloc = Some(f),
                "__anyref_drop_slice" => drop_slice = Some(f),
                "__anyref_heap_live_count" => live_count = Some(f),
                _ => continue,
            }
            to_delete.push(export.id());
        }
        for id in to_delete {
            module.exports.delete(id);
        }
        let mut clone_ref = None;
        if let Some(heap_alloc) = heap_alloc {
            // Create a shim function that looks like:
            //
            // (func __wbindgen_object_clone_ref (param i32) (result i32)
            //      (local i32)
            //      (table.set
            //          (tee_local 1 (call $heap_alloc))
            //          (table.get (local.get 0)))
            //      (local.get 1))
            let mut builder =
                walrus::FunctionBuilder::new(&mut module.types, &[ValType::I32], &[ValType::I32]);
            let arg = module.locals.add(ValType::I32);
            let local = module.locals.add(ValType::I32);

            let mut body = builder.func_body();
            body.call(heap_alloc)
                .local_tee(local)
                .local_get(arg)
                .table_get(table)
                .table_set(table)
                .local_get(local);

            let func = builder.finish(vec![arg], &mut module.funcs);
            let name = "__wbindgen_object_clone_ref".to_string();
            module.funcs.get_mut(func).name = Some(name);
            clone_ref = Some(func);
        }

        // And run the transformation!
        Transform {
            cx: self,
            intrinsic_map: HashMap::new(),
            import_map: HashMap::new(),
            shims: HashSet::new(),
            table,
            clone_ref,
            heap_alloc,
            heap_dealloc,
            stack_pointer,
        }
        .run(module)?;

        Ok(Meta {
            table,
            alloc: heap_alloc,
            drop_slice,
            live_count,
        })
    }
}

impl Transform<'_> {
    fn run(&mut self, module: &mut Module) -> Result<(), Error> {
        // Detect all the various intrinsics and such. This will also along the
        // way inject an intrinsic for cloning an anyref.
        self.find_intrinsics(module)?;

        // Perform transformations of imports, exports, and function pointers.
        self.process_imports(module)?;
        assert!(self.cx.imports.is_empty());
        self.process_exports(module)?;
        assert!(self.cx.exports.is_empty());
        self.process_elements(module)?;
        assert!(self.cx.new_elements.is_empty());

        // If we didn't actually transform anything, no need to inject or
        // rewrite anything from below.
        if self.shims.is_empty() {
            return Ok(());
        }

        // Perform all instruction transformations to rewrite calls between
        // functions and make sure everything is still hooked up right.
        self.rewrite_calls(module)?;

        Ok(())
    }

    fn find_intrinsics(&mut self, module: &mut Module) -> Result<(), Error> {
        // Build up a map of various imported intrinsics to wire them up to
        // different implementations or different functions.
        for import in module.imports.iter_mut() {
            let f = match import.kind {
                walrus::ImportKind::Function(f) => f,
                _ => continue,
            };
            if import.module == "__wbindgen_anyref_xform__" {
                match import.name.as_str() {
                    "__wbindgen_anyref_table_grow" => {
                        self.intrinsic_map.insert(f, Intrinsic::TableGrow);
                    }
                    "__wbindgen_anyref_table_set_null" => {
                        self.intrinsic_map.insert(f, Intrinsic::TableSetNull);
                    }
                    n => bail!("unknown intrinsic: {}", n),
                }
            } else if import.module == "__wbindgen_placeholder__" {
                match import.name.as_str() {
                    "__wbindgen_object_drop_ref" => {
                        self.intrinsic_map.insert(f, Intrinsic::DropRef);
                    }
                    "__wbindgen_object_clone_ref" => {
                        self.intrinsic_map.insert(f, Intrinsic::CloneRef);
                    }
                    _ => continue,
                }
            } else {
                continue;
            }

            // Make sure we don't actually end up using the original import
            // because any invocation of them should be remapped to something
            // else.
            import.name = format!("{}_unused", import.name);
        }

        Ok(())
    }

    fn heap_alloc(&self) -> Result<FunctionId, Error> {
        self.heap_alloc
            .ok_or_else(|| anyhow!("failed to find the `__wbindgen_anyref_table_alloc` function"))
    }

    fn clone_ref(&self) -> Result<FunctionId, Error> {
        self.clone_ref
            .ok_or_else(|| anyhow!("failed to find intrinsics to enable `clone_ref` function"))
    }

    fn heap_dealloc(&self) -> Result<FunctionId, Error> {
        self.heap_dealloc
            .ok_or_else(|| anyhow!("failed to find the `__wbindgen_anyref_table_dealloc` function"))
    }

    fn process_imports(&mut self, module: &mut Module) -> Result<(), Error> {
        for import in module.imports.iter_mut() {
            let f = match import.kind {
                walrus::ImportKind::Function(f) => f,
                _ => continue,
            };
            let func = match self.cx.imports.remove(&import.id()) {
                Some(s) => s,
                None => continue,
            };

            let (shim, anyref_ty) = self.append_shim(
                f,
                &import.name,
                func,
                &mut module.types,
                &mut module.funcs,
                &mut module.locals,
            )?;
            self.import_map.insert(f, shim);
            match &mut module.funcs.get_mut(f).kind {
                walrus::FunctionKind::Import(f) => f.ty = anyref_ty,
                _ => unreachable!(),
            }
        }
        Ok(())
    }

    fn process_exports(&mut self, module: &mut Module) -> Result<(), Error> {
        // let mut new_exports = Vec::new();
        for export in module.exports.iter_mut() {
            let f = match export.item {
                walrus::ExportItem::Function(f) => f,
                _ => continue,
            };
            let function = match self.cx.exports.remove(&export.id()) {
                Some(s) => s,
                None => continue,
            };
            let (shim, _anyref_ty) = self.append_shim(
                f,
                &export.name,
                function,
                &mut module.types,
                &mut module.funcs,
                &mut module.locals,
            )?;
            export.item = shim.into();
        }
        Ok(())
    }

    fn process_elements(&mut self, module: &mut Module) -> Result<(), Error> {
        let table = match module.tables.main_function_table()? {
            Some(t) => t,
            None => return Ok(()),
        };
        let table = module.tables.get_mut(table);

        // Create shims for all our functions and append them all to the segment
        // which places elements at the end.
        let mut new_segment = Vec::new();
        for (idx, function) in mem::replace(&mut self.cx.new_elements, Vec::new()) {
            let (&offset, &orig_element) = self
                .cx
                .elements
                .range(..=idx)
                .next_back()
                .ok_or(anyhow!("failed to find segment defining index {}", idx))?;
            let target = module.elements.get(orig_element).members[(idx - offset) as usize].ok_or(
                anyhow!("function index {} not present in element segment", idx),
            )?;
            let (shim, _anyref_ty) = self.append_shim(
                target,
                &format!("closure{}", idx),
                function,
                &mut module.types,
                &mut module.funcs,
                &mut module.locals,
            )?;
            new_segment.push(Some(shim));
        }

        // ... and next update the limits of the table in case any are listed.
        let new_max = self.cx.new_element_offset + new_segment.len() as u32;
        table.initial = cmp::max(table.initial, new_max);
        if let Some(max) = table.maximum {
            table.maximum = Some(cmp::max(max, new_max));
        }
        let kind = walrus::ElementKind::Active {
            table: table.id(),
            offset: InitExpr::Value(Value::I32(self.cx.new_element_offset as i32)),
        };
        let segment = module.elements.add(kind, ValType::Funcref, new_segment);
        table.elem_segments.insert(segment);

        Ok(())
    }

    fn append_shim(
        &mut self,
        shim_target: FunctionId,
        name: &str,
        mut func: Function,
        types: &mut walrus::ModuleTypes,
        funcs: &mut walrus::ModuleFunctions,
        locals: &mut walrus::ModuleLocals,
    ) -> Result<(FunctionId, TypeId), Error> {
        let target = funcs.get_mut(shim_target);
        let (is_export, ty) = match &target.kind {
            walrus::FunctionKind::Import(f) => (false, f.ty),
            walrus::FunctionKind::Local(f) => (true, f.ty()),
            _ => unreachable!(),
        };

        let target_ty = types.get(ty);
        let target_ty_params = target_ty.params().to_vec();
        let target_ty_results = target_ty.results().to_vec();

        // Learn about the various operations we're doing up front. Afterwards
        // we'll have a better idea bout what sort of code we're gonna be
        // generating.
        enum Convert {
            None,
            Store { owned: bool },
            Load { owned: bool },
        }
        let mut param_tys = Vec::new();
        let mut param_convert = Vec::new();
        let mut anyref_stack = 0;

        for (i, old_ty) in target_ty.params().iter().enumerate() {
            let is_owned = func.args.remove(&i);
            let new_ty = is_owned
                .map(|_which| ValType::Anyref)
                .unwrap_or(old_ty.clone());
            param_tys.push(new_ty.clone());
            if new_ty == *old_ty {
                param_convert.push(Convert::None);
            } else if is_export {
                // We're calling an export, so we need to push this anyref into
                // a table somehow.
                param_convert.push(Convert::Store {
                    owned: is_owned.unwrap(),
                });
                if is_owned == Some(false) {
                    anyref_stack += 1;
                }
            } else {
                // We're calling an import, so we just need to fetch our table
                // value.
                param_convert.push(Convert::Load {
                    owned: is_owned.unwrap(),
                });
            }
        }

        let new_ret = if func.ret_anyref {
            assert_eq!(target_ty.results(), &[ValType::I32]);
            vec![ValType::Anyref]
        } else {
            target_ty.results().to_vec()
        };
        let anyref_ty = types.add(&param_tys, &new_ret);

        // If we're an export then our shim is what's actually going to get
        // exported, and it's going to have the anyref signature.
        //
        // If we're an import, then our shim is what the Rust code calls, which
        // means it'll have the original signature. The existing import's
        // signature, however, is transformed to be an anyref signature.
        let shim_ty = if is_export { anyref_ty } else { ty };

        let mut builder = walrus::FunctionBuilder::new(
            types,
            if is_export {
                &param_tys
            } else {
                &target_ty_params
            },
            if is_export {
                &new_ret
            } else {
                &target_ty_results
            },
        );
        let mut body = builder.func_body();
        let params = types
            .get(shim_ty)
            .params()
            .iter()
            .cloned()
            .map(|ty| locals.add(ty))
            .collect::<Vec<_>>();

        // Unconditionally allocate some locals which get cleaned up in later
        // gc passes if we don't actually end up using them.
        let fp = locals.add(ValType::I32);
        let scratch_i32 = locals.add(ValType::I32);
        let scratch_anyref = locals.add(ValType::Anyref);

        // Update our stack pointer if there's any borrowed anyref objects.
        if anyref_stack > 0 {
            body.global_get(self.stack_pointer)
                .const_(Value::I32(anyref_stack))
                .binop(BinaryOp::I32Sub)
                .local_tee(fp)
                .global_set(self.stack_pointer);
        }
        let mut next_stack_offset = 0;

        for (i, convert) in param_convert.iter().enumerate() {
            match *convert {
                Convert::None => {
                    body.local_get(params[i]);
                }
                Convert::Load { owned: true } => {
                    // load the anyref onto the stack, then afterwards
                    // deallocate our index, leaving the anyref on the stack.
                    body.local_get(params[i])
                        .table_get(self.table)
                        .local_get(params[i])
                        .call(self.heap_dealloc()?);
                }
                Convert::Load { owned: false } => {
                    body.local_get(params[i]).table_get(self.table);
                }
                Convert::Store { owned: true } => {
                    // Allocate space for the anyref, store it, and then leave
                    // the index of the allocated anyref on the stack.
                    body.call(self.heap_alloc()?)
                        .local_tee(scratch_i32)
                        .local_get(params[i])
                        .table_set(self.table)
                        .local_get(scratch_i32);
                }
                Convert::Store { owned: false } => {
                    // Store an anyref at an offset from our function's stack
                    // pointer frame.
                    body.local_get(fp);
                    let idx_local = if next_stack_offset == 0 {
                        fp
                    } else {
                        body.i32_const(next_stack_offset)
                            .binop(BinaryOp::I32Add)
                            .local_tee(scratch_i32);
                        scratch_i32
                    };
                    next_stack_offset += 1;
                    body.local_get(params[i])
                        .table_set(self.table)
                        .local_get(idx_local);
                }
            }
        }

        // Now that we've converted all the arguments, call the original
        // function. This may be either an import or an export which we're
        // wrapping.
        body.call(shim_target);

        // If an anyref value is returned, then we need to be sure to apply
        // special treatment to convert it to an i32 as well. Note that only
        // owned anyref values can be returned, so that's all that's handled
        // here.
        if func.ret_anyref {
            if is_export {
                // We're an export so we have an i32 on the stack and need to
                // convert it to an anyref, basically by doing the same as an
                // owned load above: get the value then deallocate our slot.
                body.local_tee(scratch_i32)
                    .table_get(self.table)
                    .local_get(scratch_i32)
                    .call(self.heap_dealloc()?);
            } else {
                // Imports are the opposite, we have any anyref on the stack
                // and convert it to an i32 by allocating space for it and
                // storing it there.
                body.local_set(scratch_anyref)
                    .call(self.heap_alloc()?)
                    .local_tee(scratch_i32)
                    .local_get(scratch_anyref)
                    .table_set(self.table)
                    .local_get(scratch_i32);
            }
        }

        // On function exit restore our anyref stack pointer if we decremented
        // it to start off.
        //
        // Note that we pave over all our stack slots with `ref.null` to ensure
        // that the table doesn't accidentally hold a strong reference to items
        // no longer in use by our wasm instance.
        //
        // TODO: use `table.fill` once that's spec'd
        if anyref_stack > 0 {
            for i in 0..anyref_stack {
                body.local_get(fp);
                if i > 0 {
                    body.i32_const(i).binop(BinaryOp::I32Add);
                }
                body.ref_null();
                body.table_set(self.table);
            }

            body.local_get(fp)
                .i32_const(anyref_stack)
                .binop(BinaryOp::I32Add)
                .global_set(self.stack_pointer);
        }

        // Create the final expression node and then finish the function builder
        // with a fresh type we've been calculating so far. Give the function a
        // nice name for debugging and then we're good to go!
        let id = builder.finish(params, funcs);
        let name = format!("{} anyref shim", name);
        funcs.get_mut(id).name = Some(name);
        self.shims.insert(id);
        Ok((id, anyref_ty))
    }

    fn rewrite_calls(&mut self, module: &mut Module) -> Result<(), Error> {
        for (id, func) in module.funcs.iter_local_mut() {
            if self.shims.contains(&id) {
                continue;
            }
            let entry = func.entry_block();
            dfs_pre_order_mut(
                &mut Rewrite {
                    clone_ref: self.clone_ref()?,
                    heap_dealloc: self.heap_dealloc()?,
                    xform: self,
                },
                func,
                entry,
            );
        }

        return Ok(());

        struct Rewrite<'a, 'b> {
            xform: &'a Transform<'b>,
            clone_ref: FunctionId,
            heap_dealloc: FunctionId,
        }

        impl VisitorMut for Rewrite<'_, '_> {
            fn start_instr_seq_mut(&mut self, seq: &mut InstrSeq) {
                for i in (0..seq.instrs.len()).rev() {
                    let call = match &mut seq.instrs[i].0 {
                        Instr::Call(call) => call,
                        _ => continue,
                    };
                    let intrinsic = match self.xform.intrinsic_map.get(&call.func) {
                        Some(f) => f,
                        None => {
                            // If this wasn't a call of an intrinsic, but it was a
                            // call of one of our old import functions then we
                            // switch the functions we're calling here.
                            if let Some(f) = self.xform.import_map.get(&call.func) {
                                call.func = *f;
                            }
                            continue;
                        }
                    };

                    match intrinsic {
                        Intrinsic::TableGrow => {
                            // Switch this to a `table.grow` instruction...
                            seq.instrs[i].0 = TableGrow {
                                table: self.xform.table,
                            }
                            .into();
                            // ... and then insert a `ref.null` before the
                            // preceding instruction as the value to grow the
                            // table with.
                            seq.instrs
                                .insert(i - 1, (RefNull {}.into(), InstrLocId::default()));
                        }
                        Intrinsic::TableSetNull => {
                            // Switch this to a `table.set` instruction...
                            seq.instrs[i].0 = TableSet {
                                table: self.xform.table,
                            }
                            .into();
                            // ... and then insert a `ref.null` as the
                            // preceding instruction
                            seq.instrs
                                .insert(i, (RefNull {}.into(), InstrLocId::default()));
                        }
                        Intrinsic::DropRef => call.func = self.heap_dealloc,
                        Intrinsic::CloneRef => call.func = self.clone_ref,
                    }
                }
            }
        }
    }
}
