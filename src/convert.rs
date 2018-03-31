use std::mem::{self, ManuallyDrop};
use std::ops::{Deref, DerefMut};
use std::slice;
use std::str;

use super::JsValue;

#[derive(PartialEq, Eq, Copy, Clone)]
pub struct Descriptor {
    #[doc(hidden)]
    pub __x: [u8; 4],
}

// keep in sync with shared/src/lib.rs TYPE constants
// pub const DESCRIPTOR_CUSTOM_REF_FLAG: Descriptor = Descriptor { __x: *b"  1", };
pub const DESCRIPTOR_NUMBER: Descriptor = Descriptor { __x: *b"   2", };
pub const DESCRIPTOR_BORROWED_STR: Descriptor = Descriptor { __x: *b"   3", };
pub const DESCRIPTOR_STRING: Descriptor = Descriptor { __x: *b"   4", };
pub const DESCRIPTOR_BOOLEAN: Descriptor = Descriptor { __x: *b"   5", };

pub const DESCRIPTOR_JS_OWNED: Descriptor = Descriptor { __x: *b"  22", };
pub const DESCRIPTOR_JS_REF: Descriptor = Descriptor { __x: *b"  23", };

pub trait WasmBoundary {
    type Abi: WasmAbi;
    const DESCRIPTOR: Descriptor;

    fn into_abi(self, extra: &mut Stack) -> Self::Abi;
    unsafe fn from_abi(js: Self::Abi, extra: &mut Stack) -> Self;
}

pub trait FromRefWasmBoundary {
    type Abi: WasmAbi;
    const DESCRIPTOR: Descriptor;
    type RefAnchor: Deref<Target = Self>;

    unsafe fn from_abi_ref(js: Self::Abi, extra: &mut Stack) -> Self::RefAnchor;
}

pub trait FromRefMutWasmBoundary {
    type Abi: WasmAbi;
    const DESCRIPTOR: Descriptor;
    type RefAnchor: DerefMut<Target = Self>;

    unsafe fn from_abi_ref_mut(js: Self::Abi, extra: &mut Stack) -> Self::RefAnchor;
}

pub trait ToRefWasmBoundary {
    type Abi: WasmAbi;
    const DESCRIPTOR: Descriptor;

    fn to_abi_ref(&self, extra: &mut Stack) -> u32;
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
unsafe impl WasmAbi for f32 {}
unsafe impl WasmAbi for f64 {}

macro_rules! simple {
    ($($t:tt)*) => ($(
        impl WasmBoundary for $t {
            type Abi = $t;
            const DESCRIPTOR: Descriptor = DESCRIPTOR_NUMBER;

            fn into_abi(self, _extra: &mut Stack) -> $t { self }
            unsafe fn from_abi(js: $t, _extra: &mut Stack) -> $t { js }
        }
    )*)
}

simple!(u32 u64 f32 f64);

macro_rules! as_u32 {
    ($($t:tt)*) => ($(
        impl WasmBoundary for $t {
            type Abi = u32;
            const DESCRIPTOR: Descriptor = DESCRIPTOR_NUMBER;

            fn into_abi(self, _extra: &mut Stack) -> u32 { self as u32 }
            unsafe fn from_abi(js: u32, _extra: &mut Stack) -> $t { js as $t }
        }
    )*)
}

as_u32!(i8 u8 i16 u16 i32 isize usize);

impl WasmBoundary for bool {
    type Abi = u32;
    const DESCRIPTOR: Descriptor = DESCRIPTOR_BOOLEAN;

    fn into_abi(self, _extra: &mut Stack) -> u32 { self as u32 }
    unsafe fn from_abi(js: u32, _extra: &mut Stack) -> bool { js != 0 }
}

impl<T> WasmBoundary for *const T {
    type Abi = u32;
    const DESCRIPTOR: Descriptor = DESCRIPTOR_NUMBER;

    fn into_abi(self, _extra: &mut Stack) -> u32 { self as u32 }
    unsafe fn from_abi(js: u32, _extra: &mut Stack) -> *const T { js as *const T }
}

impl<T> WasmBoundary for *mut T {
    type Abi = u32;
    const DESCRIPTOR: Descriptor = DESCRIPTOR_NUMBER;

