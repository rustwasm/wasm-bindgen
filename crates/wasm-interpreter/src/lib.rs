//! A tiny and incomplete wasm interpreter
//!
//! This module contains a tiny and incomplete wasm interpreter built on top of
//! `walrus`'s module structure. Each `Interpreter` contains some state
//! about the execution of a wasm instance. The "incomplete" part here is
//! related to the fact that this is *only* used to execute the various
//! descriptor functions for wasm-bindgen.
//!
//! As a recap, the wasm-bindgen macro generate "descriptor functions" which
//! basically as a mapping of rustc's trait resolution in executable code. This
//! allows us to detect, after the macro is invoke, what trait selection did and
//! what types of functions look like. By executing descriptor functions they'll
//! each invoke a known import (with only one argument) some number of times,
//! which gives us a list of `u32` values to then decode.
//!
//! The interpreter here is only geared towards this one exact use case, so it's
//! quite small and likely not extra-efficient.

#![deny(missing_docs)]

use std::collections::{BTreeMap, HashMap, HashSet};
use walrus::ir::ExprId;
use walrus::{FunctionId, LocalFunction, LocalId, Module, TableId};

/// A ready-to-go interpreter of a wasm module.
///
/// An interpreter currently represents effectively cached state. It is reused
/// between calls to `interpret` and is precomputed from a `Module`. It houses
/// state like the wasm stack, wasm memory, etc.
#[derive(Default)]
pub struct Interpreter {
    // Function index of the `__wbindgen_describe` and
    // `__wbindgen_describe_closure` imported functions. We special case this
    // to know when the environment's imported function is called.
    describe_id: Option<FunctionId>,
    describe_closure_id: Option<FunctionId>,

    // Id of the function table
    functions: Option<TableId>,

    // A mapping of string names to the function index, filled with all exported
    // functions.
    name_map: HashMap<String, FunctionId>,

    // The current stack pointer (global 0) and wasm memory (the stack). Only
    // used in a limited capacity.
    sp: i32,
    mem: Vec<i32>,

    // The descriptor which we're assembling, a list of `u32` entries. This is
    // very specific to wasm-bindgen and is the purpose for the existence of
    // this module.
    descriptor: Vec<u32>,

    // When invoking the `__wbindgen_describe_closure` imported function, this
    // stores the last table index argument, used for finding a different
    // descriptor.
    descriptor_table_idx: Option<u32>,
}

impl Interpreter {
    /// Creates a new interpreter from a provided `Module`, precomputing all
    /// information necessary to interpret further.
    ///
    /// Note that the `module` passed in to this function must be the same as
    /// the `module` passed to `interpret` below.
    pub fn new(module: &Module) -> Result<Interpreter, failure::Error> {
        let mut ret = Interpreter::default();

        // The descriptor functions shouldn't really use all that much memory
        // (the LLVM call stack, now the wasm stack). To handle that let's give
        // our selves a little bit of memory and set the stack pointer (global
        // 0) to the top.
        ret.mem = vec![0; 0x100];
        ret.sp = ret.mem.len() as i32;

        // Figure out where the `__wbindgen_describe` imported function is, if
        // it exists. We'll special case calls to this function as our
        // interpretation should only invoke this function as an imported
        // function.
        for import in module.imports.iter() {
            let id = match import.kind {
                walrus::ImportKind::Function(id) => id,
                _ => continue,
            };
            if import.module != "__wbindgen_placeholder__" {
                continue;
            }
            if import.name == "__wbindgen_describe" {
                ret.describe_id = Some(id);
            } else if import.name == "__wbindgen_describe_closure" {
                ret.describe_closure_id = Some(id);
            }
        }

        // Build up the mapping of exported functions to function ids.
        for export in module.exports.iter() {
            let id = match export.item {
                walrus::ExportItem::Function(id) => id,
                _ => continue,
            };
            ret.name_map.insert(export.name.to_string(), id);
        }

        ret.functions = module.tables.main_function_table()?;

        return Ok(ret);
    }

    /// Interprets the execution of the descriptor function `func`.
    ///
    /// This function will execute `func` in the `module` provided. Note that
    /// the `module` provided here must be the same as the one passed to `new`
    /// when this `Interpreter` was constructed.
    ///
    /// The `func` must be a wasm-bindgen descriptor function meaning that it
    /// doesn't do anything like use floats or i64. Instead all it should do is
    /// call other functions, sometimes some stack pointer manipulation, and
    /// then call the one imported `__wbindgen_describe` function. Anything else
    /// will cause this interpreter to panic.
    ///
    /// When the descriptor has finished running the assembled descriptor list
    /// is returned. The descriptor returned can then be re-parsed into an
    /// actual `Descriptor` in the cli-support crate.
    ///
    /// # Return value
    ///
    /// Returns `Some` if `func` was found in the `module` and `None` if it was
    /// not found in the `module`.
    pub fn interpret_descriptor(&mut self, func: &str, module: &Module) -> Option<&[u32]> {
        let id = *self.name_map.get(func)?;
        self.interpret_descriptor_id(id, module)
    }

