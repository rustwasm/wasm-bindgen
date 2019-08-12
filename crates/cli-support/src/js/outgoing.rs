//! Implementation of translating a `NonstandardOutgoing` expression to an
//! actual JS shim and code snippet which ensures that bindings behave as we'd
//! expect.

use crate::descriptor::VectorKind;
use crate::js::binding::JsBuilder;
use crate::js::Context;
use crate::webidl::NonstandardOutgoing;
use failure::{bail, Error};
use wasm_webidl_bindings::ast;

pub struct Outgoing<'a, 'b> {
    cx: &'a mut Context<'b>,
    js: &'a mut JsBuilder,
}

impl<'a, 'b> Outgoing<'a, 'b> {
    pub fn new(cx: &'a mut Context<'b>, js: &'a mut JsBuilder) -> Outgoing<'a, 'b> {
        Outgoing { cx, js }
    }

    pub fn process(&mut self, outgoing: &NonstandardOutgoing) -> Result<String, Error> {
        let before = self.js.typescript_len();
        let ret = self.nonstandard(outgoing)?;
        assert_eq!(before + 1, self.js.typescript_len());
        Ok(ret)
    }

    fn nonstandard(&mut self, outgoing: &NonstandardOutgoing) -> Result<String, Error> {
        match outgoing {
            NonstandardOutgoing::Standard(expr) => self.standard(expr),

            // Converts the wasm argument, a single code unit, to a string.
            NonstandardOutgoing::Char { idx } => {
                self.js.typescript_required("string");
                Ok(format!("String.fromCodePoint({})", self.arg(*idx)))
            }

            // Just need to wrap up the pointer we get from Rust into a JS type
            // and then we can pass that along
            NonstandardOutgoing::RustType { class, idx } => {
                self.js.typescript_required(class);
                self.cx.require_class_wrap(class);
                Ok(format!("{}.__wrap({})", class, self.arg(*idx)))
            }

            // Just a small wrapper around `getObject`
            NonstandardOutgoing::BorrowedAnyref { idx } => {
                self.js.typescript_required("any");
                self.cx.expose_get_object();
                Ok(format!("getObject({})", self.arg(*idx)))
            }

            // given the low/high bits we get from Rust, store them into a
            // temporary 64-bit conversion array and then load the BigInt out of
            // it.
            NonstandardOutgoing::Number64 {
                lo_idx,
                hi_idx,
                signed,
            } => {
                self.js.typescript_required("BigInt");
                let f = if *signed {
                    self.cx.expose_int64_cvt_shim()
                } else {
                    self.cx.expose_uint64_cvt_shim()
                };
                let i = self.js.tmp();
                self.js.prelude(&format!(
                    "\
                         u32CvtShim[0] = {low};
                         u32CvtShim[1] = {high};
                         const n{i} = {f}[0];
                     ",
                    low = self.arg(*lo_idx),
                    high = self.arg(*hi_idx),
                    f = f,
                    i = i,
                ));
                Ok(format!("n{}", i))
            }

            // Similar to `View` below, except using 64-bit types which don't
            // fit into webidl scalar types right now.
            NonstandardOutgoing::View64 {
                offset,
                length,
                signed,
            } => {
                let ptr = self.arg(*offset);
                let len = self.arg(*length);
                let kind = if *signed {
                    VectorKind::I64
                } else {
                    VectorKind::U64
                };
                self.js.typescript_required(kind.js_ty());
                let f = self.cx.expose_get_vector_from_wasm(kind)?;
                Ok(format!("{}({}, {})", f, ptr, len))
            }

            // Similar to `View` below, except using anyref types which have
            // fancy conversion functions on our end.
            NonstandardOutgoing::ViewAnyref { offset, length } => {
                let ptr = self.arg(*offset);
                let len = self.arg(*length);
                self.js.typescript_required(VectorKind::Anyref.js_ty());
                let f = self.cx.expose_get_vector_from_wasm(VectorKind::Anyref)?;
                Ok(format!("{}({}, {})", f, ptr, len))
            }

            // Similar to `View` below, except we free the memory in JS right
            // now.
            //
            // TODO: we should free the memory in Rust to allow using standard
            // webidl bindings.
            NonstandardOutgoing::Vector {
                offset,
                length,
                kind,
            } => {
                let ptr = self.arg(*offset);
                let len = self.arg(*length);
                self.js.typescript_required(kind.js_ty());
                let f = self.cx.expose_get_vector_from_wasm(*kind)?;
                let i = self.js.tmp();
                self.js
                    .prelude(&format!("const v{} = {}({}, {}).slice();", i, f, ptr, len));
                self.prelude_free_vector(*offset, *length, *kind)?;
                Ok(format!("v{}", i))
            }

            NonstandardOutgoing::CachedString {
                offset,
                length,
                owned,
                optional,
            } => {
                let ptr = self.arg(*offset);
                let len = self.arg(*length);
                let tmp = self.js.tmp();

                if *optional {
                    self.js.typescript_optional("string");
                } else {
                    self.js.typescript_required("string");
                }

                self.cx.expose_get_cached_string_from_wasm()?;

                self.js.prelude(&format!(
                    "const v{} = getCachedStringFromWasm({}, {});",
                    tmp, ptr, len
                ));

                if *owned {
                    self.prelude_free_cached_string(&ptr, &len)?;
                }

                Ok(format!("v{}", tmp))
            }

            NonstandardOutgoing::StackClosure {
                a,
                b,
                binding_idx,
                nargs,
                mutable,
            } => {
                self.js.typescript_optional("any");
                let i = self.js.tmp();
                self.js.prelude(&format!(
                    "const state{} = {{a: {}, b: {}}};",
                    i,
                    self.arg(*a),
                    self.arg(*b),
                ));
                let args = (0..*nargs)
                    .map(|i| format!("arg{}", i))
                    .collect::<Vec<_>>()
                    .join(", ");
                if *mutable {
                    // Mutable closures need protection against being called
                    // recursively, so ensure that we clear out one of the
                    // internal pointers while it's being invoked.
                    self.js.prelude(&format!(
                        "const cb{i} = ({args}) => {{
                            const a = state{i}.a;
                            state{i}.a = 0;
                            try {{
                                return __wbg_elem_binding{idx}(a, state{i}.b, {args});
                            }} finally {{
                                state{i}.a = a;
                            }}
                        }};",
                        i = i,
                        args = args,
                        idx = binding_idx,
                    ));
                } else {
                    self.js.prelude(&format!(
                        "const cb{i} = ({args}) => __wbg_elem_binding{idx}(state{i}.a, state{i}.b, {args});",
                        i = i,
                        args = args,
                        idx = binding_idx,
                    ));
                }

                // Make sure to null out our internal pointers when we return
                // back to Rust to ensure that any lingering references to the
                // closure will fail immediately due to null pointers passed in
                // to Rust.
                self.js.finally(&format!("state{}.a = state{0}.b = 0;", i));
                Ok(format!("cb{}", i))
            }

