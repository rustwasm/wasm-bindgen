use std::iter::FromIterator;

use backend;
use backend::util::{ident_ty, leading_colon_path_ty, raw_ident, rust_ident};
use heck::{CamelCase, SnakeCase};
use proc_macro2::Ident;
use syn;
use weedle;
use weedle::attribute::{ExtendedAttributeList, ExtendedAttribute};
use weedle::argument::{Argument, SingleArgument};
use weedle::common::Identifier;
use weedle::types::*;
use weedle::literal::{ConstValue, FloatLit, IntegerLit};

use first_pass::FirstPassRecord;

/// Take a type and create an immutable shared reference to that type.
fn shared_ref(ty: syn::Type) -> syn::Type {
    syn::TypeReference {
        and_token: Default::default(),
        lifetime: None,
        mutability: None,
        elem: Box::new(ty),
    }.into()
}

/// Fix camelcase of identifiers like HTMLBRElement
pub fn camel_case_ident(identifier: &str) -> String {
  identifier.replace("HTML", "HTML_").to_camel_case()
}

// Returns a link to MDN
pub fn mdn_doc(class: &str, method: Option<&str>) -> String {
    let mut link = format!("https://developer.mozilla.org/en-US/docs/Web/API/{}", class);
    if let Some(method) = method {
        link.push_str(&format!("/{}", method));
    }
    format!("[Documentation]({})", link).into()
}

pub(crate) trait ToSynType<'src> {
    fn to_syn_type(&self, record: &FirstPassRecord<'src>, pos: TypePosition)
        -> Option<syn::Type>;
}

impl<'src, T: ToSynType<'src>> ToSynType<'src> for MayBeNull<T> {
    fn to_syn_type(&self, record: &FirstPassRecord<'src>, pos: TypePosition)
        -> Option<syn::Type>
    {
        let ty = self.type_.to_syn_type(record, pos)?;
        if self.q_mark.is_some() {
            Some(option_ty(ty))
        } else {
            Some(ty)
        }
    }
}

impl<'src> ToSynType<'src> for ConstType<'src> {
    fn to_syn_type(&self, record: &FirstPassRecord<'src>, pos: TypePosition)
        -> Option<syn::Type>
    {
        match self {
            ConstType::Integer(l) => l.to_syn_type(record, pos),
            ConstType::FloatingPoint(l) => l.to_syn_type(record, pos),
            ConstType::Boolean(l) => l.to_syn_type(record, pos),
            ConstType::Byte(l) => l.to_syn_type(record, pos),
            ConstType::Octet(l) => l.to_syn_type(record, pos),
            ConstType::Identifier(l) => l.to_syn_type(record, pos),
        }
    }
}

