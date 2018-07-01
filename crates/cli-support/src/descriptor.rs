use std::char;

macro_rules! tys {
    ($($a:ident)*) => (tys! { @ ($($a)*) 0 });
    (@ () $v:expr) => {};
    (@ ($a:ident $($b:ident)*) $v:expr) => {
        const $a: u32 = $v;
        tys!(@ ($($b)*) $v+1);
    }
}

// NB: this list must be kept in sync with `src/describe.rs`
tys! {
    I8
    U8
    I16
    U16
    I32
    U32
    I64
    U64
    F32
    F64
    BOOLEAN
    FUNCTION
    CLOSURE
    STRING
    REF
    REFMUT
    SLICE
    VECTOR
    ANYREF
    ENUM
    RUST_STRUCT
    CHAR
}

#[derive(Debug)]
pub enum Descriptor {
    I8,
    U8,
    I16,
    U16,
    I32,
    U32,
    I64,
    U64,
    F32,
    F64,
    Boolean,
    Function(Box<Function>),
    Closure(Box<Closure>),
    Ref(Box<Descriptor>),
    RefMut(Box<Descriptor>),
    Slice(Box<Descriptor>),
    Vector(Box<Descriptor>),
    String,
    Anyref,
    Enum,
    RustStruct(String),
    Char,
}

#[derive(Debug)]
pub struct Function {
    pub arguments: Vec<Descriptor>,
    pub ret: Option<Descriptor>,
}

#[derive(Debug)]
pub struct Closure {
    pub function: Function,
    pub mutable: bool,
}

#[derive(Copy, Clone)]
pub enum VectorKind {
    I8,
    U8,
    I16,
    U16,
    I32,
    U32,
    I64,
    U64,
    F32,
    F64,
    String,
    Anyref,
}

impl Descriptor {
    pub fn decode(mut data: &[u32]) -> Descriptor {
        let descriptor = Descriptor::_decode(&mut data);
        assert!(data.is_empty());
        descriptor
    }

    fn _decode(data: &mut &[u32]) -> Descriptor {
        match get(data) {
            I8 => Descriptor::I8,
            I16 => Descriptor::I16,
            I32 => Descriptor::I32,
            I64 => Descriptor::I64,
            U8 => Descriptor::U8,
            U16 => Descriptor::U16,
            U32 => Descriptor::U32,
            U64 => Descriptor::U64,
            F32 => Descriptor::F32,
            F64 => Descriptor::F64,
            BOOLEAN => Descriptor::Boolean,
            FUNCTION => Descriptor::Function(Box::new(Function::decode(data))),
            CLOSURE => Descriptor::Closure(Box::new(Closure::decode(data))),
            REF => Descriptor::Ref(Box::new(Descriptor::_decode(data))),
            REFMUT => Descriptor::RefMut(Box::new(Descriptor::_decode(data))),
            SLICE => Descriptor::Slice(Box::new(Descriptor::_decode(data))),
            VECTOR => Descriptor::Vector(Box::new(Descriptor::_decode(data))),
            STRING => Descriptor::String,
            ANYREF => Descriptor::Anyref,
            ENUM => Descriptor::Enum,
            RUST_STRUCT => {
                let name = (0..get(data))
                    .map(|_| char::from_u32(get(data)).unwrap())
                    .collect();
                Descriptor::RustStruct(name)
            }
            CHAR => Descriptor::Char,
            other => panic!("unknown descriptor: {}", other),
        }
    }

    pub fn unwrap_function(&self) -> &Function {
        match *self {
            Descriptor::Function(ref f) => f,
            _ => panic!("not a function"),
        }
    }

    pub fn is_number(&self) -> bool {
        match *self {
            Descriptor::I8 |
            Descriptor::U8 |
            Descriptor::I16 |
            Descriptor::U16 |
            Descriptor::I32 |
            Descriptor::U32 |
            Descriptor::F32 |
            Descriptor::F64 |
            Descriptor::Enum => true,
            _ => return false,
        }
    }

    pub fn get_64bit(&self) -> Option<bool> {
        match *self {
            Descriptor::I64 => Some(true),
            Descriptor::U64 => Some(false),
            _ => None,
        }
    }

    pub fn is_ref_anyref(&self) -> bool {
        match *self {
            Descriptor::Ref(ref s) => s.is_anyref(),
            _ => return false,
        }
    }

    pub fn ref_closure(&self) -> Option<&Closure> {
        match *self {
            Descriptor::Ref(ref s) => s.closure(),
            _ => None,
        }
    }

