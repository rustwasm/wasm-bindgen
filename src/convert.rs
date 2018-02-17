use std::mem::{self, ManuallyDrop};
use std::ops::{Deref, DerefMut};

use super::JsValue;

// keep in sync with shared/src/lib.rs TYPE constants
pub const DESCRIPTOR_CUSTOM_REF_FLAG: u32 = 0x1;
pub const DESCRIPTOR_NUMBER: u32 = 0x5e;
pub const DESCRIPTOR_BOOLEAN: u32 = 0x61;
pub const DESCRIPTOR_JS_OWNED: u32 = 0x72;

pub trait WasmBoundary {
    type Js: WasmAbi;
    const DESCRIPTOR: u32;

    fn into_js(self) -> Self::Js;
    unsafe fn from_js(js: Self::Js) -> Self;
}

pub trait FromRefWasmBoundary: WasmBoundary {
    type RefAnchor: Deref<Target = Self>;

    unsafe fn from_js_ref(js: Self::Js) -> Self::RefAnchor;
}

pub trait FromRefMutWasmBoundary: WasmBoundary {
    type RefAnchor: DerefMut<Target = Self>;

    unsafe fn from_js_ref_mut(js: Self::Js) -> Self::RefAnchor;
}

pub trait ToRefWasmBoundary: WasmBoundary {
    fn to_js_ref(&self) -> u32;
}

pub unsafe trait WasmAbi {}

unsafe impl WasmAbi for u32 {}
unsafe impl WasmAbi for u64 {}
unsafe impl WasmAbi for f32 {}
unsafe impl WasmAbi for f64 {}

macro_rules! simple {
    ($($t:tt)*) => ($(
        impl WasmBoundary for $t {
            type Js = $t;
            const DESCRIPTOR: u32 = DESCRIPTOR_NUMBER;

            fn into_js(self) -> $t { self }
            unsafe fn from_js(js: $t) -> $t { js }
        }
    )*)
}

simple!(u32 u64 f32 f64);

macro_rules! as_u32 {
    ($($t:tt)*) => ($(
        impl WasmBoundary for $t {
            type Js = u32;
            const DESCRIPTOR: u32 = DESCRIPTOR_NUMBER;

            fn into_js(self) -> u32 { self as u32 }
            unsafe fn from_js(js: u32) -> $t { js as $t }
        }
    )*)
}

as_u32!(i8 u8 i16 u16 i32 isize usize);

impl WasmBoundary for bool {
    type Js = u32;
    const DESCRIPTOR: u32 = DESCRIPTOR_BOOLEAN;

    fn into_js(self) -> u32 { self as u32 }
    unsafe fn from_js(js: u32) -> bool { js != 0 }
}

impl<T> WasmBoundary for *const T {
    type Js = u32;
    const DESCRIPTOR: u32 = DESCRIPTOR_NUMBER;

    fn into_js(self) -> u32 { self as u32 }
    unsafe fn from_js(js: u32) -> *const T { js as *const T }
}

impl<T> WasmBoundary for *mut T {
    type Js = u32;
    const DESCRIPTOR: u32 = DESCRIPTOR_NUMBER;

    fn into_js(self) -> u32 { self as u32 }
    unsafe fn from_js(js: u32) -> *mut T { js as *mut T }
}

impl WasmBoundary for JsValue {
    type Js = u32;
    const DESCRIPTOR: u32 = DESCRIPTOR_JS_OWNED;

    fn into_js(self) -> u32 {
        let ret = self.idx;
        mem::forget(self);
        return ret
    }

    unsafe fn from_js(js: u32) -> JsValue {
        JsValue { idx: js }
    }
}

impl ToRefWasmBoundary for JsValue {
    fn to_js_ref(&self) -> u32 {
        self.idx
    }
}

impl FromRefWasmBoundary for JsValue {
    type RefAnchor = ManuallyDrop<JsValue>;

    unsafe fn from_js_ref(js: u32) -> ManuallyDrop<JsValue> {
        ManuallyDrop::new(JsValue { idx: js })
    }
}
