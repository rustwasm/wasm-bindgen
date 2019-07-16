//! Nonstandard and wasm-bindgen specific definition of incoming bindings to a
//! wasm module.
//!
//! This module provides a builder which is used to translate Rust types (aka a
//! `Descriptor`) to a `NonstandardIncoming` definition which describes how the
//! JS type is converted into a Rust type. We try to use standard webidl
//! bindings as much as possible, but we have quite a few other bindings which
//! require custom code and shims currently still.
//!
//! Note that the mirror operation, going from WebAssembly to JS, is found in
//! the `outgoing.rs` module.

use crate::descriptor::{Descriptor, VectorKind};
use failure::{bail, format_err, Error};
use walrus::ValType;
use wasm_webidl_bindings::ast;

/// A list of all incoming bindings from JS to WebAssembly that wasm-bindgen
/// will take advantage of.
#[derive(Debug, Clone)]
pub enum NonstandardIncoming {
    /// This is a standard vanilla incoming binding. When WebIDL bindings are
    /// implemented, this can be used as-is.
    Standard(ast::IncomingBindingExpression),

    /// JS is passing a `BigInt` to Rust.
    Int64 {
        val: ast::IncomingBindingExpression,
        /// Whether it's a `u64` or `i64` in Rust.
        signed: bool,
    },

    /// JS is passing a `BigInt64Array` or `BigUint64Array` to Rust
    ///
    /// A copy of the array needs to be made into the Rust address space.
    AllocCopyInt64 {
        alloc_func_name: String,
        expr: Box<ast::IncomingBindingExpression>,
        /// Whether or not this is for &[u64] or &[i64]
        signed: bool,
    },

    /// JS is passing an array of anyref values into Rust, and all the values
    /// need to be copied in.
    AllocCopyAnyrefArray {
        alloc_func_name: String,
        expr: Box<ast::IncomingBindingExpression>,
    },

    /// A mutable slice of values going from JS to Rust, and after Rust finishes
    /// the JS slice is updated with the current value of the slice.
    MutableSlice {
        kind: VectorKind,
        val: ast::IncomingBindingExpression,
    },

    /// This is either a slice or `undefined` being passed into Rust.
    OptionSlice {
        kind: VectorKind,
        val: ast::IncomingBindingExpression,
        mutable: bool,
    },

    /// This is either a vector or `undefined` being passed into Rust.
    OptionVector {
        kind: VectorKind,
        val: ast::IncomingBindingExpression,
    },

    /// Not actually used for `JsValue` but used for imported types, this is
    /// either `undefined` or the imported type getting passed into Rust.
    OptionAnyref { val: ast::IncomingBindingExpression },

    /// An optional "native type" which includes i32/u32/f32/f64, all of which
    /// require a discriminant.
    OptionNative { val: ast::IncomingBindingExpression },

    /// An optional integer type which uses an 0xffffff sentinel value for
    /// "none"
    OptionU32Sentinel { val: ast::IncomingBindingExpression },

    /// An optional boolean using a special ABI for communicating `undefined`
    OptionBool { val: ast::IncomingBindingExpression },

    /// An optional `char` which uses an ABI where `undefined` is a hole in the
    /// range of valid values for a `char` in Rust. Note that in JS a string is
    /// passed in.
    OptionChar { val: ast::IncomingBindingExpression },

    /// An optional integral enum where `undefined` is the hole specified.
    OptionIntegerEnum {
        val: ast::IncomingBindingExpression,
        hole: u32,
    },

    /// An optional `BigInt`.
    OptionInt64 {
        val: ast::IncomingBindingExpression,
        signed: bool,
    },

    /// An optional Rust-based type which internally has a pointer that's
    /// wrapped up in a JS class. This transfers ownership from JS to Rust.
    RustType {
        class: String,
        val: ast::IncomingBindingExpression,
    },

    /// A reference to a Rust-based type where Rust won't take ownership of the
    /// value, it just has a temporary borrow on the input.
    RustTypeRef {
        class: String,
        val: ast::IncomingBindingExpression,
    },

    /// An optional owned Rust type being transferred from JS to Rust.
    OptionRustType {
        class: String,
        val: ast::IncomingBindingExpression,
    },

    /// A string from JS where the first character goes through to Rust.
    Char { val: ast::IncomingBindingExpression },

