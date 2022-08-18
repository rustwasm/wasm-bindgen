//! Definition of how to convert Rust types (`Description`) into wasm types
//! through adapter functions.
//!
//! Note that many Rust types use "nonstandard" instructions which only work in
//! the JS output, not for the "pure wasm interface types" output.
//!
//! Note that the mirror operation, going from WebAssembly to JS, is found in
//! the `outgoing.rs` module.

use crate::descriptor::Descriptor;
use crate::wit::InstructionData;
use crate::wit::{AdapterType, Instruction, InstructionBuilder, StackChange};
use anyhow::{bail, format_err, Error};
use walrus::ValType;

impl InstructionBuilder<'_, '_> {
    /// Process a `Descriptor` as if it's being passed from JS to Rust. This
    /// will skip `Unit` and otherwise internally add instructions necessary to
    /// convert the foreign type into the Rust bits.
    pub fn incoming(&mut self, arg: &Descriptor) -> Result<(), Error> {
        if let Descriptor::Unit = arg {
            return Ok(());
        }
        // This is a wrapper around `_incoming` to have a number of sanity checks
        // that we don't forget things. We should always produce at least one
        // wasm arge and exactly one webidl arg. Additionally the number of
        // bindings should always match the number of webidl types for now.
        let input_before = self.input.len();
        let output_before = self.output.len();
        self._incoming(arg)?;
        assert_eq!(
            input_before + 1,
            self.input.len(),
            "didn't push an input {:?}",
            arg
        );
        assert!(
            output_before < self.output.len(),
            "didn't push more outputs {:?}",
            arg
        );
        Ok(())
    }

    fn _incoming(&mut self, arg: &Descriptor) -> Result<(), Error> {
        use walrus::ValType as WasmVT;
        use wit_walrus::ValType as WitVT;
        match arg {
            Descriptor::Boolean => {
                self.instruction(
                    &[AdapterType::Bool],
                    Instruction::I32FromBool,
                    &[AdapterType::I32],
                );
            }
            Descriptor::Char => {
                self.instruction(
                    &[AdapterType::String],
                    Instruction::I32FromStringFirstChar,
                    &[AdapterType::I32],
                );
            }
            Descriptor::Externref => {
                self.instruction(
                    &[AdapterType::Externref],
                    Instruction::I32FromExternrefOwned,
                    &[AdapterType::I32],
                );
            }
            Descriptor::NamedExternref(name) => {
                self.instruction(
                    &[AdapterType::NamedExternref(name.clone())],
                    Instruction::I32FromExternrefOwned,
                    &[AdapterType::I32]
                )
            }
            Descriptor::RustStruct(class) => {
                self.instruction(
                    &[AdapterType::Struct(class.clone())],
                    Instruction::I32FromExternrefRustOwned {
                        class: class.clone(),
                    },
                    &[AdapterType::I32],
                );
            }
            Descriptor::I8 => self.number(WitVT::S8, WasmVT::I32),
            Descriptor::U8 => self.number(WitVT::U8, WasmVT::I32),
            Descriptor::I16 => self.number(WitVT::S16, WasmVT::I32),
            Descriptor::U16 => self.number(WitVT::U16, WasmVT::I32),
            Descriptor::I32 => self.number(WitVT::S32, WasmVT::I32),
            Descriptor::U32 => self.number(WitVT::U32, WasmVT::I32),
            Descriptor::I64 => self.number(WitVT::S64, WasmVT::I64),
            Descriptor::U64 => self.number(WitVT::U64, WasmVT::I64),
            Descriptor::F32 => {
                self.get(AdapterType::F32);
                self.output.push(AdapterType::F32);
            }
            Descriptor::F64 => {
                self.get(AdapterType::F64);
                self.output.push(AdapterType::F64);
            }
            Descriptor::Enum { .. } => self.number(WitVT::U32, WasmVT::I32),
            Descriptor::Ref(d) => self.incoming_ref(false, d)?,
            Descriptor::RefMut(d) => self.incoming_ref(true, d)?,
            Descriptor::Option(d) => self.incoming_option(d)?,

            Descriptor::String | Descriptor::CachedString => {
                self.instruction(
                    &[AdapterType::String],
                    Instruction::StringToMemory {
                        malloc: self.cx.malloc()?,
                        realloc: self.cx.realloc(),
                        mem: self.cx.memory()?,
                    },
                    &[AdapterType::I32, AdapterType::I32],
                );
            }

            Descriptor::Vector(_) => {
                let kind = arg.vector_kind().ok_or_else(|| {
                    format_err!("unsupported argument type for calling Rust function from JS {:?}", arg)
                })?;
                self.instruction(
                    &[AdapterType::Vector(kind.clone())],
                    Instruction::VectorToMemory {
                        kind,
                        malloc: self.cx.malloc()?,
                        mem: self.cx.memory()?,
                    },
                    &[AdapterType::I32, AdapterType::I32],
                );
            }

            // Can't be passed from JS to Rust yet
            Descriptor::Function(_) |
            Descriptor::Closure(_) |

            Descriptor::Result(_) |
            // Always behind a `Ref`
            Descriptor::Slice(_) => bail!(
                "unsupported argument type for calling Rust function from JS: {:?}",
                arg
            ),

            // nothing to do
            Descriptor::Unit => {}

            // Largely synthetic and can't show up
            Descriptor::ClampedU8 => unreachable!(),
        }
        Ok(())
    }