impl<'src> ToSynType<'src> for IntegerType {
    fn to_syn_type(&self, record: &FirstPassRecord<'src>, pos: TypePosition)
        -> Option<syn::Type>
    {
        match self {
            IntegerType::LongLong(l) => l.to_syn_type(record, pos),
            IntegerType::Long(l) => l.to_syn_type(record, pos),
            IntegerType::Short(l) => l.to_syn_type(record, pos),
        }
    }
}

impl<'src> ToSynType<'src> for ShortType {
    fn to_syn_type(&self, _record: &FirstPassRecord<'src>, _pos: TypePosition)
        -> Option<syn::Type>
    {
        if self.unsigned.is_some() {
            Some(ident_ty(raw_ident("u16")))
        } else {
            Some(ident_ty(raw_ident("i16")))
        }
    }
}

impl<'src> ToSynType<'src> for LongType {
    fn to_syn_type(&self, _record: &FirstPassRecord<'src>, _pos: TypePosition)
        -> Option<syn::Type>
    {
        if self.unsigned.is_some() {
            Some(ident_ty(raw_ident("u32")))
        } else {
            Some(ident_ty(raw_ident("i32")))
        }
    }
}

impl<'src> ToSynType<'src> for LongLongType {
    fn to_syn_type(&self, _record: &FirstPassRecord<'src>, _pos: TypePosition)
        -> Option<syn::Type>
    {
        if self.unsigned.is_some() {
            Some(ident_ty(raw_ident("u64")))
        } else {
            Some(ident_ty(raw_ident("i64")))
        }
    }
}

impl<'src> ToSynType<'src> for FloatingPointType {
    fn to_syn_type(&self, _record: &FirstPassRecord<'src>, _pos: TypePosition)
        -> Option<syn::Type>
    {
        match self {
            FloatingPointType::Float(_) => Some(ident_ty(raw_ident("f32"))),
            FloatingPointType::Double(_) => Some(ident_ty(raw_ident("f64"))),
        }
    }
}

impl<'src> ToSynType<'src> for weedle::term::Boolean {
    fn to_syn_type(&self, _record: &FirstPassRecord<'src>, _pos: TypePosition)
        -> Option<syn::Type>
    {
        Some(ident_ty(raw_ident("bool")))
    }
}

impl<'src> ToSynType<'src> for weedle::term::Byte {
    fn to_syn_type(&self, _record: &FirstPassRecord<'src>, _pos: TypePosition)
        -> Option<syn::Type>
    {
        Some(ident_ty(raw_ident("i8")))
    }
}

impl<'src> ToSynType<'src> for weedle::term::Octet {
    fn to_syn_type(&self, _record: &FirstPassRecord<'src>, _pos: TypePosition)
        -> Option<syn::Type>
    {
        Some(ident_ty(raw_ident("u8")))
    }
}

impl<'src> ToSynType<'src> for weedle::common::Identifier<'src> {
    fn to_syn_type(&self, record: &FirstPassRecord<'src>, pos: TypePosition)
        -> Option<syn::Type>
    {
        if let Some(other) = record.typedefs.get(&self.0) {
            return other.to_syn_type(record, pos)
        }
        // A reference to a type by name becomes the same thing in the
        // bindings.
        let ty = ident_ty(rust_ident(camel_case_ident(self.0).as_str()));
        Some(if record.interfaces.contains_key(self.0) {
            if pos == TypePosition::Argument {
                shared_ref(ty)
            } else {
                ty
            }
        } else if record.dictionaries.contains(self.0) {
            ty
        } else if record.enums.contains(self.0) {
            ty
        } else {
            warn!("unrecognized type {}", self.0);
            ty
        })
    }
}

impl<'src> ToSynType<'src> for weedle::term::DOMString {
    fn to_syn_type(&self, _record: &FirstPassRecord<'src>, pos: TypePosition)
        -> Option<syn::Type>
    {
        // strings -> `&str` for arguments and `String` for return
        Some(match pos {
            TypePosition::Argument => shared_ref(ident_ty(raw_ident("str"))),
            TypePosition::Return => ident_ty(raw_ident("String")),
        })
    }
}

impl<'src> ToSynType<'src> for weedle::term::ByteString {
    fn to_syn_type(&self, record: &FirstPassRecord<'src>, pos: TypePosition)
        -> Option<syn::Type>
    {
        // ByteString maps to String in JS -
        // https://developer.mozilla.org/en-US/docs/Web/API/ByteString
        weedle::term::DOMString.to_syn_type(record, pos)
    }
}

impl<'src> ToSynType<'src> for weedle::term::USVString {
    fn to_syn_type(&self, record: &FirstPassRecord<'src>, pos: TypePosition)
        -> Option<syn::Type>
    {
        // USVString maps to String in JS -
        // https://developer.mozilla.org/en-US/docs/Web/API/USVString
        weedle::term::DOMString.to_syn_type(record, pos)
    }
}

// Array type is borrowed for arguments (`&[T]`) and owned for return value (`Vec<T>`).
fn array(base_ty: &str, pos: TypePosition) -> syn::Type {
    match pos {
        TypePosition::Argument => {
            shared_ref(slice_ty(ident_ty(raw_ident(base_ty))))
        }
        TypePosition::Return => {
            vec_ty(ident_ty(raw_ident(base_ty)))
        }
    }
}

impl<'src> ToSynType<'src> for weedle::term::Float32Array {
    fn to_syn_type(&self, _record: &FirstPassRecord<'src>, pos: TypePosition)
        -> Option<syn::Type>
    {
        Some(array("f32", pos))
    }
}

impl<'src> ToSynType<'src> for weedle::term::Float64Array {
    fn to_syn_type(&self, _record: &FirstPassRecord<'src>, pos: TypePosition)
        -> Option<syn::Type>
    {
        Some(array("f64", pos))
    }
}

impl<'src> ToSynType<'src> for weedle::term::Int8Array {
    fn to_syn_type(&self, _record: &FirstPassRecord<'src>, pos: TypePosition)
        -> Option<syn::Type>
    {
        Some(array("i8", pos))
    }
}

impl<'src> ToSynType<'src> for weedle::term::Int16Array {
    fn to_syn_type(&self, _record: &FirstPassRecord<'src>, pos: TypePosition)
        -> Option<syn::Type>
    {
        Some(array("i16", pos))
    }
}

impl<'src> ToSynType<'src> for weedle::term::Int32Array {
    fn to_syn_type(&self, _record: &FirstPassRecord<'src>, pos: TypePosition)
        -> Option<syn::Type>
    {
        Some(array("i32", pos))
    }
}

impl<'src> ToSynType<'src> for weedle::term::Uint8Array {
    fn to_syn_type(&self, _record: &FirstPassRecord<'src>, pos: TypePosition)
        -> Option<syn::Type>
    {
        Some(array("u8", pos))
    }
}

impl<'src> ToSynType<'src> for weedle::term::Uint8ClampedArray {
    fn to_syn_type(&self, _record: &FirstPassRecord<'src>, pos: TypePosition)
        -> Option<syn::Type>
    {
        Some(array("u8", pos))
    }
}

impl<'src> ToSynType<'src> for weedle::term::Uint16Array {
    fn to_syn_type(&self, _record: &FirstPassRecord<'src>, pos: TypePosition)
        -> Option<syn::Type>
    {
        Some(array("u16", pos))
    }
}

impl<'src> ToSynType<'src> for weedle::term::Uint32Array {
    fn to_syn_type(&self, _record: &FirstPassRecord<'src>, pos: TypePosition)
        -> Option<syn::Type>
    {
        Some(array("u32", pos))
    }
}

impl<'src> ToSynType<'src> for weedle::term::ArrayBuffer {
    fn to_syn_type(&self, _record: &FirstPassRecord<'src>, _pos: TypePosition)
        -> Option<syn::Type>
    {
        let path = vec![rust_ident("js_sys"), rust_ident("ArrayBuffer")];
        Some(leading_colon_path_ty(path))
    }
}

impl<'src> ToSynType<'src> for weedle::term::Object {
    fn to_syn_type(&self, _record: &FirstPassRecord<'src>, _pos: TypePosition)
        -> Option<syn::Type>
    {
        let path = vec![rust_ident("js_sys"), rust_ident("Object")];
        Some(leading_colon_path_ty(path))
    }
}

impl<'src> ToSynType<'src> for weedle::types::Type<'src> {
    fn to_syn_type(&self, record: &FirstPassRecord<'src>, pos: TypePosition)
        -> Option<syn::Type>
    {
        use weedle::types::NonAnyType::*;
        let single = match self {
            Type::Single(s) => s,
            Type::Union(_) => return None,
        };

        let ty = match single {
            // `any` becomes `::wasm_bindgen::JsValue`.
            SingleType::Any(_) => {
                let path = vec![rust_ident("wasm_bindgen"), rust_ident("JsValue")];
                return Some(leading_colon_path_ty(path))
            }
            SingleType::NonAny(other) => other,
        };

        match ty {
            Boolean(s) => s.to_syn_type(record, pos),
            Octet(s) => s.to_syn_type(record, pos),
            Byte(s) => s.to_syn_type(record, pos),
            Identifier(s) => s.to_syn_type(record, pos),
            Integer(s) => s.to_syn_type(record, pos),
            FloatingPoint(s) => s.to_syn_type(record, pos),

            Float32Array(s) => s.to_syn_type(record, pos),
            Float64Array(s) => s.to_syn_type(record, pos),
            Int8Array(s) => s.to_syn_type(record, pos),
            Int16Array(s) => s.to_syn_type(record, pos),
            Int32Array(s) => s.to_syn_type(record, pos),
            Uint8Array(s) => s.to_syn_type(record, pos),
            Uint8ClampedArray(s) => s.to_syn_type(record, pos),
            Uint16Array(s) => s.to_syn_type(record, pos),
            Uint32Array(s) => s.to_syn_type(record, pos),

            DOMString(s) => s.to_syn_type(record, pos),
            ByteString(s) => s.to_syn_type(record, pos),
            USVString(s) => s.to_syn_type(record, pos),
            ArrayBuffer(b) => b.to_syn_type(record, pos),
            Object(o) => o.to_syn_type(record, pos),

            // Support for these types is not yet implemented, so skip
            // generating any bindings for this function.
            | DataView(_)
            | Error(_)
            | FrozenArrayType(_)
            | Promise(_)
            | RecordType(..)
            | Sequence(_)
            | Symbol(_) => {
                None
            }
        }
    }
}


/// Map a webidl const value to the correct wasm-bindgen const value
pub fn webidl_const_v_to_backend_const_v(v: &ConstValue) -> backend::ast::ConstValue {
    use std::f64::{NEG_INFINITY, INFINITY, NAN};
    use backend::ast;

    match *v {
        ConstValue::Boolean(b) => ast::ConstValue::BooleanLiteral(b.0),
        ConstValue::Float(FloatLit::NegInfinity(_)) => {
            ast::ConstValue::FloatLiteral(NEG_INFINITY)
        }
        ConstValue::Float(FloatLit::Infinity(_)) => {
            ast::ConstValue::FloatLiteral(INFINITY)
        }
        ConstValue::Float(FloatLit::NaN(_)) => {
            ast::ConstValue::FloatLiteral(NAN)
        }
        ConstValue::Float(FloatLit::Value(s)) => {
            ast::ConstValue::FloatLiteral(s.0.parse().unwrap())
        }
        ConstValue::Integer(lit) => {
            let mklit = |orig_text: &str, base: u32, offset: usize| {
                let (negative, text) = if orig_text.starts_with("-") {
                    (true, &orig_text[1..])
                } else {
                    (false, orig_text)
                };
                if text == "0" {
                    return ast::ConstValue::SignedIntegerLiteral(0)
                }
                let text = &text[offset..];
                let n = u64::from_str_radix(text, base)
                    .unwrap_or_else(|_| panic!("literal too big: {}", orig_text));
                if negative {
                    let n = if n > (i64::min_value() as u64).wrapping_neg() {
                        panic!("literal too big: {}", orig_text)
                    } else {
                        n.wrapping_neg() as i64
                    };
                    ast::ConstValue::SignedIntegerLiteral(n)
                } else {
                    ast::ConstValue::UnsignedIntegerLiteral(n)
                }
            };
            match lit {
                IntegerLit::Hex(h) => mklit(h.0, 16, 2), // leading 0x
                IntegerLit::Oct(h) => mklit(h.0, 8, 1), // leading 0
                IntegerLit::Dec(h) => mklit(h.0, 10, 0),
            }
        }
        ConstValue::Null(_) => ast::ConstValue::Null,
    }
}

/// From `ident` and `Ty`, create `ident: Ty` for use in e.g. `fn(ident: Ty)`.
fn simple_fn_arg(ident: Ident, ty: syn::Type) -> syn::ArgCaptured {
    syn::ArgCaptured {
        pat: syn::Pat::Ident(syn::PatIdent {
            by_ref: None,
            mutability: None,
            ident,
            subpat: None,
        }),
        colon_token: Default::default(),
        ty,
    }
}

/// Create `()`.
fn unit_ty() -> syn::Type {
    syn::Type::Tuple(syn::TypeTuple {
        paren_token: Default::default(),
        elems: syn::punctuated::Punctuated::new(),
    })
}

/// From `T` create `Result<T, ::wasm_bindgen::JsValue>`.
fn result_ty(t: syn::Type) -> syn::Type {
    let js_value = leading_colon_path_ty(vec![rust_ident("wasm_bindgen"), rust_ident("JsValue")]);

    let arguments = syn::PathArguments::AngleBracketed(syn::AngleBracketedGenericArguments {
        colon2_token: None,
        lt_token: Default::default(),
        args: FromIterator::from_iter(vec![
            syn::GenericArgument::Type(t),
            syn::GenericArgument::Type(js_value),
        ]),
        gt_token: Default::default(),
    });

    let ident = raw_ident("Result");
    let seg = syn::PathSegment { ident, arguments };
    let path: syn::Path = seg.into();
    let ty = syn::TypePath { qself: None, path };
    ty.into()
}

/// From `T` create `[T]`.
fn slice_ty(t: syn::Type) -> syn::Type {
    syn::TypeSlice {
        bracket_token: Default::default(),
        elem: Box::new(t),
    }.into()
}

/// From `T` create `Vec<T>`.
fn vec_ty(t: syn::Type) -> syn::Type {
    let arguments = syn::PathArguments::AngleBracketed(syn::AngleBracketedGenericArguments {
        colon2_token: None,
        lt_token: Default::default(),
        args: FromIterator::from_iter(vec![
            syn::GenericArgument::Type(t),
        ]),
        gt_token: Default::default(),
    });

    let ident = raw_ident("Vec");
    let seg = syn::PathSegment { ident, arguments };
    let path: syn::Path = seg.into();
    let ty = syn::TypePath { qself: None, path };
    ty.into()
}

/// From `T` create `Option<T>`
fn option_ty(t: syn::Type) -> syn::Type {
    let arguments = syn::PathArguments::AngleBracketed(syn::AngleBracketedGenericArguments {
        colon2_token: None,
        lt_token: Default::default(),
        args: FromIterator::from_iter(vec![syn::GenericArgument::Type(t)]),
        gt_token: Default::default(),
    });

    let ident = raw_ident("Option");
    let seg = syn::PathSegment { ident, arguments };
    let path: syn::Path = seg.into();
    let ty = syn::TypePath { qself: None, path };
    ty.into()
}

/// Possible positions for a type in a function signature.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum TypePosition {
    Argument,
    Return,
}

/// Implemented on an AST type node to generate a snake case name.
trait TypeToString {
    fn type_to_string(&self, record: &FirstPassRecord, dst: &mut String);
}

impl<T: TypeToString> TypeToString for MayBeNull<T> {
    fn type_to_string(&self, record: &FirstPassRecord, dst: &mut String) {
        if self.q_mark.is_some() {
            dst.push_str("opt_");
        }
        self.type_.type_to_string(record, dst);
    }
}

impl<'src> TypeToString for weedle::types::ReturnType<'src> {
    fn type_to_string(&self, record: &FirstPassRecord, dst: &mut String) {
        match self {
            weedle::types::ReturnType::Type(ty) => (*ty).type_to_string(record, dst),
            weedle::types::ReturnType::Void(_) => dst.push_str("void"),
        }
    }
}

impl TypeToString for weedle::types::StringType {
    fn type_to_string(&self, _record: &FirstPassRecord, dst: &mut String) {
        match self {
            weedle::types::StringType::Byte(_) => dst.push_str("byte_str"),
            weedle::types::StringType::DOM(_) => dst.push_str("dom_str"),
            weedle::types::StringType::USV(_) => dst.push_str("usv_str"),
        }
    }
}

impl TypeToString for weedle::term::Byte {
    fn type_to_string(&self, _record: &FirstPassRecord, dst: &mut String) {
        dst.push_str("i8");
    }
}

impl TypeToString for weedle::term::Octet {
    fn type_to_string(&self, _record: &FirstPassRecord, dst: &mut String) {
        dst.push_str("u8");
    }
}

impl TypeToString for weedle::term::Boolean {
    fn type_to_string(&self, _record: &FirstPassRecord, dst: &mut String) {
        dst.push_str("bool");
    }
}

impl TypeToString for weedle::term::USVString {
    fn type_to_string(&self, _record: &FirstPassRecord, dst: &mut String) {
        dst.push_str("usv_str");
    }
}

impl TypeToString for weedle::term::ByteString {
    fn type_to_string(&self, _record: &FirstPassRecord, dst: &mut String) {
        dst.push_str("byte_str");
    }
}

impl TypeToString for weedle::term::DOMString {
    fn type_to_string(&self, _record: &FirstPassRecord, dst: &mut String) {
        dst.push_str("dom_str");
    }
}

impl TypeToString for weedle::term::Float32Array {
    fn type_to_string(&self, _record: &FirstPassRecord, dst: &mut String) {
        dst.push_str("f32_array");
    }
}

impl TypeToString for weedle::term::Float64Array {
    fn type_to_string(&self, _record: &FirstPassRecord, dst: &mut String) {
        dst.push_str("f64_array");
    }
}

impl TypeToString for weedle::term::Int8Array {
    fn type_to_string(&self, _record: &FirstPassRecord, dst: &mut String) {
        dst.push_str("i8_array");
    }
}

impl TypeToString for weedle::term::Int16Array {
    fn type_to_string(&self, _record: &FirstPassRecord, dst: &mut String) {
        dst.push_str("i16_array");
    }
}

impl TypeToString for weedle::term::Int32Array {
    fn type_to_string(&self, _record: &FirstPassRecord, dst: &mut String) {
        dst.push_str("i32_array");
    }
}

impl TypeToString for weedle::term::Uint8Array {
    fn type_to_string(&self, _record: &FirstPassRecord, dst: &mut String) {
        dst.push_str("u8_array");
    }
}

impl TypeToString for weedle::term::Uint8ClampedArray {
    fn type_to_string(&self, _record: &FirstPassRecord, dst: &mut String) {
        dst.push_str("u8_clamped_array");
    }
}

impl TypeToString for weedle::term::Uint16Array {
    fn type_to_string(&self, _record: &FirstPassRecord, dst: &mut String) {
        dst.push_str("u16_array");
    }
}

impl TypeToString for weedle::term::Uint32Array {
    fn type_to_string(&self, _record: &FirstPassRecord, dst: &mut String) {
        dst.push_str("u32_array");
    }
}

impl<'src> TypeToString for weedle::common::Identifier<'src> {
    fn type_to_string(&self, record: &FirstPassRecord, dst: &mut String) {
        match record.typedefs.get(self.0) {
            Some(other) => other.type_to_string(record, dst),
            None => dst.push_str(&self.0.to_snake_case()),
        }
    }
}

impl TypeToString for IntegerType {
    fn type_to_string(&self, _record: &FirstPassRecord, dst: &mut String) {
        match self {
            IntegerType::LongLong(l) if l.unsigned.is_some() => dst.push_str("u64"),
            IntegerType::LongLong(_) => dst.push_str("i64"),
            IntegerType::Long(l) if l.unsigned.is_some() => dst.push_str("u32"),
            IntegerType::Long(_) => dst.push_str("i32"),
            IntegerType::Short(l) if l.unsigned.is_some() => dst.push_str("u16"),
            IntegerType::Short(_) => dst.push_str("i16"),
        }
    }
}

impl TypeToString for FloatingPointType {
    fn type_to_string(&self, _record: &FirstPassRecord, dst: &mut String) {
        match self {
            FloatingPointType::Float(_) => dst.push_str("f32"),
            FloatingPointType::Double(_) => dst.push_str("f64"),
        }
    }
}

impl TypeToString for weedle::term::ArrayBuffer {
    fn type_to_string(&self, _record: &FirstPassRecord, dst: &mut String) {
        dst.push_str("array_buffer");
    }
}

impl TypeToString for weedle::term::Symbol {
    fn type_to_string(&self, _record: &FirstPassRecord, dst: &mut String) {
        dst.push_str("symbol");
    }
}

impl TypeToString for weedle::term::Object {
    fn type_to_string(&self, _record: &FirstPassRecord, dst: &mut String) {
        dst.push_str("object");
    }
}

impl TypeToString for weedle::term::DataView {
    fn type_to_string(&self, _record: &FirstPassRecord, dst: &mut String) {
        dst.push_str("data_view");
    }
}

impl TypeToString for weedle::term::Error {
    fn type_to_string(&self, _record: &FirstPassRecord, dst: &mut String) {
        dst.push_str("error");
    }
}

impl<'src> TypeToString for weedle::types::SequenceType<'src> {
    fn type_to_string(&self, record: &FirstPassRecord, dst: &mut String) {
        dst.push_str("seq_");
        self.generics.body.type_to_string(record, dst);
    }
}

impl<'src> TypeToString for weedle::types::PromiseType<'src> {
    fn type_to_string(&self, record: &FirstPassRecord, dst: &mut String) {
        dst.push_str("promise_");
        self.generics.body.type_to_string(record, dst);
    }
}

impl<'src> TypeToString for weedle::types::FrozenArrayType<'src> {
    fn type_to_string(&self, record: &FirstPassRecord, dst: &mut String) {
        dst.push_str("frozen_array_");
        self.generics.body.type_to_string(record, dst);
    }
}

impl<'src> TypeToString for weedle::types::RecordType<'src> {
    fn type_to_string(&self, record: &FirstPassRecord, dst: &mut String) {
        dst.push_str("record_from_");
        self.generics.body.0.type_to_string(record, dst);
        dst.push_str("_to_");
        self.generics.body.2.type_to_string(record, dst);
    }
}

impl<'a> TypeToString for weedle::types::Type<'a> {
    fn type_to_string(&self, record: &FirstPassRecord, dst: &mut String) {
        use weedle::types::NonAnyType::*;

        let single = match self {
            Type::Single(s) => s,
            Type::Union(_) => panic!("unions not supported"),
        };

        let ty = match single {
            SingleType::Any(_) => return dst.push_str("any"),
            SingleType::NonAny(other) => other,
        };

        match ty {
            Boolean(s) => s.type_to_string(record, dst),
            Octet(s) => s.type_to_string(record, dst),
            Byte(s) => s.type_to_string(record, dst),
            Identifier(s) => s.type_to_string(record, dst),
            Integer(s) => s.type_to_string(record, dst),
            FloatingPoint(s) => s.type_to_string(record, dst),

            Float32Array(s) => s.type_to_string(record, dst),
            Float64Array(s) => s.type_to_string(record, dst),
            Int8Array(s) => s.type_to_string(record, dst),
            Int16Array(s) => s.type_to_string(record, dst),
            Int32Array(s) => s.type_to_string(record, dst),
            Uint8Array(s) => s.type_to_string(record, dst),
            Uint8ClampedArray(s) => s.type_to_string(record, dst),
            Uint16Array(s) => s.type_to_string(record, dst),
            Uint32Array(s) => s.type_to_string(record, dst),

            DOMString(s) => s.type_to_string(record, dst),
            ByteString(s) => s.type_to_string(record, dst),
            USVString(s) => s.type_to_string(record, dst),
            ArrayBuffer(s) => s.type_to_string(record, dst),

            DataView(s) => s.type_to_string(record, dst),
            Error(s) => s.type_to_string(record, dst),
            FrozenArrayType(s) => s.type_to_string(record, dst),
            Object(s) => s.type_to_string(record, dst),
            Promise(s) => s.type_to_string(record, dst),
            RecordType(s) => s.type_to_string(record, dst),
            Sequence(s) => s.type_to_string(record, dst),
            Symbol(s) => s.type_to_string(record, dst),
        }
    }
}

impl<'src> FirstPassRecord<'src> {
    /// Use the first pass to convert webidl function arguments to rust arguments.
    ///
    /// `kind` is whether the function is a method, in which case we would need a `self`
    /// parameter.
    fn webidl_arguments_to_syn_arg_captured(
        &self,
        arguments: &[Argument],
        kind: &backend::ast::ImportFunctionKind,
    ) -> Option<Vec<syn::ArgCaptured>>
    {
        let mut res = if let backend::ast::ImportFunctionKind::Method {
            ty,
            kind:
                backend::ast::MethodKind::Operation(backend::ast::Operation {
                    is_static: false, ..
                }),
            ..
        } = kind
        {
            let mut res = Vec::with_capacity(arguments.len() + 1);
            res.push(simple_fn_arg(raw_ident("self_"), shared_ref(ty.clone())));
            res
        } else {
            Vec::with_capacity(arguments.len())
        };

        for argument in arguments {
            let argument = match argument {
                Argument::Single(arg) => arg,
                Argument::Variadic(_) => return None,
            };
            match argument.type_.type_.to_syn_type(self, TypePosition::Argument) {
                None => {
                    warn!("Argument's type is not yet supported: {:?}", argument);
                    return None;
                }
                Some(ty) => {
                    let name = argument.identifier.0.to_snake_case();
                    res.push(simple_fn_arg(rust_ident(&name), ty))
                }
            }
        }

        Some(res)
    }

