use std::convert::{TryFrom, TryInto};
#[cfg(feature = "std")]
use std::prelude::v1::*;

use core::ops::{Deref, DerefMut};
use core::str;

use crate::__wbindgen_copy_to_typed_array;
use crate::cast::JsObject;
use crate::convert::OptionIntoWasmAbi;
use crate::convert::{
    FromWasmAbi, IntoWasmAbi, LongRefFromWasmAbi, RefFromWasmAbi, RefMutFromWasmAbi, WasmAbi,
};
use crate::describe;
use crate::describe::WasmDescribe;
use cfg_if::cfg_if;

if_std! {
    use core::mem;
    use crate::convert::OptionFromWasmAbi;
}

#[repr(C)]
pub struct WasmSlice {
    pub ptr: u32,
    pub len: u32,
}

unsafe impl WasmAbi for WasmSlice {}

#[inline]
fn null_slice() -> WasmSlice {
    WasmSlice { ptr: 0, len: 0 }
}

if_std! {
    #[repr(C)]
    pub struct WasmMutSlice {
        pub slice: WasmSlice,
        pub idx: u32,
    }

    unsafe impl WasmAbi for WasmMutSlice {}

    /// The representation of a mutable slice passed from JS to Rust.
    pub struct MutSlice<T> {
        /// A copy of the data in the JS typed array.
        contents: Box<[T]>,
        /// A reference to the original JS typed array.
        js: JsValue,
    }

    impl<T> Drop for MutSlice<T> {
        fn drop(&mut self) {
            unsafe {
                __wbindgen_copy_to_typed_array(
                    self.contents.as_ptr() as *const u8,
                    self.contents.len() * mem::size_of::<T>(),
                    self.js.idx
                );
            }
        }
    }

    impl<T> Deref for MutSlice<T> {
        type Target = [T];

        fn deref(&self) -> &[T] {
            &self.contents
        }
    }

    impl<T> DerefMut for MutSlice<T> {
        fn deref_mut(&mut self) -> &mut [T] {
            &mut self.contents
        }
    }
}

macro_rules! vectors {
    ($($t:ident)*) => ($(
        if_std! {
            impl WasmDescribe for Box<[$t]> {
                fn describe() {
                    describe::inform(describe::VECTOR);
                    $t::describe();
                }
            }

            impl IntoWasmAbi for Box<[$t]> {
                type Abi = WasmSlice;

                #[inline]
                fn into_abi(self) -> WasmSlice {
                    let ptr = self.as_ptr();
                    let len = self.len();
                    mem::forget(self);
                    WasmSlice {
                        ptr: ptr.into_abi(),
                        len: len as u32,
                    }
                }
            }

            impl OptionIntoWasmAbi for Box<[$t]> {
                #[inline]
                fn none() -> WasmSlice { null_slice() }
            }

            impl FromWasmAbi for Box<[$t]> {
                type Abi = WasmSlice;

                #[inline]
                unsafe fn from_abi(js: WasmSlice) -> Self {
                    let ptr = <*mut $t>::from_abi(js.ptr);
                    let len = js.len as usize;
                    Vec::from_raw_parts(ptr, len, len).into_boxed_slice()
                }
            }

            impl OptionFromWasmAbi for Box<[$t]> {
                #[inline]
                fn is_none(slice: &WasmSlice) -> bool { slice.ptr == 0 }
            }
        }

        impl<'a> IntoWasmAbi for &'a [$t] {
            type Abi = WasmSlice;

            #[inline]
            fn into_abi(self) -> WasmSlice {
                WasmSlice {
                    ptr: self.as_ptr().into_abi(),
                    len: self.len() as u32,
                }
            }
        }

        impl<'a> OptionIntoWasmAbi for &'a [$t] {
            #[inline]
            fn none() -> WasmSlice { null_slice() }
        }

        impl<'a> IntoWasmAbi for &'a mut [$t] {
            type Abi = WasmSlice;

            #[inline]
            fn into_abi(self) -> WasmSlice {
                (&*self).into_abi()
            }
        }

        impl<'a> OptionIntoWasmAbi for &'a mut [$t] {
            #[inline]
            fn none() -> WasmSlice { null_slice() }
        }

        impl RefFromWasmAbi for [$t] {
            type Abi = WasmSlice;
            type Anchor = Box<[$t]>;

            #[inline]
            unsafe fn ref_from_abi(js: WasmSlice) -> Box<[$t]> {
                <Box<[$t]> as FromWasmAbi>::from_abi(js)
            }
        }

        impl RefMutFromWasmAbi for [$t] {
            type Abi = WasmMutSlice;
            type Anchor = MutSlice<$t>;

            #[inline]
            unsafe fn ref_mut_from_abi(js: WasmMutSlice) -> MutSlice<$t> {
                let contents = <Box<[$t]> as FromWasmAbi>::from_abi(js.slice);
                let js = JsValue::from_abi(js.idx);
                MutSlice { contents, js }
            }
        }

        impl LongRefFromWasmAbi for [$t] {
            type Abi = WasmSlice;
            type Anchor = Box<[$t]>;

            #[inline]
            unsafe fn long_ref_from_abi(js: WasmSlice) -> Box<[$t]> {
                Self::ref_from_abi(js)
            }
        }
    )*)
}