    fn incoming_ref(&mut self, mutable: bool, arg: &Descriptor) -> Result<(), Error> {
        match arg {
            Descriptor::RustStruct(class) => {
                self.instruction(
                    &[AdapterType::Struct(class.clone())],
                    Instruction::I32FromExternrefRustBorrow {
                        class: class.clone(),
                    },
                    &[AdapterType::I32],
                );
            }
            Descriptor::Externref => {
                self.instruction(
                    &[AdapterType::Externref],
                    Instruction::I32FromExternrefBorrow,
                    &[AdapterType::I32],
                );
            }
            Descriptor::NamedExternref(name) => {
                self.instruction(
                    &[AdapterType::NamedExternref(name.clone())],
                    Instruction::I32FromExternrefBorrow,
                    &[AdapterType::I32],
                );
            }
            Descriptor::String | Descriptor::CachedString => {
                // This allocation is cleaned up once it's received in Rust.
                self.instruction(
                    &[AdapterType::String],
                    Instruction::StringToMemory {
                        malloc: self.cx.malloc()?,
                        realloc: self.cx.realloc(),
                        mem: self.cx.memory()?,
                    },
                    &[AdapterType::I32, AdapterType::I32],
                );
            }
            Descriptor::Slice(_) => {
                // like strings, this allocation is cleaned up after being
                // received in Rust.
                let kind = arg.vector_kind().ok_or_else(|| {
                    format_err!(
                        "unsupported argument type for calling Rust function from JS {:?}",
                        arg
                    )
                })?;
                if mutable {
                    self.instruction(
                        &[AdapterType::Vector(kind.clone())],
                        Instruction::MutableSliceToMemory {
                            kind,
                            malloc: self.cx.malloc()?,
                            mem: self.cx.memory()?,
                            free: self.cx.free()?,
                        },
                        &[AdapterType::I32, AdapterType::I32],
                    );
                } else {
                    self.instruction(
                        &[AdapterType::Vector(kind.clone())],
                        Instruction::VectorToMemory {
                            kind,
                            malloc: self.cx.malloc()?,
                            mem: self.cx.memory()?,
                        },
                        &[AdapterType::I32, AdapterType::I32],
                    );
                }
            }
            _ => bail!(
                "unsupported reference argument type for calling Rust function from JS: {:?}",
                arg
            ),
        }
        Ok(())
    }

