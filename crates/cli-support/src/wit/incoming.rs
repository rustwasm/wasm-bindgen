//! Definition of how to convert Rust types (`Description`) into wasm types
//! through adapter functions.
//!
//! Note that many Rust types use "nonstandard" instructions which only work in
//! the JS output, not for the "pure wasm interface types" output.
//!
//! Note that the mirror operation, going from WebAssembly to JS, is found in
//! the `outgoing.rs` module.

use crate::descriptor::{Descriptor, VectorKind};
use crate::wit::{AdapterType, Instruction};
use anyhow::{bail, format_err, Error};
use walrus::ValType;

#[derive(Default)]
pub struct IncomingBuilder {
    pub input: Vec<AdapterType>,
    pub output: Vec<AdapterType>,
    pub instructions: Vec<Instruction>,
}

impl IncomingBuilder {
    /// Process a `Descriptor` as if it's being passed from JS to Rust. This
    /// will skip `Unit` and otherwise internally add instructions necessary to
    /// convert the foreign type into the Rust bits.
    pub fn process(&mut self, arg: &Descriptor) -> Result<(), Error> {
        if let Descriptor::Unit = arg {
            return Ok(());
        }
        // This is a wrapper around `_process` to have a number of sanity checks
        // that we don't forget things. We should always produce at least one
        // wasm arge and exactly one webidl arg. Additionally the number of
        // bindings should always match the number of webidl types for now.
        let input_before = self.input.len();
        let output_before = self.output.len();
        self._process(arg)?;
        assert_eq!(output_before + 1, self.output.len());
        assert!(input_before < self.input.len());
        Ok(())
    }

