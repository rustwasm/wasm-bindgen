use core::char;
use core::mem::{self, ManuallyDrop};

use convert::slices::WasmSlice;
use convert::{Stack, FromWasmAbi, IntoWasmAbi, RefFromWasmAbi};
use JsValue;

macro_rules! simple {
    ($($t:tt)*) => ($(
        impl IntoWasmAbi for $t {
            type Abi = $t;

            #[inline]
            fn into_abi(self, _extra: &mut Stack) -> $t { self }
        }

        impl FromWasmAbi for $t {
            type Abi = $t;

            #[inline]
            unsafe fn from_abi(js: $t, _extra: &mut Stack) -> $t { js }
        }
    )*)
}

simple!(u32 i32 f32 f64);

macro_rules! sixtyfour {
    ($($t:tt)*) => ($(
        impl IntoWasmAbi for $t {
            type Abi = WasmSlice;

            #[inline]
            fn into_abi(self, _extra: &mut Stack) -> WasmSlice {
                WasmSlice {
                    ptr: self as u32,
                    len: (self >> 32) as u32,
                }
            }
        }

        impl FromWasmAbi for $t {
            type Abi = WasmSlice;

            #[inline]
            unsafe fn from_abi(js: WasmSlice, _extra: &mut Stack) -> $t {
                (js.ptr as $t) | ((js.len as $t) << 32)
            }
        }
    )*)
}

sixtyfour!(i64 u64);

macro_rules! as_u32 {
    ($($t:tt)*) => ($(
        impl IntoWasmAbi for $t {
            type Abi = u32;

            #[inline]
            fn into_abi(self, _extra: &mut Stack) -> u32 { self as u32 }
        }

        impl FromWasmAbi for $t {
            type Abi = u32;

            #[inline]
            unsafe fn from_abi(js: u32, _extra: &mut Stack) -> $t { js as $t }
        }
    )*)
}

as_u32!(i8 u8 i16 u16 isize usize);

impl IntoWasmAbi for bool {
    type Abi = u32;

    #[inline]
    fn into_abi(self, _extra: &mut Stack) -> u32 {
        self as u32
    }
}

impl FromWasmAbi for bool {
    type Abi = u32;

    #[inline]
    unsafe fn from_abi(js: u32, _extra: &mut Stack) -> bool {
        js != 0
    }
}

impl IntoWasmAbi for char {
    type Abi = u32;

    #[inline]
    fn into_abi(self, _extra: &mut Stack) -> u32 {
        self as u32
    }
}

impl FromWasmAbi for char {
    type Abi = u32;

    #[inline]
    unsafe fn from_abi(js: u32, _extra: &mut Stack) -> char {
        char::from_u32_unchecked(js)
    }
}

impl<T> IntoWasmAbi for *const T {
    type Abi = u32;

    fn into_abi(self, _extra: &mut Stack) -> u32 {
        self as u32
    }
}

impl<T> FromWasmAbi for *const T {
    type Abi = u32;

    unsafe fn from_abi(js: u32, _extra: &mut Stack) -> *const T {
        js as *const T
    }
}

impl<T> IntoWasmAbi for *mut T {
    type Abi = u32;

    fn into_abi(self, _extra: &mut Stack) -> u32 {
        self as u32
    }
}

impl<T> FromWasmAbi for *mut T {
    type Abi = u32;

    unsafe fn from_abi(js: u32, _extra: &mut Stack) -> *mut T {
        js as *mut T
    }
}

impl IntoWasmAbi for JsValue {
    type Abi = u32;

    #[inline]
    fn into_abi(self, _extra: &mut Stack) -> u32 {
        let ret = self.idx;
        mem::forget(self);
        return ret;
    }
}

impl FromWasmAbi for JsValue {
    type Abi = u32;

    #[inline]
    unsafe fn from_abi(js: u32, _extra: &mut Stack) -> JsValue {
        JsValue { idx: js }
    }
}

impl<'a> IntoWasmAbi for &'a JsValue {
    type Abi = u32;

    #[inline]
    fn into_abi(self, _extra: &mut Stack) -> u32 {
        self.idx
    }
}

impl RefFromWasmAbi for JsValue {
    type Abi = u32;
    type Anchor = ManuallyDrop<JsValue>;

    #[inline]
    unsafe fn ref_from_abi(js: u32, _extra: &mut Stack) -> Self::Anchor {
        ManuallyDrop::new(JsValue { idx: js })
    }
}
