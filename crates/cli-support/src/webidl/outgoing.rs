//! This module is used to define `NonstandardOutgoing`, a list of possible ways
//! values in Rust can be passed to JS.
//!
//! Like the `NonstandardIncoming` list we attempt to use a standard
//! `OutgoingBindingExpression` wherever possible but we naturally have a lot of
//! features in `wasm-bindgen` which haven't been upstreamed into the WebIDL
//! bindings standard yet (nor which are likely to ever get standardized). We
//! attempt to use standard bindings aggressively and wherever possible, but
//! sometimes we need to resort to our own custom bindings with our own custom
//! JS shims for now.
//!
//! This module also houses the definition of converting a `Descriptor` to a
//! `NonstandardOutgoing` binding, effectively defining how to translate from a
//! Rust type to an outgoing binding.

use crate::descriptor::{Descriptor, VectorKind};
use crate::webidl::NonstandardWebidlSection;
use failure::{bail, format_err, Error};
use walrus::{Module, ValType};
use wasm_webidl_bindings::ast;

/// A list of all possible outgoing bindings which can be used when converting
/// Rust types to JS. This is predominantly used when calling an imported JS
/// function.
#[derive(Debug, Clone)]
pub enum NonstandardOutgoing {
    /// This is a standard upstream WebIDL outgoing binding expression. Where
    /// possible we can actually leave this in the wasm file and generate even
    /// less JS shim code.
    Standard(ast::OutgoingBindingExpression),

    /// We're returning a pointer from Rust to JS to get wrapped in a JS class
    /// which has memory management around it.
    RustType { class: String, idx: u32 },

    /// A single rust `char` value which is converted to a `string` in JS.
    Char { idx: u32 },

    /// An `i64` or `u64` in Rust converted to a `BigInt` in JS
    Number64 {
        lo_idx: u32,
        hi_idx: u32,
        signed: bool,
    },

    /// A *borrowed* anyref value which has special meanings about ownership,
    /// namely Rust is still using the underlying value after the call returns.
    BorrowedAnyref { idx: u32 },

    /// An owned vector is passed from Rust to JS. Note that this is currently a
    /// special binding because it requires memory management via deallocation
    /// in the JS shim.
    ///
    /// TODO: we should strive to not have this nonstandard binding and instead
    /// do all the memory management in Rust. Ideally we'd use `AllocCopy` in
    /// place of this.
    Vector {
        offset: u32,
        length: u32,
        kind: VectorKind,
    },

    /// A Rust String (or &str) which might be cached, or might be `None`.
    ///
    /// If `offset` is 0 then it is cached, and the cached JsValue's index is in `length`.
    ///
    /// If `offset` and `length` are both 0, then it is `None`.
    CachedString {
        offset: u32,
        length: u32,
        owned: bool,
        optional: bool,
    },

    /// A `&[u64]` or `&[i64]` is being passed to JS, and the 64-bit sizes here
    /// aren't supported by WebIDL bindings yet.
    View64 {
        offset: u32,
        length: u32,
        signed: bool,
    },

    /// A list of `anyref` is being passed to JS, and it's got a somewhat
    /// magical representation with indics which doesn't map to WebIDL bindings.
    ViewAnyref { offset: u32, length: u32 },

    /// An optional owned vector of data is being passed to JS.
    ///
    /// TODO: with some cleverness this could probably use `AllocCopy`.
    OptionVector {
        offset: u32,
        length: u32,
        kind: VectorKind,
    },

    /// An optional slice of data is being passed into JS.
    ///
    /// TODO: with some cleverness this could probably use `AllocCopy`.
    OptionSlice {
        kind: VectorKind,
        offset: u32,
        length: u32,
    },

    /// An optional "native type" like i32/u32/f32/f64 is being passed to JS,
    /// and this requires a discriminant in the ABI.
    OptionNative {
        present: u32,
        val: u32,
        signed: bool,
    },