    fn into_abi(self, _extra: &mut Stack) -> u32 { self as u32 }
    unsafe fn from_abi(js: u32, _extra: &mut Stack) -> *mut T { js as *mut T }
}

macro_rules! vectors {
    ($($t:ident => ($slice:expr, $owned:expr))*) => ($(
        impl WasmBoundary for Box<[$t]> {
            type Abi = u32;
            const DESCRIPTOR: Descriptor = Descriptor { __x: *$owned };

            fn into_abi(self, extra: &mut Stack) -> u32 {
                let ptr = self.as_ptr();
                let len = self.len();
                mem::forget(self);
                extra.push(len as u32);
                ptr.into_abi(extra)
            }

            unsafe fn from_abi(js: u32, extra: &mut Stack) -> Box<[$t]> {
                let ptr = <*mut $t>::from_abi(js, extra);
                let len = extra.pop() as usize;
                Vec::from_raw_parts(ptr, len, len).into_boxed_slice()
            }
        }

        impl ToRefWasmBoundary for [$t] {
            type Abi = u32;
            const DESCRIPTOR: Descriptor = Descriptor { __x: *$slice };

            fn to_abi_ref(&self, extra: &mut Stack) -> u32 {
                let ptr = self.as_ptr();
                let len = self.len();
                extra.push(len as u32);
                ptr.into_abi(extra)
            }
        }

        impl FromRefWasmBoundary for [$t] {
            type Abi = u32;
            const DESCRIPTOR: Descriptor = Descriptor { __x: *$slice };
            type RefAnchor = SliceAnchor<$t>;

            unsafe fn from_abi_ref(js: u32, extra: &mut Stack) -> SliceAnchor<$t> {
                SliceAnchor {
                    ptr: <*mut $t>::from_abi(js, extra),
                    len: extra.pop() as usize,
                }
            }
        }
    )*)
}

pub struct SliceAnchor<T> {
    ptr: *mut T,
    len: usize,
}

impl<T> Deref for SliceAnchor<T> {
    type Target = [T];
    fn deref(&self) -> &[T] {
        unsafe { slice::from_raw_parts(self.ptr, self.len) }
    }
}

vectors! {
    u8 => (b"   6", b"   7")
    i8 => (b"   8", b"   9")
    u16 => (b"  10", b"  11")
    i16 => (b"  12", b"  13")
    u32 => (b"  14", b"  15")
    i32 => (b"  16", b"  17")
    f32 => (b"  18", b"  19")
    f64 => (b"  20", b"  21")
}

impl<T> WasmBoundary for Vec<T> where Box<[T]>: WasmBoundary {
    type Abi = <Box<[T]> as WasmBoundary>::Abi;
    const DESCRIPTOR: Descriptor = <Box<[T]> as WasmBoundary>::DESCRIPTOR;

    fn into_abi(self, extra: &mut Stack) -> Self::Abi {
        self.into_boxed_slice().into_abi(extra)
    }

    unsafe fn from_abi(js: Self::Abi, extra: &mut Stack) -> Self {
        <Box<[T]>>::from_abi(js, extra).into()
    }
}

impl WasmBoundary for String {
    type Abi = u32;
    const DESCRIPTOR: Descriptor = DESCRIPTOR_STRING;

    fn into_abi(self, extra: &mut Stack) -> u32 {
        self.into_bytes().into_abi(extra)
    }

    unsafe fn from_abi(js: u32, extra: &mut Stack) -> String {
        String::from_utf8_unchecked(Vec::from_abi(js, extra))
    }
}

impl ToRefWasmBoundary for str {
    type Abi = <[u8] as ToRefWasmBoundary>::Abi;
    const DESCRIPTOR: Descriptor = DESCRIPTOR_BORROWED_STR;

    fn to_abi_ref(&self, extra: &mut Stack) -> Self::Abi {
        self.as_bytes().to_abi_ref(extra)
    }
}

impl FromRefWasmBoundary for str {
    type Abi = <[u8] as ToRefWasmBoundary>::Abi;
    const DESCRIPTOR: Descriptor = DESCRIPTOR_BORROWED_STR;
    type RefAnchor = StrAnchor;

    unsafe fn from_abi_ref(js: Self::Abi, extra: &mut Stack) -> Self::RefAnchor {
        StrAnchor { inner: <[u8]>::from_abi_ref(js, extra) }
    }
}

pub struct StrAnchor {
    inner: SliceAnchor<u8>,
}

impl Deref for StrAnchor {
    type Target = str;

    fn deref(&self) -> &str {
        unsafe { str::from_utf8_unchecked(&self.inner) }
    }
}

impl WasmBoundary for JsValue {
    type Abi = u32;
    const DESCRIPTOR: Descriptor = DESCRIPTOR_JS_OWNED;

    fn into_abi(self, _extra: &mut Stack) -> u32 {
        let ret = self.idx;
        mem::forget(self);
        return ret
    }

    unsafe fn from_abi(js: u32, _extra: &mut Stack) -> JsValue {
        JsValue { idx: js }
    }
}

impl ToRefWasmBoundary for JsValue {
    type Abi = u32;
    const DESCRIPTOR: Descriptor = DESCRIPTOR_JS_REF;
    fn to_abi_ref(&self, _extra: &mut Stack) -> u32 {
        self.idx
    }
}

impl FromRefWasmBoundary for JsValue {
    type Abi = u32;
    const DESCRIPTOR: Descriptor = DESCRIPTOR_JS_REF;
    type RefAnchor = ManuallyDrop<JsValue>;

    unsafe fn from_abi_ref(js: u32, _extra: &mut Stack) -> ManuallyDrop<JsValue> {
        ManuallyDrop::new(JsValue { idx: js })
    }
}

impl WasmBoundary for Box<[JsValue]> {
    type Abi = u32;
    const DESCRIPTOR: Descriptor = Descriptor { __x: *b"   0" };

    fn into_abi(self, extra: &mut Stack) -> u32 {
        let ptr = self.as_ptr();
        let len = self.len();
        mem::forget(self);
        extra.push(len as u32);
        ptr.into_abi(extra)
    }

    unsafe fn from_abi(js: u32, extra: &mut Stack) -> Box<[JsValue]> {
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
