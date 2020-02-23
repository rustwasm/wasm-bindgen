use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `VideoStreamTrack` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VideoStreamTrack)\n\n*This API requires the following crate features to be activated: `VideoStreamTrack`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct VideoStreamTrack {
    obj: MediaStreamTrack,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_VideoStreamTrack: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for VideoStreamTrack {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(16u32);
            inform(86u32);
            inform(105u32);
            inform(100u32);
            inform(101u32);
            inform(111u32);
            inform(83u32);
            inform(116u32);
            inform(114u32);
            inform(101u32);
            inform(97u32);
            inform(109u32);
            inform(84u32);
            inform(114u32);
            inform(97u32);
            inform(99u32);
            inform(107u32);
        }
    }
    impl core::ops::Deref for VideoStreamTrack {
        type Target = MediaStreamTrack;
        #[inline]
        fn deref(&self) -> &MediaStreamTrack {
            &self.obj
        }
    }
    impl IntoWasmAbi for VideoStreamTrack {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for VideoStreamTrack {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a VideoStreamTrack {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for VideoStreamTrack {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            VideoStreamTrack {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for VideoStreamTrack {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a VideoStreamTrack {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for VideoStreamTrack {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<VideoStreamTrack>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(VideoStreamTrack {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for VideoStreamTrack {
        #[inline]
        fn from(obj: JsValue) -> VideoStreamTrack {
            VideoStreamTrack { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for VideoStreamTrack {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<VideoStreamTrack> for VideoStreamTrack {
        #[inline]
        fn as_ref(&self) -> &VideoStreamTrack {
            self
        }
    }
    impl From<VideoStreamTrack> for JsValue {
        #[inline]
        fn from(obj: VideoStreamTrack) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for VideoStreamTrack {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_VideoStreamTrack(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_VideoStreamTrack(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_VideoStreamTrack(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            VideoStreamTrack { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const VideoStreamTrack) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<VideoStreamTrack> for MediaStreamTrack {
    #[inline]
    fn from(obj: VideoStreamTrack) -> MediaStreamTrack {
        use wasm_bindgen::JsCast;
        MediaStreamTrack::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<MediaStreamTrack> for VideoStreamTrack {
    #[inline]
    fn as_ref(&self) -> &MediaStreamTrack {
        use wasm_bindgen::JsCast;
        MediaStreamTrack::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<VideoStreamTrack> for EventTarget {
    #[inline]
    fn from(obj: VideoStreamTrack) -> EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<EventTarget> for VideoStreamTrack {
    #[inline]
    fn as_ref(&self) -> &EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<VideoStreamTrack> for ::js_sys::Object {
    #[inline]
    fn from(obj: VideoStreamTrack) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for VideoStreamTrack {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_2fb646b56ef3e0f0: [u8; 161usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}_\0\0\0\0\0\x01\0\0\x02\x10VideoStreamTrack\"__widl_instanceof_VideoStreamTrack\0\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