    /// Create a wasm-bindgen function, if possible.
    pub fn create_function(
        &self,
        name: &str,
        overloaded: bool,
        same_argument_names: bool,
        arguments: &[Argument],
        mut ret: Option<syn::Type>,
        kind: backend::ast::ImportFunctionKind,
        structural: bool,
        catch: bool,
        doc_comment: Option<String>,
    ) -> Option<backend::ast::ImportFunction>
    {
        let ast_arguments = self.webidl_arguments_to_syn_arg_captured(arguments, &kind)?;

        let rust_name = rust_ident(
            &if overloaded && !arguments.is_empty() {
                let mut argument_type_names = String::new();
                for arg in arguments {
                    let arg = match arg {
                        Argument::Single(single) => single,
                        Argument::Variadic(_) => return None,
                    };
                    if argument_type_names.len() > 0 {
                        argument_type_names.push_str("_and_");
                    }
                    if same_argument_names {
                        arg.type_.type_.type_to_string(self, &mut argument_type_names);
                    } else {
                        argument_type_names.push_str(&arg.identifier.0.to_snake_case());
                    }
                }
                if name == "new" {
                    "with_".to_owned() + &argument_type_names
                } else {
                    name.to_snake_case() + "_with_" + &argument_type_names
                }
            } else {
                name.to_snake_case()
            }
        );
        let name = name.to_string();

        let js_ret = ret.clone();

        if catch {
            ret = Some(ret.map_or_else(|| result_ty(unit_ty()), result_ty))
        }

        let shim = {
            let ns = match kind {
                backend::ast::ImportFunctionKind::Normal => "",
                backend::ast::ImportFunctionKind::Method { ref class, .. } => class,
            };

            raw_ident(&format!("__widl_f_{}_{}", rust_name, ns))
        };

        Some(backend::ast::ImportFunction {
            function: backend::ast::Function {
                name,
                arguments: ast_arguments,
                ret,
                rust_attrs: vec![],
                rust_vis: public(),
            },
            rust_name,
            js_ret,
            catch,
            structural,
            kind,
            shim,
            doc_comment,
        })
    }

