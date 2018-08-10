use std::iter::FromIterator;
use std::iter;
use std::collections::BTreeMap;

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

impl<'src> ToSynType<'src> for NonAnyType<'src> {
    fn to_syn_type(&self, record: &FirstPassRecord<'src>, pos: TypePosition)
        -> Option<syn::Type>
    {
        match self {
            NonAnyType::Boolean(s) => s.to_syn_type(record, pos),
            NonAnyType::Octet(s) => s.to_syn_type(record, pos),
            NonAnyType::Byte(s) => s.to_syn_type(record, pos),
            NonAnyType::Identifier(s) => s.to_syn_type(record, pos),
            NonAnyType::Integer(s) => s.to_syn_type(record, pos),
            NonAnyType::FloatingPoint(s) => s.to_syn_type(record, pos),

            NonAnyType::Float32Array(s) => s.to_syn_type(record, pos),
            NonAnyType::Float64Array(s) => s.to_syn_type(record, pos),
            NonAnyType::Int8Array(s) => s.to_syn_type(record, pos),
            NonAnyType::Int16Array(s) => s.to_syn_type(record, pos),
            NonAnyType::Int32Array(s) => s.to_syn_type(record, pos),
            NonAnyType::Uint8Array(s) => s.to_syn_type(record, pos),
            NonAnyType::Uint8ClampedArray(s) => s.to_syn_type(record, pos),
            NonAnyType::Uint16Array(s) => s.to_syn_type(record, pos),
            NonAnyType::Uint32Array(s) => s.to_syn_type(record, pos),

            NonAnyType::DOMString(s) => s.to_syn_type(record, pos),
            NonAnyType::ByteString(s) => s.to_syn_type(record, pos),
            NonAnyType::USVString(s) => s.to_syn_type(record, pos),
            NonAnyType::ArrayBuffer(b) => b.to_syn_type(record, pos),
            NonAnyType::Object(o) => o.to_syn_type(record, pos),

            // Support for these types is not yet implemented, so skip
            // generating any bindings for this function.
            | NonAnyType::DataView(_)
            | NonAnyType::Error(_)
            | NonAnyType::FrozenArrayType(_)
            | NonAnyType::Promise(_)
            | NonAnyType::RecordType(..)
            | NonAnyType::Sequence(_)
            | NonAnyType::Symbol(_) => {
                None
            }
        }
    }
}

impl<'src> ToSynType<'src> for SingleType<'src> {
    fn to_syn_type(&self, record: &FirstPassRecord<'src>, pos: TypePosition)
        -> Option<syn::Type>
    {
        match self {
            // `any` becomes `::wasm_bindgen::JsValue`.
            SingleType::Any(_) => {
                let path = vec![rust_ident("wasm_bindgen"), rust_ident("JsValue")];
                Some(leading_colon_path_ty(path))
            }
            SingleType::NonAny(non_any) => non_any.to_syn_type(record, pos),
        }
    }
}

