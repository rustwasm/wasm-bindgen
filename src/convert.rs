use std::mem::{self, ManuallyDrop};
use std::ops::{Deref, DerefMut};

use super::JsObject;

// keep in sync with shared/src/lib.rs TYPE constants
pub const DESCRIPTOR_CUSTOM_REF_FLAG: u32 = 0x1;
pub const DESCRIPTOR_NUMBER: char = '\u{5e}';
pub const DESCRIPTOR_BOOLEAN: char = '\u{61}';
pub const DESCRIPTOR_JS_OWNED: char = '\u{62}';

pub trait WasmBoundary {
    type Js: WasmAbi;
    const DESCRIPTOR: char;

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
            const DESCRIPTOR: char = DESCRIPTOR_NUMBER;

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
            const DESCRIPTOR: char = DESCRIPTOR_NUMBER;

            fn into_js(self) -> u32 { self as u32 }
            unsafe fn from_js(js: u32) -> $t { js as $t }
        }
    )*)
}

as_u32!(i8 u8 i16 u16 i32 isize usize);

impl WasmBoundary for bool {
    type Js = u32;
    const DESCRIPTOR: char = DESCRIPTOR_BOOLEAN;

    fn into_js(self) -> u32 { self as u32 }
    unsafe fn from_js(js: u32) -> bool { js != 0 }
}

impl<T> WasmBoundary for *const T {
    type Js = u32;
    const DESCRIPTOR: char = DESCRIPTOR_NUMBER;

    fn into_js(self) -> u32 { self as u32 }
    unsafe fn from_js(js: u32) -> *const T { js as *const T }
}

impl<T> WasmBoundary for *mut T {
    type Js = u32;
    const DESCRIPTOR: char = DESCRIPTOR_NUMBER;

    fn into_js(self) -> u32 { self as u32 }
    unsafe fn from_js(js: u32) -> *mut T { js as *mut T }
}

impl WasmBoundary for JsObject {
    type Js = u32;
    const DESCRIPTOR: char = DESCRIPTOR_JS_OWNED;

    fn into_js(self) -> u32 {
        let ret = self.idx;
        mem::forget(self);
        return ret
    }

    unsafe fn from_js(js: u32) -> JsObject {
        JsObject { idx: js }
    }
}

impl ToRefWasmBoundary for JsObject {
    fn to_js_ref(&self) -> u32 {
        self.idx
    }
}

impl FromRefWasmBoundary for JsObject {
    type RefAnchor = ManuallyDrop<JsObject>;

    unsafe fn from_js_ref(js: u32) -> ManuallyDrop<JsObject> {
        ManuallyDrop::new(JsObject { idx: js })
    }
}