            NonstandardOutgoing::OptionBool { idx } => {
                self.js.typescript_optional("boolean");
                Ok(format!(
                    "{0} === 0xFFFFFF ? undefined : {0} !== 0",
                    self.arg(*idx)
                ))
            }

            NonstandardOutgoing::OptionChar { idx } => {
                self.js.typescript_optional("string");
                Ok(format!(
                    "{0} === 0xFFFFFF ? undefined : String.fromCodePoint({0})",
                    self.arg(*idx)
                ))
            }

            NonstandardOutgoing::OptionIntegerEnum { idx, hole } => {
                self.js.typescript_optional("number");
                Ok(format!(
                    "{0} === {1} ? undefined : {0}",
                    self.arg(*idx),
                    hole
                ))
            }

            NonstandardOutgoing::OptionRustType { class, idx } => {
                self.cx.require_class_wrap(class);
                self.js.typescript_optional(class);
                Ok(format!(
                    "{0} === 0 ? undefined : {1}.__wrap({0})",
                    self.arg(*idx),
                    class,
                ))
            }

            NonstandardOutgoing::OptionU32Sentinel { idx } => {
                self.js.typescript_optional("number");
                Ok(format!(
                    "{0} === 0xFFFFFF ? undefined : {0}",
                    self.arg(*idx)
                ))
            }

            NonstandardOutgoing::OptionNative {
                signed,
                present,
                val,
            } => {
                self.js.typescript_optional("number");
                Ok(format!(
                    "{} === 0 ? undefined : {}{}",
                    self.arg(*present),
                    self.arg(*val),
                    if *signed { "" } else { " >>> 0" },
                ))
            }

            NonstandardOutgoing::OptionInt64 {
                present,
                _ignored,
                lo,
                hi,
                signed,
            } => {
                self.js.typescript_optional("BigInt");
                let f = if *signed {
                    self.cx.expose_int64_cvt_shim()
                } else {
                    self.cx.expose_uint64_cvt_shim()
                };
                let i = self.js.tmp();
                self.js.prelude(&format!(
                    "
                        u32CvtShim[0] = {low};
                        u32CvtShim[1] = {high};
                        const n{i} = {present} === 0 ? undefined : {f}[0];
                    ",
                    present = self.arg(*present),
                    low = self.arg(*lo),
                    high = self.arg(*hi),
                    f = f,
                    i = i,
                ));
                Ok(format!("n{}", i))
            }

            NonstandardOutgoing::OptionSlice {
                kind,
                offset,
                length,
            } => {
                let ptr = self.arg(*offset);
                let len = self.arg(*length);
                self.js.typescript_optional(kind.js_ty());
                let f = self.cx.expose_get_vector_from_wasm(*kind)?;
                Ok(format!(
                    "{ptr} === 0 ? undefined : {f}({ptr}, {len})",
                    ptr = ptr,
                    len = len,
                    f = f
                ))
            }

