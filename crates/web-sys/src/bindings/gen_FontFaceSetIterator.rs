use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `FontFaceSetIterator` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FontFaceSetIterator)\n\n*This API requires the following crate features to be activated: `FontFaceSetIterator`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct FontFaceSetIterator {
    obj: ::js_sys::Object,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_FontFaceSetIterator: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for FontFaceSetIterator {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(19u32);
            inform(70u32);
            inform(111u32);
            inform(110u32);
            inform(116u32);
            inform(70u32);
            inform(97u32);
            inform(99u32);
            inform(101u32);
            inform(83u32);
            inform(101u32);
            inform(116u32);
            inform(73u32);
            inform(116u32);
            inform(101u32);
            inform(114u32);
            inform(97u32);
            inform(116u32);
            inform(111u32);
            inform(114u32);
        }
    }
    impl core::ops::Deref for FontFaceSetIterator {
        type Target = ::js_sys::Object;
        #[inline]
        fn deref(&self) -> &::js_sys::Object {
            &self.obj
        }
    }
    impl IntoWasmAbi for FontFaceSetIterator {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for FontFaceSetIterator {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a FontFaceSetIterator {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for FontFaceSetIterator {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            FontFaceSetIterator {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for FontFaceSetIterator {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a FontFaceSetIterator {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for FontFaceSetIterator {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<FontFaceSetIterator>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(FontFaceSetIterator {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for FontFaceSetIterator {
        #[inline]
        fn from(obj: JsValue) -> FontFaceSetIterator {
            FontFaceSetIterator { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for FontFaceSetIterator {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<FontFaceSetIterator> for FontFaceSetIterator {
        #[inline]
        fn as_ref(&self) -> &FontFaceSetIterator {
            self
        }
    }
    impl From<FontFaceSetIterator> for JsValue {
        #[inline]
        fn from(obj: FontFaceSetIterator) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for FontFaceSetIterator {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_FontFaceSetIterator(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_FontFaceSetIterator(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_FontFaceSetIterator(idx) != 0
            }
        }
        #[inline]
        fn is_type_of(val: &JsValue) -> bool {
            let is_type_of: fn(&JsValue) -> bool = |_| false;
            is_type_of(val)
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            FontFaceSetIterator { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const FontFaceSetIterator) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<FontFaceSetIterator> for ::js_sys::Object {
    #[inline]
    fn from(obj: FontFaceSetIterator) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for FontFaceSetIterator {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "FontFaceSetIterator", feature = "FontFaceSetIteratorResult",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_next_FontFaceSetIterator() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&FontFaceSetIterator as WasmDescribe>::describe();
    <FontFaceSetIteratorResult as WasmDescribe>::describe();
}
impl FontFaceSetIterator {
    #[cfg(all(feature = "FontFaceSetIterator", feature = "FontFaceSetIteratorResult",))]
    #[allow(bad_style)]
    #[doc = "The `next()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FontFaceSetIterator/next)\n\n*This API requires the following crate features to be activated: `FontFaceSetIterator`, `FontFaceSetIteratorResult`*"]
    #[allow(clippy::all)]
    pub fn next(&self) -> Result<FontFaceSetIteratorResult, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "FontFaceSetIterator", feature = "FontFaceSetIteratorResult",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_next_FontFaceSetIterator(
                self_: <&FontFaceSetIterator as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <FontFaceSetIteratorResult as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_next_FontFaceSetIterator(
            self_: <&FontFaceSetIterator as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <FontFaceSetIteratorResult as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&FontFaceSetIterator as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_next_FontFaceSetIterator(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<FontFaceSetIteratorResult as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_fe4509224a734a4e: [u8; 244usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\xB2\0\0\0\0\0\x02\0\0\x02\x13FontFaceSetIterator%__widl_instanceof_FontFaceSetIterator\0\0\0\0!__widl_f_next_FontFaceSetIterator\x01\0\0\x01\x13FontFaceSetIterator\x01\0\0\x01\x01\x05self_\x04next\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