    /// An arbitrary `anyref` being passed into Rust, but explicitly one that's
    /// borrowed and doesn't need to be persisted in a heap table.
    BorrowedAnyref { val: ast::IncomingBindingExpression },
}

/// Builder used to create a incomig binding from a `Descriptor`.
#[derive(Default)]
pub struct IncomingBuilder {
    /// The wasm types that needs to be used to represent all the descriptors in
    /// Rust.
    pub wasm: Vec<ValType>,
    /// The WebIDL scalar types which match what JS will be providing.
    pub webidl: Vec<ast::WebidlScalarType>,
    /// The list of bindings necessary to connect `wasm` to `webidl` above.
    pub bindings: Vec<NonstandardIncoming>,
}

impl IncomingBuilder {
    /// Adds an initial argument which is passed through verbatim, currently
    /// used to handle return pointers in Rust.
    pub fn process_retptr(&mut self) {
        self.number(ValType::I32, ast::WebidlScalarType::Long);
    }

    /// Process a `Descriptor` as if it's being passed from JS to Rust. This
    /// will skip `Unit` and otherwise internally add a `NonstandardIncoming`
    /// binding necessary for the descriptor.
    pub fn process(&mut self, arg: &Descriptor) -> Result<(), Error> {
        if let Descriptor::Unit = arg {
            return Ok(());
        }
        // This is a wrapper around `_process` to have a number of sanity checks
        // that we don't forget things. We should always produce at least one
        // wasm arge and exactly one webidl arg. Additionally the number of
        // bindings should always match the number of webidl types for now.
        assert_eq!(self.webidl.len(), self.bindings.len());
        let wasm_before = self.wasm.len();
        let webidl_before = self.webidl.len();
        self._process(arg)?;
        assert_eq!(self.webidl.len(), self.bindings.len());
        assert_eq!(webidl_before + 1, self.webidl.len());
        assert!(wasm_before < self.wasm.len());
        Ok(())
    }

