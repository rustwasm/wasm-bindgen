//! This is mostly an internal module, no stability guarantees are provied. Use
//! at your own risk.

use std::mem::{self, ManuallyDrop};
use std::ops::{Deref, DerefMut};
use std::slice;
use std::str;

use {JsValue, throw};
use describe::*;

#[derive(PartialEq, Eq, Copy, Clone)]
pub struct Descriptor {
    #[doc(hidden)]
    pub __x: [u8; 4],
}

pub trait IntoWasmAbi: WasmDescribe {
    type Abi: WasmAbi;
    fn into_abi(self, extra: &mut Stack) -> Self::Abi;
}

pub trait FromWasmAbi: WasmDescribe {
    type Abi: WasmAbi;
    unsafe fn from_abi(js: Self::Abi, extra: &mut Stack) -> Self;

}

pub trait RefFromWasmAbi: WasmDescribe {
    type Abi: WasmAbi;
    type Anchor: Deref<Target=Self>;
    unsafe fn ref_from_abi(js: Self::Abi, extra: &mut Stack) -> Self::Anchor;
}

pub trait RefMutFromWasmAbi: WasmDescribe {
    type Abi: WasmAbi;
    type Anchor: DerefMut<Target=Self>;
    unsafe fn ref_mut_from_abi(js: Self::Abi, extra: &mut Stack) -> Self::Anchor;
}

pub trait Stack {
    fn push(&mut self, bits: u32);
    fn pop(&mut self) -> u32;
}

/// An unsafe trait which represents types that are ABI-safe to pass via wasm
/// arguments.
///
/// This is an unsafe trait to implement as there's no guarantee the type is
/// actually safe to transfer across the was boundary, it's up to you to
/// guarantee this so codegen works correctly.
pub unsafe trait WasmAbi {}

unsafe impl WasmAbi for u32 {}
unsafe impl WasmAbi for u64 {}
unsafe impl WasmAbi for i32 {}
unsafe impl WasmAbi for i64 {}
unsafe impl WasmAbi for f32 {}
unsafe impl WasmAbi for f64 {}

macro_rules! simple {
    ($($t:tt)*) => ($(
        impl IntoWasmAbi for $t {
            type Abi = $t;
            fn into_abi(self, _extra: &mut Stack) -> $t { self }
        }

        impl FromWasmAbi for $t {
            type Abi = $t;
            unsafe fn from_abi(js: $t, _extra: &mut Stack) -> $t { js }
        }
    )*)
}

simple!(u32 u64 i32 i64 f32 f64);

macro_rules! as_u32 {
    ($($t:tt)*) => ($(
        impl IntoWasmAbi for $t {
            type Abi = u32;
            fn into_abi(self, _extra: &mut Stack) -> u32 { self as u32 }
        }

        impl FromWasmAbi for $t {
            type Abi = u32;
            unsafe fn from_abi(js: u32, _extra: &mut Stack) -> $t { js as $t }
        }
    )*)
}

as_u32!(i8 u8 i16 u16 isize usize);

impl IntoWasmAbi for bool {
    type Abi = u32;

    fn into_abi(self, _extra: &mut Stack) -> u32 { self as u32 }
}

impl FromWasmAbi for bool {
    type Abi = u32;

    unsafe fn from_abi(js: u32, _extra: &mut Stack) -> bool { js != 0 }
}

impl<T> IntoWasmAbi for *const T {
    type Abi = u32;

    fn into_abi(self, _extra: &mut Stack) -> u32 { self as u32 }
}

impl<T> FromWasmAbi for *const T {
    type Abi = u32;

    unsafe fn from_abi(js: u32, _extra: &mut Stack) -> *const T {
        js as *const T
    }
}

impl<T> IntoWasmAbi for *mut T {
    type Abi = u32;

    fn into_abi(self, _extra: &mut Stack) -> u32 { self as u32 }
}

impl<T> FromWasmAbi for *mut T {
    type Abi = u32;

    unsafe fn from_abi(js: u32, _extra: &mut Stack) -> *mut T {
        js as *mut T
    }
}