            NonstandardOutgoing::OptionVector {
                offset,
                length,
                kind,
            } => {
                let ptr = self.arg(*offset);
                let len = self.arg(*length);
                self.js.typescript_optional(kind.js_ty());
                let f = self.cx.expose_get_vector_from_wasm(*kind)?;
                let i = self.js.tmp();
                self.js.prelude(&format!("let v{};", i));
                self.js.prelude(&format!("if ({} !== 0) {{", ptr));
                self.js
                    .prelude(&format!("v{} = {}({}, {}).slice();", i, f, ptr, len));
                self.prelude_free_vector(*offset, *length, *kind)?;
                self.js.prelude("}");
                Ok(format!("v{}", i))
            }
        }
    }

    /// Evaluates the `standard` binding expression, returning the JS expression
    /// needed to evaluate the binding.
    fn standard(&mut self, standard: &ast::OutgoingBindingExpression) -> Result<String, Error> {
        match standard {
            ast::OutgoingBindingExpression::As(expr) => match expr.ty {
                ast::WebidlTypeRef::Scalar(ast::WebidlScalarType::Any) => {
                    self.js.typescript_required("any");
                    if self.cx.config.anyref {
                        Ok(self.arg(expr.idx))
                    } else {
                        self.cx.expose_take_object();
                        Ok(format!("takeObject({})", self.arg(expr.idx)))
                    }
                }
                ast::WebidlTypeRef::Scalar(ast::WebidlScalarType::Boolean) => {
                    self.js.typescript_required("boolean");
                    Ok(format!("{} !== 0", self.arg(expr.idx)))
                }
                ast::WebidlTypeRef::Scalar(ast::WebidlScalarType::UnsignedLong) => {
                    self.js.typescript_required("number");
                    Ok(format!("{} >>> 0", self.arg(expr.idx)))
                }
                _ => {
                    self.js.typescript_required("number");
                    Ok(self.arg(expr.idx))
                }
            },
            ast::OutgoingBindingExpression::View(view) => {
                // TODO: deduplicate with same match statement in incoming
                // bindings
                let scalar = match view.ty {
                    ast::WebidlTypeRef::Scalar(s) => s,
                    ast::WebidlTypeRef::Id(_) => {
                        bail!("unsupported type passed to `view` in webidl binding")
                    }
                };
                let kind = match scalar {
                    ast::WebidlScalarType::Int8Array => VectorKind::I8,
                    ast::WebidlScalarType::Uint8Array => VectorKind::U8,
                    ast::WebidlScalarType::Uint8ClampedArray => VectorKind::ClampedU8,
                    ast::WebidlScalarType::Int16Array => VectorKind::I16,
                    ast::WebidlScalarType::Uint16Array => VectorKind::U16,
                    ast::WebidlScalarType::Int32Array => VectorKind::I32,
                    ast::WebidlScalarType::Uint32Array => VectorKind::U32,
                    ast::WebidlScalarType::Float32Array => VectorKind::F32,
                    ast::WebidlScalarType::Float64Array => VectorKind::F64,
                    _ => bail!("unsupported type passed to `view`: {:?}", scalar),
                };
                self.js.typescript_required(kind.js_ty());
                let ptr = self.arg(view.offset);
                let len = self.arg(view.length);
                let f = self.cx.expose_get_vector_from_wasm(kind)?;
                Ok(format!("{}({}, {})", f, ptr, len))
            }

            ast::OutgoingBindingExpression::Utf8Str(expr) => {
                assert_eq!(expr.ty, ast::WebidlScalarType::DomString.into());
                self.js.typescript_required("string");
                let ptr = self.arg(expr.offset);
                let len = self.arg(expr.length);
                self.cx.expose_get_string_from_wasm()?;
                Ok(format!("getStringFromWasm({}, {})", ptr, len))
            }

            ast::OutgoingBindingExpression::Utf8CStr(_) => {
                bail!("unsupported `utf8-cstr` found in outgoing webidl bindings");
            }
            ast::OutgoingBindingExpression::I32ToEnum(_) => {
                bail!("unsupported `i32-to-enum` found in outgoing webidl bindings");
            }
            ast::OutgoingBindingExpression::Copy(_) => {
                bail!("unsupported `copy` found in outgoing webidl bindings");
            }
            ast::OutgoingBindingExpression::Dict(_) => {
                bail!("unsupported `dict` found in outgoing webidl bindings");
            }
            ast::OutgoingBindingExpression::BindExport(_) => {
                bail!("unsupported `bind-export` found in outgoing webidl bindings");
            }
        }
    }

    fn arg(&self, idx: u32) -> String {
        self.js.arg(idx).to_string()
    }

    fn prelude_free_vector(
        &mut self,
        offset: u32,
        length: u32,
        kind: VectorKind,
    ) -> Result<(), Error> {
        self.js.prelude(&format!(
            "wasm.__wbindgen_free({0}, {1} * {size});",
            self.arg(offset),
            self.arg(length),
            size = kind.size(),
        ));
        self.cx.require_internal_export("__wbindgen_free")
    }

    fn prelude_free_cached_string(&mut self, ptr: &str, len: &str) -> Result<(), Error> {
        self.js.prelude(&format!(
            "if ({ptr} !== 0) {{ wasm.__wbindgen_free({ptr}, {len}); }}",
            ptr = ptr,
            len = len,
        ));

        self.cx.require_internal_export("__wbindgen_free")
    }
}