    /// An optional number is being passed to JS where the number uses a
    /// sentinel value to represent `None`
    OptionU32Sentinel { idx: u32 },

    /// An optional boolean with a special value for `None`
    OptionBool { idx: u32 },

    /// An optional character with a special value for `None`
    OptionChar { idx: u32 },

    /// An optional integral enum value with the specified `hole` being used for
    /// `None`.
    OptionIntegerEnum { idx: u32, hole: u32 },

    /// An optional 64-bit integer being used.
    OptionInt64 {
        present: u32,
        _ignored: u32,
        lo: u32,
        hi: u32,
        signed: bool,
    },

    /// An optional owned Rust type being transferred from Rust to JS.
    OptionRustType { class: String, idx: u32 },

    /// A temporary stack closure being passed from Rust to JS. A JS function is
    /// manufactured and then neutered just before the call returns.
    StackClosure {
        /// Argument index of the first data pointer Rust needs
        a: u32,
        /// Argument index of the second data pointer Rust needs
        b: u32,
        /// The index of the shim in the element bindings section that we're
        /// going to be invoking.
        binding_idx: u32,
        /// Number of arguments to the closure
        nargs: usize,
        /// Whether or not this is a mutable closure (affects codegen and how
        /// it's called recursively)
        mutable: bool,
    },
}

/// A definition of building `NonstandardOutgoing` expressions from a
/// `Descriptor`.
///
/// This will internally keep track of wasm/webidl types generated as we visit
/// `Descriptor` arguments and add more for a function signature.
#[derive(Default)]
pub struct OutgoingBuilder<'a> {
    /// All wasm types used so far to produce the resulting JS values.
    pub wasm: Vec<ValType>,
    /// The WebIDL types that we're passing along out of wasm.
    pub webidl: Vec<ast::WebidlScalarType>,
    /// The list of bindings we've created, currently 1:1 with the webidl above.
    pub bindings: Vec<NonstandardOutgoing>,

    // These two arguments are optional and, if set, will enable creating
    // `StackClosure` bindings. They're not present for return values from
    // exported Rust functions, but they are available for the arguments of
    // calling imported functions.
    pub module: Option<&'a mut Module>,
    pub bindings_section: Option<&'a mut NonstandardWebidlSection>,
}

