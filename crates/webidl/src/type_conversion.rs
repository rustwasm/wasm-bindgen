use backend::util::{ident_ty, raw_ident, rust_ident};
use syn;
use weedle::{self, term, types::*};

use first_pass::FirstPassRecord;
use util::{js_value, rust_type_name, shared_ref};

pub(crate) trait ToSynType {
    fn to_syn_type(&self, first_pass: &FirstPassRecord<'_>, must_own: bool) -> Option<syn::Type>;
}

impl ToSynType for term::Any {
    fn to_syn_type(&self, _: &FirstPassRecord<'_>, _: bool) -> Option<syn::Type> {
        Some(js_value())
    }
}

impl ToSynType for term::Boolean {
    fn to_syn_type(&self, _: &FirstPassRecord<'_>, _: bool) -> Option<syn::Type> {
        Some(ident_ty(raw_ident("bool")))
    }
}

impl ToSynType for term::Byte {
    fn to_syn_type(&self, _: &FirstPassRecord<'_>, _: bool) -> Option<syn::Type> {
        Some(ident_ty(raw_ident("i8")))
    }
}

impl ToSynType for term::Octet {
    fn to_syn_type(&self, _: &FirstPassRecord<'_>, _: bool) -> Option<syn::Type> {
        Some(ident_ty(raw_ident("u8")))
    }
}

impl ToSynType for ShortType {
    fn to_syn_type(&self, _: &FirstPassRecord<'_>, _: bool) -> Option<syn::Type> {
        Some(ident_ty(raw_ident(if self.unsigned.is_some() {
            "u16"
        } else {
            "i16"
        })))
    }
}

impl ToSynType for LongType {
    fn to_syn_type(&self, _: &FirstPassRecord<'_>, _: bool) -> Option<syn::Type> {
        Some(ident_ty(raw_ident(if self.unsigned.is_some() {
            "u32"
        } else {
            "i32"
        })))
    }
}

impl ToSynType for LongLongType {
    fn to_syn_type(&self, _: &FirstPassRecord<'_>, _: bool) -> Option<syn::Type> {
        Some(ident_ty(raw_ident(if self.unsigned.is_some() {
            "u64"
        } else {
            "i64"
        })))
    }
}

impl ToSynType for IntegerType {
    fn to_syn_type(&self, first_pass: &FirstPassRecord<'_>, must_own: bool) -> Option<syn::Type> {
        use self::IntegerType::*;

        match self {
            Short(short) => short.to_syn_type(first_pass, must_own),
            LongLong(long_long) => long_long.to_syn_type(first_pass, must_own),
            Long(long) => long.to_syn_type(first_pass, must_own),
        }
    }
}

impl ToSynType for FloatType {
    fn to_syn_type(&self, _: &FirstPassRecord<'_>, _: bool) -> Option<syn::Type> {
        Some(ident_ty(raw_ident("f32")))
    }
}

impl ToSynType for DoubleType {
    fn to_syn_type(&self, _: &FirstPassRecord<'_>, _: bool) -> Option<syn::Type> {
        Some(ident_ty(raw_ident("f64")))
    }
}

impl ToSynType for FloatingPointType {
    fn to_syn_type(&self, first_pass: &FirstPassRecord<'_>, must_own: bool) -> Option<syn::Type> {
        use self::FloatingPointType::*;

        match self {
            Float(float) => float.to_syn_type(first_pass, must_own),
            Double(double) => double.to_syn_type(first_pass, must_own),
        }
    }
}

impl ToSynType for term::DOMString {
    fn to_syn_type(&self, _: &FirstPassRecord<'_>, must_own: bool) -> Option<syn::Type> {
        Some(if must_own {
            ident_ty(raw_ident("String"))
        } else {
            shared_ref(ident_ty(raw_ident("str")))
        })
    }
}

impl ToSynType for weedle::common::Identifier {
    fn to_syn_type(&self, first_pass: &FirstPassRecord<'_>, must_own: bool) -> Option<syn::Type> {
        let ty = ident_ty(rust_ident(&rust_type_name(self)));
        Some(if first_pass.interfaces.contains(&*self.name) {
            if must_own {
                ty
            } else {
                shared_ref(ty)
            }
        } else if first_pass.dictionaries.contains(&*self.name) {
            ty
        } else if first_pass.enums.contains(&*self.name) {
            ty
        } else {
            warn!("unrecognized type {}", self.name);
            ty
        })
    }
}

impl ToSynType for ConstType {
    fn to_syn_type(&self, first_pass: &FirstPassRecord<'_>, must_own: bool) -> Option<syn::Type> {
        use self::ConstType::*;

        match self {
            Integer(integer) => integer.to_syn_type(first_pass, must_own),
            FloatingPoint(floating_point) => floating_point.to_syn_type(first_pass, must_own),
            Boolean(boolean) => boolean.to_syn_type(first_pass, must_own),
            Byte(byte) => byte.to_syn_type(first_pass, must_own),
            Octet(octet) => octet.to_syn_type(first_pass, must_own),
            Identifier(identifier) => {
                // TODO: insure that this is actually a typedef to a primitive type
                identifier.to_syn_type(first_pass, must_own)
            }
        }
    }
}

impl ToSynType for SingleType {
    fn to_syn_type(&self, first_pass: &FirstPassRecord<'_>, must_own: bool) -> Option<syn::Type> {
        use self::SingleType::*;

        match self {
            Any(any) => any.to_syn_type(first_pass, must_own),
            Integer(integer) => integer.to_syn_type(first_pass, must_own),
            FloatingPoint(floating_point) => floating_point.to_syn_type(first_pass, must_own),
            Boolean(boolean) => boolean.to_syn_type(first_pass, must_own),
            Byte(byte) => byte.to_syn_type(first_pass, must_own),
            Octet(octet) => octet.to_syn_type(first_pass, must_own),
            DOMString(dom_string) => dom_string.to_syn_type(first_pass, must_own),
            Identifier(identifier) => identifier.to_syn_type(first_pass, must_own),
            Promise(_) | ByteString(_) | USVString(_) | Sequence(_) | Object(_) | Symbol(_)
            | Error(_) | ArrayBuffer(_) | DataView(_) | Int8Array(_) | Int16Array(_)
            | Int32Array(_) | Uint8Array(_) | Uint16Array(_) | Uint32Array(_)
            | Uint8ClampedArray(_) | Float32Array(_) | Float64Array(_) | FrozenArrayType(_)
            | RecordType(_) => {
                warn!("unsupported type {:?}", self);
                None
            }
        }
    }
}

impl ToSynType for Type {
    fn to_syn_type(&self, first_pass: &FirstPassRecord<'_>, must_own: bool) -> Option<syn::Type> {
        use self::Type::*;

        match self {
            Single(single) => single.to_syn_type(first_pass, must_own),
            Union(_) => {
                warn!("unsupported type {:?}", self);
                None
            }
        }
    }
}

impl ToSynType for AttributedType {
    fn to_syn_type(&self, first_pass: &FirstPassRecord<'_>, must_own: bool) -> Option<syn::Type> {
        if self.attributes.is_some() {
            warn!("ignoring argument attributes");
        }
        self.type_.to_syn_type(first_pass, must_own)
    }
}

impl<T: ToSynType> ToSynType for MayBeNull<T> {
    fn to_syn_type(&self, first_pass: &FirstPassRecord<'_>, must_own: bool) -> Option<syn::Type> {
        if self.q_mark.is_some() {
            warn!("nullable types are not yet supported (see issue #14)");
            None
        } else {
            self.type_.to_syn_type(first_pass, must_own)
        }
    }
}

impl<'a, T: ToSynType> ToSynType for &'a T {
    fn to_syn_type(&self, first_pass: &FirstPassRecord<'_>, must_own: bool) -> Option<syn::Type> {
        (*self).to_syn_type(first_pass, must_own)
    }
}