    fn incoming_option(&mut self, arg: &Descriptor) -> Result<(), Error> {
        match arg {
            Descriptor::Externref => {
                self.instruction(
                    &[AdapterType::Externref.option()],
                    Instruction::I32FromOptionExternref {
                        table_and_alloc: None,
                    },
                    &[AdapterType::I32],
                );
            }
            Descriptor::NamedExternref(name) => {
                self.instruction(
                    &[AdapterType::NamedExternref(name.clone()).option()],
                    Instruction::I32FromOptionExternref {
                        table_and_alloc: None,
                    },
                    &[AdapterType::I32],
                );
            }
            Descriptor::I8 => self.in_option_sentinel(AdapterType::S8),
            Descriptor::U8 => self.in_option_sentinel(AdapterType::U8),
            Descriptor::I16 => self.in_option_sentinel(AdapterType::S16),
            Descriptor::U16 => self.in_option_sentinel(AdapterType::U16),
            Descriptor::I32 => self.in_option_native(ValType::I32),
            Descriptor::U32 => self.in_option_native(ValType::I32),
            Descriptor::F32 => self.in_option_native(ValType::F32),
            Descriptor::F64 => self.in_option_native(ValType::F64),
            Descriptor::I64 | Descriptor::U64 => self.in_option_native(ValType::I64),
            Descriptor::Boolean => {
                self.instruction(
                    &[AdapterType::Bool.option()],
                    Instruction::I32FromOptionBool,
                    &[AdapterType::I32],
                );
            }
            Descriptor::Char => {
                self.instruction(
                    &[AdapterType::String.option()],
                    Instruction::I32FromOptionChar,
                    &[AdapterType::I32],
                );
            }
            Descriptor::Enum { hole } => {
                self.instruction(
                    &[AdapterType::U32.option()],
                    Instruction::I32FromOptionEnum { hole: *hole },
                    &[AdapterType::I32],
                );
            }
            Descriptor::RustStruct(name) => {
                self.instruction(
                    &[AdapterType::Struct(name.clone()).option()],
                    Instruction::I32FromOptionRust {
                        class: name.to_string(),
                    },
                    &[AdapterType::I32],
                );
            }

            Descriptor::String | Descriptor::CachedString => {
                let malloc = self.cx.malloc()?;
                let mem = self.cx.memory()?;
                let realloc = self.cx.realloc();
                self.instruction(
                    &[AdapterType::String.option()],
                    Instruction::OptionString {
                        malloc,
                        mem,
                        realloc,
                    },
                    &[AdapterType::I32, AdapterType::I32],
                );
            }

            Descriptor::Vector(_) => {
                let kind = arg.vector_kind().ok_or_else(|| {
                    format_err!(
                        "unsupported optional slice type for calling Rust function from JS {:?}",
                        arg
                    )
                })?;
                let malloc = self.cx.malloc()?;
                let mem = self.cx.memory()?;
                self.instruction(
                    &[AdapterType::Vector(kind.clone()).option()],
                    Instruction::OptionVector { kind, malloc, mem },
                    &[AdapterType::I32, AdapterType::I32],
                );
            }

            _ => bail!(
                "unsupported optional argument type for calling Rust function from JS: {:?}",
                arg
            ),
        }
        Ok(())
    }

    pub fn get(&mut self, ty: AdapterType) {
        self.input.push(ty);

        // If we're generating instructions in the return position then the
        // arguments are already on the stack to consume, otherwise we need to
        // fetch them from the parameters.
        if !self.return_position {
            let idx = self.input.len() as u32 - 1;
            let std = wit_walrus::Instruction::ArgGet(idx);
            self.instructions.push(InstructionData {
                instr: Instruction::Standard(std),
                stack_change: StackChange::Modified {
                    pushed: 1,
                    popped: 0,
                },
            });
        }
    }

    pub fn instruction(
        &mut self,
        inputs: &[AdapterType],
        instr: Instruction,
        outputs: &[AdapterType],
    ) {
        // If we're generating instructions in the return position then the
        // arguments are already on the stack to consume, otherwise we need to
        // fetch them from the parameters.
        if !self.return_position {
            for input in inputs {
                self.get(input.clone());
            }
        } else {
            self.input.extend_from_slice(inputs);
        }

        self.instructions.push(InstructionData {
            instr,
            stack_change: StackChange::Modified {
                popped: inputs.len(),
                pushed: outputs.len(),
            },
        });
        self.output.extend_from_slice(outputs);
    }

    fn number(&mut self, input: wit_walrus::ValType, output: walrus::ValType) {
        let std = wit_walrus::Instruction::IntToWasm {
            input,
            output,
            trap: false,
        };
        self.instruction(
            &[AdapterType::from_wit(input)],
            Instruction::Standard(std),
            &[AdapterType::from_wasm(output).unwrap()],
        );
    }

    fn in_option_native(&mut self, wasm: ValType) {
        let ty = AdapterType::from_wasm(wasm).unwrap();
        self.instruction(
            &[ty.clone().option()],
            Instruction::FromOptionNative { ty: wasm },
            &[AdapterType::I32, ty],
        );
    }

    fn in_option_sentinel(&mut self, ty: AdapterType) {
        self.instruction(
            &[ty.option()],
            Instruction::I32FromOptionU32Sentinel,
            &[AdapterType::I32],
        );
    }
}