    /// Create a wasm-bindgen method, if possible.
    pub fn create_basic_method(
        &self,
        arguments: &[weedle::argument::Argument],
        operation_id: ::first_pass::OperationId,
        return_type: &weedle::types::ReturnType,
        self_name: &str,
        is_static: bool,
        structural: bool,
        catch: bool,
    ) -> Option<backend::ast::ImportFunction> {
        let (overloaded, same_argument_names) = self.get_operation_overloading(
            arguments,
            &operation_id,
            self_name,
        );

        let name = match &operation_id {
            ::first_pass::OperationId::Constructor => panic!("constructors are unsupported"),
            ::first_pass::OperationId::Operation(name) => match name {
                None => {
                    warn!("Operations without a name are unsupported");
                    return None;
                }
                Some(name) => name.to_string(),
            },
            ::first_pass::OperationId::IndexingGetter => "get".to_string(),
            ::first_pass::OperationId::IndexingSetter => "set".to_string(),
            ::first_pass::OperationId::IndexingDeleter => "delete".to_string(),
        };

        let kind = backend::ast::ImportFunctionKind::Method {
            class: self_name.to_string(),
            ty: ident_ty(rust_ident(camel_case_ident(&self_name).as_str())),
            kind: backend::ast::MethodKind::Operation(backend::ast::Operation {
                is_static,
                kind: match &operation_id {
                    ::first_pass::OperationId::Constructor => panic!("constructors are unsupported"),
                    ::first_pass::OperationId::Operation(_) => backend::ast::OperationKind::Regular,
                    ::first_pass::OperationId::IndexingGetter => backend::ast::OperationKind::IndexingGetter,
                    ::first_pass::OperationId::IndexingSetter => backend::ast::OperationKind::IndexingSetter,
                    ::first_pass::OperationId::IndexingDeleter => backend::ast::OperationKind::IndexingDeleter,
                },
            }),
        };

        let ret = match return_type {
            weedle::types::ReturnType::Void(_) => None,
            weedle::types::ReturnType::Type(ty) => {
                match ty.to_syn_type(self, TypePosition::Return) {
                    None => {
                        warn!("Operation's return type is not yet supported: {:?}", ty);
                        return None;
                    }
                    Some(ty) => Some(ty),
                }
            }
        };
        let doc_comment = match &operation_id {
            ::first_pass::OperationId::Constructor => panic!("constructors are unsupported"),
            ::first_pass::OperationId::Operation(_) => Some(
                format!(
                    "The `{}()` method\n\n{}",
                    name,
                    mdn_doc(self_name, Some(&name))
                )
            ),
            ::first_pass::OperationId::IndexingGetter => Some("The getter\n\n".to_string()),
            ::first_pass::OperationId::IndexingSetter => Some("The setter\n\n".to_string()),
            ::first_pass::OperationId::IndexingDeleter => Some("The deleter\n\n".to_string()),
        };

        self.create_function(
            &name,
            overloaded,
            same_argument_names,
            arguments,
            ret,
            kind,
            structural,
            catch,
            doc_comment,
        )
    }

