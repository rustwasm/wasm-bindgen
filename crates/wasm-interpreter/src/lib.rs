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

    // Function index of the `__wbindgen_describe` and
    // `__wbindgen_describe_closure` imported functions. We special case this
    // to know when the environment's imported function is called.
    describe_idx: Option<u32>,
    describe_closure_idx: Option<u32>,

    // A mapping of string names to the function index, filled with all exported
    // functions.
    name_map: HashMap<String, u32>,

    // The numerical index of the sections in the wasm module, indexed into
    // the module's list of sections.
    code_idx: Option<usize>,
    types_idx: Option<usize>,
    functions_idx: Option<usize>,
    elements_idx: Option<usize>,

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

    // When invoking the `__wbindgen_describe_closure` imported function, this
    // stores the last table index argument, used for finding a different
    // descriptor.
    descriptor_table_idx: Option<u32>,
}

struct Sections<'a> {
    code: &'a CodeSection,
    types: &'a TypeSection,
    functions: &'a FunctionSection,
    elements: &'a ElementSection,
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
                Section::Element(_) => ret.elements_idx = Some(i),
                Section::Type(_) => ret.types_idx = Some(i),
                Section::Function(_) => ret.functions_idx = Some(i),
                _ => {}
            }
        }

        // Figure out where the `__wbindgen_describe` imported function is, if
        // it exists. We'll special case calls to this function as our
        // interpretation should only invoke this function as an imported
        // function.
        if let Some(i) = module.import_section() {
            ret.imports = i.functions();
            let mut idx = 0;
            for entry in i.entries() {
                match entry.external() {
                    External::Function(_) => idx += 1,
                    _ => continue,
                }
                if entry.module() != "__wbindgen_placeholder__" {
                    continue
                }
                if entry.field() == "__wbindgen_describe" {
                    ret.describe_idx = Some(idx - 1 as u32);
                } else if entry.field() == "__wbindgen_describe_closure" {
                    ret.describe_closure_idx = Some(idx - 1 as u32);
                }
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
    pub fn interpret_descriptor(
        &mut self,
        func: &str,
        module: &Module,
    ) -> Option<&[u32]> {
        let idx = *self.name_map.get(func)?;
        self.with_sections(module, |me, sections| {
            me.interpret_descriptor_idx(idx, sections)
        })
    }

    fn interpret_descriptor_idx(
        &mut self,
        idx: u32,
        sections: &Sections,
    ) -> Option<&[u32]> {
        self.descriptor.truncate(0);

        // We should have a blank wasm and LLVM stack at both the start and end
        // of the call.
        assert_eq!(self.sp, self.mem.len() as i32);
        assert_eq!(self.stack.len(), 0);
        self.call(idx, sections);
        assert_eq!(self.stack.len(), 0);
        assert_eq!(self.sp, self.mem.len() as i32);
        Some(&self.descriptor)
    }

    /// Interprets a "closure descriptor", figuring out the signature of the
    /// closure that was intended.
    ///
    /// This function will take a `code_idx` which is known to internally
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
        code_idx: usize,
        module: &Module,
        entry_removal_list: &mut Vec<(usize, usize)>,
    ) -> Option<&[u32]> {
        self.with_sections(module, |me, sections| {
            me._interpret_closure_descriptor(code_idx, sections, entry_removal_list)
        })
    }

    fn _interpret_closure_descriptor(
        &mut self,
        code_idx: usize,
        sections: &Sections,
        entry_removal_list: &mut Vec<(usize, usize)>,
    ) -> Option<&[u32]> {
        // Call the `code_idx` function. This is an internal `#[inline(never)]`
        // whose code is completely controlled by the `wasm-bindgen` crate, so
        // it should take some arguments (the number of arguments depends on the
        // optimization level) and return one (all of which we don't care about
        // here). What we're interested in is that while executing this function
        // it'll call `__wbindgen_describe_closure` with an argument that we
        // look for.
        assert!(self.descriptor_table_idx.is_none());
        let closure_descriptor_idx = (code_idx + self.imports) as u32;

        let code_sig = sections.functions.entries()[code_idx].type_ref();
        let function_ty = match &sections.types.types()[code_sig as usize] {
            Type::Function(t) => t,
        };
        for _ in 0..function_ty.params().len() {
            self.stack.push(0);
        }

        self.call(closure_descriptor_idx, sections);
        assert_eq!(self.stack.len(), 1);
        self.stack.pop();
        let descriptor_table_idx = self.descriptor_table_idx.take().unwrap();

        // After we've got the table index of the descriptor function we're
        // interested go take a look in the function table to find what the
        // actual index of the function is.
        let (entry_idx, offset, entry) = sections.elements.entries()
            .iter()
            .enumerate()
            .filter_map(|(i, entry)| {
                let code = entry.offset().code();
                if code.len() != 2 {
                    return None
                }
                if code[1] != Instruction::End {
                    return None
                }
                match code[0] {
                    Instruction::I32Const(x) => Some((i, x as u32, entry)),
                    _ => None,
                }
            })
            .find(|(_i, offset, entry)| {
                *offset <= descriptor_table_idx &&
                    descriptor_table_idx < (*offset + entry.members().len() as u32)
            })
            .expect("failed to find index in table elements");
        let idx = (descriptor_table_idx - offset) as usize;
        let descriptor_idx = entry.members()[idx];

        // This is used later to actually remove the entry from the table, but
        // we don't do the removal just yet
        entry_removal_list.push((entry_idx, idx));

        // And now execute the descriptor!
        self.interpret_descriptor_idx(descriptor_idx, sections)
    }

    /// Returns the function space index of the `__wbindgen_describe_closure`
    /// imported function.
    pub fn describe_closure_idx(&self) -> Option<u32> {
        self.describe_closure_idx
    }

    fn call(&mut self, idx: u32, sections: &Sections) {
        use parity_wasm::elements::Instruction::*;

        let idx = idx as usize;
        assert!(idx >= self.imports); // can't call imported functions
        let code_idx = idx - self.imports;
        let body = &sections.code.bodies()[code_idx];

        // Allocate space for our call frame's local variables. All local
        // variables should be of the `i32` type.
        assert!(body.locals().len() <= 1, "too many local types");
        let nlocals = body.locals()
            .get(0)
            .map(|i| {
                assert_eq!(i.value_type(), ValueType::I32);
                i.count()
            })
            .unwrap_or(0);

        let code_sig = sections.functions.entries()[code_idx].type_ref();
        let function_ty = match &sections.types.types()[code_sig as usize] {
            Type::Function(t) => t,
        };
        let mut locals = Vec::with_capacity(function_ty.params().len() + nlocals as usize);
        // Any function parameters we have get popped off the stack and put into
        // the first few locals ...
        for param in function_ty.params() {
            assert_eq!(*param, ValueType::I32);
            locals.push(self.stack.pop().unwrap());
        }
        // ... and the remaining locals all start as zero ...
        for _ in 0..nlocals {
            locals.push(0);
        }
        // ... and we expect one stack slot at the end if there's a returned
        // value
        let before = self.stack.len();
        let stack_after = match function_ty.return_type() {
            Some(t) => {
                assert_eq!(t, ValueType::I32);
                before + 1
            }
            None => before,
        };

        // Actual interpretation loop! We keep track of our stack's length to
        // recover it as part of the `Return` instruction, and otherwise this is
        // a pretty straightforward interpretation loop.
        for instr in body.code().elements() {
            match instr {
                I32Const(x) => self.stack.push(*x),
                SetLocal(i) => locals[*i as usize] = self.stack.pop().unwrap(),
                GetLocal(i) => self.stack.push(locals[*i as usize]),
                Call(idx) => {
                    // If this function is calling the `__wbindgen_describe`
                    // function, which we've precomputed the index for, then
                    // it's telling us about the next `u32` element in the
                    // descriptor to return. We "call" the imported function
                    // here by directly inlining it.
                    //
                    // Otherwise this is a normal call so we recurse.
                    if Some(*idx) == self.describe_idx {
                        self.descriptor.push(self.stack.pop().unwrap() as u32);
                    } else if Some(*idx) == self.describe_closure_idx {
                        self.descriptor_table_idx =
                            Some(self.stack.pop().unwrap() as u32);
                        self.stack.pop();
                        self.stack.pop();
                        self.stack.push(0);
                    } else {
                        self.call(*idx, sections);
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
                Return => self.stack.truncate(stack_after),
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
        assert_eq!(self.stack.len(), stack_after);
    }

    fn with_sections<'a, T>(
        &'a mut self,
        module: &Module,
        f: impl FnOnce(&'a mut Self, &Sections) -> T,
    ) -> T {
        macro_rules! access_with_defaults {
            ($(
                    let $var: ident = module.sections[self.$field:ident]
                    ($name:ident);
            )*) => {$(
                let default = Default::default();
                let $var = match self.$field {
                    Some(i) => {
                        match &module.sections()[i] {
                            Section::$name(s) => s,
                            _ => panic!(),
                        }
                    }
                    None => &default,
                };
            )*}
        }
        access_with_defaults! {
            let code = module.sections[self.code_idx] (Code);
            let types = module.sections[self.types_idx] (Type);
            let functions = module.sections[self.functions_idx] (Function);
            let elements = module.sections[self.elements_idx] (Element);
        }
        f(self, &Sections { code, types, functions, elements })
    }
}
