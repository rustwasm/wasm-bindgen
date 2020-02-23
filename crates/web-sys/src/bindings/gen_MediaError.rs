use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `MediaError` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaError)\n\n*This API requires the following crate features to be activated: `MediaError`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct MediaError {
    obj: ::js_sys::Object,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_MediaError: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for MediaError {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(10u32);
            inform(77u32);
            inform(101u32);
            inform(100u32);
            inform(105u32);
            inform(97u32);
            inform(69u32);
            inform(114u32);
            inform(114u32);
            inform(111u32);
            inform(114u32);
        }
    }
    impl core::ops::Deref for MediaError {
        type Target = ::js_sys::Object;
        #[inline]
        fn deref(&self) -> &::js_sys::Object {
            &self.obj
        }
    }
    impl IntoWasmAbi for MediaError {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for MediaError {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a MediaError {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for MediaError {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            MediaError {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for MediaError {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a MediaError {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for MediaError {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<MediaError>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(MediaError {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for MediaError {
        #[inline]
        fn from(obj: JsValue) -> MediaError {
            MediaError { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for MediaError {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<MediaError> for MediaError {
        #[inline]
        fn as_ref(&self) -> &MediaError {
            self
        }
    }
    impl From<MediaError> for JsValue {
        #[inline]
        fn from(obj: MediaError) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for MediaError {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_MediaError(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_MediaError(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_MediaError(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            MediaError { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const MediaError) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<MediaError> for ::js_sys::Object {
    #[inline]
    fn from(obj: MediaError) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for MediaError {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "MediaError",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_code_MediaError() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&MediaError as WasmDescribe>::describe();
    <u16 as WasmDescribe>::describe();
}
impl MediaError {
    #[cfg(all(feature = "MediaError",))]
    #[allow(bad_style)]
    #[doc = "The `code` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaError/code)\n\n*This API requires the following crate features to be activated: `MediaError`*"]
    #[allow(clippy::all)]
    pub fn code(&self) -> u16 {
        #[cfg(all(feature = "MediaError",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_code_MediaError(
                self_: <&MediaError as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <u16 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_code_MediaError(
            self_: <&MediaError as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <u16 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&MediaError as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_code_MediaError(self_)
            };
            <u16 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "MediaError",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_message_MediaError() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&MediaError as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl MediaError {
    #[cfg(all(feature = "MediaError",))]
    #[allow(bad_style)]
    #[doc = "The `message` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaError/message)\n\n*This API requires the following crate features to be activated: `MediaError`*"]
    #[allow(clippy::all)]
    pub fn message(&self) -> String {
        #[cfg(all(feature = "MediaError",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_message_MediaError(
                self_: <&MediaError as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_message_MediaError(
            self_: <&MediaError as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&MediaError as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_message_MediaError(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
impl MediaError {
    pub const MEDIA_ERR_ABORTED: u16 = 1u64 as u16;
}
impl MediaError {
    pub const MEDIA_ERR_NETWORK: u16 = 2u64 as u16;
}
impl MediaError {
    pub const MEDIA_ERR_DECODE: u16 = 3u64 as u16;
}
impl MediaError {
    pub const MEDIA_ERR_SRC_NOT_SUPPORTED: u16 = 4u64 as u16;
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_8fbea1ee3db04542: [u8; 286usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\xDC\0\0\0\0\0\x03\0\0\x02\nMediaError\x1C__widl_instanceof_MediaError\0\0\0\0\x18__widl_f_code_MediaError\0\0\0\x01\nMediaError\x01\0\x01\x04code\x01\x01\x05self_\x04code\0\0\0\x1B__widl_f_message_MediaError\0\0\0\x01\nMediaError\x01\0\x01\x07message\x01\x01\x05self_\x07message\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
