use crate::descriptor::Descriptor;
use crate::wit::{AdapterType, Instruction, InstructionBuilder};
use crate::wit::{InstructionData, StackChange};
use anyhow::{bail, format_err, Error};
use walrus::ValType;

impl InstructionBuilder<'_, '_> {
    /// Processes one more `Descriptor` as an argument to a JS function that
    /// wasm is calling.
    ///
    /// This will internally skip `Unit` and otherwise build up the `bindings`
    /// map and ensure that it's correctly mapped from wasm to JS.
    pub fn outgoing(&mut self, arg: &Descriptor) -> Result<(), Error> {
        if let Descriptor::Unit = arg {
            return Ok(());
        }
        // Similar rationale to `incoming.rs` around these sanity checks.
        let input_before = self.input.len();
        let output_before = self.output.len();
        self._outgoing(arg)?;
        assert_eq!(output_before + 1, self.output.len());
        assert!(input_before < self.input.len());
        Ok(())
    }

    fn _outgoing(&mut self, arg: &Descriptor) -> Result<(), Error> {
        match arg {
            Descriptor::Boolean => {
                self.instruction(
                    &[AdapterType::I32],
                    Instruction::BoolFromI32,
                    &[AdapterType::Bool],
                );
            }
            Descriptor::Externref => {
                self.instruction(
                    &[AdapterType::I32],
                    Instruction::ExternrefLoadOwned,
                    &[AdapterType::Externref],
                );
            }
            Descriptor::NamedExternref(name) => {
                self.instruction(
                    &[AdapterType::I32],
                    Instruction::ExternrefLoadOwned,
                    &[AdapterType::NamedExternref(name.clone())],
                );
            }
            Descriptor::I8 => self.outgoing_i32(AdapterType::S8),
            Descriptor::U8 => self.outgoing_i32(AdapterType::U8),
            Descriptor::I16 => self.outgoing_i32(AdapterType::S16),
            Descriptor::U16 => self.outgoing_i32(AdapterType::U16),
            Descriptor::I32 => self.outgoing_i32(AdapterType::S32),
            Descriptor::U32 => self.outgoing_i32(AdapterType::U32),
            Descriptor::F32 => {
                self.get(AdapterType::F32);
                self.output.push(AdapterType::F32);
            }
            Descriptor::F64 => {
                self.get(AdapterType::F64);
                self.output.push(AdapterType::F64);
            }
            Descriptor::Enum { .. } => self.outgoing_i32(AdapterType::U32),

            Descriptor::Char => {
                self.instruction(
                    &[AdapterType::I32],
                    Instruction::StringFromChar,
                    &[AdapterType::String],
                );
            }

            Descriptor::I64 | Descriptor::U64 => {
                let signed = match arg {
                    Descriptor::I64 => true,
                    _ => false,
                };
                self.instruction(
                    &[AdapterType::I32, AdapterType::I32],
                    Instruction::I64FromLoHi { signed },
                    &[if signed {
                        AdapterType::S64
                    } else {
                        AdapterType::U64
                    }],
                );
            }

            Descriptor::RustStruct(class) => {
                self.instruction(
                    &[AdapterType::I32],
                    Instruction::RustFromI32 {
                        class: class.to_string(),
                    },
                    &[AdapterType::Struct(class.clone())],
                );
            }
            Descriptor::Ref(d) => self.outgoing_ref(false, d)?,
            Descriptor::RefMut(d) => self.outgoing_ref(true, d)?,

            Descriptor::CachedString => self.cached_string(false, true)?,

            Descriptor::String => {
                // fetch the ptr/length ...
                self.get(AdapterType::I32);
                self.get(AdapterType::I32);

                // ... then defer a call to `free` to happen later
                let free = self.cx.free()?;
                let std = wit_walrus::Instruction::DeferCallCore(free);
                self.instructions.push(InstructionData {
                    instr: Instruction::Standard(std),
                    stack_change: StackChange::Modified {
                        popped: 2,
                        pushed: 2,
                    },
                });

                // ... and then convert it to a string type
                let std = wit_walrus::Instruction::MemoryToString(self.cx.memory()?);
                self.instructions.push(InstructionData {
                    instr: Instruction::Standard(std),
                    stack_change: StackChange::Modified {
                        popped: 2,
                        pushed: 1,
                    },
                });
                self.output.push(AdapterType::String);
            }

            Descriptor::Vector(_) => {
                let kind = arg.vector_kind().ok_or_else(|| {
                    format_err!(
                        "unsupported argument type for calling JS function from Rust {:?}",
                        arg
                    )
                })?;
                let mem = self.cx.memory()?;
                let free = self.cx.free()?;
                self.instruction(
                    &[AdapterType::I32, AdapterType::I32],
                    Instruction::VectorLoad { kind, mem, free },
                    &[AdapterType::Vector(kind)],
                );
            }

            Descriptor::Option(d) => self.outgoing_option(d)?,

            Descriptor::Function(_) | Descriptor::Closure(_) | Descriptor::Slice(_) => bail!(
                "unsupported argument type for calling JS function from Rust: {:?}",
                arg
            ),

            // nothing to do
            Descriptor::Unit => {}

            // Largely synthetic and can't show up
            Descriptor::ClampedU8 => unreachable!(),
        }
        Ok(())
    }