    /// Whether operation is overloaded and
    /// whether there overloads with same argument names for given argument types
    pub fn get_operation_overloading(
        &self,
        arguments: &[weedle::argument::Argument],
        id: &::first_pass::OperationId,
        self_name: &str,
    ) -> (bool, bool) {
        let data = match self.interfaces.get(self_name) {
            Some(data) => data,
            None => return (false, false),
        };
        let data = match data.operations.get(id) {
            Some(data) => data,
            None => return (false, false),
        };
        let mut names = Vec::with_capacity(arguments.len());
        for arg in arguments {
            match arg {
                Argument::Single(arg) => names.push(arg.identifier.0),
                Argument::Variadic(_) => return (false, false),
            }
        }
        (
            data.overloaded,
            *data
                .argument_names_same
                .get(&names)
                .unwrap_or(&false)
        )
    }

    /// Create a wasm-bindgen getter method, if possible.
    pub fn create_getter(
        &self,
        name: &str,
        ty: &weedle::types::Type,
        self_name: &str,
        is_static: bool,
        is_structural: bool,
        catch: bool,
    ) -> Option<backend::ast::ImportFunction> {
        let ret = match ty.to_syn_type(self, TypePosition::Return) {
            None => {
                warn!("Attribute's type does not yet support reading: {:?}", ty);
                return None;
            }
            Some(ty) => Some(ty),
        };

        let kind = backend::ast::ImportFunctionKind::Method {
            class: self_name.to_string(),
            ty: ident_ty(rust_ident(camel_case_ident(&self_name).as_str())),
            kind: backend::ast::MethodKind::Operation(backend::ast::Operation {
                is_static,
                kind: backend::ast::OperationKind::Getter(Some(raw_ident(name))),
            }),
        };
        let doc_comment = Some(format!("The `{}` getter\n\n{}", name, mdn_doc(self_name, Some(name))));

        self.create_function(name, false, false, &[], ret, kind, is_structural, catch, doc_comment)
    }

