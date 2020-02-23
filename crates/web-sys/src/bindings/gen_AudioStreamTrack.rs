use super::*;
use js_sys::Object;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `AudioStreamTrack` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioStreamTrack)\n\n*This API requires the following crate features to be activated: `AudioStreamTrack`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct AudioStreamTrack {
    obj: MediaStreamTrack,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_AudioStreamTrack: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for AudioStreamTrack {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(16u32);
            inform(65u32);
            inform(117u32);
            inform(100u32);
            inform(105u32);
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
    impl core::ops::Deref for AudioStreamTrack {
        type Target = MediaStreamTrack;
        #[inline]
        fn deref(&self) -> &MediaStreamTrack {
            &self.obj
        }
    }
    impl IntoWasmAbi for AudioStreamTrack {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for AudioStreamTrack {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a AudioStreamTrack {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for AudioStreamTrack {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            AudioStreamTrack {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for AudioStreamTrack {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a AudioStreamTrack {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for AudioStreamTrack {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<AudioStreamTrack>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(AudioStreamTrack {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for AudioStreamTrack {
        #[inline]
        fn from(obj: JsValue) -> AudioStreamTrack {
            AudioStreamTrack { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for AudioStreamTrack {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<AudioStreamTrack> for AudioStreamTrack {
        #[inline]
        fn as_ref(&self) -> &AudioStreamTrack {
            self
        }
    }
    impl From<AudioStreamTrack> for JsValue {
        #[inline]
        fn from(obj: AudioStreamTrack) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for AudioStreamTrack {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_AudioStreamTrack(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_AudioStreamTrack(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_AudioStreamTrack(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            AudioStreamTrack { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const AudioStreamTrack) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<AudioStreamTrack> for MediaStreamTrack {
    #[inline]
    fn from(obj: AudioStreamTrack) -> MediaStreamTrack {
        use wasm_bindgen::JsCast;
        MediaStreamTrack::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<MediaStreamTrack> for AudioStreamTrack {
    #[inline]
    fn as_ref(&self) -> &MediaStreamTrack {
        use wasm_bindgen::JsCast;
        MediaStreamTrack::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<AudioStreamTrack> for EventTarget {
    #[inline]
    fn from(obj: AudioStreamTrack) -> EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<EventTarget> for AudioStreamTrack {
    #[inline]
    fn as_ref(&self) -> &EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<AudioStreamTrack> for ::js_sys::Object {
    #[inline]
    fn from(obj: AudioStreamTrack) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for AudioStreamTrack {
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
pub static __WASM_BINDGEN_GENERATED_ab2169ac4d4fad35: [u8; 161usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}_\0\0\0\0\0\x01\0\0\x02\x10AudioStreamTrack\"__widl_instanceof_AudioStreamTrack\0\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
