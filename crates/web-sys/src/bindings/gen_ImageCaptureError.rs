use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `ImageCaptureError` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ImageCaptureError)\n\n*This API requires the following crate features to be activated: `ImageCaptureError`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct ImageCaptureError {
    obj: ::js_sys::Object,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_ImageCaptureError: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for ImageCaptureError {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(17u32);
            inform(73u32);
            inform(109u32);
            inform(97u32);
            inform(103u32);
            inform(101u32);
            inform(67u32);
            inform(97u32);
            inform(112u32);
            inform(116u32);
            inform(117u32);
            inform(114u32);
            inform(101u32);
            inform(69u32);
            inform(114u32);
            inform(114u32);
            inform(111u32);
            inform(114u32);
        }
    }
    impl core::ops::Deref for ImageCaptureError {
        type Target = ::js_sys::Object;
        #[inline]
        fn deref(&self) -> &::js_sys::Object {
            &self.obj
        }
    }
    impl IntoWasmAbi for ImageCaptureError {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for ImageCaptureError {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a ImageCaptureError {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for ImageCaptureError {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            ImageCaptureError {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for ImageCaptureError {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a ImageCaptureError {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for ImageCaptureError {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<ImageCaptureError>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(ImageCaptureError {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for ImageCaptureError {
        #[inline]
        fn from(obj: JsValue) -> ImageCaptureError {
            ImageCaptureError { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for ImageCaptureError {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<ImageCaptureError> for ImageCaptureError {
        #[inline]
        fn as_ref(&self) -> &ImageCaptureError {
            self
        }
    }
    impl From<ImageCaptureError> for JsValue {
        #[inline]
        fn from(obj: ImageCaptureError) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for ImageCaptureError {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_ImageCaptureError(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_ImageCaptureError(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_ImageCaptureError(idx) != 0
            }
        }
        #[inline]
        fn is_type_of(val: &JsValue) -> bool {
            let is_type_of: fn(&JsValue) -> bool = |_| false;
            is_type_of(val)
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            ImageCaptureError { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const ImageCaptureError) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<ImageCaptureError> for ::js_sys::Object {
    #[inline]
    fn from(obj: ImageCaptureError) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for ImageCaptureError {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "ImageCaptureError",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_code_ImageCaptureError() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&ImageCaptureError as WasmDescribe>::describe();
    <u16 as WasmDescribe>::describe();
}
impl ImageCaptureError {
    #[cfg(all(feature = "ImageCaptureError",))]
    #[allow(bad_style)]
    #[doc = "The `code` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ImageCaptureError/code)\n\n*This API requires the following crate features to be activated: `ImageCaptureError`*"]
    #[allow(clippy::all)]
    pub fn code(&self) -> u16 {
        #[cfg(all(feature = "ImageCaptureError",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_code_ImageCaptureError(
                self_: <&ImageCaptureError as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <u16 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_code_ImageCaptureError(
            self_: <&ImageCaptureError as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <u16 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&ImageCaptureError as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_code_ImageCaptureError(self_)
            };
            <u16 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "ImageCaptureError",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_message_ImageCaptureError() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&ImageCaptureError as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl ImageCaptureError {
    #[cfg(all(feature = "ImageCaptureError",))]
    #[allow(bad_style)]
    #[doc = "The `message` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ImageCaptureError/message)\n\n*This API requires the following crate features to be activated: `ImageCaptureError`*"]
    #[allow(clippy::all)]
    pub fn message(&self) -> String {
        #[cfg(all(feature = "ImageCaptureError",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_message_ImageCaptureError(
                self_: <&ImageCaptureError as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_message_ImageCaptureError(
            self_: <&ImageCaptureError as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&ImageCaptureError as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_message_ImageCaptureError(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
impl ImageCaptureError {
    pub const FRAME_GRAB_ERROR: u16 = 1u64 as u16;
}
impl ImageCaptureError {
    pub const SETTINGS_ERROR: u16 = 2u64 as u16;
}
impl ImageCaptureError {
    pub const PHOTO_ERROR: u16 = 3u64 as u16;
}
impl ImageCaptureError {
    pub const ERROR_UNKNOWN: u16 = 4u64 as u16;
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_7abadfc42929dc4d: [u8; 328usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\x06\x01\0\0\0\0\x03\0\0\x02\x11ImageCaptureError#__widl_instanceof_ImageCaptureError\0\0\0\0\x1F__widl_f_code_ImageCaptureError\0\0\0\x01\x11ImageCaptureError\x01\0\x01\x04code\x01\x01\x05self_\x04code\0\0\0\"__widl_f_message_ImageCaptureError\0\0\0\x01\x11ImageCaptureError\x01\0\x01\x07message\x01\x01\x05self_\x07message\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
