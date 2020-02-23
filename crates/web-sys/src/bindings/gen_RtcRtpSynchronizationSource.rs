use super::*;
#[derive(Clone, Debug)]
#[repr(transparent)]
#[allow(clippy::all)]
#[doc = "The `RTCRtpSynchronizationSource` dictionary\n\n\n*This API requires the following crate features to be activated: `RtcRtpSynchronizationSource`*"]
pub struct RtcRtpSynchronizationSource {
    obj: ::js_sys::Object,
}
#[allow(clippy::all)]
impl RtcRtpSynchronizationSource {
    #[doc = "Construct a new `RTCRtpSynchronizationSource`\n\n\n*This API requires the following crate features to be activated: `RtcRtpSynchronizationSource`*"]
    pub fn new(source: u32, timestamp: f64) -> RtcRtpSynchronizationSource {
        let mut _ret = RtcRtpSynchronizationSource {
            obj: ::js_sys::Object::new(),
        };
        _ret.source(source);
        _ret.timestamp(timestamp);
        return _ret;
    }
    #[allow(clippy::all)]
    #[doc = "Configure the `audioLevel` field of this object\n\n\n*This API requires the following crate features to be activated: `RtcRtpSynchronizationSource`*"]
    pub fn audio_level(&mut self, val: f64) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.obj.as_ref(),
            &JsValue::from("audioLevel"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[allow(clippy::all)]
    #[doc = "Configure the `source` field of this object\n\n\n*This API requires the following crate features to be activated: `RtcRtpSynchronizationSource`*"]
    pub fn source(&mut self, val: u32) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.obj.as_ref(),
            &JsValue::from("source"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[allow(clippy::all)]
    #[doc = "Configure the `timestamp` field of this object\n\n\n*This API requires the following crate features to be activated: `RtcRtpSynchronizationSource`*"]
    pub fn timestamp(&mut self, val: f64) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.obj.as_ref(),
            &JsValue::from("timestamp"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[allow(clippy::all)]
    #[doc = "Configure the `voiceActivityFlag` field of this object\n\n\n*This API requires the following crate features to be activated: `RtcRtpSynchronizationSource`*"]
    pub fn voice_activity_flag(&mut self, val: Option<bool>) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.obj.as_ref(),
            &JsValue::from("voiceActivityFlag"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
}
#[allow(bad_style)]
#[allow(clippy::all)]
const _CONST_RtcRtpSynchronizationSource: () = {
    use js_sys::Object;
    use wasm_bindgen::__rt::core::mem::ManuallyDrop;
    use wasm_bindgen::convert::*;
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl From<RtcRtpSynchronizationSource> for JsValue {
        #[inline]
        fn from(val: RtcRtpSynchronizationSource) -> JsValue {
            val.obj.into()
        }
    }
    impl AsRef<JsValue> for RtcRtpSynchronizationSource {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl WasmDescribe for RtcRtpSynchronizationSource {
        fn describe() {
            Object::describe();
        }
    }
    impl IntoWasmAbi for RtcRtpSynchronizationSource {
        type Abi = <Object as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl<'a> IntoWasmAbi for &'a RtcRtpSynchronizationSource {
        type Abi = <&'a Object as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl FromWasmAbi for RtcRtpSynchronizationSource {
        type Abi = <Object as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(abi: Self::Abi) -> Self {
            RtcRtpSynchronizationSource {
                obj: Object::from_abi(abi),
            }
        }
    }
    impl OptionIntoWasmAbi for RtcRtpSynchronizationSource {
        #[inline]
        fn none() -> Self::Abi {
            Object::none()
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a RtcRtpSynchronizationSource {
        #[inline]
        fn none() -> Self::Abi {
            <&'a Object>::none()
        }
    }
    impl OptionFromWasmAbi for RtcRtpSynchronizationSource {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            Object::is_none(abi)
        }
    }
    impl RefFromWasmAbi for RtcRtpSynchronizationSource {
        type Abi = <Object as RefFromWasmAbi>::Abi;
        type Anchor = ManuallyDrop<RtcRtpSynchronizationSource>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <Object as RefFromWasmAbi>::ref_from_abi(js);
            ManuallyDrop::new(RtcRtpSynchronizationSource {
                obj: ManuallyDrop::into_inner(tmp),
            })
        }
    }
    impl JsCast for RtcRtpSynchronizationSource {
        #[inline]
        fn instanceof(val: &JsValue) -> bool {
            Object::instanceof(val)
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            RtcRtpSynchronizationSource {
                obj: Object::unchecked_from_js(val),
            }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const RtcRtpSynchronizationSource) }
        }
    }
};
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_866d9f12ca7141b2: [u8; 105usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}'\0\0\0\0\0\0\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