impl<'src> ToSynType<'src> for Type<'src> {
    fn to_syn_type(&self, record: &FirstPassRecord<'src>, pos: TypePosition)
        -> Option<syn::Type>
    {
        match self {
            Type::Single(single) => single.to_syn_type(record, pos),
            Type::Union(_) => None,
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

/// Implemented on an AST type node to get equivalent list of syn types and type names that do not have unions.
/// For example, it turns `Promise<(sequence<object> or short)>` into
/// corresponding syn types and type names of `[Promise<sequence<object>>, Promise<short>]`.
trait GetArgumentPossibilities<'src> {
    /// Returns option that contains a value if the conversion succeeds.
    /// The value is a vector of argument possibilities.
    /// Each possibility is a tuple of converted argument (syn) types and type names
    fn get_argument_possibilities(&self, record: &FirstPassRecord<'src>) -> Option<Vec<(syn::Type, String)>>;
}

impl<'src, T: GetArgumentPossibilities<'src>> GetArgumentPossibilities<'src> for MayBeNull<T> {
    fn get_argument_possibilities(&self, record: &FirstPassRecord<'src>) -> Option<Vec<(syn::Type, String)>> {
        Some(
            self
                .type_
                .get_argument_possibilities(record)?
                .into_iter()
                .map(|(ty, type_name)|
                    if self.q_mark.is_some() {
                        (option_ty(ty), "opt_".to_string() + &type_name)
                    } else {
                        (ty, type_name)
                    }
                )
                .collect()
        )
    }
}

impl<'src> GetArgumentPossibilities<'src> for weedle::common::Identifier<'src> {
    fn get_argument_possibilities(&self, record: &FirstPassRecord<'src>) -> Option<Vec<(syn::Type, String)>> {
        if let Some(other) = record.typedefs.get(&self.0) {
            other.get_argument_possibilities(record)
        } else {
            let syn_type = self.to_syn_type(record, TypePosition::Argument)?;
            let type_name = self.get_type_name(record);
            Some(vec![(syn_type, type_name)])
        }
    }
}

impl<'src> GetArgumentPossibilities<'src> for NonAnyType<'src> {
    fn get_argument_possibilities(&self, record: &FirstPassRecord<'src>) -> Option<Vec<(syn::Type, String)>> {
        if let NonAnyType::Identifier(identifier) = self {
            identifier.get_argument_possibilities(record)
        } else {
            let syn_type = self.to_syn_type(record, TypePosition::Argument)?;
            let type_name = self.get_type_name(record);
            Some(vec![(syn_type, type_name)])
        }
    }
}

impl<'src> GetArgumentPossibilities<'src> for SingleType<'src> {
    fn get_argument_possibilities(&self, record: &FirstPassRecord<'src>) -> Option<Vec<(syn::Type, String)>> {
        if let SingleType::NonAny(non_any) = self {
            non_any.get_argument_possibilities(record)
        } else {
            let syn_type = self.to_syn_type(record, TypePosition::Argument)?;
            let type_name = self.get_type_name(record);
            Some(vec![(syn_type, type_name)])
        }
    }
}

impl<'src> GetArgumentPossibilities<'src> for UnionMemberType<'src> {
    fn get_argument_possibilities(&self, record: &FirstPassRecord<'src>) -> Option<Vec<(syn::Type, String)>> {
        match self {
            UnionMemberType::Single(single) => single.get_argument_possibilities(record),
            UnionMemberType::Union(union) => union.get_argument_possibilities(record),
        }
    }
}

impl<'src> GetArgumentPossibilities<'src> for UnionType<'src> {
    fn get_argument_possibilities(&self, record: &FirstPassRecord<'src>) -> Option<Vec<(syn::Type, String)>> {
        let mut result = Vec::new();
        for ty in &self.body.list {
            result.extend(ty.get_argument_possibilities(record)?.into_iter());
        }
        Some(result)
    }
}

impl<'src> GetArgumentPossibilities<'src> for Type<'src> {
    fn get_argument_possibilities(&self, record: &FirstPassRecord<'src>) -> Option<Vec<(syn::Type, String)>> {
        match self {
            Type::Single(single) => single.get_argument_possibilities(record),
            Type::Union(union) => union.get_argument_possibilities(record),
        }
    }
}

/// Implemented on an AST type node to generate a snake case name.
trait GetTypeName {
    fn push_type_name(&self, record: &FirstPassRecord, dst: &mut String);

    fn get_type_name(&self, record: &FirstPassRecord) -> String {
        let mut string = String::new();
        self.push_type_name(record, &mut string);
        return string;
    }
}

impl<T: GetTypeName> GetTypeName for [T] {
    fn push_type_name(&self, record: &FirstPassRecord, dst: &mut String) {
        let mut first = true;
        for union_member_type in self {
            if first {
                first = false;
            } else {
                dst.push_str("_and_");
            }
            union_member_type.push_type_name(record, dst);
        }
    }
}

impl<T: GetTypeName> GetTypeName for MayBeNull<T> {
    fn push_type_name(&self, record: &FirstPassRecord, dst: &mut String) {
        if self.q_mark.is_some() {
            dst.push_str("opt_");
        }
        self.type_.push_type_name(record, dst);
    }
}