    fn interpret_descriptor_id(&mut self, id: FunctionId, module: &Module) -> Option<&[u32]> {
        self.descriptor.truncate(0);

        // We should have a blank wasm and LLVM stack at both the start and end
        // of the call.
        assert_eq!(self.sp, self.mem.len() as i32);
        self.call(id, module, &[]);
        assert_eq!(self.sp, self.mem.len() as i32);
        Some(&self.descriptor)
    }

    /// Interprets a "closure descriptor", figuring out the signature of the
    /// closure that was intended.
    ///
    /// This function will take an `id` which is known to internally
    /// execute `__wbindgen_describe_closure` and interpret it. The
    /// `wasm-bindgen` crate controls all callers of this internal import. It
    /// will then take the index passed to `__wbindgen_describe_closure` and
    /// interpret it as a function pointer. This means it'll look up within the
    /// element section (function table) which index it points to. Upon finding
    /// the relevant entry it'll assume that function is a descriptor function,
    /// and then it will execute the descriptor function.
    ///
    /// The returned value is the return value of the descriptor function found.
    /// The `entry_removal_list` list is also then populated with an index of
    /// the entry in the elements section (and then the index within that
    /// section) of the function that needs to be snip'd out.
    pub fn interpret_closure_descriptor(
        &mut self,
        id: FunctionId,
        module: &Module,
        entry_removal_list: &mut HashSet<usize>,
    ) -> Option<&[u32]> {
        // Call the `id` function. This is an internal `#[inline(never)]`
        // whose code is completely controlled by the `wasm-bindgen` crate, so
        // it should take some arguments (the number of arguments depends on the
        // optimization level) and return one (all of which we don't care about
        // here). What we're interested in is that while executing this function
        // it'll call `__wbindgen_describe_closure` with an argument that we
        // look for.
        assert!(self.descriptor_table_idx.is_none());

        let func = module.funcs.get(id);
        let params = module.types.get(func.ty()).params();
        assert!(
            params.iter().all(|p| *p == walrus::ValType::I32),
            "closure descriptors should only have i32 params"
        );
        let num_params = params.len();
        assert!(
            num_params <= 2,
            "closure descriptors have 2 parameters, but might lose some parameters due to LTO"
        );

        let args = vec![0; num_params];
        self.call(id, module, &args);
        let descriptor_table_idx =
            self.descriptor_table_idx
                .take()
                .expect("descriptor function should return index") as usize;

        // After we've got the table index of the descriptor function we're
        // interested go take a look in the function table to find what the
        // actual index of the function is.
        let functions = self.functions.expect("function table should be present");
        let functions = match &module.tables.get(functions).kind {
            walrus::TableKind::Function(f) => f,
            _ => unreachable!(),
        };
        let descriptor_id = functions
            .elements
            .get(descriptor_table_idx)
            .expect("out of bounds read of function table")
            .expect("attempting to execute null function");

        // This is used later to actually remove the entry from the table, but
        // we don't do the removal just yet
        entry_removal_list.insert(descriptor_table_idx);

        // And now execute the descriptor!
        self.interpret_descriptor_id(descriptor_id, module)
    }

    /// Returns the function id of the `__wbindgen_describe_closure`
    /// imported function.
    pub fn describe_closure_id(&self) -> Option<FunctionId> {
        self.describe_closure_id
    }

    /// Returns the detected id of the function table.
    pub fn function_table_id(&self) -> Option<TableId> {
        self.functions
    }

    fn call(&mut self, id: FunctionId, module: &Module, args: &[i32]) -> Option<i32> {
        let func = module.funcs.get(id);
        log::debug!("starting a call of {:?} {:?}", id, func.name);
        log::debug!("arguments {:?}", args);
        let local = match &func.kind {
            walrus::FunctionKind::Local(l) => l,
            _ => panic!("can only call locally defined functions"),
        };

        let entry = local.entry_block();
        let block = local.block(entry);

        let mut frame = Frame {
            module,
            local,
            interp: self,
            locals: BTreeMap::new(),
            done: false,
        };

        assert_eq!(local.args.len(), args.len());
        for (arg, val) in local.args.iter().zip(args) {
            frame.locals.insert(*arg, *val);
        }

        if block.exprs.len() > 0 {
            for expr in block.exprs[..block.exprs.len() - 1].iter() {
                let ret = frame.eval(*expr);
                if frame.done {
                    return ret;
                }
            }
        }
        block.exprs.last().and_then(|e| frame.eval(*e))
    }
}

