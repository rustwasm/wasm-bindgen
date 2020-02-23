use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `ImageCaptureErrorEvent` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ImageCaptureErrorEvent)\n\n*This API requires the following crate features to be activated: `ImageCaptureErrorEvent`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct ImageCaptureErrorEvent {
    obj: Event,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_ImageCaptureErrorEvent: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for ImageCaptureErrorEvent {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(22u32);
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
            inform(69u32);
            inform(118u32);
            inform(101u32);
            inform(110u32);
            inform(116u32);
        }
    }
    impl core::ops::Deref for ImageCaptureErrorEvent {
        type Target = Event;
        #[inline]
        fn deref(&self) -> &Event {
            &self.obj
        }
    }
    impl IntoWasmAbi for ImageCaptureErrorEvent {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for ImageCaptureErrorEvent {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a ImageCaptureErrorEvent {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for ImageCaptureErrorEvent {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            ImageCaptureErrorEvent {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for ImageCaptureErrorEvent {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a ImageCaptureErrorEvent {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for ImageCaptureErrorEvent {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<ImageCaptureErrorEvent>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(ImageCaptureErrorEvent {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for ImageCaptureErrorEvent {
        #[inline]
        fn from(obj: JsValue) -> ImageCaptureErrorEvent {
            ImageCaptureErrorEvent { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for ImageCaptureErrorEvent {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<ImageCaptureErrorEvent> for ImageCaptureErrorEvent {
        #[inline]
        fn as_ref(&self) -> &ImageCaptureErrorEvent {
            self
        }
    }
    impl From<ImageCaptureErrorEvent> for JsValue {
        #[inline]
        fn from(obj: ImageCaptureErrorEvent) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for ImageCaptureErrorEvent {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_ImageCaptureErrorEvent(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_ImageCaptureErrorEvent(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_ImageCaptureErrorEvent(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            ImageCaptureErrorEvent { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const ImageCaptureErrorEvent) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<ImageCaptureErrorEvent> for Event {
    #[inline]
    fn from(obj: ImageCaptureErrorEvent) -> Event {
        use wasm_bindgen::JsCast;
        Event::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<Event> for ImageCaptureErrorEvent {
    #[inline]
    fn as_ref(&self) -> &Event {
        use wasm_bindgen::JsCast;
        Event::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<ImageCaptureErrorEvent> for ::js_sys::Object {
    #[inline]
    fn from(obj: ImageCaptureErrorEvent) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for ImageCaptureErrorEvent {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "ImageCaptureErrorEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_ImageCaptureErrorEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&str as WasmDescribe>::describe();
    <ImageCaptureErrorEvent as WasmDescribe>::describe();
}
impl ImageCaptureErrorEvent {
    #[cfg(all(feature = "ImageCaptureErrorEvent",))]
    #[allow(bad_style)]
    #[doc = "The `new ImageCaptureErrorEvent(..)` constructor, creating a new instance of `ImageCaptureErrorEvent`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ImageCaptureErrorEvent/ImageCaptureErrorEvent)\n\n*This API requires the following crate features to be activated: `ImageCaptureErrorEvent`*"]
    #[allow(clippy::all)]
    pub fn new(type_: &str) -> Result<ImageCaptureErrorEvent, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "ImageCaptureErrorEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_ImageCaptureErrorEvent(
                type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <ImageCaptureErrorEvent as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_ImageCaptureErrorEvent(
            type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <ImageCaptureErrorEvent as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(type_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let type_ = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_);
                __widl_f_new_ImageCaptureErrorEvent(type_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<ImageCaptureErrorEvent as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(
    feature = "ImageCaptureErrorEvent",
    feature = "ImageCaptureErrorEventInit",
))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_with_image_capture_error_init_dict_ImageCaptureErrorEvent(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&str as WasmDescribe>::describe();
    <&ImageCaptureErrorEventInit as WasmDescribe>::describe();
    <ImageCaptureErrorEvent as WasmDescribe>::describe();
}
impl ImageCaptureErrorEvent {
    #[cfg(all(
        feature = "ImageCaptureErrorEvent",
        feature = "ImageCaptureErrorEventInit",
    ))]
    #[allow(bad_style)]
    #[doc = "The `new ImageCaptureErrorEvent(..)` constructor, creating a new instance of `ImageCaptureErrorEvent`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ImageCaptureErrorEvent/ImageCaptureErrorEvent)\n\n*This API requires the following crate features to be activated: `ImageCaptureErrorEvent`, `ImageCaptureErrorEventInit`*"]
    #[allow(clippy::all)]
    pub fn new_with_image_capture_error_init_dict(
        type_: &str,
        image_capture_error_init_dict: &ImageCaptureErrorEventInit,
    ) -> Result<ImageCaptureErrorEvent, ::wasm_bindgen::JsValue> {
        #[cfg(all(
            feature = "ImageCaptureErrorEvent",
            feature = "ImageCaptureErrorEventInit",
        ))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_with_image_capture_error_init_dict_ImageCaptureErrorEvent(
                type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                image_capture_error_init_dict : < & ImageCaptureErrorEventInit as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> <ImageCaptureErrorEvent as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_with_image_capture_error_init_dict_ImageCaptureErrorEvent(
            type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            image_capture_error_init_dict : < & ImageCaptureErrorEventInit as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
        ) -> <ImageCaptureErrorEvent as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(type_);
            drop(image_capture_error_init_dict);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let type_ = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_);
                let image_capture_error_init_dict =
                    <&ImageCaptureErrorEventInit as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        image_capture_error_init_dict,
                    );
                __widl_f_new_with_image_capture_error_init_dict_ImageCaptureErrorEvent(
                    type_,
                    image_capture_error_init_dict,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<ImageCaptureErrorEvent as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "ImageCaptureError", feature = "ImageCaptureErrorEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_image_capture_error_ImageCaptureErrorEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&ImageCaptureErrorEvent as WasmDescribe>::describe();
    <Option<ImageCaptureError> as WasmDescribe>::describe();
}
impl ImageCaptureErrorEvent {
    #[cfg(all(feature = "ImageCaptureError", feature = "ImageCaptureErrorEvent",))]
    #[allow(bad_style)]
    #[doc = "The `imageCaptureError` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ImageCaptureErrorEvent/imageCaptureError)\n\n*This API requires the following crate features to be activated: `ImageCaptureError`, `ImageCaptureErrorEvent`*"]
    #[allow(clippy::all)]
    pub fn image_capture_error(&self) -> Option<ImageCaptureError> {
        #[cfg(all(feature = "ImageCaptureError", feature = "ImageCaptureErrorEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_image_capture_error_ImageCaptureErrorEvent(
                self_: <&ImageCaptureErrorEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<ImageCaptureError> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_image_capture_error_ImageCaptureErrorEvent(
            self_: <&ImageCaptureErrorEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<ImageCaptureError> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&ImageCaptureErrorEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_image_capture_error_ImageCaptureErrorEvent(self_)
            };
            <Option<ImageCaptureError> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_b2f68f1431f48e8e: [u8; 525usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\xCB\x01\0\0\0\0\x04\0\0\x02\x16ImageCaptureErrorEvent(__widl_instanceof_ImageCaptureErrorEvent\0\0\0\0#__widl_f_new_ImageCaptureErrorEvent\x01\0\0\x01\x16ImageCaptureErrorEvent\0\x01\x01\x05type_\x03new\0\0\0F__widl_f_new_with_image_capture_error_init_dict_ImageCaptureErrorEvent\x01\0\0\x01\x16ImageCaptureErrorEvent\0\x01\x02\x05type_\x1Dimage_capture_error_init_dict\x03new\0\0\03__widl_f_image_capture_error_ImageCaptureErrorEvent\0\0\0\x01\x16ImageCaptureErrorEvent\x01\0\x01\x11imageCaptureError\x01\x01\x05self_\x11imageCaptureError\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