macro_rules! term_type_names {
    ($($t:tt => $r:tt)*) => ($(
        impl GetTypeName for weedle::term::$t {
            fn push_type_name(&self, _record: &FirstPassRecord, dst: &mut String) {
                dst.push_str($r);
            }
        }
    )*)
}

term_type_names!(
    Boolean => "bool"
    Byte => "i8"
    Octet => "u8"
    Int8Array => "i8_array"
    Uint8Array => "u8_array"
    Uint8ClampedArray => "u8_clamped_array"
    Int16Array => "i16_array"
    Uint16Array => "u16_array"
    Int32Array => "i32_array"
    Uint32Array => "u32_array"
    Float32Array => "f32_array"
    Float64Array => "f64_array"
    USVString => "usv_str"
    ByteString => "byte_str"
    DOMString => "dom_str"
    ArrayBuffer => "array_buffer"
    Symbol => "symbol"
    Object => "object"
    DataView => "data_view"
    Error => "error"
);

impl<'src> GetTypeName for weedle::types::ReturnType<'src> {
    fn push_type_name(&self, record: &FirstPassRecord, dst: &mut String) {
        match self {
            weedle::types::ReturnType::Type(ty) => (*ty).push_type_name(record, dst),
            weedle::types::ReturnType::Void(_) => dst.push_str("void"),
        }
    }
}

impl GetTypeName for weedle::types::StringType {
    fn push_type_name(&self, _record: &FirstPassRecord, dst: &mut String) {
        match self {
            weedle::types::StringType::Byte(_) => dst.push_str("byte_str"),
            weedle::types::StringType::DOM(_) => dst.push_str("dom_str"),
            weedle::types::StringType::USV(_) => dst.push_str("usv_str"),
        }
    }
}

impl<'src> GetTypeName for weedle::common::Identifier<'src> {
    fn push_type_name(&self, record: &FirstPassRecord, dst: &mut String) {
        match record.typedefs.get(self.0) {
            Some(other) => other.push_type_name(record, dst),
            None => dst.push_str(&self.0.to_snake_case()),
        }
    }
}