struct Frame<'a> {
    module: &'a Module,
    local: &'a LocalFunction,
    interp: &'a mut Interpreter,
    locals: BTreeMap<LocalId, i32>,
    done: bool,
}

impl Frame<'_> {
    fn local(&self, id: LocalId) -> i32 {
        self.locals.get(&id).cloned().unwrap_or(0)
    }

    fn eval(&mut self, expr: ExprId) -> Option<i32> {
        use walrus::ir::*;

        match self.local.get(expr) {
            Expr::Const(c) => match c.value {
                Value::I32(n) => Some(n),
                _ => panic!("non-i32 constant"),
            },
            Expr::LocalGet(e) => Some(self.local(e.local)),
            Expr::LocalSet(e) => {
                let val = self.eval(e.value).expect("must eval to i32");
                self.locals.insert(e.local, val);
                None
            }

            // Blindly assume all globals are the stack pointer
            Expr::GlobalGet(_) => Some(self.interp.sp),
            Expr::GlobalSet(e) => {
                let val = self.eval(e.value).expect("must eval to i32");
                self.interp.sp = val;
                None
            }

            // Support simple arithmetic, mainly for the stack pointer
            // manipulation
            Expr::Binop(e) => {
                let lhs = self.eval(e.lhs).expect("must eval to i32");
                let rhs = self.eval(e.rhs).expect("must eval to i32");
                match e.op {
                    BinaryOp::I32Sub => Some(lhs - rhs),
                    BinaryOp::I32Add => Some(lhs + rhs),
                    op => panic!("invalid binary op {:?}", op),
                }
            }

            // Support small loads/stores to the stack. These show up in debug
            // mode where there's some traffic on the linear stack even when in
            // theory there doesn't need to be.
            Expr::Load(e) => {
                let address = self.eval(e.address).expect("must eval to i32");
                let address = address as u32 + e.arg.offset;
                assert!(address % 4 == 0);
                Some(self.interp.mem[address as usize / 4])
            }
            Expr::Store(e) => {
                let address = self.eval(e.address).expect("must eval to i32");
                let value = self.eval(e.value).expect("must eval to i32");
                let address = address as u32 + e.arg.offset;
                assert!(address % 4 == 0);
                self.interp.mem[address as usize / 4] = value;
                None
            }

            Expr::Return(e) => {
                log::debug!("return");
                self.done = true;
                assert!(e.values.len() <= 1);
                e.values.get(0).and_then(|id| self.eval(*id))
            }

            Expr::Drop(e) => {
                log::debug!("drop");
                self.eval(e.expr);
                None
            }

            Expr::WithSideEffects(e) => {
                log::debug!("side effects");
                for x in e.before.iter() {
                    self.eval(*x);
                }
                let ret = self.eval(e.value);
                for x in e.after.iter() {
                    self.eval(*x);
                }
                return ret;
            }

            Expr::Call(e) => {
                // If this function is calling the `__wbindgen_describe`
                // function, which we've precomputed the id for, then
                // it's telling us about the next `u32` element in the
                // descriptor to return. We "call" the imported function
                // here by directly inlining it.
                if Some(e.func) == self.interp.describe_id {
                    assert_eq!(e.args.len(), 1);
                    let val = self.eval(e.args[0]).expect("must eval to i32");
                    log::debug!("__wbindgen_describe({})", val);
                    self.interp.descriptor.push(val as u32);
                    None

                // If this function is calling the `__wbindgen_describe_closure`
                // function then it's similar to the above, except there's a
                // slightly different signature. Note that we don't eval the
                // previous arguments because they shouldn't have any side
                // effects we're interested in.
                } else if Some(e.func) == self.interp.describe_closure_id {
                    assert_eq!(e.args.len(), 3);
                    let val = self.eval(e.args[2]).expect("must eval to i32");
                    log::debug!("__wbindgen_describe_closure({})", val);
                    self.interp.descriptor_table_idx = Some(val as u32);
                    Some(0)

                // ... otherwise this is a normal call so we recurse.
                } else {
                    let args = e
                        .args
                        .iter()
                        .map(|e| self.eval(*e).expect("must eval to i32"))
                        .collect::<Vec<_>>();
                    self.interp.call(e.func, self.module, &args);
                    None
                }
            }

            // All other instructions shouldn't be used by our various
            // descriptor functions. LLVM optimizations may mean that some
            // of the above instructions aren't actually needed either, but
            // the above instructions have empirically been required when
            // executing our own test suite in wasm-bindgen.
            //
            // Note that LLVM may change over time to generate new
            // instructions in debug mode, and we'll have to react to those
            // sorts of changes as they arise.
            s => panic!("unknown instruction {:?}", s),
        }
    }
}
