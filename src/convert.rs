//! This is mostly an internal module, no stability guarantees are provied. Use
//! at your own risk.

use std::mem::{self, ManuallyDrop};
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
    type Temp;

    // SUPER UNSAFE
    //
    // Many types implement `FromWasmAbi` which have a lifetime parameter and
    // the lifetime isn't connected to the lifetime of `js` itself.
    unsafe fn from_abi(js: Self::Abi, extra: &mut Stack) -> Self::Temp;
    unsafe fn from_temp(temp: &mut Self::Temp) -> Self;

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
            type Temp = $t;
            unsafe fn from_abi(js: $t, _extra: &mut Stack) -> $t { js }
            unsafe fn from_temp(temp: &mut $t) -> $t { *temp }
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
            type Temp = $t;
            unsafe fn from_abi(js: u32, _extra: &mut Stack) -> $t { js as $t }
            unsafe fn from_temp(temp: &mut $t) -> $t { *temp }
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
    type Temp = bool;

    unsafe fn from_abi(js: u32, _extra: &mut Stack) -> bool { js != 0 }
    unsafe fn from_temp(temp: &mut bool) -> bool { *temp }
}

impl<T> IntoWasmAbi for *const T {
    type Abi = u32;

    fn into_abi(self, _extra: &mut Stack) -> u32 { self as u32 }
}

impl<T> FromWasmAbi for *const T {
    type Abi = u32;
    type Temp = *const T;

    unsafe fn from_abi(js: u32, _extra: &mut Stack) -> *const T {
        js as *const T
    }
    unsafe fn from_temp(temp: &mut *const T) -> *const T { *temp }
}

impl<T> IntoWasmAbi for *mut T {
    type Abi = u32;

    fn into_abi(self, _extra: &mut Stack) -> u32 { self as u32 }
}

impl<T> FromWasmAbi for *mut T {
    type Abi = u32;
    type Temp = *mut T;

    unsafe fn from_abi(js: u32, _extra: &mut Stack) -> *mut T {
        js as *mut T
    }
    unsafe fn from_temp(temp: &mut *mut T) -> *mut T { *temp }
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
            type Temp = (*mut $t, usize);

            unsafe fn from_abi(js: u32, extra: &mut Stack) -> Self::Temp {
                let ptr = <*mut $t>::from_abi(js, extra);
                let len = extra.pop() as usize;
                (ptr, len)
            }

            unsafe fn from_temp(temp: &mut Self::Temp) -> Box<[$t]> {
                Vec::from_raw_parts(temp.0, temp.1, temp.1).into_boxed_slice()
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

        impl<'a> FromWasmAbi for &'a [$t] {
            type Abi = u32;
            type Temp = &'a [$t];

            unsafe fn from_abi(js: u32, extra: &mut Stack) -> &'a [$t] {
                slice::from_raw_parts(
                    <*const $t>::from_abi(js, extra),
                    extra.pop() as usize,
                )
            }
            unsafe fn from_temp(temp: &mut &'a [$t]) -> &'a [$t] { *temp }
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
    type Temp = <Box<[T]> as FromWasmAbi>::Temp;
    unsafe fn from_abi(js: Self::Abi, extra: &mut Stack) -> Self::Temp {
        <Box<[T]>>::from_abi(js, extra)
    }

    unsafe fn from_temp(temp: &mut Self::Temp) -> Self {
        <Box<[T]>>::from_temp(temp).into()
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
    type Temp = <Vec<u8> as FromWasmAbi>::Temp;

    unsafe fn from_abi(js: u32, extra: &mut Stack) -> Self::Temp {
        <Vec<u8>>::from_abi(js, extra)
    }

    unsafe fn from_temp(temp: &mut Self::Temp) -> Self {
        String::from_utf8_unchecked(Vec::from_temp(temp))
    }
}

impl<'a> IntoWasmAbi for &'a str {
    type Abi = <&'a [u8] as IntoWasmAbi>::Abi;

    fn into_abi(self, extra: &mut Stack) -> Self::Abi {
        self.as_bytes().into_abi(extra)
    }
}

impl<'a> FromWasmAbi for &'a str {
    type Abi = <&'a [u8] as FromWasmAbi>::Abi;
    type Temp = <&'a [u8] as FromWasmAbi>::Temp;

    unsafe fn from_abi(js: Self::Abi, extra: &mut Stack) -> Self::Temp {
        <&'a [u8]>::from_abi(js, extra)
    }
    unsafe fn from_temp(temp: &mut Self::Temp) -> Self {
        str::from_utf8_unchecked(<&'a [u8]>::from_temp(temp))
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
    type Temp = u32;
    unsafe fn from_abi(js: u32, _extra: &mut Stack) -> u32 { js }

    unsafe fn from_temp(temp: &mut u32) -> JsValue {
        JsValue { idx: *temp }
    }
}

impl<'a> IntoWasmAbi for &'a JsValue {
    type Abi = u32;
    fn into_abi(self, _extra: &mut Stack) -> u32 {
        self.idx
    }
}

impl<'a> FromWasmAbi for &'a JsValue {
    type Abi = u32;
    type Temp = ManuallyDrop<JsValue>;

    unsafe fn from_abi(js: u32, _extra: &mut Stack) -> Self::Temp {
        ManuallyDrop::new(JsValue { idx: js })
    }

    unsafe fn from_temp(temp: &mut ManuallyDrop<JsValue>) -> Self {
        &*(&**temp as *const JsValue)
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
    type Temp = (*mut JsValue, usize);

    unsafe fn from_abi(js: u32, extra: &mut Stack) -> Self::Temp {
        let ptr = <*mut JsValue>::from_abi(js, extra);
        let len = extra.pop() as usize;
        (ptr ,len)
    }

    unsafe fn from_temp(temp: &mut Self::Temp) -> Box<[JsValue]> {
        Vec::from_raw_parts(temp.0, temp.1, temp.1).into_boxed_slice()
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