    pub fn closure(&self) -> Option<&Closure> {
        match *self {
            Descriptor::Closure(ref s) => Some(s),
            _ => None,
        }
    }

    pub fn is_anyref(&self) -> bool {
        match *self {
            Descriptor::Anyref => true,
            _ => false,
        }
    }

    pub fn vector_kind(&self) -> Option<VectorKind> {
        let inner = match *self {
            Descriptor::String => return Some(VectorKind::String),
            Descriptor::Vector(ref d) => &**d,
            Descriptor::Ref(ref d) => {
                match **d {
                    Descriptor::Slice(ref d) => &**d,
                    Descriptor::String => return Some(VectorKind::String),
                    _ => return None,
                }
            }
            Descriptor::RefMut(ref d) => {
                match **d {
                    Descriptor::Slice(ref d) => &**d,
                    _ => return None,
                }
            }
            _ => return None,
        };
        match *inner {
            Descriptor::I8 => Some(VectorKind::I8),
            Descriptor::I16 => Some(VectorKind::I16),
            Descriptor::I32 => Some(VectorKind::I32),
            Descriptor::I64 => Some(VectorKind::I64),
            Descriptor::U8 => Some(VectorKind::U8),
            Descriptor::U16 => Some(VectorKind::U16),
            Descriptor::U32 => Some(VectorKind::U32),
            Descriptor::U64 => Some(VectorKind::U64),
            Descriptor::F32 => Some(VectorKind::F32),
            Descriptor::F64 => Some(VectorKind::F64),
            Descriptor::Anyref => Some(VectorKind::Anyref),
            _ => None
        }
    }

    pub fn rust_struct(&self) -> Option<&str> {
        let inner = match *self {
            Descriptor::Ref(ref d) => &**d,
            Descriptor::RefMut(ref d) => &**d,
            ref d => d,
        };
        match *inner {
            Descriptor::RustStruct(ref s) => Some(s),
            _ => None,
        }
    }

    pub fn stack_closure(&self) -> Option<(&Function, bool)> {
        let (inner, mutable) = match *self {
            Descriptor::Ref(ref d) => (&**d, false),
            Descriptor::RefMut(ref d) => (&**d, true),
            _ => return None,
        };
        match *inner {
            Descriptor::Function(ref f) => Some((f, mutable)),
            _ => None,
        }
    }

    pub fn is_by_ref(&self) -> bool {
        match *self {
            Descriptor::Ref(_) |
            Descriptor::RefMut(_) => true,
            _ => false,
        }
    }

    pub fn is_mut_ref(&self) -> bool {
        match *self {
            Descriptor::RefMut(_) => true,
            _ => false,
        }
    }
}

fn get(a: &mut &[u32]) -> u32 {
    let ret = a[0];
    *a = &a[1..];
    ret
}

impl Closure {
    fn decode(data: &mut &[u32]) -> Closure {
        let mutable = get(data) == REFMUT;
        assert_eq!(get(data), FUNCTION);
        Closure {
            mutable,
            function: Function::decode(data),
        }
    }
}

impl Function {
    fn decode(data: &mut &[u32]) -> Function {
        let arguments = (0..get(data))
            .map(|_| Descriptor::_decode(data))
            .collect::<Vec<_>>();
        let ret = if get(data) == 0 {
            None
        } else {
            Some(Descriptor::_decode(data))
        };
        Function { arguments, ret }
    }
}

impl VectorKind {
    pub fn js_ty(&self) -> &str {
        match *self {
            VectorKind::String => "string",
            VectorKind::I8 => "Int8Array",
            VectorKind::U8 => "Uint8Array",
            VectorKind::I16 => "Int16Array",
            VectorKind::U16 => "Uint16Array",
            VectorKind::I32 => "Int32Array",
            VectorKind::U32 => "Uint32Array",
            VectorKind::I64 => "BigInt64Array",
            VectorKind::U64 => "BigUint64Array",
            VectorKind::F32 => "Float32Array",
            VectorKind::F64 => "Float64Array",
            VectorKind::Anyref => "any[]",
        }
    }

    pub fn size(&self) -> usize {
        match *self {
            VectorKind::String => 1,
            VectorKind::I8 => 1,
            VectorKind::U8 => 1,
            VectorKind::I16 => 2,
            VectorKind::U16 => 2,
            VectorKind::I32 => 4,
            VectorKind::U32 => 4,
            VectorKind::I64 => 8,
            VectorKind::U64 => 8,
            VectorKind::F32 => 4,
            VectorKind::F64 => 8,
            VectorKind::Anyref => 4,
        }
    }
}
