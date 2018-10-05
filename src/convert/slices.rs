#[cfg(feature = "std")]
use std::prelude::v1::*;

use core::slice;
use core::str;

use convert::{FromWasmAbi, IntoWasmAbi, RefFromWasmAbi, RefMutFromWasmAbi};
use convert::{OptionIntoWasmAbi, Stack};
use convert::abi::WasmSlice;

if_std! {
    use core::mem;
    use convert::OptionFromWasmAbi;
}

#[inline]
fn null_slice() -> WasmSlice {
    WasmSlice { ptr: 0, len: 0 }
}

macro_rules! vectors {
    ($($t:ident)*) => ($(
        if_std! {
            impl IntoWasmAbi for Box<[$t]> {
                type Abi = WasmSlice;

                #[inline]
                fn into_abi(self, extra: &mut Stack) -> WasmSlice {
                    let ptr = self.as_ptr();
                    let len = self.len();
                    mem::forget(self);
                    WasmSlice {
                        ptr: ptr.into_abi(extra),
                        len: len as u32,
                    }
                }
            }

            impl OptionIntoWasmAbi for Box<[$t]> {
                fn none() -> WasmSlice { null_slice() }
            }

            impl FromWasmAbi for Box<[$t]> {
                type Abi = WasmSlice;

                #[inline]
                unsafe fn from_abi(js: WasmSlice, extra: &mut Stack) -> Self {
                    let ptr = <*mut $t>::from_abi(js.ptr, extra);
                    let len = js.len as usize;
                    Vec::from_raw_parts(ptr, len, len).into_boxed_slice()
                }
            }

            impl OptionFromWasmAbi for Box<[$t]> {
                fn is_none(slice: &WasmSlice) -> bool { slice.ptr == 0 }
            }
        }

        impl<'a> IntoWasmAbi for &'a [$t] {
            type Abi = WasmSlice;

            #[inline]
            fn into_abi(self, extra: &mut Stack) -> WasmSlice {
                WasmSlice {
                    ptr: self.as_ptr().into_abi(extra),
                    len: self.len() as u32,
                }
            }
        }

        impl<'a> OptionIntoWasmAbi for &'a [$t] {
            fn none() -> WasmSlice { null_slice() }
        }

        impl<'a> IntoWasmAbi for &'a mut [$t] {
            type Abi = WasmSlice;

            #[inline]
            fn into_abi(self, extra: &mut Stack) -> WasmSlice {
                (&*self).into_abi(extra)
            }
        }

        impl<'a> OptionIntoWasmAbi for &'a mut [$t] {
            fn none() -> WasmSlice { null_slice() }
        }

        impl RefFromWasmAbi for [$t] {
            type Abi = WasmSlice;
            type Anchor = &'static [$t];

            #[inline]
            unsafe fn ref_from_abi(js: WasmSlice, extra: &mut Stack) -> &'static [$t] {
                slice::from_raw_parts(
                    <*const $t>::from_abi(js.ptr, extra),
                    js.len as usize,
                )
            }
        }

        impl RefMutFromWasmAbi for [$t] {
            type Abi = WasmSlice;
            type Anchor = &'static mut [$t];

            #[inline]
            unsafe fn ref_mut_from_abi(js: WasmSlice, extra: &mut Stack)
                -> &'static mut [$t]
            {
                slice::from_raw_parts_mut(
                    <*mut $t>::from_abi(js.ptr, extra),
                    js.len as usize,
                )
            }
        }
    )*)
}

vectors! {
    u8 i8 u16 i16 u32 i32 u64 i64 f32 f64
}

if_std! {
    impl<T> IntoWasmAbi for Vec<T> where Box<[T]>: IntoWasmAbi<Abi = WasmSlice> {
        type Abi = <Box<[T]> as IntoWasmAbi>::Abi;

        fn into_abi(self, extra: &mut Stack) -> Self::Abi {
            self.into_boxed_slice().into_abi(extra)
        }
    }

    impl<T> OptionIntoWasmAbi for Vec<T> where Box<[T]>: IntoWasmAbi<Abi = WasmSlice> {
        fn none() -> WasmSlice { null_slice() }
    }

    impl<T> FromWasmAbi for Vec<T> where Box<[T]>: FromWasmAbi<Abi = WasmSlice> {
        type Abi = <Box<[T]> as FromWasmAbi>::Abi;

        unsafe fn from_abi(js: Self::Abi, extra: &mut Stack) -> Self {
            <Box<[T]>>::from_abi(js, extra).into()
        }
    }

    impl<T> OptionFromWasmAbi for Vec<T> where Box<[T]>: FromWasmAbi<Abi = WasmSlice> {
        fn is_none(abi: &WasmSlice) -> bool { abi.ptr == 0 }
    }

    impl IntoWasmAbi for String {
        type Abi = <Vec<u8> as IntoWasmAbi>::Abi;

        #[inline]
        fn into_abi(self, extra: &mut Stack) -> Self::Abi {
            self.into_bytes().into_abi(extra)
        }
    }

    impl OptionIntoWasmAbi for String {
        fn none() -> WasmSlice { null_slice() }
    }

    impl FromWasmAbi for String {
        type Abi = <Vec<u8> as FromWasmAbi>::Abi;

        #[inline]
        unsafe fn from_abi(js: Self::Abi, extra: &mut Stack) -> Self {
            String::from_utf8_unchecked(<Vec<u8>>::from_abi(js, extra))
        }
    }

    impl OptionFromWasmAbi for String {
        fn is_none(slice: &WasmSlice) -> bool { slice.ptr == 0 }
    }
}

impl<'a> IntoWasmAbi for &'a str {
    type Abi = <&'a [u8] as IntoWasmAbi>::Abi;

    #[inline]
    fn into_abi(self, extra: &mut Stack) -> Self::Abi {
        self.as_bytes().into_abi(extra)
    }
}

impl<'a> OptionIntoWasmAbi for &'a str {
    fn none() -> WasmSlice {
        null_slice()
    }
}

impl RefFromWasmAbi for str {
    type Abi = <[u8] as RefFromWasmAbi>::Abi;
    type Anchor = &'static str;

    #[inline]
    unsafe fn ref_from_abi(js: Self::Abi, extra: &mut Stack) -> Self::Anchor {
        str::from_utf8_unchecked(<[u8]>::ref_from_abi(js, extra))
    }
}

if_std! {
    use JsValue;

    impl IntoWasmAbi for Box<[JsValue]> {
        type Abi = WasmSlice;

        #[inline]
        fn into_abi(self, extra: &mut Stack) -> WasmSlice {
            let ptr = self.as_ptr();
            let len = self.len();
            mem::forget(self);
            WasmSlice {
                ptr: ptr.into_abi(extra),
                len: len as u32,
            }
        }
    }

    impl OptionIntoWasmAbi for Box<[JsValue]> {
        fn none() -> WasmSlice { null_slice() }
    }

    impl FromWasmAbi for Box<[JsValue]> {
        type Abi = WasmSlice;

        #[inline]
        unsafe fn from_abi(js: WasmSlice, extra: &mut Stack) -> Self {
            let ptr = <*mut JsValue>::from_abi(js.ptr, extra);
            let len = js.len as usize;
            Vec::from_raw_parts(ptr, len, len).into_boxed_slice()
        }
    }

    impl OptionFromWasmAbi for Box<[JsValue]> {
        fn is_none(slice: &WasmSlice) -> bool { slice.ptr == 0 }
    }
}
