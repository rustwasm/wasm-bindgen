use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `ImageCapture` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ImageCapture)\n\n*This API requires the following crate features to be activated: `ImageCapture`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct ImageCapture {
    obj: EventTarget,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_ImageCapture: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for ImageCapture {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(12u32);
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
        }
    }
    impl core::ops::Deref for ImageCapture {
        type Target = EventTarget;
        #[inline]
        fn deref(&self) -> &EventTarget {
            &self.obj
        }
    }
    impl IntoWasmAbi for ImageCapture {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for ImageCapture {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a ImageCapture {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for ImageCapture {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            ImageCapture {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for ImageCapture {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a ImageCapture {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for ImageCapture {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<ImageCapture>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(ImageCapture {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for ImageCapture {
        #[inline]
        fn from(obj: JsValue) -> ImageCapture {
            ImageCapture { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for ImageCapture {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<ImageCapture> for ImageCapture {
        #[inline]
        fn as_ref(&self) -> &ImageCapture {
            self
        }
    }
    impl From<ImageCapture> for JsValue {
        #[inline]
        fn from(obj: ImageCapture) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for ImageCapture {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_ImageCapture(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_ImageCapture(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_ImageCapture(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            ImageCapture { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const ImageCapture) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<ImageCapture> for EventTarget {
    #[inline]
    fn from(obj: ImageCapture) -> EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<EventTarget> for ImageCapture {
    #[inline]
    fn as_ref(&self) -> &EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<ImageCapture> for ::js_sys::Object {
    #[inline]
    fn from(obj: ImageCapture) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for ImageCapture {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "ImageCapture", feature = "VideoStreamTrack",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_ImageCapture() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&VideoStreamTrack as WasmDescribe>::describe();
    <ImageCapture as WasmDescribe>::describe();
}
impl ImageCapture {
    #[cfg(all(feature = "ImageCapture", feature = "VideoStreamTrack",))]
    #[allow(bad_style)]
    #[doc = "The `new ImageCapture(..)` constructor, creating a new instance of `ImageCapture`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ImageCapture/ImageCapture)\n\n*This API requires the following crate features to be activated: `ImageCapture`, `VideoStreamTrack`*"]
    #[allow(clippy::all)]
    pub fn new(track: &VideoStreamTrack) -> Result<ImageCapture, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "ImageCapture", feature = "VideoStreamTrack",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_ImageCapture(
                track: <&VideoStreamTrack as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <ImageCapture as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_ImageCapture(
            track: <&VideoStreamTrack as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <ImageCapture as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(track);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let track =
                    <&VideoStreamTrack as wasm_bindgen::convert::IntoWasmAbi>::into_abi(track);
                __widl_f_new_ImageCapture(track)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<ImageCapture as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "ImageCapture",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_take_photo_ImageCapture() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&ImageCapture as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl ImageCapture {
    #[cfg(all(feature = "ImageCapture",))]
    #[allow(bad_style)]
    #[doc = "The `takePhoto()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ImageCapture/takePhoto)\n\n*This API requires the following crate features to be activated: `ImageCapture`*"]
    #[allow(clippy::all)]
    pub fn take_photo(&self) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "ImageCapture",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_take_photo_ImageCapture(
                self_: <&ImageCapture as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_take_photo_ImageCapture(
            self_: <&ImageCapture as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&ImageCapture as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_take_photo_ImageCapture(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "ImageCapture", feature = "VideoStreamTrack",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_video_stream_track_ImageCapture() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&ImageCapture as WasmDescribe>::describe();
    <VideoStreamTrack as WasmDescribe>::describe();
}
impl ImageCapture {
    #[cfg(all(feature = "ImageCapture", feature = "VideoStreamTrack",))]
    #[allow(bad_style)]
    #[doc = "The `videoStreamTrack` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ImageCapture/videoStreamTrack)\n\n*This API requires the following crate features to be activated: `ImageCapture`, `VideoStreamTrack`*"]
    #[allow(clippy::all)]
    pub fn video_stream_track(&self) -> VideoStreamTrack {
        #[cfg(all(feature = "ImageCapture", feature = "VideoStreamTrack",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_video_stream_track_ImageCapture(
                self_: <&ImageCapture as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <VideoStreamTrack as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_video_stream_track_ImageCapture(
            self_: <&ImageCapture as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <VideoStreamTrack as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&ImageCapture as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_video_stream_track_ImageCapture(self_)
            };
            <VideoStreamTrack as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "ImageCapture",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onphoto_ImageCapture() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&ImageCapture as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl ImageCapture {
    #[cfg(all(feature = "ImageCapture",))]
    #[allow(bad_style)]
    #[doc = "The `onphoto` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ImageCapture/onphoto)\n\n*This API requires the following crate features to be activated: `ImageCapture`*"]
    #[allow(clippy::all)]
    pub fn onphoto(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "ImageCapture",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onphoto_ImageCapture(
                self_: <&ImageCapture as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onphoto_ImageCapture(
            self_: <&ImageCapture as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&ImageCapture as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onphoto_ImageCapture(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "ImageCapture",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onphoto_ImageCapture() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&ImageCapture as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl ImageCapture {
    #[cfg(all(feature = "ImageCapture",))]
    #[allow(bad_style)]
    #[doc = "The `onphoto` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ImageCapture/onphoto)\n\n*This API requires the following crate features to be activated: `ImageCapture`*"]
    #[allow(clippy::all)]
    pub fn set_onphoto(&self, onphoto: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "ImageCapture",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onphoto_ImageCapture(
                self_: <&ImageCapture as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onphoto: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onphoto_ImageCapture(
            self_: <&ImageCapture as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onphoto: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(onphoto);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&ImageCapture as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onphoto =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onphoto,
                    );
                __widl_f_set_onphoto_ImageCapture(self_, onphoto)
            };
            ()
        }
    }
}
#[cfg(all(feature = "ImageCapture",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onerror_ImageCapture() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&ImageCapture as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl ImageCapture {
    #[cfg(all(feature = "ImageCapture",))]
    #[allow(bad_style)]
    #[doc = "The `onerror` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ImageCapture/onerror)\n\n*This API requires the following crate features to be activated: `ImageCapture`*"]
    #[allow(clippy::all)]
    pub fn onerror(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "ImageCapture",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onerror_ImageCapture(
                self_: <&ImageCapture as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onerror_ImageCapture(
            self_: <&ImageCapture as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&ImageCapture as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onerror_ImageCapture(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "ImageCapture",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onerror_ImageCapture() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&ImageCapture as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl ImageCapture {
    #[cfg(all(feature = "ImageCapture",))]
    #[allow(bad_style)]
    #[doc = "The `onerror` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ImageCapture/onerror)\n\n*This API requires the following crate features to be activated: `ImageCapture`*"]
    #[allow(clippy::all)]
    pub fn set_onerror(&self, onerror: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "ImageCapture",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onerror_ImageCapture(
                self_: <&ImageCapture as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onerror: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onerror_ImageCapture(
            self_: <&ImageCapture as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onerror: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(onerror);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&ImageCapture as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onerror =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onerror,
                    );
                __widl_f_set_onerror_ImageCapture(self_, onerror)
            };
            ()
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_93e4d7d9c9cd5512: [u8; 724usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\x92\x02\0\0\0\0\x08\0\0\x02\x0CImageCapture\x1E__widl_instanceof_ImageCapture\0\0\0\0\x19__widl_f_new_ImageCapture\x01\0\0\x01\x0CImageCapture\0\x01\x01\x05track\x03new\0\0\0 __widl_f_take_photo_ImageCapture\x01\0\0\x01\x0CImageCapture\x01\0\0\x01\x01\x05self_\ttakePhoto\0\0\0(__widl_f_video_stream_track_ImageCapture\0\0\0\x01\x0CImageCapture\x01\0\x01\x10videoStreamTrack\x01\x01\x05self_\x10videoStreamTrack\0\0\0\x1D__widl_f_onphoto_ImageCapture\0\0\0\x01\x0CImageCapture\x01\0\x01\x07onphoto\x01\x01\x05self_\x07onphoto\0\0\0!__widl_f_set_onphoto_ImageCapture\0\0\0\x01\x0CImageCapture\x01\0\x02\x07onphoto\x01\x02\x05self_\x07onphoto\x07onphoto\0\0\0\x1D__widl_f_onerror_ImageCapture\0\0\0\x01\x0CImageCapture\x01\0\x01\x07onerror\x01\x01\x05self_\x07onerror\0\0\0!__widl_f_set_onerror_ImageCapture\0\0\0\x01\x0CImageCapture\x01\0\x02\x07onerror\x01\x02\x05self_\x07onerror\x07onerror\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