macro_rules! vectors {
    ($($t:ident)*) => ($(
        impl IntoWasmAbi for Box<[$t]> {
            type Abi = u32;

            fn into_abi(self, extra: &mut Stack) -> u32 {
                let ptr = self.as_ptr();
                let len = self.len();
                mem::forget(self);
                extra.push(len as u32);
                ptr.into_abi(extra)
            }
        }

        impl FromWasmAbi for Box<[$t]> {
            type Abi = u32;

            unsafe fn from_abi(js: u32, extra: &mut Stack) -> Self {
                let ptr = <*mut $t>::from_abi(js, extra);
                let len = extra.pop() as usize;
                Vec::from_raw_parts(ptr, len, len).into_boxed_slice()
            }
        }

        impl<'a> IntoWasmAbi for &'a [$t] {
            type Abi = u32;

            fn into_abi(self, extra: &mut Stack) -> u32 {
                let ptr = self.as_ptr();
                let len = self.len();
                extra.push(len as u32);
                ptr.into_abi(extra)
            }
        }

        impl RefFromWasmAbi for [$t] {
            type Abi = u32;
            type Anchor = &'static [$t];

            unsafe fn ref_from_abi(js: u32, extra: &mut Stack) -> &'static [$t] {
                slice::from_raw_parts(
                    <*const $t>::from_abi(js, extra),
                    extra.pop() as usize,
                )
            }
        }
    )*)
}

vectors! {
    u8 i8 u16 i16 u32 i32 f32 f64
}

impl<T> IntoWasmAbi for Vec<T> where Box<[T]>: IntoWasmAbi {
    type Abi = <Box<[T]> as IntoWasmAbi>::Abi;
    fn into_abi(self, extra: &mut Stack) -> Self::Abi {
        self.into_boxed_slice().into_abi(extra)
    }
}

impl<T> FromWasmAbi for Vec<T> where Box<[T]>: FromWasmAbi {
    type Abi = <Box<[T]> as FromWasmAbi>::Abi;

    unsafe fn from_abi(js: Self::Abi, extra: &mut Stack) -> Self {
        <Box<[T]>>::from_abi(js, extra).into()
    }
}

impl IntoWasmAbi for String {
    type Abi = u32;

    fn into_abi(self, extra: &mut Stack) -> u32 {
        self.into_bytes().into_abi(extra)
    }
}

impl FromWasmAbi for String {
    type Abi = u32;

    unsafe fn from_abi(js: u32, extra: &mut Stack) -> Self {
        String::from_utf8_unchecked(<Vec<u8>>::from_abi(js, extra))
    }
}

impl<'a> IntoWasmAbi for &'a str {
    type Abi = <&'a [u8] as IntoWasmAbi>::Abi;

    fn into_abi(self, extra: &mut Stack) -> Self::Abi {
        self.as_bytes().into_abi(extra)
    }
}

impl RefFromWasmAbi for str {
    type Abi = <[u8] as RefFromWasmAbi>::Abi;
    type Anchor = &'static str;

    unsafe fn ref_from_abi(js: Self::Abi, extra: &mut Stack) -> Self::Anchor {
        str::from_utf8_unchecked(<[u8]>::ref_from_abi(js, extra))
    }
}

impl IntoWasmAbi for JsValue {
    type Abi = u32;

    fn into_abi(self, _extra: &mut Stack) -> u32 {
        let ret = self.idx;
        mem::forget(self);
        return ret
    }
}

impl FromWasmAbi for JsValue {
    type Abi = u32;

    unsafe fn from_abi(js: u32, _extra: &mut Stack) -> JsValue {
        JsValue { idx: js }
    }
}

impl<'a> IntoWasmAbi for &'a JsValue {
    type Abi = u32;
    fn into_abi(self, _extra: &mut Stack) -> u32 {
        self.idx
    }
}

impl RefFromWasmAbi for JsValue {
    type Abi = u32;
    type Anchor = ManuallyDrop<JsValue>;

    unsafe fn ref_from_abi(js: u32, _extra: &mut Stack) -> Self::Anchor {
        ManuallyDrop::new(JsValue { idx: js })
    }
}

impl IntoWasmAbi for Box<[JsValue]> {
    type Abi = u32;

    fn into_abi(self, extra: &mut Stack) -> u32 {
        let ptr = self.as_ptr();
        let len = self.len();
        mem::forget(self);
        extra.push(len as u32);
        ptr.into_abi(extra)
    }
}

impl FromWasmAbi for Box<[JsValue]> {
    type Abi = u32;

    unsafe fn from_abi(js: u32, extra: &mut Stack) -> Self {
        let ptr = <*mut JsValue>::from_abi(js, extra);
        let len = extra.pop() as usize;
        Vec::from_raw_parts(ptr, len, len).into_boxed_slice()
    }
}

pub struct GlobalStack { next: usize }

const GLOBAL_STACK_CAP: usize = 16;
static mut GLOBAL_STACK: [u32; GLOBAL_STACK_CAP] = [0; GLOBAL_STACK_CAP];

impl GlobalStack {
    pub unsafe fn new() -> GlobalStack {
        GlobalStack { next: 0 }
    }
}

