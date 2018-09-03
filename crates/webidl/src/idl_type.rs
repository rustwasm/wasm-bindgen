use backend::util::{ident_ty, leading_colon_path_ty, raw_ident, rust_ident};
use syn;
use weedle::common::Identifier;
use weedle::term;
use weedle::types::*;

use first_pass::FirstPassRecord;
use util::{TypePosition, camel_case_ident, snake_case_ident, shared_ref, option_ty, array};

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Debug)]
pub(crate) enum IdlType<'a> {
    Boolean,
    Byte,
    Octet,
    Short,
    UnsignedShort,
    Long,
    UnsignedLong,
    LongLong,
    UnsignedLongLong,
    Float,
    UnrestrictedFloat,
    Double,
    UnrestrictedDouble,
    DomString,
    ByteString,
    UsvString,
    Object,
    Symbol,
    Error,

    ArrayBuffer,
    DataView,
    Int8Array,
    Uint8Array,
    Uint8ClampedArray,
    Int16Array,
    Uint16Array,
    Int32Array,
    Uint32Array,
    Float32Array,
    Float64Array,

    Interface(&'a str),
    Dictionary(&'a str),
    Enum(&'a str),

    Nullable(Box<IdlType<'a>>),
    FrozenArray(Box<IdlType<'a>>),
    Sequence(Box<IdlType<'a>>),
    Promise(Box<IdlType<'a>>),
    Record(Box<IdlType<'a>>, Box<IdlType<'a>>),
    Union(Vec<IdlType<'a>>),

    Any,
    Void,
}

pub(crate) trait ToIdlType<'a> {
    fn to_idl_type(&self, record: &FirstPassRecord<'a>) -> Option<IdlType<'a>>;
}

impl<'a> ToIdlType<'a> for UnionType<'a> {
    fn to_idl_type(&self, record: &FirstPassRecord<'a>) -> Option<IdlType<'a>> {
        let mut idl_types = Vec::with_capacity(self.body.list.len());
        for t in &self.body.list {
            idl_types.push(t.to_idl_type(record)?);
        }
        Some(IdlType::Union(idl_types))
    }
}

impl<'a> ToIdlType<'a> for Type<'a> {
    fn to_idl_type(&self, record: &FirstPassRecord<'a>) -> Option<IdlType<'a>> {
        match self {
            Type::Single(t) => t.to_idl_type(record),
            Type::Union(t) => t.to_idl_type(record),
        }
    }
}

impl<'a> ToIdlType<'a> for SingleType<'a> {
    fn to_idl_type(&self, record: &FirstPassRecord<'a>) -> Option<IdlType<'a>> {
        match self {
            SingleType::Any(t) => t.to_idl_type(record),
            SingleType::NonAny(t) => t.to_idl_type(record),
        }
    }
}

impl<'a> ToIdlType<'a> for NonAnyType<'a> {
    fn to_idl_type(&self, record: &FirstPassRecord<'a>) -> Option<IdlType<'a>> {
        match self {
            NonAnyType::Promise(t) => t.to_idl_type(record),
            NonAnyType::Integer(t) => t.to_idl_type(record),
            NonAnyType::FloatingPoint(t) => t.to_idl_type(record),
            NonAnyType::Boolean(t) => t.to_idl_type(record),
            NonAnyType::Byte(t) => t.to_idl_type(record),
            NonAnyType::Octet(t) => t.to_idl_type(record),
            NonAnyType::ByteString(t) => t.to_idl_type(record),
            NonAnyType::DOMString(t) => t.to_idl_type(record),
            NonAnyType::USVString(t) => t.to_idl_type(record),
            NonAnyType::Sequence(t) => t.to_idl_type(record),
            NonAnyType::Object(t) => t.to_idl_type(record),
            NonAnyType::Symbol(t) => t.to_idl_type(record),
            NonAnyType::Error(t) => t.to_idl_type(record),
            NonAnyType::ArrayBuffer(t) => t.to_idl_type(record),
            NonAnyType::DataView(t) => t.to_idl_type(record),
            NonAnyType::Int8Array(t) => t.to_idl_type(record),
            NonAnyType::Int16Array(t) => t.to_idl_type(record),
            NonAnyType::Int32Array(t) => t.to_idl_type(record),
            NonAnyType::Uint8Array(t) => t.to_idl_type(record),
            NonAnyType::Uint16Array(t) => t.to_idl_type(record),
            NonAnyType::Uint32Array(t) => t.to_idl_type(record),
            NonAnyType::Uint8ClampedArray(t) => t.to_idl_type(record),
            NonAnyType::Float32Array(t) => t.to_idl_type(record),
            NonAnyType::Float64Array(t) => t.to_idl_type(record),
            NonAnyType::FrozenArrayType(t) => t.to_idl_type(record),
            NonAnyType::RecordType(t) => t.to_idl_type(record),
            NonAnyType::Identifier(t) => t.to_idl_type(record),
        }
    }
}

impl<'a> ToIdlType<'a> for SequenceType<'a> {
    fn to_idl_type(&self, record: &FirstPassRecord<'a>) -> Option<IdlType<'a>> {
        Some(IdlType::Sequence(Box::new(self.generics.body.to_idl_type(record)?)))
    }
}

impl<'a> ToIdlType<'a> for FrozenArrayType<'a> {
    fn to_idl_type(&self, record: &FirstPassRecord<'a>) -> Option<IdlType<'a>> {
        Some(IdlType::FrozenArray(Box::new(self.generics.body.to_idl_type(record)?)))
    }
}

impl<'a, T: ToIdlType<'a>> ToIdlType<'a> for MayBeNull<T> {
    fn to_idl_type(&self, record: &FirstPassRecord<'a>) -> Option<IdlType<'a>> {
        let inner_idl_type = self.type_.to_idl_type(record)?;
        if self.q_mark.is_some() {
            Some(IdlType::Nullable(Box::new(inner_idl_type)))
        } else {
            Some(inner_idl_type)
        }
    }
}

impl<'a> ToIdlType<'a> for PromiseType<'a> {
    fn to_idl_type(&self, record: &FirstPassRecord<'a>) -> Option<IdlType<'a>> {
        Some(IdlType::Promise(Box::new(self.generics.body.to_idl_type(record)?)))
    }
}

impl<'a> ToIdlType<'a> for IntegerType {
    fn to_idl_type(&self, record: &FirstPassRecord<'a>) -> Option<IdlType<'a>> {
        match self {
            IntegerType::LongLong(t) => t.to_idl_type(record),
            IntegerType::Long(t) => t.to_idl_type(record),
            IntegerType::Short(t) => t.to_idl_type(record),
        }
    }
}

impl<'a> ToIdlType<'a> for LongLongType {
    fn to_idl_type(&self, _record: &FirstPassRecord<'a>) -> Option<IdlType<'a>> {
        if self.unsigned.is_some() {
            Some(IdlType::UnsignedLongLong)
        } else {
            Some(IdlType::LongLong)
        }
    }
}

impl<'a> ToIdlType<'a> for LongType {
    fn to_idl_type(&self, _record: &FirstPassRecord<'a>) -> Option<IdlType<'a>> {
        if self.unsigned.is_some() {
            Some(IdlType::UnsignedLong)
        } else {
            Some(IdlType::Long)
        }
    }
}

impl<'a> ToIdlType<'a> for ShortType {
    fn to_idl_type(&self, _record: &FirstPassRecord<'a>) -> Option<IdlType<'a>> {
        if self.unsigned.is_some() {
            Some(IdlType::UnsignedShort)
        } else {
            Some(IdlType::Short)
        }
    }
}

impl<'a> ToIdlType<'a> for FloatingPointType {
    fn to_idl_type(&self, record: &FirstPassRecord<'a>) -> Option<IdlType<'a>> {
        match self {
            FloatingPointType::Float(t) => t.to_idl_type(record),
            FloatingPointType::Double(t) => t.to_idl_type(record),
        }
    }
}

impl<'a> ToIdlType<'a> for FloatType {
    fn to_idl_type(&self, _record: &FirstPassRecord<'a>) -> Option<IdlType<'a>> {
        if self.unrestricted.is_some() {
            Some(IdlType::UnrestrictedFloat)
        } else {
            Some(IdlType::Float)
        }
    }
}

impl<'a> ToIdlType<'a> for DoubleType {
    fn to_idl_type(&self, _record: &FirstPassRecord<'a>) -> Option<IdlType<'a>> {
        if self.unrestricted.is_some() {
            Some(IdlType::UnrestrictedDouble)
        } else {
            Some(IdlType::Double)
        }
    }
}

impl<'a> ToIdlType<'a> for RecordType<'a> {
    fn to_idl_type(&self, record: &FirstPassRecord<'a>) -> Option<IdlType<'a>> {
        Some(
            IdlType::Record(
                Box::new(self.generics.body.0.to_idl_type(record)?),
                Box::new(self.generics.body.2.to_idl_type(record)?)
            )
        )
    }
}

impl<'a> ToIdlType<'a> for StringType {
    fn to_idl_type(&self, record: &FirstPassRecord<'a>) -> Option<IdlType<'a>> {
        match self {
            StringType::Byte(t) => t.to_idl_type(record),
            StringType::DOM(t) => t.to_idl_type(record),
            StringType::USV(t) => t.to_idl_type(record),
        }
    }
}

impl<'a> ToIdlType<'a> for UnionMemberType<'a> {
    fn to_idl_type(&self, record: &FirstPassRecord<'a>) -> Option<IdlType<'a>> {
        match self {
            UnionMemberType::Single(t) => t.to_idl_type(record),
            UnionMemberType::Union(t) => t.to_idl_type(record),
        }
    }
}

impl<'a> ToIdlType<'a> for ConstType<'a> {
    fn to_idl_type(&self, record: &FirstPassRecord<'a>) -> Option<IdlType<'a>> {
        match self {
            ConstType::Integer(t) => t.to_idl_type(record),
            ConstType::FloatingPoint(t) => t.to_idl_type(record),
            ConstType::Boolean(t) => t.to_idl_type(record),
            ConstType::Byte(t) => t.to_idl_type(record),
            ConstType::Octet(t) => t.to_idl_type(record),
            ConstType::Identifier(t) => t.to_idl_type(record),
        }
    }
}

impl<'a> ToIdlType<'a> for ReturnType<'a> {
    fn to_idl_type(&self, record: &FirstPassRecord<'a>) -> Option<IdlType<'a>> {
        match self {
            ReturnType::Void(t) => t.to_idl_type(record),
            ReturnType::Type(t) => t.to_idl_type(record),
        }
    }
}

impl<'a> ToIdlType<'a> for AttributedType<'a> {
    fn to_idl_type(&self, record: &FirstPassRecord<'a>) -> Option<IdlType<'a>> {
        self.type_.to_idl_type(record)
    }
}

impl<'a> ToIdlType<'a> for Identifier<'a> {
    fn to_idl_type(&self, record: &FirstPassRecord<'a>) -> Option<IdlType<'a>> {
        if let Some(idl_type) = record.typedefs.get(&self.0) {
            idl_type.to_idl_type(record)
        } else if record.interfaces.contains_key(self.0) {
            Some(IdlType::Interface(self.0))
        } else if record.dictionaries.contains_key(self.0) {
            Some(IdlType::Dictionary(self.0))
        } else if record.enums.contains_key(self.0) {
            Some(IdlType::Enum(self.0))
        } else {
            warn!("Unrecognized type: {}", self.0);
            None
        }
    }
}

macro_rules! terms_to_idl_type {
    ($($t:tt => $r:tt)*) => ($(
        impl<'a> ToIdlType<'a> for term::$t {
            fn to_idl_type(&self, _record: &FirstPassRecord<'a>) -> Option<IdlType<'a>> {
                Some(IdlType::$r)
            }
        }
    )*)
}

terms_to_idl_type! {
    Symbol => Symbol
    ByteString => ByteString
    DOMString => DomString
    USVString => UsvString
    Any => Any
    Boolean => Boolean
    Byte => Byte
    Double => Double
    Float => Float
    Long => Long
    Object => Object
    Octet => Octet
    Short => Short
    Void => Void
    ArrayBuffer => ArrayBuffer
    DataView => DataView
    Int8Array => Int8Array
    Int16Array => Int16Array
    Int32Array => Int32Array
    Uint8Array => Uint8Array
    Uint16Array => Uint16Array
    Uint32Array => Uint32Array
    Uint8ClampedArray => Uint8ClampedArray
    Float32Array => Float32Array
    Float64Array => Float64Array
    Error => Error
}

impl<'a> IdlType<'a> {
    /// Generates a camel case type name.
    #[allow(dead_code)]
    pub(crate) fn push_camel_case_name(&self, dst: &mut String) {
        match self {
            IdlType::Boolean => dst.push_str("Bool"),
            IdlType::Byte => dst.push_str("I8"),
            IdlType::Octet => dst.push_str("U8"),
            IdlType::Short => dst.push_str("I16"),
            IdlType::UnsignedShort => dst.push_str("U16"),
            IdlType::Long => dst.push_str("I32"),
            IdlType::UnsignedLong => dst.push_str("U32"),
            IdlType::LongLong => dst.push_str("I64"),
            IdlType::UnsignedLongLong => dst.push_str("U64"),
            | IdlType::Float
            | IdlType::UnrestrictedFloat => dst.push_str("F32"),
            | IdlType::Double
            | IdlType::UnrestrictedDouble => dst.push_str("F64"),
            | IdlType::DomString
            | IdlType::ByteString
            | IdlType::UsvString => dst.push_str("Str"),
            IdlType::Object => dst.push_str("Object"),
            IdlType::Symbol => dst.push_str("Symbol"),
            IdlType::Error => dst.push_str("Error"),

            IdlType::ArrayBuffer => dst.push_str("ArrayBuffer"),
            IdlType::DataView => dst.push_str("DataView"),
            IdlType::Int8Array => dst.push_str("I8Array"),
            IdlType::Uint8Array => dst.push_str("U8Array"),
            IdlType::Uint8ClampedArray => dst.push_str("U8ClampedArray"),
            IdlType::Int16Array => dst.push_str("I16Array"),
            IdlType::Uint16Array => dst.push_str("U16Array"),
            IdlType::Int32Array => dst.push_str("I32Array"),
            IdlType::Uint32Array => dst.push_str("U32Array"),
            IdlType::Float32Array => dst.push_str("F32Array"),
            IdlType::Float64Array => dst.push_str("F64Array"),

            IdlType::Interface(name) => dst.push_str(&camel_case_ident(name)),
            IdlType::Dictionary(name) => dst.push_str(&camel_case_ident(name)),
            IdlType::Enum(name) => dst.push_str(&camel_case_ident(name)),

            IdlType::Nullable(idl_type) => {
                dst.push_str("Opt");
                idl_type.push_snake_case_name(dst);
            },
            IdlType::FrozenArray(idl_type) => {
                idl_type.push_snake_case_name(dst);
                dst.push_str("FrozenArray");
            },
            IdlType::Sequence(idl_type) => {
                idl_type.push_snake_case_name(dst);
                dst.push_str("Sequence");
            },
            IdlType::Promise(idl_type) => {
                idl_type.push_snake_case_name(dst);
                dst.push_str("Promise");
            },
            IdlType::Record(idl_type_from, idl_type_to) => {
                dst.push_str("RecordFrom");
                idl_type_from.push_snake_case_name(dst);
                dst.push_str("To");
                idl_type_to.push_snake_case_name(dst);
            },
            IdlType::Union(idl_types) => {
                dst.push_str("UnionOf");
                let mut first = true;
                for idl_type in idl_types {
                    if first {
                        first = false;
                    } else {
                        dst.push_str("And");
                    }
                    idl_type.push_snake_case_name(dst);
                }
            },

            IdlType::Any => dst.push_str("Any"),
            IdlType::Void => dst.push_str("Void"),
        }
    }

    /// Generates a snake case type name.
    pub(crate) fn push_snake_case_name(&self, dst: &mut String) {
        match self {
            IdlType::Boolean => dst.push_str("bool"),
            IdlType::Byte => dst.push_str("i8"),
            IdlType::Octet => dst.push_str("u8"),
            IdlType::Short => dst.push_str("i16"),
            IdlType::UnsignedShort => dst.push_str("u16"),
            IdlType::Long => dst.push_str("i32"),
            IdlType::UnsignedLong => dst.push_str("u32"),
            IdlType::LongLong => dst.push_str("i64"),
            IdlType::UnsignedLongLong => dst.push_str("u64"),
            | IdlType::Float
            | IdlType::UnrestrictedFloat => dst.push_str("f32"),
            | IdlType::Double
            | IdlType::UnrestrictedDouble => dst.push_str("f64"),
            | IdlType::DomString
            | IdlType::ByteString
            | IdlType::UsvString => dst.push_str("str"),
            IdlType::Object => dst.push_str("object"),
            IdlType::Symbol => dst.push_str("symbol"),
            IdlType::Error => dst.push_str("error"),

            IdlType::ArrayBuffer => dst.push_str("array_buffer"),
            IdlType::DataView => dst.push_str("data_view"),
            IdlType::Int8Array => dst.push_str("i8_array"),
            IdlType::Uint8Array => dst.push_str("u8_array"),
            IdlType::Uint8ClampedArray => dst.push_str("u8_clamped_array"),
            IdlType::Int16Array => dst.push_str("i16_array"),
            IdlType::Uint16Array => dst.push_str("u16_array"),
            IdlType::Int32Array => dst.push_str("i32_array"),
            IdlType::Uint32Array => dst.push_str("u32_array"),
            IdlType::Float32Array => dst.push_str("f32_array"),
            IdlType::Float64Array => dst.push_str("f64_array"),

            IdlType::Interface(name) => dst.push_str(&snake_case_ident(name)),
            IdlType::Dictionary(name) => dst.push_str(&snake_case_ident(name)),
            IdlType::Enum(name) => dst.push_str(&snake_case_ident(name)),

            IdlType::Nullable(idl_type) => {
                dst.push_str("opt_");
                idl_type.push_snake_case_name(dst);
            },
            IdlType::FrozenArray(idl_type) => {
                idl_type.push_snake_case_name(dst);
                dst.push_str("_frozen_array");
            },
            IdlType::Sequence(idl_type) => {
                idl_type.push_snake_case_name(dst);
                dst.push_str("_sequence");
            },
            IdlType::Promise(idl_type) => {
                idl_type.push_snake_case_name(dst);
                dst.push_str("_promise");
            },
            IdlType::Record(idl_type_from, idl_type_to) => {
                dst.push_str("record_from_");
                idl_type_from.push_snake_case_name(dst);
                dst.push_str("_to_");
                idl_type_to.push_snake_case_name(dst);
            },
            IdlType::Union(idl_types) => {
                dst.push_str("union_of_");
                let mut first = true;
                for idl_type in idl_types {
                    if first {
                        first = false;
                    } else {
                        dst.push_str("_and_");
                    }
                    idl_type.push_snake_case_name(dst);
                }
            },

            IdlType::Any => dst.push_str("any"),
            IdlType::Void => dst.push_str("void"),
        }
    }

    /// Converts to syn type if possible.
    pub(crate) fn to_syn_type(&self, pos: TypePosition) -> Option<syn::Type> {
        match self {
            IdlType::Boolean => Some(ident_ty(raw_ident("bool"))),
            IdlType::Byte => Some(ident_ty(raw_ident("i8"))),
            IdlType::Octet => Some(ident_ty(raw_ident("u8"))),
            IdlType::Short => Some(ident_ty(raw_ident("i16"))),
            IdlType::UnsignedShort => Some(ident_ty(raw_ident("u16"))),
            IdlType::Long => Some(ident_ty(raw_ident("i32"))),
            IdlType::UnsignedLong => Some(ident_ty(raw_ident("u32"))),
            IdlType::LongLong => Some(ident_ty(raw_ident("i64"))),
            IdlType::UnsignedLongLong => Some(ident_ty(raw_ident("u64"))),
            IdlType::Float => Some(ident_ty(raw_ident("f32"))),
            IdlType::UnrestrictedFloat => Some(ident_ty(raw_ident("f32"))),
            IdlType::Double => Some(ident_ty(raw_ident("f64"))),
            IdlType::UnrestrictedDouble => Some(ident_ty(raw_ident("f64"))),
            | IdlType::DomString
            | IdlType::ByteString
            | IdlType::UsvString => match pos {
                TypePosition::Argument => Some(shared_ref(ident_ty(raw_ident("str")))),
                TypePosition::Return => Some(ident_ty(raw_ident("String"))),
            },
            IdlType::Object => {
                let path = vec![rust_ident("js_sys"), rust_ident("Object")];
                Some(leading_colon_path_ty(path))
            },
            IdlType::Symbol => None,
            IdlType::Error => None,

            IdlType::ArrayBuffer => {
                let path = vec![rust_ident("js_sys"), rust_ident("ArrayBuffer")];
                Some(leading_colon_path_ty(path))
            },
            IdlType::DataView => None,
            IdlType::Int8Array => Some(array("i8", pos)),
            IdlType::Uint8Array => Some(array("u8", pos)),
            IdlType::Uint8ClampedArray => None, // FIXME(#421)
            IdlType::Int16Array => Some(array("i16", pos)),
            IdlType::Uint16Array => Some(array("u16", pos)),
            IdlType::Int32Array => Some(array("i32", pos)),
            IdlType::Uint32Array => Some(array("u32", pos)),
            IdlType::Float32Array => Some(array("f32", pos)),
            IdlType::Float64Array => Some(array("f64", pos)),

            | IdlType::Interface(name)
            | IdlType::Dictionary(name) => {
                let ty = ident_ty(rust_ident(camel_case_ident(name).as_str()));
                if pos == TypePosition::Argument {
                    Some(shared_ref(ty))
                } else {
                    Some(ty)
                }
            },
            IdlType::Enum(name) => Some(ident_ty(rust_ident(camel_case_ident(name).as_str()))),

            IdlType::Nullable(idl_type) => {
                match **idl_type {
                    IdlType::Union(ref idl_types) =>
                        if idl_types
                            .iter()
                            .all(|idl_type|
                                match idl_type {
                                    IdlType::Interface(..) => true,
                                    _ => false,
                                }
                            ) {
                            IdlType::Nullable(Box::new(IdlType::Object)).to_syn_type(pos)
                        } else {
                            IdlType::Any.to_syn_type(pos)
                        },
                    _ => Some(option_ty(idl_type.to_syn_type(pos)?))
                }
            },
            IdlType::FrozenArray(_idl_type) => None,
            IdlType::Sequence(_idl_type) => None,
            IdlType::Promise(_idl_type) => {
                let path = vec![rust_ident("js_sys"), rust_ident("Promise")];
                let ty = leading_colon_path_ty(path);
                if pos == TypePosition::Argument {
                    Some(shared_ref(ty))
                } else {
                    Some(ty)
                }
            }
            IdlType::Record(_idl_type_from, _idl_type_to) => None,
            IdlType::Union(idl_types) => {
                // Handles union types in all places except operation argument types.
                // Currently treats them as object or any types.
                // TODO: add better support for union types here?
                if idl_types
                    .iter()
                    .all(|idl_type|
                        match idl_type {
                            IdlType::Interface(..) => true,
                            _ => false,
                        }
                    ) {
                    IdlType::Object.to_syn_type(pos)
                } else {
                    IdlType::Any.to_syn_type(pos)
                }
            },

            IdlType::Any => {
                let path = vec![rust_ident("wasm_bindgen"), rust_ident("JsValue")];
                Some(leading_colon_path_ty(path))
            },
            IdlType::Void => None,
        }
    }

    /// Flattens unions recursively.
    ///
    /// Works similarly to [flattened union member types],
    /// but also flattens unions inside generics of other types.
    ///
    /// [flattened union member types]: https://heycam.github.io/webidl/#dfn-flattened-union-member-types
    pub(crate) fn flatten(&self) -> Vec<Self> {
        match self {
            IdlType::Nullable(idl_type) => idl_type
                .flatten()
                .into_iter()
                .map(Box::new)
                .map(IdlType::Nullable)
                .collect(),
            IdlType::FrozenArray(idl_type) => idl_type
                .flatten()
                .into_iter()
                .map(Box::new)
                .map(IdlType::FrozenArray)
                .collect(),
            IdlType::Sequence(idl_type) => idl_type
                .flatten()
                .into_iter()
                .map(Box::new)
                .map(IdlType::Sequence)
                .collect(),
            IdlType::Promise(idl_type) => idl_type
                .flatten()
                .into_iter()
                .map(Box::new)
                .map(IdlType::Promise)
                .collect(),
            IdlType::Record(idl_type_from, idl_type_to) => {
                let mut idl_types = Vec::new();
                for idl_type_from in idl_type_from.flatten() {
                    for idl_type_to in idl_type_to.flatten() {
                        idl_types.push(
                            IdlType::Record(
                                Box::new(idl_type_from.clone()),
                                Box::new(idl_type_to.clone())
                            )
                        );
                    }
                }
                idl_types
            },
            IdlType::Union(idl_types) => idl_types
                .iter()
                .flat_map(|idl_type| idl_type.flatten())
                .collect(),

            idl_type @ _ => vec![idl_type.clone()],
        }
    }
}

#[test]
fn idl_type_flatten_test() {
    use self::IdlType::*;

    assert_eq!(
        Union(vec![
            Interface("Node"),
            Union(vec![
                Sequence(
                    Box::new(Long),
                ),
                Interface("Event"),
            ]),
            Nullable(
                Box::new(Union(vec![
                    Interface("XMLHttpRequest"),
                    DomString,
                ])),
            ),
            Sequence(
                Box::new(Union(vec![
                    Sequence(
                        Box::new(Double),
                    ),
                    Interface("NodeList"),
                ])),
            ),
        ]).flatten(),
        vec![
            Interface("Node"),
            Sequence(Box::new(Long)),
            Interface("Event"),
            Nullable(Box::new(Interface("XMLHttpRequest"))),
            Nullable(Box::new(DomString)),
            Sequence(Box::new(Sequence(Box::new(Double)))),
            Sequence(Box::new(Interface("NodeList"))),
        ],
    );
}