impl GetTypeName for IntegerType {
    fn push_type_name(&self, _record: &FirstPassRecord, dst: &mut String) {
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

impl GetTypeName for FloatingPointType {
    fn push_type_name(&self, _record: &FirstPassRecord, dst: &mut String) {
        match self {
            FloatingPointType::Float(_) => dst.push_str("f32"),
            FloatingPointType::Double(_) => dst.push_str("f64"),
        }
    }
}

impl<'src> GetTypeName for SequenceType<'src> {
    fn push_type_name(&self, record: &FirstPassRecord, dst: &mut String) {
        self.generics.body.push_type_name(record, dst);
        dst.push_str("_seq");
    }
}

impl<'src> GetTypeName for PromiseType<'src> {
    fn push_type_name(&self, record: &FirstPassRecord, dst: &mut String) {
        self.generics.body.push_type_name(record, dst);
        dst.push_str("_promise");
    }
}

impl<'src> GetTypeName for FrozenArrayType<'src> {
    fn push_type_name(&self, record: &FirstPassRecord, dst: &mut String) {
        self.generics.body.push_type_name(record, dst);
        dst.push_str("_frozen_array");
    }
}

impl<'src> GetTypeName for RecordType<'src> {
    fn push_type_name(&self, record: &FirstPassRecord, dst: &mut String) {
        dst.push_str("record_from_");
        self.generics.body.0.push_type_name(record, dst);
        dst.push_str("_to_");
        self.generics.body.2.push_type_name(record, dst);
    }
}

impl<'a> GetTypeName for NonAnyType<'a> {
    fn push_type_name(&self, record: &FirstPassRecord, dst: &mut String) {
        match self {
            NonAnyType::Boolean(s) => s.push_type_name(record, dst),
            NonAnyType::Octet(s) => s.push_type_name(record, dst),
            NonAnyType::Byte(s) => s.push_type_name(record, dst),
            NonAnyType::Identifier(s) => s.push_type_name(record, dst),
            NonAnyType::Integer(s) => s.push_type_name(record, dst),
            NonAnyType::FloatingPoint(s) => s.push_type_name(record, dst),

            NonAnyType::Float32Array(s) => s.push_type_name(record, dst),
            NonAnyType::Float64Array(s) => s.push_type_name(record, dst),
            NonAnyType::Int8Array(s) => s.push_type_name(record, dst),
            NonAnyType::Int16Array(s) => s.push_type_name(record, dst),
            NonAnyType::Int32Array(s) => s.push_type_name(record, dst),
            NonAnyType::Uint8Array(s) => s.push_type_name(record, dst),
            NonAnyType::Uint8ClampedArray(s) => s.push_type_name(record, dst),
            NonAnyType::Uint16Array(s) => s.push_type_name(record, dst),
            NonAnyType::Uint32Array(s) => s.push_type_name(record, dst),

            NonAnyType::DOMString(s) => s.push_type_name(record, dst),
            NonAnyType::ByteString(s) => s.push_type_name(record, dst),
            NonAnyType::USVString(s) => s.push_type_name(record, dst),
            NonAnyType::ArrayBuffer(s) => s.push_type_name(record, dst),

            NonAnyType::DataView(s) => s.push_type_name(record, dst),
            NonAnyType::Error(s) => s.push_type_name(record, dst),
            NonAnyType::FrozenArrayType(s) => s.push_type_name(record, dst),
            NonAnyType::Object(s) => s.push_type_name(record, dst),
            NonAnyType::Promise(s) => s.push_type_name(record, dst),
            NonAnyType::RecordType(s) => s.push_type_name(record, dst),
            NonAnyType::Sequence(s) => s.push_type_name(record, dst),
            NonAnyType::Symbol(s) => s.push_type_name(record, dst),
        }
    }
}

impl<'a> GetTypeName for SingleType<'a> {
    fn push_type_name(&self, record: &FirstPassRecord, dst: &mut String) {
        match self {
            SingleType::Any(_) => dst.push_str("any"),
            SingleType::NonAny(non_any) => non_any.push_type_name(record, dst),
        }
    }
}

impl<'a> GetTypeName for UnionMemberType<'a> {
    fn push_type_name(&self, record: &FirstPassRecord, dst: &mut String) {
        match self {
            UnionMemberType::Single(single) => single.push_type_name(record, dst),
            UnionMemberType::Union(union) => union.push_type_name(record, dst),
        }
    }
}

impl<'a> GetTypeName for UnionType<'a> {
    fn push_type_name(&self, record: &FirstPassRecord, dst: &mut String) {
        dst.push_str("union_of_");
        self.body.list.push_type_name(record, dst);
    }
}

impl<'a> GetTypeName for Type<'a> {
    fn push_type_name(&self, record: &FirstPassRecord, dst: &mut String) {
        match self {
            Type::Single(single) => single.push_type_name(record, dst),
            Type::Union(union) => union.push_type_name(record, dst),
        }
    }
}

impl<'src> FirstPassRecord<'src> {
    /// Use the first pass to convert webidl function arguments to rust arguments.
    ///
    /// `kind` is whether the function is a method, in which case we would need a `self`
    /// parameter.
    ///
    /// Returns option that contains a value if the conversion succeeds.
    /// The value is a vector of argument variants.
    /// Each variant is a vector of original arguments, converted argument types and type names.
    fn get_variants(
        &self,
        arguments: &'src [Argument],
        kind: &backend::ast::ImportFunctionKind,
    ) -> Option<Vec<Vec<(Option<&'src Argument>, (syn::ArgCaptured, Option<String>))>>>
    {
        let arguments_possibilities = {
            fn get_argument_possibilities(record: &FirstPassRecord, argument: &Argument) -> Option<Vec<(syn::Type, String)>> {
                let single = match argument {
                    Argument::Single(single) => single,
                    Argument::Variadic(_) => return None,
                };
                match single.type_.type_.get_argument_possibilities(record) {
                    None => {
                        warn!("Argument's type is not yet supported: {:?}", argument);
                        None
                    },
                    Some(value) => Some(value),
                }
            }
            if !arguments.is_empty() {
                let mut optional_arguments_possibilities = Vec::new();
                if let Argument::Single(ref single) = arguments[0] {
                    if single.optional.is_some() {
                        optional_arguments_possibilities.push(Vec::new());
                    }
                }
                let mut arguments_possibilities: Vec<_> = get_argument_possibilities(
                    self,
                    &arguments[0]
                )?
                    .into_iter()
                    .map(|argument_possibility| vec![argument_possibility])
                    .collect();
                for argument in arguments[1..].iter() {
                    let mut new_arguments_possibilities = Vec::new();
                    for arguments_possibility in arguments_possibilities {
                        if let Argument::Single(single) = argument {
                            if single.optional.is_some() {
                                optional_arguments_possibilities.push(arguments_possibility.clone());
                            }
                        }
                        let mut element_argument_possibilities = get_argument_possibilities(
                            self,
                            &argument
                        )?;
                        for element_argument_possibility in element_argument_possibilities {
                            new_arguments_possibilities.push(
                                arguments_possibility
                                    .iter()
                                    .cloned()
                                    .chain(iter::once(element_argument_possibility))
                                    .collect()
                            )
                        }
                    }
                    arguments_possibilities = new_arguments_possibilities
                }
                optional_arguments_possibilities.extend(arguments_possibilities.into_iter());
                optional_arguments_possibilities
            } else {
                vec![Vec::new()]
            }
        };
        let mut result = Vec::new();
        for arguments_possibility in arguments_possibilities {
            let mut res = if let backend::ast::ImportFunctionKind::Method {
                ty,
                kind: backend::ast::MethodKind::Operation(
                    backend::ast::Operation {
                        is_static: false, ..
                    }
                ),
                ..
            } = kind {
                let mut res = Vec::with_capacity(arguments.len() + 1);
                res.push(
                    (
                        None,
                        (
                            simple_fn_arg(raw_ident("self_"), shared_ref(ty.clone())),
                            None,
                        ),
                    )
                );
                res
            } else {
                Vec::with_capacity(arguments.len())
            };
            for (argument, argument_possibility) in arguments.iter().zip(arguments_possibility) {
                let single = match argument {
                    Argument::Single(single) => single,
                    Argument::Variadic(_) => return None,
                };
                res.push(
                    (
                        Some(argument),
                        (
                            simple_fn_arg(
                                rust_ident(&single.identifier.0.to_snake_case()),
                                argument_possibility.0.clone()
                            ),
                            Some(argument_possibility.1.clone()),
                        ),
                    )
                );
            }
            result.push(res);
        }

        Some(result)
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
    ) -> Option<Vec<backend::ast::ImportFunction>>
    {
        let rust_name = if overloaded && !arguments.is_empty() {
            let mut argument_type_names = String::new();
            for argument in arguments {
                let argument = match argument {
                    Argument::Single(single) => single,
                    Argument::Variadic(_) => return None,
                };
                if argument_type_names.len() > 0 {
                    argument_type_names.push_str("_and_");
                }
                if same_argument_names {
                    argument.type_.type_.push_type_name(self, &mut argument_type_names);
                } else {
                    argument_type_names.push_str(&argument.identifier.0.to_snake_case());
                }
            }
            if name == "new" {
                "with_".to_owned() + &argument_type_names
            } else {
                name.to_snake_case() + "_with_" + &argument_type_names
            }
        } else {
            name.to_snake_case()
        };

        let js_ret = ret.clone();

        if catch {
            ret = Some(ret.map_or_else(|| result_ty(unit_ty()), result_ty))
        }

        let variants = self.get_variants(arguments, &kind)?;
        let multiple_variants = variants.len() > 1;
        let mut arguments_count_variants_count = BTreeMap::new();
        for variant in &variants {
            arguments_count_variants_count
                .entry(variant.len())
                .and_modify(|variants_count| { *variants_count += 1usize; })
                .or_insert(1usize);
        }
        let mut result = Vec::new();
        for variant in variants {
            let (arguments, variant_types_variant_names): (Vec<_>, Vec<(_, _)>) = variant.into_iter().unzip();
            let (variant_types, variant_names): (Vec<_>, Vec<_>) = variant_types_variant_names.into_iter().unzip();
            let variant_names_len = variant_names.len();
            let rust_name = if multiple_variants {
                let mut rust_name = rust_name.clone();
                let mut first = true;
                for (argument, variant_name) in arguments.iter().zip(variant_names) {
                    if let Some(type_name) = variant_name {
                        if first {
                            rust_name.push_str("_using_");
                            first = false;
                        } else {
                            rust_name.push_str("_and_");
                        }
                        if arguments_count_variants_count[&variant_names_len] == 1 {
                            if let Some(argument) = argument {
                                let argument = match argument {
                                    Argument::Single(single) => single,
                                    Argument::Variadic(_) => return None,
                                };
                                rust_name.push_str(&argument.identifier.0.to_snake_case());
                            } else {
                                rust_name.push_str(&type_name);
                            }
                        } else {
                            rust_name.push_str(&type_name);
                        }
                    }
                }
                rust_name
            } else {
                rust_name.clone()
            };
            let rust_name = rust_ident(&rust_name);
            let shim = {
                let ns = match kind {
                    backend::ast::ImportFunctionKind::Normal => "",
                    backend::ast::ImportFunctionKind::Method { ref class, .. } => class,
                };

                raw_ident(&format!("__widl_f_{}_{}", rust_name, ns))
            };

            result.push(backend::ast::ImportFunction {
                function: backend::ast::Function {
                    name: name.to_string(),
                    arguments: variant_types,
                    ret: ret.clone(),
                    rust_attrs: vec![],
                    rust_vis: public(),
                },
                rust_name,
                js_ret: js_ret.clone(),
                catch,
                structural,
                kind: kind.clone(),
                shim,
                doc_comment: doc_comment.clone(),
            })
        }
        Some(result)
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
    ) -> Option<Vec<backend::ast::ImportFunction>> {
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
            ::first_pass::OperationId::IndexingGetter => Some("The indexing getter\n\n".to_string()),
            ::first_pass::OperationId::IndexingSetter => Some("The indexing setter\n\n".to_string()),
            ::first_pass::OperationId::IndexingDeleter => Some("The indexing deleter\n\n".to_string()),
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
        fn get_operation_data<'src>(
            record: &'src FirstPassRecord,
            id: &'src ::first_pass::OperationId,
            self_name: &str,
            mixin_name: &str,
        ) -> Option<&'src ::first_pass::OperationData<'src>> {
            if let Some(mixin_data) = record.mixins.get(mixin_name) {
                if let Some(operation_data) = mixin_data.operations.get(id) {
                    return Some(operation_data);
                }
            }
            if let Some(mixin_names) = record.includes.get(mixin_name) {
                for mixin_name in mixin_names {
                    if let Some(operation_data) = get_operation_data(record, id, self_name, mixin_name) {
                        return Some(operation_data);
                    }
                }
            }
            None
        }

        let operation_data = self
            .interfaces
            .get(self_name)
            .and_then(|interface_data| interface_data.operations.get(id))
            .unwrap_or_else(||
                get_operation_data(self, id, self_name, self_name)
                    .expect(&format!("not found operation {:?} in interface {}", id, self_name))
            );

        let mut names = Vec::with_capacity(arguments.len());
        for arg in arguments {
            match arg {
                Argument::Single(arg) => names.push(arg.identifier.0),
                Argument::Variadic(_) => return (false, false),
            }
        }
        (
            operation_data.overloaded,
            *operation_data
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
    ) -> Option<Vec<backend::ast::ImportFunction>> {
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
    ) -> Option<Vec<backend::ast::ImportFunction>> {
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