impl OutgoingBuilder<'_> {
    /// Adds a dummy first argument which is passed through as an integer
    /// representing the return pointer.
    pub fn process_retptr(&mut self) {
        self.standard_as(ValType::I32, ast::WebidlScalarType::Long);
    }

    /// Processes one more `Descriptor` as an argument to a JS function that
    /// wasm is calling.
    ///
    /// This will internally skip `Unit` and otherwise build up the `bindings`
    /// map and ensure that it's correctly mapped from wasm to JS.
    pub fn process(&mut self, arg: &Descriptor) -> Result<(), Error> {
        if let Descriptor::Unit = arg {
            return Ok(());
        }
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
            Descriptor::Boolean => self.standard_as(ValType::I32, ast::WebidlScalarType::Boolean),
            Descriptor::Anyref => self.standard_as(ValType::Anyref, ast::WebidlScalarType::Any),
            Descriptor::I8 => self.standard_as(ValType::I32, ast::WebidlScalarType::Byte),
            Descriptor::U8 => self.standard_as(ValType::I32, ast::WebidlScalarType::Octet),
            Descriptor::I16 => self.standard_as(ValType::I32, ast::WebidlScalarType::Short),
            Descriptor::U16 => self.standard_as(ValType::I32, ast::WebidlScalarType::UnsignedShort),
            Descriptor::I32 => self.standard_as(ValType::I32, ast::WebidlScalarType::Long),
            Descriptor::U32 => self.standard_as(ValType::I32, ast::WebidlScalarType::UnsignedLong),
            Descriptor::F32 => {
                self.standard_as(ValType::F32, ast::WebidlScalarType::UnrestrictedFloat)
            }
            Descriptor::F64 => {
                self.standard_as(ValType::F64, ast::WebidlScalarType::UnrestrictedDouble)
            }
            Descriptor::Enum { .. } => self.standard_as(ValType::I32, ast::WebidlScalarType::Long),

            Descriptor::Char => {
                let idx = self.push_wasm(ValType::I32);
                self.webidl.push(ast::WebidlScalarType::DomString);
                self.bindings.push(NonstandardOutgoing::Char { idx });
            }

            Descriptor::I64 | Descriptor::U64 => {
                let signed = match arg {
                    Descriptor::I64 => true,
                    _ => false,
                };
                let lo_idx = self.push_wasm(ValType::I32);
                let hi_idx = self.push_wasm(ValType::I32);
                self.webidl.push(ast::WebidlScalarType::Any);
                self.bindings.push(NonstandardOutgoing::Number64 {
                    lo_idx,
                    hi_idx,
                    signed,
                });
            }

            Descriptor::RustStruct(class) => {
                let idx = self.push_wasm(ValType::I32);
                self.webidl.push(ast::WebidlScalarType::Any);
                self.bindings.push(NonstandardOutgoing::RustType {
                    idx,
                    class: class.to_string(),
                });
            }
            Descriptor::Ref(d) => self.process_ref(false, d)?,
            Descriptor::RefMut(d) => self.process_ref(true, d)?,

            Descriptor::CachedString => self.cached_string(false, true),

            Descriptor::Vector(_) | Descriptor::String => {
                let kind = arg.vector_kind().ok_or_else(|| {
                    format_err!(
                        "unsupported argument type for calling JS function from Rust {:?}",
                        arg
                    )
                })?;
                let offset = self.push_wasm(ValType::I32);
                let length = self.push_wasm(ValType::I32);
                self.webidl.push(ast::WebidlScalarType::Any);
                self.bindings.push(NonstandardOutgoing::Vector {
                    offset,
                    kind,
                    length,
                })
            }

            Descriptor::Option(d) => self.process_option(d)?,

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

    fn process_ref(&mut self, mutable: bool, arg: &Descriptor) -> Result<(), Error> {
        match arg {
            Descriptor::Anyref => {
                let idx = self.push_wasm(ValType::Anyref);
                self.webidl.push(ast::WebidlScalarType::Any);
                self.bindings
                    .push(NonstandardOutgoing::BorrowedAnyref { idx });
            }
            Descriptor::CachedString => self.cached_string(false, false),
            Descriptor::Slice(_) | Descriptor::String => {
                use wasm_webidl_bindings::ast::WebidlScalarType::*;

                let kind = arg.vector_kind().ok_or_else(|| {
                    format_err!(
                        "unsupported argument type for calling JS function from Rust {:?}",
                        arg
                    )
                })?;
                let offset = self.push_wasm(ValType::I32);
                let length = self.push_wasm(ValType::I32);
                match kind {
                    VectorKind::I8 => self.standard_view(offset, length, Int8Array),
                    VectorKind::U8 => self.standard_view(offset, length, Uint8Array),
                    VectorKind::ClampedU8 => self.standard_view(offset, length, Uint8ClampedArray),
                    VectorKind::I16 => self.standard_view(offset, length, Int16Array),
                    VectorKind::U16 => self.standard_view(offset, length, Uint16Array),
                    VectorKind::I32 => self.standard_view(offset, length, Int32Array),
                    VectorKind::U32 => self.standard_view(offset, length, Uint32Array),
                    VectorKind::F32 => self.standard_view(offset, length, Float32Array),
                    VectorKind::F64 => self.standard_view(offset, length, Float64Array),
                    VectorKind::String => {
                        self.webidl.push(DomString);
                        let binding = ast::OutgoingBindingExpressionUtf8Str {
                            ty: ast::WebidlScalarType::DomString.into(),
                            offset,
                            length,
                        };
                        self.bindings
                            .push(NonstandardOutgoing::Standard(binding.into()));
                    }
                    VectorKind::I64 | VectorKind::U64 => {
                        let signed = match kind {
                            VectorKind::I64 => true,
                            _ => false,
                        };
                        self.webidl.push(Any);
                        self.bindings.push(NonstandardOutgoing::View64 {
                            offset,
                            length,
                            signed,
                        });
                    }
                    VectorKind::Anyref => {
                        self.webidl.push(Any);
                        self.bindings
                            .push(NonstandardOutgoing::ViewAnyref { offset, length });
                    }
                }
            }

            Descriptor::Function(descriptor) => {
                let module = self
                    .module
                    .as_mut()
                    .ok_or_else(|| format_err!("cannot return a closure from Rust"))?;
                let section = self.bindings_section.as_mut().unwrap();
                // synthesize the a/b arguments that aren't present in the
                // signature from wasm-bindgen but are present in the wasm file.
                let mut descriptor = (**descriptor).clone();
                let nargs = descriptor.arguments.len();
                descriptor.arguments.insert(0, Descriptor::I32);
                descriptor.arguments.insert(0, Descriptor::I32);
                let binding_idx = super::bindings::register_table_element(
                    module,
                    section,
                    descriptor.shim_idx,
                    descriptor,
                )?;
                let a = self.push_wasm(ValType::I32);
                let b = self.push_wasm(ValType::I32);
                self.webidl.push(ast::WebidlScalarType::Any);
                self.bindings.push(NonstandardOutgoing::StackClosure {
                    a,
                    b,
                    binding_idx,
                    nargs,
                    mutable,
                });
            }

            _ => bail!(
                "unsupported reference argument type for calling JS function from Rust: {:?}",
                arg
            ),
        }
        Ok(())
    }

    fn process_option(&mut self, arg: &Descriptor) -> Result<(), Error> {
        match arg {
            Descriptor::Anyref => self.standard_as(ValType::Anyref, ast::WebidlScalarType::Any),
            Descriptor::I8 => self.option_sentinel(),
            Descriptor::U8 => self.option_sentinel(),
            Descriptor::I16 => self.option_sentinel(),
            Descriptor::U16 => self.option_sentinel(),
            Descriptor::I32 => self.option_native(true, ValType::I32),
            Descriptor::U32 => self.option_native(false, ValType::I32),
            Descriptor::F32 => self.option_native(true, ValType::F32),
            Descriptor::F64 => self.option_native(true, ValType::F64),
            Descriptor::I64 | Descriptor::U64 => {
                let signed = match arg {
                    Descriptor::I64 => true,
                    _ => false,
                };
                let binding = NonstandardOutgoing::OptionInt64 {
                    present: self.push_wasm(ValType::I32),
                    _ignored: self.push_wasm(ValType::I32),
                    lo: self.push_wasm(ValType::I32),
                    hi: self.push_wasm(ValType::I32),
                    signed,
                };
                self.webidl.push(ast::WebidlScalarType::Any);
                self.bindings.push(binding);
            }
            Descriptor::Boolean => {
                let idx = self.push_wasm(ValType::I32);
                self.webidl.push(ast::WebidlScalarType::Any);
                self.bindings.push(NonstandardOutgoing::OptionBool { idx });
            }
            Descriptor::Char => {
                let idx = self.push_wasm(ValType::I32);
                self.webidl.push(ast::WebidlScalarType::Any);
                self.bindings.push(NonstandardOutgoing::OptionChar { idx });
            }
            Descriptor::Enum { hole } => {
                let idx = self.push_wasm(ValType::I32);
                self.webidl.push(ast::WebidlScalarType::Long);
                self.bindings
                    .push(NonstandardOutgoing::OptionIntegerEnum { idx, hole: *hole });
            }
            Descriptor::RustStruct(name) => {
                let idx = self.push_wasm(ValType::I32);
                self.webidl.push(ast::WebidlScalarType::Any);
                self.bindings.push(NonstandardOutgoing::OptionRustType {
                    idx,
                    class: name.to_string(),
                });
            }
            Descriptor::Ref(d) => self.process_option_ref(false, d)?,
            Descriptor::RefMut(d) => self.process_option_ref(true, d)?,

            Descriptor::CachedString => self.cached_string(true, true),

            Descriptor::String | Descriptor::Vector(_) => {
                let kind = arg.vector_kind().ok_or_else(|| {
                    format_err!(
                        "unsupported optional slice type for calling JS function from Rust {:?}",
                        arg
                    )
                })?;
                let offset = self.push_wasm(ValType::I32);
                let length = self.push_wasm(ValType::I32);
                self.webidl.push(ast::WebidlScalarType::Any);
                self.bindings.push(NonstandardOutgoing::OptionVector {
                    kind,
                    offset,
                    length,
                })
            }

            _ => bail!(
                "unsupported optional argument type for calling JS function from Rust: {:?}",
                arg
            ),
        }
        Ok(())
    }

    fn process_option_ref(&mut self, _mutable: bool, arg: &Descriptor) -> Result<(), Error> {
        match arg {
            Descriptor::Anyref => {
                let idx = self.push_wasm(ValType::Anyref);
                self.webidl.push(ast::WebidlScalarType::Any);
                self.bindings
                    .push(NonstandardOutgoing::BorrowedAnyref { idx });
            }
            Descriptor::CachedString => self.cached_string(true, false),
            Descriptor::String | Descriptor::Slice(_) => {
                let kind = arg.vector_kind().ok_or_else(|| {
                    format_err!(
                        "unsupported optional slice type for calling JS function from Rust {:?}",
                        arg
                    )
                })?;
                let offset = self.push_wasm(ValType::I32);
                let length = self.push_wasm(ValType::I32);
                self.webidl.push(ast::WebidlScalarType::Any);
                self.bindings.push(NonstandardOutgoing::OptionSlice {
                    kind,
                    offset,
                    length,
                });
            }
            _ => bail!(
                "unsupported optional ref argument type for calling JS function from Rust: {:?}",
                arg
            ),
        }
        Ok(())
    }

    fn push_wasm(&mut self, ty: ValType) -> u32 {
        self.wasm.push(ty);
        self.wasm.len() as u32 - 1
    }

    fn standard_as(&mut self, wasm: ValType, webidl: ast::WebidlScalarType) {
        let binding = ast::OutgoingBindingExpressionAs {
            ty: webidl.into(),
            idx: self.push_wasm(wasm),
        };
        self.webidl.push(webidl);
        self.bindings
            .push(NonstandardOutgoing::Standard(binding.into()));
    }

    fn standard_view(&mut self, offset: u32, length: u32, ty: ast::WebidlScalarType) {
        let binding = ast::OutgoingBindingExpressionView {
            ty: ty.into(),
            offset,
            length,
        };
        self.webidl.push(ty);
        self.bindings
            .push(NonstandardOutgoing::Standard(binding.into()));
    }

    fn cached_string(&mut self, optional: bool, owned: bool) {
        let offset = self.push_wasm(ValType::I32);
        let length = self.push_wasm(ValType::I32);
        self.webidl.push(ast::WebidlScalarType::DomString);
        self.bindings.push(NonstandardOutgoing::CachedString {
            offset,
            length,
            owned,
            optional,
        })
    }

    fn option_native(&mut self, signed: bool, ty: ValType) {
        let present = self.push_wasm(ValType::I32);
        let val = self.push_wasm(ty);
        self.webidl.push(ast::WebidlScalarType::Any);
        self.bindings.push(NonstandardOutgoing::OptionNative {
            signed,
            present,
            val,
        });
    }

    fn option_sentinel(&mut self) {
        let idx = self.push_wasm(ValType::I32);
        self.webidl.push(ast::WebidlScalarType::Any);
        self.bindings
            .push(NonstandardOutgoing::OptionU32Sentinel { idx });
    }
}