    fn _process(&mut self, arg: &Descriptor) -> Result<(), Error> {
        match arg {
            Descriptor::Boolean => {
                let binding = self.expr_as(ValType::I32);
                self.wasm.push(ValType::I32);
                self.webidl.push(ast::WebidlScalarType::Boolean);
                self.bindings.push(NonstandardIncoming::Standard(binding));
            }
            Descriptor::Char => {
                let expr = self.expr_get();
                self.wasm.push(ValType::I32);
                self.webidl.push(ast::WebidlScalarType::DomString);
                self.bindings.push(NonstandardIncoming::Char { val: expr });
            }
            Descriptor::Anyref => {
                let expr = self.expr_as(ValType::Anyref);
                self.wasm.push(ValType::Anyref);
                self.webidl.push(ast::WebidlScalarType::Any);
                self.bindings.push(NonstandardIncoming::Standard(expr));
            }
            Descriptor::RustStruct(class) => {
                let expr = self.expr_get();
                self.wasm.push(ValType::I32);
                self.webidl.push(ast::WebidlScalarType::Any);
                self.bindings.push(NonstandardIncoming::RustType {
                    val: expr,
                    class: class.to_string(),
                });
            }
            Descriptor::I8 => self.number(ValType::I32, ast::WebidlScalarType::Byte),
            Descriptor::U8 => self.number(ValType::I32, ast::WebidlScalarType::Octet),
            Descriptor::I16 => self.number(ValType::I32, ast::WebidlScalarType::Short),
            Descriptor::U16 => self.number(ValType::I32, ast::WebidlScalarType::UnsignedShort),
            Descriptor::I32 => self.number(ValType::I32, ast::WebidlScalarType::Long),
            Descriptor::U32 => self.number(ValType::I32, ast::WebidlScalarType::UnsignedLong),
            Descriptor::I64 => self.number64(true),
            Descriptor::U64 => self.number64(false),
            Descriptor::F32 => self.number(ValType::F32, ast::WebidlScalarType::Float),
            Descriptor::F64 => self.number(ValType::F64, ast::WebidlScalarType::Double),
            Descriptor::Enum { .. } => self.number(ValType::I32, ast::WebidlScalarType::Long),
            Descriptor::Ref(d) => self.process_ref(false, d)?,
            Descriptor::RefMut(d) => self.process_ref(true, d)?,
            Descriptor::Option(d) => self.process_option(d)?,

            Descriptor::String | Descriptor::CachedString | Descriptor::Vector(_) => {
                let kind = arg.vector_kind().ok_or_else(|| {
                    format_err!("unsupported argument type for calling Rust function from JS {:?}", arg)
                })? ;
                self.wasm.extend(&[ValType::I32; 2]);
                self.alloc_copy_kind(kind)
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
                let expr = self.expr_get();
                self.wasm.push(ValType::I32);
                self.webidl.push(ast::WebidlScalarType::Any);
                self.bindings.push(NonstandardIncoming::RustTypeRef {
                    val: expr,
                    class: class.to_string(),
                });
            }
            Descriptor::Anyref => {
                let expr = self.expr_get();
                self.wasm.push(ValType::Anyref);
                self.webidl.push(ast::WebidlScalarType::Any);
                self.bindings
                    .push(NonstandardIncoming::BorrowedAnyref { val: expr });
            }
            Descriptor::String | Descriptor::CachedString | Descriptor::Slice(_) => {
                let kind = arg.vector_kind().ok_or_else(|| {
                    format_err!(
                        "unsupported slice type for calling Rust function from JS {:?}",
                        arg
                    )
                })?;
                self.wasm.extend(&[ValType::I32; 2]);
                if mutable {
                    self.bindings.push(NonstandardIncoming::MutableSlice {
                        kind,
                        val: self.expr_get(),
                    });
                    self.webidl.push(ast::WebidlScalarType::Any);
                } else {
                    self.alloc_copy_kind(kind)
                }
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
                self.wasm.push(ValType::I32);
                self.bindings.push(NonstandardIncoming::OptionAnyref {
                    val: self.expr_get(),
                });
                self.webidl.push(ast::WebidlScalarType::Any);
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
                let expr = self.expr_get();
                let signed = match arg {
                    Descriptor::I64 => true,
                    _ => false,
                };
                self.wasm.extend(&[walrus::ValType::I32; 4]);
                self.webidl.push(ast::WebidlScalarType::Any);
                self.bindings
                    .push(NonstandardIncoming::OptionInt64 { val: expr, signed });
            }
            Descriptor::Boolean => {
                let expr = self.expr_get();
                self.wasm.push(walrus::ValType::I32);
                self.webidl.push(ast::WebidlScalarType::Any);
                self.bindings
                    .push(NonstandardIncoming::OptionBool { val: expr });
            }
            Descriptor::Char => {
                let expr = self.expr_get();
                self.wasm.push(walrus::ValType::I32);
                self.webidl.push(ast::WebidlScalarType::Any);
                self.bindings
                    .push(NonstandardIncoming::OptionChar { val: expr });
            }
            Descriptor::Enum { hole } => {
                let expr = self.expr_get();
                self.wasm.push(walrus::ValType::I32);
                self.webidl.push(ast::WebidlScalarType::Any);
                self.bindings.push(NonstandardIncoming::OptionIntegerEnum {
                    val: expr,
                    hole: *hole,
                });
            }
            Descriptor::RustStruct(name) => {
                let expr = self.expr_get();
                self.wasm.push(walrus::ValType::I32);
                self.webidl.push(ast::WebidlScalarType::Any);
                self.bindings.push(NonstandardIncoming::OptionRustType {
                    val: expr,
                    class: name.to_string(),
                });
            }

            Descriptor::Ref(_) | Descriptor::RefMut(_) => {
                let mutable = match arg {
                    Descriptor::Ref(_) => false,
                    _ => true,
                };
                let kind = arg.vector_kind().ok_or_else(|| {
                    format_err!(
                        "unsupported optional slice type for calling Rust function from JS {:?}",
                        arg
                    )
                })?;
                self.bindings.push(NonstandardIncoming::OptionSlice {
                    kind,
                    val: self.expr_get(),
                    mutable,
                });
                self.wasm.extend(&[ValType::I32; 2]);
                self.webidl.push(ast::WebidlScalarType::Any);
            }

            Descriptor::String | Descriptor::CachedString | Descriptor::Vector(_) => {
                let kind = arg.vector_kind().ok_or_else(|| {
                    format_err!(
                        "unsupported optional slice type for calling Rust function from JS {:?}",
                        arg
                    )
                })?;
                self.bindings.push(NonstandardIncoming::OptionVector {
                    kind,
                    val: self.expr_get(),
                });
                self.wasm.extend(&[ValType::I32; 2]);
                self.webidl.push(ast::WebidlScalarType::Any);
            }

            _ => bail!(
                "unsupported optional argument type for calling Rust function from JS: {:?}",
                arg
            ),
        }
        Ok(())
    }

