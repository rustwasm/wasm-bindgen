//! A tiny and incomplete wasm interpreter
//!
//! This module contains a tiny and incomplete wasm interpreter built on top of
//! `parity-wasm`'s module structure. Each `Interpreter` contains some state
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

extern crate parity_wasm;

use std::collections::HashMap;

use parity_wasm::elements::*;

/// A ready-to-go interpreter of a wasm module.
///
/// An interpreter currently represents effectively cached state. It is reused
/// between calls to `interpret` and is precomputed from a `Module`. It houses
/// state like the wasm stack, wasm memory, etc.
#[derive(Default)]
pub struct Interpreter {
    // Number of imported functions in the wasm module (used in index
    // calculations)
    imports: usize,

    // Function index of the `__wbindgen_describe` imported function. We special
    // case this to know when the environment's imported function is called.
    describe_idx: Option<u32>,

    // A mapping of string names to the function index, filled with all exported
    // functions.
    name_map: HashMap<String, u32>,

    // The numerical index of the code section in the wasm module, indexed into
    // the module's list of sections.
    code_idx: Option<usize>,

    // The current stack pointer (global 0) and wasm memory (the stack). Only
    // used in a limited capacity.
    sp: i32,
    mem: Vec<i32>,

    // The wasm stack. Note how it's just `i32` which is intentional, we don't
    // support other types.
    stack: Vec<i32>,

    // The descriptor which we're assembling, a list of `u32` entries. This is
    // very specific to wasm-bindgen and is the purpose for the existence of
    // this module.
    descriptor: Vec<u32>,
}

impl Interpreter {
    /// Creates a new interpreter from a provided `Module`, precomputing all
    /// information necessary to interpret further.
    ///
    /// Note that the `module` passed in to this function must be the same as
    /// the `module` passed to `interpret` below.
    pub fn new(module: &Module) -> Interpreter {
        let mut ret = Interpreter::default();

        // The descriptor functions shouldn't really use all that much memory
        // (the LLVM call stack, now the wasm stack). To handle that let's give
        // our selves a little bit of memory and set the stack pointer (global
        // 0) to the top.
        ret.mem = vec![0; 0x100];
        ret.sp = ret.mem.len() as i32;

        // Figure out where our code section, if any, is.
        for (i, s) in module.sections().iter().enumerate() {
            match s {
                Section::Code(_) => ret.code_idx = Some(i),
                _ => {}
            }
        }

        // Figure out where the `__wbindgen_describe` imported function is, if
        // it exists. We'll special case calls to this function as our
        // interpretation should only invoke this function as an imported
        // function.
        if let Some(i) = module.import_section() {
            ret.imports = i.functions();
            for (i, entry) in i.entries().iter().enumerate() {
                if entry.module() != "__wbindgen_placeholder__" {
                    continue
                }
                if entry.field() != "__wbindgen_describe" {
                    continue
                }
                ret.describe_idx = Some(i as u32);
            }
        }

        // Build up the mapping of exported functions to function indices.
        if let Some(e) = module.export_section() {
            for e in e.entries() {
                let i = match e.internal() {
                    Internal::Function(i) => i,
                    _ => continue,
                };
                ret.name_map.insert(e.field().to_string(), *i);
            }
        }

        return ret
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
    pub fn interpret(&mut self, func: &str, module: &Module) -> Option<&[u32]> {
        self.descriptor.truncate(0);
        let idx = *self.name_map.get(func)?;
        let code = match &module.sections()[self.code_idx.unwrap()] {
            Section::Code(s) => s,
            _ => panic!(),
        };

        // We should have a blank wasm and LLVM stack at both the start and end
        // of the call.
        assert_eq!(self.sp, self.mem.len() as i32);
        assert_eq!(self.stack.len(), 0);
        self.call(idx, code);
        assert_eq!(self.stack.len(), 0);
        assert_eq!(self.sp, self.mem.len() as i32);
        Some(&self.descriptor)
    }

    fn call(&mut self, idx: u32, code: &CodeSection) {
        use parity_wasm::elements::Instruction::*;

        let idx = idx as usize;
        assert!(idx >= self.imports); // can't call imported functions
        let body = &code.bodies()[idx - self.imports];

        // Allocate space for our call frame's local variables. All local
        // variables should be of the `i32` type.
        assert!(body.locals().len() <= 1, "too many local types");
        let locals = body.locals()
            .get(0)
            .map(|i| {
                assert_eq!(i.value_type(), ValueType::I32);
                i.count()
            })
            .unwrap_or(0);
        let mut locals = vec![0; locals as usize];

        // Actual interpretation loop! We keep track of our stack's length to
        // recover it as part of the `Return` instruction, and otherwise this is
        // a pretty straightforward interpretation loop.
        let before = self.stack.len();
        for instr in body.code().elements() {
            match instr {
                I32Const(x) => self.stack.push(*x),
                SetLocal(i) => locals[*i as usize] = self.stack.pop().unwrap(),
                GetLocal(i) => self.stack.push(locals[*i as usize]),
                Call(idx) => {
                    if Some(*idx) == self.describe_idx {
                        self.descriptor.push(self.stack.pop().unwrap() as u32);
                    } else {
                        self.call(*idx, code);
                    }
                }
                GetGlobal(0) => self.stack.push(self.sp),
                SetGlobal(0) => self.sp = self.stack.pop().unwrap(),
                I32Sub => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    self.stack.push(a - b);
                }
                I32Add => {
                    let a = self.stack.pop().unwrap();
                    let b = self.stack.pop().unwrap();
                    self.stack.push(a + b);
                }
                I32Store(/* align = */ 2, offset) => {
                    let val = self.stack.pop().unwrap();
                    let addr = self.stack.pop().unwrap() as u32;
                    self.mem[((addr + *offset) as usize) / 4] = val;
                }
                I32Load(/* align = */ 2, offset) => {
                    let addr = self.stack.pop().unwrap() as u32;
                    self.stack.push(self.mem[((addr + *offset) as usize) / 4]);
                }
                Return => self.stack.truncate(before),
                End => break,

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
        assert_eq!(self.stack.len(), before);
    }
}