    fn outgoing_ref(&mut self, mutable: bool, arg: &Descriptor) -> Result<(), Error> {
        match arg {
            Descriptor::Externref => {
                self.instruction(
                    &[AdapterType::I32],
                    Instruction::TableGet,
                    &[AdapterType::Externref],
                );
            }
            Descriptor::NamedExternref(name) => {
                self.instruction(
                    &[AdapterType::I32],
                    Instruction::TableGet,
                    &[AdapterType::NamedExternref(name.clone())],
                );
            }
            Descriptor::CachedString => self.cached_string(false, false)?,

            Descriptor::String => {
                let std = wit_walrus::Instruction::MemoryToString(self.cx.memory()?);
                self.instruction(
                    &[AdapterType::I32, AdapterType::I32],
                    Instruction::Standard(std),
                    &[AdapterType::String],
                );
            }
            Descriptor::Slice(_) => {
                let kind = arg.vector_kind().ok_or_else(|| {
                    format_err!(
                        "unsupported argument type for calling JS function from Rust {:?}",
                        arg
                    )
                })?;
                let mem = self.cx.memory()?;
                self.instruction(
                    &[AdapterType::I32, AdapterType::I32],
                    Instruction::View { kind, mem },
                    &[AdapterType::Vector(kind)],
                );
            }

            Descriptor::Function(descriptor) => {
                // synthesize the a/b arguments that aren't present in the
                // signature from wasm-bindgen but are present in the wasm file.
                let mut descriptor = (**descriptor).clone();
                let nargs = descriptor.arguments.len();
                descriptor.arguments.insert(0, Descriptor::I32);
                descriptor.arguments.insert(0, Descriptor::I32);
                let adapter = self
                    .cx
                    .table_element_adapter(descriptor.shim_idx, descriptor)?;
                self.instruction(
                    &[AdapterType::I32, AdapterType::I32],
                    Instruction::StackClosure {
                        adapter,
                        nargs,
                        mutable,
                    },
                    &[AdapterType::Function],
                );
            }

            _ => bail!(
                "unsupported reference argument type for calling JS function from Rust: {:?}",
                arg
            ),
        }
        Ok(())
    }