vectors! {
    u8 i8 u16 i16 u32 i32 u64 i64 usize isize f32 f64
}

/// Enables blanket implementations of `WasmDescribe`, `IntoWasmAbi`,
/// `FromWasmAbi` and `OptionIntoWasmAbi` functionality on boxed slices of
/// types which can be converted to and from `JsValue` without conflicting
/// implementations of those traits.
///
/// Implementing these traits directly with blanket implementations would
/// be much more elegant, but unfortunately that's impossible because it
/// conflicts with the implementations for `Box<[T]> where T: JsObject`.
pub trait JsValueVector {
    type ToAbi;
    type FromAbi;

    fn describe();
    fn into_abi(self) -> Self::ToAbi;
    fn none() -> Self::ToAbi;
    unsafe fn from_abi(js: Self::FromAbi) -> Self;
}

/*
 * Generates implementations for traits necessary for passing types to and from
 * JavaScript on boxed slices of values which can be converted to and from
 * `JsValue`.
 */
macro_rules! js_value_vectors {
    ($($t:ident)*) => ($(
        if_std! {
            impl WasmDescribe for Box<[$t]> {
                fn describe() {
                    <Self as JsValueVector>::describe();
                }
            }

            impl IntoWasmAbi for Box<[$t]> {
                type Abi = <Self as JsValueVector>::ToAbi;

                fn into_abi(self) -> Self::Abi {
                    <Self as JsValueVector>::into_abi(self)
                }
            }

            impl OptionIntoWasmAbi for Box<[$t]> {
                fn none() -> Self::Abi {
                    <Self as JsValueVector>::none()
                }
            }

            impl FromWasmAbi for Box<[$t]> {
                type Abi = <Self as JsValueVector>::FromAbi;

                unsafe fn from_abi(js: Self::Abi) -> Self {
                    <Self as JsValueVector>::from_abi(js)
                }
            }
        }
    )*)
}

if_std! {
    impl<T> JsValueVector for Box<[T]> where
        T: Into<JsValue> + TryFrom<JsValue>,
        <T as TryFrom<JsValue>>::Error: core::fmt::Debug {
        type ToAbi = <Box<[JsValue]> as IntoWasmAbi>::Abi;
        type FromAbi = <Box<[JsValue]> as FromWasmAbi>::Abi;

        fn describe() {
            describe::inform(describe::VECTOR);
            JsValue::describe();
        }

        fn into_abi(self) -> Self::ToAbi {
            let js_vals: Box::<[JsValue]> = self
                .into_vec()
                .into_iter()
                .map(|x| x.into())
                .collect();

            IntoWasmAbi::into_abi(js_vals)
        }

        fn none() -> Self::ToAbi {
            <Box<[JsValue]> as OptionIntoWasmAbi>::none()
        }

        unsafe fn from_abi(js: Self::FromAbi) -> Self {
            let js_vals = <Vec<JsValue> as FromWasmAbi>::from_abi(js);

            js_vals
                .into_iter()
                .map(|x| x.try_into().expect("Array element of wrong type"))
                .collect()
        }
    }

    impl TryFrom<JsValue> for String {
        type Error = ();

        fn try_from(value: JsValue) -> Result<Self, Self::Error> {
            match value.as_string() {
                Some(s) => Ok(s),
                None => Err(()),
            }
        }
    }
}

js_value_vectors! {
    String
}

cfg_if! {
    if #[cfg(feature = "enable-interning")] {
        #[inline]
        fn unsafe_get_cached_str(x: &str) -> Option<WasmSlice> {
            // This uses 0 for the ptr as an indication that it is a JsValue and not a str.
            crate::cache::intern::unsafe_get_str(x).map(|x| WasmSlice { ptr: 0, len: x })
        }

    } else {
        #[inline]
        fn unsafe_get_cached_str(_x: &str) -> Option<WasmSlice> {
            None
        }
    }
}

