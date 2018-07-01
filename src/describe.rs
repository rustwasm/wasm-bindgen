//! This is an internal module, no stability guarantees are provided. Use at
//! your own risk.

#![doc(hidden)]

use JsValue;

macro_rules! tys {
    ($($a:ident)*) => (tys! { @ ($($a)*) 0 });
    (@ () $v:expr) => {};
    (@ ($a:ident $($b:ident)*) $v:expr) => {
        pub const $a: u32 = $v;
        tys!(@ ($($b)*) $v+1);
    }
}

// NB: this list must be kept in sync with `crates/cli-support/src/descriptor.rs`
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

pub fn inform(a: u32) {
    unsafe {
        super::__wbindgen_describe(a)
    }
}

pub trait WasmDescribe {
    fn describe();
}

macro_rules! simple {
    ($($t:ident => $d:ident)*) => ($(
        impl WasmDescribe for $t {
            fn describe() { inform($d) }
        }
    )*)
}

simple! {
    i8 => I8
    u8 => U8
    i16 => I16
    u16 => U16
    i32 => I32
    u32 => U32
    i64 => I64
    u64 => U64
    isize => I32
    usize => U32
    f32 => F32
    f64 => F64
    bool => BOOLEAN
    char => CHAR
    str => STRING
    JsValue => ANYREF
}

impl<T> WasmDescribe for *const T {
    fn describe() { inform(I32) }
}

impl<T> WasmDescribe for *mut T {
    fn describe() { inform(I32) }
}

impl<T: WasmDescribe> WasmDescribe for [T] {
    fn describe() {
        inform(SLICE);
        T::describe();
    }
}

impl<'a, T: WasmDescribe + ?Sized> WasmDescribe for &'a T {
    fn describe() {
        inform(REF);
        T::describe();
    }
}

impl<'a, T: WasmDescribe + ?Sized> WasmDescribe for &'a mut T {
    fn describe() {
        inform(REFMUT);
        T::describe();
    }
}

if_std! {
    use std::prelude::v1::*;

    impl WasmDescribe for String {
        fn describe() { inform(STRING) }
    }

    impl<T: WasmDescribe> WasmDescribe for Box<[T]> {
        fn describe() {
            inform(VECTOR);
            T::describe();
        }
    }

    impl<T> WasmDescribe for Vec<T> where Box<[T]>: WasmDescribe {
        fn describe() {
            <Box<[T]>>::describe();
        }
    }
}

fn _cnt<T: WasmDescribe>() -> u32 { 1 }

macro_rules! doit {
    ($( ($($var:ident)*))*) => ($(
        impl<'a, $($var,)* R> WasmDescribe for Fn($($var),*) -> R + 'a
            where $($var: WasmDescribe,)*
                  R: WasmDescribe
        {
            fn describe() {
                inform(FUNCTION);
                inform(0 $(+ _cnt::<$var>())*);
                $(<$var as WasmDescribe>::describe();)*
                inform(1);
                <R as WasmDescribe>::describe();
            }
        }

        impl<'a, $($var,)* > WasmDescribe for Fn($($var),*) + 'a
            where $($var: WasmDescribe,)*
        {
            fn describe() {
                inform(FUNCTION);
                inform(0 $(+ _cnt::<$var>())*);
                $(<$var as WasmDescribe>::describe();)*
                inform(0);
            }
        }

        impl<'a, $($var,)* R> WasmDescribe for FnMut($($var),*) -> R + 'a
            where $($var: WasmDescribe,)*
                  R: WasmDescribe
        {
            fn describe() {
                inform(FUNCTION);
                inform(0 $(+ _cnt::<$var>())*);
                $(<$var as WasmDescribe>::describe();)*
                inform(1);
                <R as WasmDescribe>::describe();
            }
        }

        impl<'a, $($var,)* > WasmDescribe for FnMut($($var),*) + 'a
            where $($var: WasmDescribe,)*
        {
            fn describe() {
                inform(FUNCTION);
                inform(0 $(+ _cnt::<$var>())*);
                $(<$var as WasmDescribe>::describe();)*
                inform(0);
            }
        }
    )*)
}

doit! {
    ()
    (A)
    (A B)
    (A B C)
    (A B C D)
    (A B C D E)
    (A B C D E F)
    (A B C D E F G)
}