    /// Create a wasm-bindgen setter method, if possible.
    pub fn create_setter(
        &self,
        name: &str,
        ty: weedle::types::Type,
        self_name: &str,
        is_static: bool,
        is_structural: bool,
        catch: bool,
    ) -> Option<backend::ast::ImportFunction> {
        let kind = backend::ast::ImportFunctionKind::Method {
            class: self_name.to_string(),
            ty: ident_ty(rust_ident(camel_case_ident(&self_name).as_str())),
            kind: backend::ast::MethodKind::Operation(backend::ast::Operation {
                is_static,
                kind: backend::ast::OperationKind::Setter(Some(raw_ident(name))),
            }),
        };
        let doc_comment = Some(format!("The `{}` setter\n\n{}", name, mdn_doc(self_name, Some(name))));

        self.create_function(
            &format!("set_{}", name),
            false,
            false,
            &[Argument::Single(SingleArgument {
                attributes: None,
                optional: None,
                type_: AttributedType {
                    attributes: None,
                    type_: ty,
                },
                identifier: Identifier(name),
                default: None,
            })],
            None,
            kind,
            is_structural,
            catch,
            doc_comment,
        )
    }
}

/// Search for an attribute by name in some webidl object's attributes.
fn has_named_attribute(list: &Option<ExtendedAttributeList>, attribute: &str) -> bool {
    let list = match list {
        Some(list) => list,
        None => return false,
    };
    list.body.list.iter().any(|attr| match attr {
        ExtendedAttribute::NoArgs(name) => (name.0).0 == attribute,
        _ => false,
    })
}

/// ChromeOnly is for things that are only exposed to privileged code in Firefox.
pub fn is_chrome_only(ext_attrs: &Option<ExtendedAttributeList>) -> bool {
    has_named_attribute(ext_attrs, "ChromeOnly")
}

/// Whether a webidl object is marked as a no interface object.
pub fn is_no_interface_object(ext_attrs: &Option<ExtendedAttributeList>) -> bool {
    has_named_attribute(ext_attrs, "NoInterfaceObject")
}

/// Whether a webidl object is marked as structural.
pub fn is_structural(attrs: &Option<ExtendedAttributeList>) -> bool {
    has_named_attribute(attrs, "Unforgeable")
}

/// Whether a webidl object is marked as throwing.
pub fn throws(attrs: &Option<ExtendedAttributeList>) -> bool {
    has_named_attribute(attrs, "Throws")
}

/// Create a syn `pub` token
pub fn public() -> syn::Visibility {
    syn::Visibility::Public(syn::VisPublic {
        pub_token: Default::default(),
    })
}