if_std! {
    impl<T> IntoWasmAbi for Vec<T> where Box<[T]>: IntoWasmAbi<Abi = WasmSlice> {
        type Abi = <Box<[T]> as IntoWasmAbi>::Abi;

        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.into_boxed_slice().into_abi()
        }
    }

    impl<T> OptionIntoWasmAbi for Vec<T> where Box<[T]>: IntoWasmAbi<Abi = WasmSlice> {
        #[inline]
        fn none() -> WasmSlice { null_slice() }
    }

    impl<T> FromWasmAbi for Vec<T> where Box<[T]>: FromWasmAbi<Abi = WasmSlice> {
        type Abi = <Box<[T]> as FromWasmAbi>::Abi;

        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            <Box<[T]>>::from_abi(js).into()
        }
    }

    impl<T> OptionFromWasmAbi for Vec<T> where Box<[T]>: FromWasmAbi<Abi = WasmSlice> {
        #[inline]
        fn is_none(abi: &WasmSlice) -> bool { abi.ptr == 0 }
    }

    impl IntoWasmAbi for String {
        type Abi = <Vec<u8> as IntoWasmAbi>::Abi;

        #[inline]
        fn into_abi(self) -> Self::Abi {
            // This is safe because the JsValue is immediately looked up in the heap and
            // then returned, so use-after-free cannot occur.
            unsafe_get_cached_str(&self).unwrap_or_else(|| self.into_bytes().into_abi())
        }
    }

    impl OptionIntoWasmAbi for String {
        #[inline]
        fn none() -> Self::Abi { null_slice() }
    }

    impl FromWasmAbi for String {
        type Abi = <Vec<u8> as FromWasmAbi>::Abi;

        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            String::from_utf8_unchecked(<Vec<u8>>::from_abi(js))
        }
    }

    impl OptionFromWasmAbi for String {
        #[inline]
        fn is_none(slice: &WasmSlice) -> bool { slice.ptr == 0 }
    }
}

impl<'a> IntoWasmAbi for &'a str {
    type Abi = <&'a [u8] as IntoWasmAbi>::Abi;

    #[inline]
    fn into_abi(self) -> Self::Abi {
        // This is safe because the JsValue is immediately looked up in the heap and
        // then returned, so use-after-free cannot occur.
        unsafe_get_cached_str(self).unwrap_or_else(|| self.as_bytes().into_abi())
    }
}

impl<'a> OptionIntoWasmAbi for &'a str {
    #[inline]
    fn none() -> Self::Abi {
        null_slice()
    }
}

impl RefFromWasmAbi for str {
    type Abi = <[u8] as RefFromWasmAbi>::Abi;
    type Anchor = Box<str>;

    #[inline]
    unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
        mem::transmute::<Box<[u8]>, Box<str>>(<Box<[u8]>>::from_abi(js))
    }
}

impl LongRefFromWasmAbi for str {
    type Abi = <[u8] as RefFromWasmAbi>::Abi;
    type Anchor = Box<str>;

    #[inline]
    unsafe fn long_ref_from_abi(js: Self::Abi) -> Self::Anchor {
        Self::ref_from_abi(js)
    }
}

if_std! {
    use crate::JsValue;

    impl IntoWasmAbi for Box<[JsValue]> {
        type Abi = WasmSlice;

        #[inline]
        fn into_abi(self) -> WasmSlice {
            let ptr = self.as_ptr();
            let len = self.len();
            mem::forget(self);
            WasmSlice {
                ptr: ptr.into_abi(),
                len: len as u32,
            }
        }
    }

    impl OptionIntoWasmAbi for Box<[JsValue]> {
        #[inline]
        fn none() -> WasmSlice { null_slice() }
    }

    impl FromWasmAbi for Box<[JsValue]> {
        type Abi = WasmSlice;

        #[inline]
        unsafe fn from_abi(js: WasmSlice) -> Self {
            let ptr = <*mut JsValue>::from_abi(js.ptr);
            let len = js.len as usize;
            Vec::from_raw_parts(ptr, len, len).into_boxed_slice()
        }
    }

    impl OptionFromWasmAbi for Box<[JsValue]> {
        #[inline]
        fn is_none(slice: &WasmSlice) -> bool { slice.ptr == 0 }
    }

    impl<T> IntoWasmAbi for Box<[T]> where T: JsObject {
        type Abi = WasmSlice;

        #[inline]
        fn into_abi(self) -> WasmSlice {
            let ptr = self.as_ptr();
            let len = self.len();
            mem::forget(self);
            WasmSlice {
                ptr: ptr.into_abi(),
                len: len as u32,
            }
        }
    }

    impl<T> OptionIntoWasmAbi for Box<[T]> where T: JsObject {
        #[inline]
        fn none() -> WasmSlice { null_slice() }
    }

    impl<T> FromWasmAbi for Box<[T]> where T: JsObject {
        type Abi = WasmSlice;

        #[inline]
        unsafe fn from_abi(js: WasmSlice) -> Self {
            let ptr = <*mut JsValue>::from_abi(js.ptr);
            let len = js.len as usize;
            let vec: Vec<T> = Vec::from_raw_parts(ptr, len, len).drain(..).map(|js_value| T::unchecked_from_js(js_value)).collect();
            vec.into_boxed_slice()
        }
    }

    impl<T> OptionFromWasmAbi for Box<[T]> where T: JsObject {
        #[inline]
        fn is_none(slice: &WasmSlice) -> bool { slice.ptr == 0 }
    }
}