    fn _process(&mut self, arg: &Descriptor) -> Result<(), Error> {
        use walrus::ValType as WasmVT;
        use wit_walrus::ValType as WitVT;
        match arg {
            Descriptor::Boolean => {
                self.get(AdapterType::Bool);
                self.instructions.push(Instruction::I32FromBool);
                self.output.push(AdapterType::I32);
            }
            Descriptor::Char => {
                self.get(AdapterType::String);
                self.instructions.push(Instruction::I32FromStringFirstChar);
                self.output.push(AdapterType::I32);
            }
            Descriptor::Anyref => {
                self.get(AdapterType::Anyref);
                self.instructions.push(Instruction::I32FromAnyrefOwned);
                self.output.push(AdapterType::I32);
            }
            Descriptor::RustStruct(class) => {
                self.get(AdapterType::Anyref);
                self.instructions.push(Instruction::I32FromAnyrefRustOwned {
                    class: class.clone(),
                });
                self.output.push(AdapterType::I32);
            }
            Descriptor::I8 => self.number(WitVT::S8, WasmVT::I32),
            Descriptor::U8 => self.number(WitVT::U8, WasmVT::I32),
            Descriptor::I16 => self.number(WitVT::S16, WasmVT::I32),
            Descriptor::U16 => self.number(WitVT::U16, WasmVT::I32),
            Descriptor::I32 => self.number(WitVT::S32, WasmVT::I32),
            Descriptor::U32 => self.number(WitVT::U32, WasmVT::I32),
            Descriptor::I64 => self.number64(true),
            Descriptor::U64 => self.number64(false),
            Descriptor::F32 => {
                self.get(AdapterType::F32);
                self.output.push(AdapterType::F32);
            }
            Descriptor::F64 => {
                self.get(AdapterType::F64);
                self.output.push(AdapterType::F64);
            }
            Descriptor::Enum { .. } => self.number(WitVT::U32, WasmVT::I32),
            Descriptor::Ref(d) => self.process_ref(false, d)?,
            Descriptor::RefMut(d) => self.process_ref(true, d)?,
            Descriptor::Option(d) => self.process_option(d)?,

            Descriptor::String | Descriptor::CachedString | Descriptor::Vector(_) => {
                panic!()
            //     let kind = arg.vector_kind().ok_or_else(|| {
            //         format_err!("unsupported argument type for calling Rust function from JS {:?}", arg)
            //     })? ;
            //     self.wasm.extend(&[ValType::I32; 2]);
            //     self.alloc_copy_kind(kind)
            }

            // Can't be passed from JS to Rust yet
            Descriptor::Function(_) |
            Descriptor::Closure(_) |

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

    fn process_ref(&mut self, mutable: bool, arg: &Descriptor) -> Result<(), Error> {
        match arg {
            Descriptor::RustStruct(class) => {
                self.get(AdapterType::Anyref);
                self.instructions
                    .push(Instruction::I32FromAnyrefRustBorrow {
                        class: class.clone(),
                    });
                self.output.push(AdapterType::I32);
            }
            Descriptor::Anyref => {
                self.get(AdapterType::Anyref);
                self.instructions.push(Instruction::I32FromAnyrefBorrow);
                self.output.push(AdapterType::I32);
            }
            Descriptor::String | Descriptor::CachedString | Descriptor::Slice(_) => {
                panic!()
                //     let kind = arg.vector_kind().ok_or_else(|| {
                //         format_err!(
                //             "unsupported slice type for calling Rust function from JS {:?}",
                //             arg
                //         )
                //     })?;
                //     self.wasm.extend(&[ValType::I32; 2]);
                //     if mutable {
                //         self.bindings.push(NonstandardIncoming::MutableSlice {
                //             kind,
                //             val: self.expr_get(),
                //         });
                //         self.webidl.push(ast::WebidlScalarType::Any);
                //     } else {
                //         self.alloc_copy_kind(kind)
                //     }
            }
            _ => bail!(
                "unsupported reference argument type for calling Rust function from JS: {:?}",
                arg
            ),
        }
        Ok(())
    }

    fn process_option(&mut self, arg: &Descriptor) -> Result<(), Error> {
        match arg {
            Descriptor::Anyref => {
                self.get(AdapterType::Anyref);
                self.instructions.push(Instruction::I32FromOptionAnyref);
                self.output.push(AdapterType::I32);
            }
            Descriptor::I8 => self.option_sentinel(),
            Descriptor::U8 => self.option_sentinel(),
            Descriptor::I16 => self.option_sentinel(),
            Descriptor::U16 => self.option_sentinel(),
            Descriptor::I32 => self.option_native(ValType::I32),
            Descriptor::U32 => self.option_native(ValType::I32),
            Descriptor::F32 => self.option_native(ValType::F32),
            Descriptor::F64 => self.option_native(ValType::F64),
            Descriptor::I64 | Descriptor::U64 => {
                self.get(AdapterType::Anyref);
                let signed = match arg {
                    Descriptor::I64 => true,
                    _ => false,
                };
                self.instructions
                    .push(Instruction::I32SplitOption64 { signed });
                self.output.extend(&[AdapterType::I32; 4]);
            }
            Descriptor::Boolean => {
                self.get(AdapterType::Anyref);
                self.instructions.push(Instruction::I32FromOptionBool);
                self.output.push(AdapterType::I32);
            }
            Descriptor::Char => {
                self.get(AdapterType::Anyref);
                self.instructions.push(Instruction::I32FromOptionChar);
                self.output.push(AdapterType::I32);
            }
            Descriptor::Enum { hole } => {
                self.get(AdapterType::Anyref);
                self.instructions
                    .push(Instruction::I32FromOptionEnum { hole: *hole });
                self.output.push(AdapterType::I32);
            }
            Descriptor::RustStruct(name) => {
                self.get(AdapterType::Anyref);
                self.instructions.push(Instruction::I32FromOptionRust {
                    class: name.to_string(),
                });
                self.output.push(AdapterType::I32);
            }

            // Descriptor::Ref(_) | Descriptor::RefMut(_) => {
            //     let mutable = match arg {
            //         Descriptor::Ref(_) => false,
            //         _ => true,
            //     };
            //     let kind = arg.vector_kind().ok_or_else(|| {
            //         format_err!(
            //             "unsupported optional slice type for calling Rust function from JS {:?}",
            //             arg
            //         )
            //     })?;
            //     self.bindings.push(NonstandardIncoming::OptionSlice {
            //         kind,
            //         val: self.expr_get(),
            //         mutable,
            //     });
            //     self.wasm.extend(&[ValType::I32; 2]);
            //     self.webidl.push(ast::WebidlScalarType::Any);
            // }
            //
            // Descriptor::String | Descriptor::CachedString | Descriptor::Vector(_) => {
            //     let kind = arg.vector_kind().ok_or_else(|| {
            //         format_err!(
            //             "unsupported optional slice type for calling Rust function from JS {:?}",
            //             arg
            //         )
            //     })?;
            //     self.bindings.push(NonstandardIncoming::OptionVector {
            //         kind,
            //         val: self.expr_get(),
            //     });
            //     self.wasm.extend(&[ValType::I32; 2]);
            //     self.webidl.push(ast::WebidlScalarType::Any);
            // }
            _ => bail!(
                "unsupported optional argument type for calling Rust function from JS: {:?}",
                arg
            ),
        }
        Ok(())
    }

    fn get(&mut self, ty: AdapterType) {
        let idx = self.input.len() as u32 - 1;
        self.input.push(ty);
        let std = wit_walrus::Instruction::ArgGet(idx);
        self.instructions.push(Instruction::Standard(std));
    }

    // fn alloc_func_name(&self) -> String {
    //     "__wbindgen_malloc".to_string()
    // }
    //
    // fn alloc_copy_kind(&mut self, kind: VectorKind) {
    //     use wasm_webidl_bindings::ast::WebidlScalarType::*;
    //
    //     match kind {
    //         VectorKind::I8 => self.alloc_copy(Int8Array),
    //         VectorKind::U8 => self.alloc_copy(Uint8Array),
    //         VectorKind::ClampedU8 => self.alloc_copy(Uint8ClampedArray),
    //         VectorKind::I16 => self.alloc_copy(Int16Array),
    //         VectorKind::U16 => self.alloc_copy(Uint16Array),
    //         VectorKind::I32 => self.alloc_copy(Int32Array),
    //         VectorKind::U32 => self.alloc_copy(Uint32Array),
    //         VectorKind::F32 => self.alloc_copy(Float32Array),
    //         VectorKind::F64 => self.alloc_copy(Float64Array),
    //         VectorKind::String => {
    //             let expr = ast::IncomingBindingExpressionAllocUtf8Str {
    //                 alloc_func_name: self.alloc_func_name(),
    //                 expr: Box::new(self.expr_get()),
    //             };
    //             self.webidl.push(DomString);
    //             self.bindings
    //                 .push(NonstandardIncoming::Standard(expr.into()));
    //         }
    //         VectorKind::I64 | VectorKind::U64 => {
    //             let signed = match kind {
    //                 VectorKind::I64 => true,
    //                 _ => false,
    //             };
    //             self.bindings.push(NonstandardIncoming::AllocCopyInt64 {
    //                 alloc_func_name: self.alloc_func_name(),
    //                 expr: Box::new(self.expr_get()),
    //                 signed,
    //             });
    //             self.webidl.push(Any);
    //         }
    //         VectorKind::Anyref => {
    //             self.bindings
    //                 .push(NonstandardIncoming::AllocCopyAnyrefArray {
    //                     alloc_func_name: self.alloc_func_name(),
    //                     expr: Box::new(self.expr_get()),
    //                 });
    //             self.webidl.push(Any);
    //         }
    //     }
    // }
    //
    // fn alloc_copy(&mut self, webidl: ast::WebidlScalarType) {
    //     let expr = ast::IncomingBindingExpressionAllocCopy {
    //         alloc_func_name: self.alloc_func_name(),
    //         expr: Box::new(self.expr_get()),
    //     };
    //     self.webidl.push(webidl);
    //     self.bindings
    //         .push(NonstandardIncoming::Standard(expr.into()));
    // }

    fn number(&mut self, input: wit_walrus::ValType, output: walrus::ValType) {
        self.get(AdapterType::from_wit(input));
        let std = wit_walrus::Instruction::IntToWasm {
            input,
            output,
            trap: false,
        };
        self.instructions.push(Instruction::Standard(std));
        self.output.push(AdapterType::from_wasm(output).unwrap());
    }

    fn number64(&mut self, signed: bool) {
        self.get(if signed {
            AdapterType::S64
        } else {
            AdapterType::U64
        });
        self.instructions.push(Instruction::I32Split64 { signed });
        self.output.push(AdapterType::I32);
        self.output.push(AdapterType::I32);
    }

    fn option_native(&mut self, wasm: ValType) {
        self.get(AdapterType::Anyref);
        self.instructions
            .push(Instruction::OptionNative { ty: wasm });
        self.output.push(AdapterType::I32);
        self.output.push(AdapterType::from_wasm(wasm).unwrap());
    }

    fn option_sentinel(&mut self) {
        self.get(AdapterType::Anyref);
        self.instructions
            .push(Instruction::I32FromOptionU32Sentinel);
        self.output.push(AdapterType::I32);
    }
}
