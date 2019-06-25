//! Implementation of taking a `NonstandardIncoming` binding and generating JS
//! which represents it and executes it for what we need.
//!
//! This module is used to generate JS for all our incoming bindings which
//! includes arguments going into exports or return values from imports.

use crate::descriptor::VectorKind;
use crate::js::binding::JsBuilder;
use crate::js::Context;
use crate::webidl::NonstandardIncoming;
use failure::{bail, Error};
use wasm_webidl_bindings::ast;

pub struct Incoming<'a, 'b> {
    cx: &'a mut Context<'b>,
    types: &'a [ast::WebidlTypeRef],
    js: &'a mut JsBuilder,
}

impl<'a, 'b> Incoming<'a, 'b> {
    pub fn new(
        cx: &'a mut Context<'b>,
        types: &'a [ast::WebidlTypeRef],
        js: &'a mut JsBuilder,
    ) -> Incoming<'a, 'b> {
        Incoming { cx, types, js }
    }

    pub fn process(&mut self, incoming: &NonstandardIncoming) -> Result<Vec<String>, Error> {
        let before = self.js.typescript_len();
        let ret = self.nonstandard(incoming)?;
        assert_eq!(before + 1, self.js.typescript_len());
        Ok(ret)
    }

    fn nonstandard(&mut self, incoming: &NonstandardIncoming) -> Result<Vec<String>, Error> {
        let single = match incoming {
            NonstandardIncoming::Standard(val) => return self.standard(val),

            // Evaluate the `val` binding, store it into a one-element `BigInt`
            // array (appropriately typed) and then use a 32-bit view into the
            // `BigInt` array to extract the high/low bits and pass them through
            // in the ABI.
            NonstandardIncoming::Int64 { val, signed } => {
                self.js.typescript_required("BigInt");
                let (expr, ty) = self.standard_typed(val)?;
                assert_eq!(ty, ast::WebidlScalarType::Any.into());
                let f = if *signed {
                    self.cx.expose_int64_cvt_shim()
                } else {
                    self.cx.expose_uint64_cvt_shim()
                };
                self.cx.expose_uint32_memory();
                let i = self.js.tmp();
                self.js.prelude(&format!(
                    "
                     {f}[0] = {expr};
                     const low{i} = u32CvtShim[0];
                     const high{i} = u32CvtShim[1];
                     ",
                    i = i,
                    f = f,
                    expr = expr,
                ));
                return Ok(vec![format!("low{}", i), format!("high{}", i)]);
            }

            // Same as `IncomingBindingExpressionAllocCopy`, except we use a
            // different `VectorKind`
            NonstandardIncoming::AllocCopyInt64 {
                alloc_func_name: _,
                expr,
                signed,
            } => {
                let (expr, ty) = self.standard_typed(expr)?;
                assert_eq!(ty, ast::WebidlScalarType::Any.into());
                let kind = if *signed {
                    VectorKind::I64
                } else {
                    VectorKind::U64
                };
                let func = self.cx.pass_to_wasm_function(kind)?;
                self.js.typescript_required(kind.js_ty());
                return Ok(vec![
                    format!("{}({})", func, expr),
                    "WASM_VECTOR_LEN".to_string(),
                ]);
            }

            // Same as `IncomingBindingExpressionAllocCopy`, except we use a
            // different `VectorKind`
            NonstandardIncoming::AllocCopyAnyrefArray {
                alloc_func_name: _,
                expr,
            } => {
                let (expr, ty) = self.standard_typed(expr)?;
                assert_eq!(ty, ast::WebidlScalarType::Any.into());
                let func = self.cx.pass_to_wasm_function(VectorKind::Anyref)?;
                self.js.typescript_required(VectorKind::Anyref.js_ty());
                return Ok(vec![
                    format!("{}({})", func, expr),
                    "WASM_VECTOR_LEN".to_string(),
                ]);
            }

            // There's no `char` in JS, so we take a string instead and just
            // forward along the first code point to Rust.
            NonstandardIncoming::Char { val } => {
                let (expr, ty) = self.standard_typed(val)?;
                assert_eq!(ty, ast::WebidlScalarType::DomString.into());
                self.js.typescript_required("string");
                format!("{}.codePointAt(0)", expr)
            }

            // When moving a type back into Rust we need to clear out the
            // internal pointer in JS to prevent it from being reused again in
            // the future.
            NonstandardIncoming::RustType { class, val } => {
                let (expr, ty) = self.standard_typed(val)?;
                assert_eq!(ty, ast::WebidlScalarType::Any.into());
                self.assert_class(&expr, &class);
                self.assert_not_moved(&expr);
                let i = self.js.tmp();
                self.js.prelude(&format!("const ptr{} = {}.ptr;", i, expr));
                self.js.prelude(&format!("{}.ptr = 0;", expr));
                self.js.typescript_required(class);
                format!("ptr{}", i)
            }

            // Here we can simply pass along the pointer with no extra fluff
            // needed.
            NonstandardIncoming::RustTypeRef { class, val } => {
                let (expr, ty) = self.standard_typed(val)?;
                assert_eq!(ty, ast::WebidlScalarType::Any.into());
                self.assert_class(&expr, &class);
                self.assert_not_moved(&expr);
                self.js.typescript_required(class);
                format!("{}.ptr", expr)
            }

            // the "stack-ful" nature means that we're always popping from the
            // stack, and make sure that we actually clear our reference to
            // allow stale values to get GC'd
            NonstandardIncoming::BorrowedAnyref { val } => {
                let (expr, ty) = self.standard_typed(val)?;
                assert_eq!(ty, ast::WebidlScalarType::Any.into());
                self.cx.expose_borrowed_objects();
                self.cx.expose_global_stack_pointer();
                self.js.finally("heap[stack_pointer++] = undefined;");
                self.js.typescript_required("any");
                format!("addBorrowedObject({})", expr)
            }

            // Similar to `AllocCopy`, except that we deallocate in a finally
            // block.
            NonstandardIncoming::MutableSlice { kind, val } => {
                let (expr, ty) = self.standard_typed(val)?;
                assert_eq!(ty, ast::WebidlScalarType::Any.into());
                let func = self.cx.pass_to_wasm_function(*kind)?;
                let i = self.js.tmp();
                self.js
                    .prelude(&format!("const ptr{} = {}({});", i, func, expr));
                self.js
                    .prelude(&format!("const len{} = WASM_VECTOR_LEN;", i));
                self.finally_free_slice(&expr, i, *kind, true)?;
                self.js.typescript_required(kind.js_ty());
                return Ok(vec![format!("ptr{}", i), format!("len{}", i)]);
            }

            // Pass `None` as a sentinel value that `val` will never take on.
            // This is only manufactured for specific underlying types.
            NonstandardIncoming::OptionU32Sentinel { val } => {
                let (expr, ty) = self.standard_typed(val)?;
                assert_eq!(ty, ast::WebidlScalarType::Any.into());
                self.cx.expose_is_like_none();
                self.js.typescript_optional("number");
                self.assert_optional_number(&expr);
                format!("isLikeNone({0}) ? 0xFFFFFF : {0}", expr)
            }

            // Pass `true` as 1, `false` as 0, and `None` as a sentinel value.
            NonstandardIncoming::OptionBool { val } => {
                let (expr, ty) = self.standard_typed(val)?;
                assert_eq!(ty, ast::WebidlScalarType::Any.into());
                self.cx.expose_is_like_none();
                self.js.typescript_optional("boolean");
                self.assert_optional_bool(&expr);
                format!("isLikeNone({0}) ? 0xFFFFFF : {0} ? 1 : 0", expr)
            }

            // Pass `None` as a sentinel value a character can never have
            NonstandardIncoming::OptionChar { val } => {
                let (expr, ty) = self.standard_typed(val)?;
                assert_eq!(ty, ast::WebidlScalarType::Any.into());
                self.cx.expose_is_like_none();
                self.js.typescript_optional("string");
                format!("isLikeNone({0}) ? 0xFFFFFF : {0}.codePointAt(0)", expr)
            }

            // Pass `None` as the hole in the enum which no valid value can ever
            // take
            NonstandardIncoming::OptionIntegerEnum { val, hole } => {
                let (expr, ty) = self.standard_typed(val)?;
                assert_eq!(ty, ast::WebidlScalarType::Any.into());
                self.cx.expose_is_like_none();
                self.js.typescript_optional("number");
                self.assert_optional_number(&expr);
                format!("isLikeNone({0}) ? {1} : {0}", expr, hole)
            }

            // `None` here is zero, but if `Some` then we need to clear out the
            // internal pointer because the value is being moved.
            NonstandardIncoming::OptionRustType { class, val } => {
                let (expr, ty) = self.standard_typed(val)?;
                assert_eq!(ty, ast::WebidlScalarType::Any.into());
                self.cx.expose_is_like_none();
                let i = self.js.tmp();
                self.js.prelude(&format!("let ptr{} = 0;", i));
                self.js.prelude(&format!("if (!isLikeNone({0})) {{", expr));
                self.assert_class(&expr, class);
                self.assert_not_moved(&expr);
                self.js.prelude(&format!("ptr{} = {}.ptr;", i, expr));
                self.js.prelude(&format!("{}.ptr = 0;", expr));
                self.js.prelude("}");
                self.js.typescript_optional(class);
                format!("ptr{}", i)
            }

            // The ABI produces four values here, all zero for `None` and 1 in
            // the first for the last two being the low/high bits
            NonstandardIncoming::OptionInt64 { val, signed } => {
                let (expr, ty) = self.standard_typed(val)?;
                assert_eq!(ty, ast::WebidlScalarType::Any.into());
                self.cx.expose_is_like_none();
                let f = if *signed {
                    self.cx.expose_int64_cvt_shim()
                } else {
                    self.cx.expose_uint64_cvt_shim()
                };
                self.cx.expose_uint32_memory();
                let i = self.js.tmp();
                self.js.prelude(&format!(
                    "\
                        {f}[0] = isLikeNone({expr}) ? BigInt(0) : {expr};
                        const low{i} = isLikeNone({expr}) ? 0 : u32CvtShim[0];
                        const high{i} = isLikeNone({expr}) ? 0 : u32CvtShim[1];
                    ",
                    i = i,
                    f = f,
                    expr = expr,
                ));
                self.js.typescript_optional("BigInt");
                return Ok(vec![
                    format!("!isLikeNone({0})", expr),
                    "0".to_string(),
                    format!("low{}", i),
                    format!("high{}", i),
                ]);
            }

            // The ABI here is always an integral index into the anyref table,
            // and the anyref table just differs based on whether we ran the
            // anyref pass or not.
            NonstandardIncoming::OptionAnyref { val } => {
                let (expr, ty) = self.standard_typed(val)?;
                assert_eq!(ty, ast::WebidlScalarType::Any.into());
                self.cx.expose_is_like_none();
                self.js.typescript_optional("any");
                if self.cx.config.anyref {
                    self.cx.expose_add_to_anyref_table()?;
                    format!("isLikeNone({0}) ? 0 : addToAnyrefTable({0})", expr)
                } else {
                    self.cx.expose_add_heap_object();
                    format!("isLikeNone({0}) ? 0 : addHeapObject({0})", expr)
                }
            }

            // Native types of wasm take a leading discriminant to indicate
            // whether the next value is valid or not.
            NonstandardIncoming::OptionNative { val } => {
                let (expr, ty) = self.standard_typed(val)?;
                assert_eq!(ty, ast::WebidlScalarType::Any.into());
                self.cx.expose_is_like_none();
                self.js.typescript_optional("number");
                self.assert_optional_number(&expr);
                return Ok(vec![
                    format!("!isLikeNone({0})", expr),
                    format!("isLikeNone({0}) ? 0 : {0}", expr),
                ]);
            }

            // Similar to `AllocCopy`, except we're handling the undefined case
            // and passing null for the pointer value.
            NonstandardIncoming::OptionVector { kind, val } => {
                let (expr, ty) = self.standard_typed(val)?;
                assert_eq!(ty, ast::WebidlScalarType::Any.into());
                let func = self.cx.pass_to_wasm_function(*kind)?;
                self.cx.expose_is_like_none();
                let i = self.js.tmp();
                self.js.prelude(&format!(
                    "const ptr{i} = isLikeNone({0}) ? 0 : {f}({0});",
                    expr,
                    i = i,
                    f = func,
                ));
                self.js
                    .prelude(&format!("const len{} = WASM_VECTOR_LEN;", i));
                self.js.typescript_optional(kind.js_ty());
                return Ok(vec![format!("ptr{}", i), format!("len{}", i)]);
            }

            // An unfortunate smorgasboard of handling slices, transfers if
            // mutable, etc. Not the prettiest binding option here, and of
            // course never going to be standardized.
            NonstandardIncoming::OptionSlice { kind, val, mutable } => {
                let (expr, ty) = self.standard_typed(val)?;
                assert_eq!(ty, ast::WebidlScalarType::Any.into());
                let func = self.cx.pass_to_wasm_function(*kind)?;
                self.cx.expose_is_like_none();
                let i = self.js.tmp();
                self.js.prelude(&format!(
                    "const ptr{i} = isLikeNone({0}) ? 0 : {f}({0});",
                    expr,
                    i = i,
                    f = func,
                ));
                self.js
                    .prelude(&format!("const len{} = WASM_VECTOR_LEN;", i));
                self.js.finally(&format!("if (ptr{} !== 0) {{", i));
                self.finally_free_slice(&expr, i, *kind, *mutable)?;
                self.js.finally("}");
                self.js.typescript_optional(kind.js_ty());
                return Ok(vec![format!("ptr{}", i), format!("len{}", i)]);
            }
        };
        Ok(vec![single])
    }