    fn expr_get(&self) -> ast::IncomingBindingExpression {
        let idx = self.webidl.len() as u32;
        ast::IncomingBindingExpressionGet { idx }.into()
    }

    fn expr_as(&self, ty: ValType) -> ast::IncomingBindingExpression {
        ast::IncomingBindingExpressionAs {
            ty,
            expr: Box::new(self.expr_get()),
        }
        .into()
    }

    fn alloc_func_name(&self) -> String {
        "__wbindgen_malloc".to_string()
    }

    fn alloc_copy_kind(&mut self, kind: VectorKind) {
        use wasm_webidl_bindings::ast::WebidlScalarType::*;

        match kind {
            VectorKind::I8 => self.alloc_copy(Int8Array),
            VectorKind::U8 => self.alloc_copy(Uint8Array),
            VectorKind::ClampedU8 => self.alloc_copy(Uint8ClampedArray),
            VectorKind::I16 => self.alloc_copy(Int16Array),
            VectorKind::U16 => self.alloc_copy(Uint16Array),
            VectorKind::I32 => self.alloc_copy(Int32Array),
            VectorKind::U32 => self.alloc_copy(Uint32Array),
            VectorKind::F32 => self.alloc_copy(Float32Array),
            VectorKind::F64 => self.alloc_copy(Float64Array),
            VectorKind::String => {
                let expr = ast::IncomingBindingExpressionAllocUtf8Str {
                    alloc_func_name: self.alloc_func_name(),
                    expr: Box::new(self.expr_get()),
                };
                self.webidl.push(DomString);
                self.bindings
                    .push(NonstandardIncoming::Standard(expr.into()));
            }
            VectorKind::I64 | VectorKind::U64 => {
                let signed = match kind {
                    VectorKind::I64 => true,
                    _ => false,
                };
                self.bindings.push(NonstandardIncoming::AllocCopyInt64 {
                    alloc_func_name: self.alloc_func_name(),
                    expr: Box::new(self.expr_get()),
                    signed,
                });
                self.webidl.push(Any);
            }
            VectorKind::Anyref => {
                self.bindings
                    .push(NonstandardIncoming::AllocCopyAnyrefArray {
                        alloc_func_name: self.alloc_func_name(),
                        expr: Box::new(self.expr_get()),
                    });
                self.webidl.push(Any);
            }
        }
    }

    fn alloc_copy(&mut self, webidl: ast::WebidlScalarType) {
        let expr = ast::IncomingBindingExpressionAllocCopy {
            alloc_func_name: self.alloc_func_name(),
            expr: Box::new(self.expr_get()),
        };
        self.webidl.push(webidl);
        self.bindings
            .push(NonstandardIncoming::Standard(expr.into()));
    }

    fn number(&mut self, wasm: ValType, webidl: ast::WebidlScalarType) {
        let binding = self.expr_as(wasm);
        self.wasm.push(wasm);
        self.webidl.push(webidl);
        self.bindings.push(NonstandardIncoming::Standard(binding));
    }

    fn number64(&mut self, signed: bool) {
        let expr = self.expr_get();
        self.wasm.extend(&[ValType::I32; 2]);
        self.webidl.push(ast::WebidlScalarType::Any);
        self.bindings
            .push(NonstandardIncoming::Int64 { val: expr, signed });
    }

    fn option_native(&mut self, wasm: ValType) {
        let expr = self.expr_get();
        self.wasm.push(ValType::I32);
        self.wasm.push(wasm);
        self.webidl.push(ast::WebidlScalarType::Any);
        self.bindings
            .push(NonstandardIncoming::OptionNative { val: expr });
    }

    fn option_sentinel(&mut self) {
        let expr = self.expr_get();
        self.wasm.push(ValType::I32);
        self.webidl.push(ast::WebidlScalarType::Any);
        self.bindings
            .push(NonstandardIncoming::OptionU32Sentinel { val: expr });
    }
}