    fn outgoing_option(&mut self, arg: &Descriptor) -> Result<(), Error> {
        match arg {
            Descriptor::Externref => {
                // This is set to `undefined` in the `None` case and otherwise
                // is the valid owned index.
                self.instruction(
                    &[AdapterType::I32],
                    Instruction::ExternrefLoadOwned,
                    &[AdapterType::Externref.option()],
                );
            }
            Descriptor::NamedExternref(name) => {
                self.instruction(
                    &[AdapterType::I32],
                    Instruction::ExternrefLoadOwned,
                    &[AdapterType::NamedExternref(name.clone()).option()],
                );
            }
            Descriptor::I8 => self.out_option_sentinel(AdapterType::S8),
            Descriptor::U8 => self.out_option_sentinel(AdapterType::U8),
            Descriptor::I16 => self.out_option_sentinel(AdapterType::S16),
            Descriptor::U16 => self.out_option_sentinel(AdapterType::U16),
            Descriptor::I32 => self.option_native(true, ValType::I32),
            Descriptor::U32 => self.option_native(false, ValType::I32),
            Descriptor::F32 => self.option_native(true, ValType::F32),
            Descriptor::F64 => self.option_native(true, ValType::F64),
            Descriptor::I64 | Descriptor::U64 => {
                let (signed, ty) = match arg {
                    Descriptor::I64 => (true, AdapterType::S64.option()),
                    _ => (false, AdapterType::U64.option()),
                };
                self.instruction(
                    &[AdapterType::I32, AdapterType::I32, AdapterType::I32],
                    Instruction::Option64FromI32 { signed },
                    &[ty],
                );
            }
            Descriptor::Boolean => {
                self.instruction(
                    &[AdapterType::I32],
                    Instruction::OptionBoolFromI32,
                    &[AdapterType::Bool.option()],
                );
            }
            Descriptor::Char => {
                self.instruction(
                    &[AdapterType::I32],
                    Instruction::OptionCharFromI32,
                    &[AdapterType::String.option()],
                );
            }
            Descriptor::Enum { hole } => {
                self.instruction(
                    &[AdapterType::I32],
                    Instruction::OptionEnumFromI32 { hole: *hole },
                    &[AdapterType::U32.option()],
                );
            }
            Descriptor::RustStruct(name) => {
                self.instruction(
                    &[AdapterType::I32],
                    Instruction::OptionRustFromI32 {
                        class: name.to_string(),
                    },
                    &[AdapterType::Struct(name.clone()).option()],
                );
            }
            Descriptor::Ref(d) => self.outgoing_option_ref(false, d)?,
            Descriptor::RefMut(d) => self.outgoing_option_ref(true, d)?,

            Descriptor::CachedString => self.cached_string(true, true)?,

            Descriptor::String | Descriptor::Vector(_) => {
                let kind = arg.vector_kind().ok_or_else(|| {
                    format_err!(
                        "unsupported optional slice type for calling JS function from Rust {:?}",
                        arg
                    )
                })?;
                let mem = self.cx.memory()?;
                let free = self.cx.free()?;
                self.instruction(
                    &[AdapterType::I32, AdapterType::I32],
                    Instruction::OptionVectorLoad { kind, mem, free },
                    &[AdapterType::Vector(kind).option()],
                );
            }

            _ => bail!(
                "unsupported optional argument type for calling JS function from Rust: {:?}",
                arg
            ),
        }
        Ok(())
    }

    fn outgoing_option_ref(&mut self, _mutable: bool, arg: &Descriptor) -> Result<(), Error> {
        match arg {
            Descriptor::Externref => {
                // If this is `Some` then it's the index, otherwise if it's
                // `None` then it's the index pointing to undefined.
                self.instruction(
                    &[AdapterType::I32],
                    Instruction::TableGet,
                    &[AdapterType::Externref.option()],
                );
            }
            Descriptor::NamedExternref(name) => {
                self.instruction(
                    &[AdapterType::I32],
                    Instruction::TableGet,
                    &[AdapterType::NamedExternref(name.clone()).option()],
                );
            }
            Descriptor::CachedString => self.cached_string(true, false)?,
            Descriptor::String | Descriptor::Slice(_) => {
                let kind = arg.vector_kind().ok_or_else(|| {
                    format_err!(
                        "unsupported optional slice type for calling JS function from Rust {:?}",
                        arg
                    )
                })?;
                let mem = self.cx.memory()?;
                self.instruction(
                    &[AdapterType::I32, AdapterType::I32],
                    Instruction::OptionView { kind, mem },
                    &[AdapterType::Vector(kind).option()],
                );
            }
            _ => bail!(
                "unsupported optional ref argument type for calling JS function from Rust: {:?}",
                arg
            ),
        }
        Ok(())
    }

    fn outgoing_i32(&mut self, output: AdapterType) {
        let std = wit_walrus::Instruction::WasmToInt {
            input: walrus::ValType::I32,
            output: output.to_wit().unwrap(),
            trap: false,
        };
        self.instruction(&[AdapterType::I32], Instruction::Standard(std), &[output]);
    }

    fn cached_string(&mut self, optional: bool, owned: bool) -> Result<(), Error> {
        let mem = self.cx.memory()?;
        let free = self.cx.free()?;
        self.instruction(
            &[AdapterType::I32, AdapterType::I32],
            Instruction::CachedStringLoad {
                owned,
                optional,
                mem,
                free,
            },
            &[AdapterType::String],
        );
        Ok(())
    }

    fn option_native(&mut self, signed: bool, ty: ValType) {
        let adapter_ty = AdapterType::from_wasm(ty).unwrap();
        self.instruction(
            &[AdapterType::I32, adapter_ty.clone()],
            Instruction::ToOptionNative { signed, ty },
            &[adapter_ty.option()],
        );
    }

    fn out_option_sentinel(&mut self, ty: AdapterType) {
        self.instruction(
            &[AdapterType::I32],
            Instruction::OptionU32Sentinel,
            &[ty.option()],
        );
    }
}
