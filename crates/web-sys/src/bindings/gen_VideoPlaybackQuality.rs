use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `VideoPlaybackQuality` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VideoPlaybackQuality)\n\n*This API requires the following crate features to be activated: `VideoPlaybackQuality`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct VideoPlaybackQuality {
    obj: ::js_sys::Object,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_VideoPlaybackQuality: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for VideoPlaybackQuality {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(20u32);
            inform(86u32);
            inform(105u32);
            inform(100u32);
            inform(101u32);
            inform(111u32);
            inform(80u32);
            inform(108u32);
            inform(97u32);
            inform(121u32);
            inform(98u32);
            inform(97u32);
            inform(99u32);
            inform(107u32);
            inform(81u32);
            inform(117u32);
            inform(97u32);
            inform(108u32);
            inform(105u32);
            inform(116u32);
            inform(121u32);
        }
    }
    impl core::ops::Deref for VideoPlaybackQuality {
        type Target = ::js_sys::Object;
        #[inline]
        fn deref(&self) -> &::js_sys::Object {
            &self.obj
        }
    }
    impl IntoWasmAbi for VideoPlaybackQuality {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for VideoPlaybackQuality {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a VideoPlaybackQuality {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for VideoPlaybackQuality {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            VideoPlaybackQuality {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for VideoPlaybackQuality {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a VideoPlaybackQuality {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for VideoPlaybackQuality {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<VideoPlaybackQuality>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(VideoPlaybackQuality {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for VideoPlaybackQuality {
        #[inline]
        fn from(obj: JsValue) -> VideoPlaybackQuality {
            VideoPlaybackQuality { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for VideoPlaybackQuality {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<VideoPlaybackQuality> for VideoPlaybackQuality {
        #[inline]
        fn as_ref(&self) -> &VideoPlaybackQuality {
            self
        }
    }
    impl From<VideoPlaybackQuality> for JsValue {
        #[inline]
        fn from(obj: VideoPlaybackQuality) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for VideoPlaybackQuality {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_VideoPlaybackQuality(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_VideoPlaybackQuality(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_VideoPlaybackQuality(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            VideoPlaybackQuality { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const VideoPlaybackQuality) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<VideoPlaybackQuality> for ::js_sys::Object {
    #[inline]
    fn from(obj: VideoPlaybackQuality) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for VideoPlaybackQuality {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "VideoPlaybackQuality",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_creation_time_VideoPlaybackQuality() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&VideoPlaybackQuality as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
}
impl VideoPlaybackQuality {
    #[cfg(all(feature = "VideoPlaybackQuality",))]
    #[allow(bad_style)]
    #[doc = "The `creationTime` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VideoPlaybackQuality/creationTime)\n\n*This API requires the following crate features to be activated: `VideoPlaybackQuality`*"]
    #[allow(clippy::all)]
    pub fn creation_time(&self) -> f64 {
        #[cfg(all(feature = "VideoPlaybackQuality",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_creation_time_VideoPlaybackQuality(
                self_: <&VideoPlaybackQuality as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_creation_time_VideoPlaybackQuality(
            self_: <&VideoPlaybackQuality as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&VideoPlaybackQuality as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_creation_time_VideoPlaybackQuality(self_)
            };
            <f64 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "VideoPlaybackQuality",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_total_video_frames_VideoPlaybackQuality() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&VideoPlaybackQuality as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
}
impl VideoPlaybackQuality {
    #[cfg(all(feature = "VideoPlaybackQuality",))]
    #[allow(bad_style)]
    #[doc = "The `totalVideoFrames` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VideoPlaybackQuality/totalVideoFrames)\n\n*This API requires the following crate features to be activated: `VideoPlaybackQuality`*"]
    #[allow(clippy::all)]
    pub fn total_video_frames(&self) -> u32 {
        #[cfg(all(feature = "VideoPlaybackQuality",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_total_video_frames_VideoPlaybackQuality(
                self_: <&VideoPlaybackQuality as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <u32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_total_video_frames_VideoPlaybackQuality(
            self_: <&VideoPlaybackQuality as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <u32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&VideoPlaybackQuality as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_total_video_frames_VideoPlaybackQuality(self_)
            };
            <u32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "VideoPlaybackQuality",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_dropped_video_frames_VideoPlaybackQuality() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&VideoPlaybackQuality as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
}
impl VideoPlaybackQuality {
    #[cfg(all(feature = "VideoPlaybackQuality",))]
    #[allow(bad_style)]
    #[doc = "The `droppedVideoFrames` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VideoPlaybackQuality/droppedVideoFrames)\n\n*This API requires the following crate features to be activated: `VideoPlaybackQuality`*"]
    #[allow(clippy::all)]
    pub fn dropped_video_frames(&self) -> u32 {
        #[cfg(all(feature = "VideoPlaybackQuality",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_dropped_video_frames_VideoPlaybackQuality(
                self_: <&VideoPlaybackQuality as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <u32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_dropped_video_frames_VideoPlaybackQuality(
            self_: <&VideoPlaybackQuality as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <u32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&VideoPlaybackQuality as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_dropped_video_frames_VideoPlaybackQuality(self_)
            };
            <u32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "VideoPlaybackQuality",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_corrupted_video_frames_VideoPlaybackQuality() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&VideoPlaybackQuality as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
}
impl VideoPlaybackQuality {
    #[cfg(all(feature = "VideoPlaybackQuality",))]
    #[allow(bad_style)]
    #[doc = "The `corruptedVideoFrames` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VideoPlaybackQuality/corruptedVideoFrames)\n\n*This API requires the following crate features to be activated: `VideoPlaybackQuality`*"]
    #[allow(clippy::all)]
    pub fn corrupted_video_frames(&self) -> u32 {
        #[cfg(all(feature = "VideoPlaybackQuality",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_corrupted_video_frames_VideoPlaybackQuality(
                self_: <&VideoPlaybackQuality as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <u32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_corrupted_video_frames_VideoPlaybackQuality(
            self_: <&VideoPlaybackQuality as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <u32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&VideoPlaybackQuality as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_corrupted_video_frames_VideoPlaybackQuality(self_)
            };
            <u32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_cdc025d5d289863d: [u8; 662usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}T\x02\0\0\0\0\x05\0\0\x02\x14VideoPlaybackQuality&__widl_instanceof_VideoPlaybackQuality\0\0\0\0+__widl_f_creation_time_VideoPlaybackQuality\0\0\0\x01\x14VideoPlaybackQuality\x01\0\x01\x0CcreationTime\x01\x01\x05self_\x0CcreationTime\0\0\00__widl_f_total_video_frames_VideoPlaybackQuality\0\0\0\x01\x14VideoPlaybackQuality\x01\0\x01\x10totalVideoFrames\x01\x01\x05self_\x10totalVideoFrames\0\0\02__widl_f_dropped_video_frames_VideoPlaybackQuality\0\0\0\x01\x14VideoPlaybackQuality\x01\0\x01\x12droppedVideoFrames\x01\x01\x05self_\x12droppedVideoFrames\0\0\04__widl_f_corrupted_video_frames_VideoPlaybackQuality\0\0\0\x01\x14VideoPlaybackQuality\x01\0\x01\x14corruptedVideoFrames\x01\x01\x05self_\x14corruptedVideoFrames\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