    /// Evaluates the `standard` binding expression, returning the JS expression
    /// needed to evaluate the binding.
    fn standard(
        &mut self,
        standard: &ast::IncomingBindingExpression,
    ) -> Result<Vec<String>, Error> {
        let single = match standard {
            ast::IncomingBindingExpression::As(as_) => {
                let (expr, ty) = self.standard_typed(&as_.expr)?;
                match ty {
                    ast::WebidlTypeRef::Scalar(ast::WebidlScalarType::Any) => {
                        self.js.typescript_required("any");

                        // If the type here is anyref but we didn't run the
                        // anyref pass that means we have to instead actually
                        // pass in an index
                        //
                        // TODO: we should ideally move this `addHeapObject`
                        // into a nonstanard binding whenever the anyref pass
                        // doesn't already run rather than implicitly picking
                        // it up here
                        if self.cx.config.anyref {
                            expr
                        } else {
                            self.cx.expose_add_heap_object();
                            format!("addHeapObject({})", expr)
                        }
                    }
                    ast::WebidlTypeRef::Scalar(ast::WebidlScalarType::Boolean) => {
                        self.js.typescript_required("boolean");
                        self.assert_bool(&expr);
                        // JS will already coerce booleans into numbers for us
                        expr
                    }
                    _ => {
                        self.js.typescript_required("number");
                        self.assert_number(&expr);
                        expr
                    }
                }
            }
            ast::IncomingBindingExpression::Get(_) => {
                bail!("unsupported bare `get` in webidl bindings");
            }
            ast::IncomingBindingExpression::AllocUtf8Str(expr) => {
                let (expr, ty) = self.standard_typed(&expr.expr)?;
                assert_eq!(ty, ast::WebidlScalarType::DomString.into());
                self.js.typescript_required("string");
                self.cx.expose_pass_string_to_wasm()?;
                return Ok(vec![
                    format!("passStringToWasm({})", expr),
                    "WASM_VECTOR_LEN".to_string(),
                ]);
            }
            ast::IncomingBindingExpression::AllocCopy(expr) => {
                let (expr, ty) = self.standard_typed(&expr.expr)?;
                let scalar = match ty {
                    ast::WebidlTypeRef::Scalar(s) => s,
                    ast::WebidlTypeRef::Id(_) => {
                        bail!("unsupported type passed to `alloc-copy` in webidl binding")
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
                    _ => bail!("unsupported type passed to alloc-copy: {:?}", scalar),
                };
                self.js.typescript_required(kind.js_ty());
                let func = self.cx.pass_to_wasm_function(kind)?;
                return Ok(vec![
                    format!("{}({})", func, expr),
                    "WASM_VECTOR_LEN".to_string(),
                ]);
            }
            ast::IncomingBindingExpression::EnumToI32(_) => {
                bail!("unsupported enum-to-i32 conversion in webidl binding");
            }
            ast::IncomingBindingExpression::Field(_) => {
                bail!("unsupported field accessor in webidl binding");
            }
            ast::IncomingBindingExpression::BindImport(_) => {
                bail!("unsupported import binding in webidl binding");
            }
        };
        Ok(vec![single])
    }

    /// Evaluates the `standard` binding expression, returning both the
    /// JS expression to evaluate along with the WebIDL type of the expression.
    ///
    /// Currently only supports `Get`.
    fn standard_typed(
        &mut self,
        standard: &ast::IncomingBindingExpression,
    ) -> Result<(String, ast::WebidlTypeRef), Error> {
        match standard {
            ast::IncomingBindingExpression::As(_) => {
                bail!("unsupported as in webidl binding");
            }
            ast::IncomingBindingExpression::Get(expr) => {
                let arg = self.js.arg(expr.idx).to_string();
                let ty = self.types[expr.idx as usize];
                Ok((arg, ty))
            }
            ast::IncomingBindingExpression::AllocUtf8Str(_) => {
                bail!("unsupported alloc-utf8-str in webidl binding");
            }
            ast::IncomingBindingExpression::AllocCopy(_) => {
                bail!("unsupported alloc-copy in webidl binding");
            }
            ast::IncomingBindingExpression::EnumToI32(_) => {
                bail!("unsupported enum-to-i32 in webidl binding");
            }
            ast::IncomingBindingExpression::Field(_) => {
                bail!("unsupported field accessor in webidl binding");
            }
            ast::IncomingBindingExpression::BindImport(_) => {
                bail!("unsupported import binding in webidl binding");
            }
        }
    }

    fn assert_class(&mut self, arg: &str, class: &str) {
        self.cx.expose_assert_class();
        self.js
            .prelude(&format!("_assertClass({}, {});", arg, class));
    }

    fn assert_number(&mut self, arg: &str) {
        if !self.cx.config.debug {
            return;
        }
        self.cx.expose_assert_num();
        self.js.prelude(&format!("_assertNum({});", arg));
    }

    fn assert_bool(&mut self, arg: &str) {
        if !self.cx.config.debug {
            return;
        }
        self.cx.expose_assert_bool();
        self.js.prelude(&format!("_assertBoolean({});", arg));
    }

    fn assert_optional_number(&mut self, arg: &str) {
        if !self.cx.config.debug {
            return;
        }
        self.cx.expose_is_like_none();
        self.js.prelude(&format!("if (!isLikeNone({})) {{", arg));
        self.assert_number(arg);
        self.js.prelude("}");
    }

    fn assert_optional_bool(&mut self, arg: &str) {
        if !self.cx.config.debug {
            return;
        }
        self.cx.expose_is_like_none();
        self.js.prelude(&format!("if (!isLikeNone({})) {{", arg));
        self.assert_bool(arg);
        self.js.prelude("}");
    }

    fn assert_not_moved(&mut self, arg: &str) {
        if !self.cx.config.debug {
            return;
        }
        self.js.prelude(&format!(
            "\
                if ({0}.ptr === 0) {{
                    throw new Error('Attempt to use a moved value');
                }}
            ",
            arg,
        ));
    }

    fn finally_free_slice(
        &mut self,
        expr: &str,
        i: usize,
        kind: VectorKind,
        mutable: bool,
    ) -> Result<(), Error> {
        // If the slice was mutable it's currently a feature that we
        // mirror back updates to the original slice. This... is
        // arguably a misfeature of wasm-bindgen...
        if mutable {
            let get = self.cx.memview_function(kind);
            self.js.finally(&format!(
                "\
                 {arg}.set({get}().subarray(\
                 ptr{i} / {size}, \
                 ptr{i} / {size} + len{i}\
                 ));\
                 ",
                i = i,
                arg = expr,
                get = get,
                size = kind.size()
            ));
        }
        self.js.finally(&format!(
            "wasm.__wbindgen_free(ptr{i}, len{i} * {size});",
            i = i,
            size = kind.size(),
        ));
        self.cx.require_internal_export("__wbindgen_free")
    }
}