impl Stack for GlobalStack {
    fn push(&mut self, val: u32) {
        unsafe {
            assert!(self.next < GLOBAL_STACK_CAP);
            GLOBAL_STACK[self.next] = val;
            self.next += 1;
        }
    }

    fn pop(&mut self) -> u32 {
        unsafe {
            assert!(self.next < GLOBAL_STACK_CAP);
            let ret = GLOBAL_STACK[self.next];
            self.next += 1;
            ret
        }
    }
}

#[doc(hidden)]
#[no_mangle]
pub unsafe extern fn __wbindgen_global_argument_ptr() -> *mut u32 {
    GLOBAL_STACK.as_mut_ptr()
}

macro_rules! stack_closures {
    ($( ($($var:ident)*) )*) => ($(
        impl<'a, $($var,)* R> IntoWasmAbi for &'a (Fn($($var),*) -> R + 'a)
            where $($var: WasmAbi + WasmDescribe,)*
                  R: WasmAbi + WasmDescribe
        {
            type Abi = u32;

            fn into_abi(self, extra: &mut Stack) -> u32 {
                #[allow(non_snake_case)]
                unsafe extern fn invoke<$($var,)* R>(
                    a: usize,
                    b: usize,
                    $($var: $var),*
                ) -> R {
                    if a == 0 {
                        throw("closure has been destroyed already");
                    }
                    let f: &Fn($($var),*) -> R = mem::transmute((a, b));
                    f($($var),*)
                }
                unsafe {
                    let (a, b): (usize, usize) = mem::transmute(self);
                    extra.push(a as u32);
                    extra.push(b as u32);
                    invoke::<$($var,)* R> as u32
                }
            }
        }

        impl<'a, $($var,)*> IntoWasmAbi for &'a (Fn($($var),*) + 'a)
            where $($var: WasmAbi + WasmDescribe,)*
        {
            type Abi = u32;

            fn into_abi(self, extra: &mut Stack) -> u32 {
                #[allow(non_snake_case)]
                unsafe extern fn invoke<$($var,)* >(
                    a: usize,
                    b: usize,
                    $($var: $var),*
                ) {
                    if a == 0 {
                        throw("closure has been destroyed already");
                    }
                    let f: &Fn($($var),*) = mem::transmute((a, b));
                    f($($var),*)
                }
                unsafe {
                    let (a, b): (usize, usize) = mem::transmute(self);
                    extra.push(a as u32);
                    extra.push(b as u32);
                    invoke::<$($var,)*> as u32
                }
            }
        }

        impl<'a, $($var,)* R> IntoWasmAbi for &'a mut (FnMut($($var),*) -> R + 'a)
            where $($var: WasmAbi + WasmDescribe,)*
                  R: WasmAbi + WasmDescribe
        {
            type Abi = u32;

            fn into_abi(self, extra: &mut Stack) -> u32 {
                #[allow(non_snake_case)]
                unsafe extern fn invoke<$($var,)* R>(
                    a: usize,
                    b: usize,
                    $($var: $var),*
                ) -> R {
                    if a == 0 {
                        throw("closure invoked recursively or destroyed already");
                    }
                    let f: &mut FnMut($($var),*) -> R = mem::transmute((a, b));
                    f($($var),*)
                }
                unsafe {
                    let (a, b): (usize, usize) = mem::transmute(self);
                    extra.push(a as u32);
                    extra.push(b as u32);
                    invoke::<$($var,)* R> as u32
                }
            }
        }

        impl<'a, $($var,)*> IntoWasmAbi for &'a mut (FnMut($($var),*) + 'a)
            where $($var: WasmAbi + WasmDescribe,)*
        {
            type Abi = u32;

            fn into_abi(self, extra: &mut Stack) -> u32 {
                #[allow(non_snake_case)]
                unsafe extern fn invoke<$($var,)* >(
                    a: usize,
                    b: usize,
                    $($var: $var),*
                ) {
                    if a == 0 {
                        throw("closure invoked recursively or destroyed already");
                    }
                    let f: &mut FnMut($($var),*) = mem::transmute((a, b));
                    f($($var),*)
                }
                unsafe {
                    let (a, b): (usize, usize) = mem::transmute(self);
                    extra.push(a as u32);
                    extra.push(b as u32);
                    invoke::<$($var,)*> as u32
                }
            }
        }
    )*)
}

stack_closures! {
    ()
    (A)
    (A B)
    (A B C)
    (A B C D)
    (A B C D E)
    (A B C D E F)
    (A B C D E F G)
}
