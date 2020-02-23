use super::*;
#[derive(Clone, Debug)]
#[repr(transparent)]
#[allow(clippy::all)]
#[doc = "The `RTCIceCandidateInit` dictionary\n\n\n*This API requires the following crate features to be activated: `RtcIceCandidateInit`*"]
pub struct RtcIceCandidateInit {
    obj: ::js_sys::Object,
}
#[allow(clippy::all)]
impl RtcIceCandidateInit {
    #[doc = "Construct a new `RTCIceCandidateInit`\n\n\n*This API requires the following crate features to be activated: `RtcIceCandidateInit`*"]
    pub fn new(candidate: &str) -> RtcIceCandidateInit {
        let mut _ret = RtcIceCandidateInit {
            obj: ::js_sys::Object::new(),
        };
        _ret.candidate(candidate);
        return _ret;
    }
    #[allow(clippy::all)]
    #[doc = "Configure the `candidate` field of this object\n\n\n*This API requires the following crate features to be activated: `RtcIceCandidateInit`*"]
    pub fn candidate(&mut self, val: &str) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.obj.as_ref(),
            &JsValue::from("candidate"),
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
    #[doc = "Configure the `sdpMLineIndex` field of this object\n\n\n*This API requires the following crate features to be activated: `RtcIceCandidateInit`*"]
    pub fn sdp_m_line_index(&mut self, val: Option<u16>) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.obj.as_ref(),
            &JsValue::from("sdpMLineIndex"),
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
    #[doc = "Configure the `sdpMid` field of this object\n\n\n*This API requires the following crate features to be activated: `RtcIceCandidateInit`*"]
    pub fn sdp_mid(&mut self, val: Option<&str>) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.obj.as_ref(),
            &JsValue::from("sdpMid"),
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
const _CONST_RtcIceCandidateInit: () = {
    use js_sys::Object;
    use wasm_bindgen::__rt::core::mem::ManuallyDrop;
    use wasm_bindgen::convert::*;
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl From<RtcIceCandidateInit> for JsValue {
        #[inline]
        fn from(val: RtcIceCandidateInit) -> JsValue {
            val.obj.into()
        }
    }
    impl AsRef<JsValue> for RtcIceCandidateInit {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl WasmDescribe for RtcIceCandidateInit {
        fn describe() {
            Object::describe();
        }
    }
    impl IntoWasmAbi for RtcIceCandidateInit {
        type Abi = <Object as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl<'a> IntoWasmAbi for &'a RtcIceCandidateInit {
        type Abi = <&'a Object as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl FromWasmAbi for RtcIceCandidateInit {
        type Abi = <Object as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(abi: Self::Abi) -> Self {
            RtcIceCandidateInit {
                obj: Object::from_abi(abi),
            }
        }
    }
    impl OptionIntoWasmAbi for RtcIceCandidateInit {
        #[inline]
        fn none() -> Self::Abi {
            Object::none()
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a RtcIceCandidateInit {
        #[inline]
        fn none() -> Self::Abi {
            <&'a Object>::none()
        }
    }
    impl OptionFromWasmAbi for RtcIceCandidateInit {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            Object::is_none(abi)
        }
    }
    impl RefFromWasmAbi for RtcIceCandidateInit {
        type Abi = <Object as RefFromWasmAbi>::Abi;
        type Anchor = ManuallyDrop<RtcIceCandidateInit>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <Object as RefFromWasmAbi>::ref_from_abi(js);
            ManuallyDrop::new(RtcIceCandidateInit {
                obj: ManuallyDrop::into_inner(tmp),
            })
        }
    }
    impl JsCast for RtcIceCandidateInit {
        #[inline]
        fn instanceof(val: &JsValue) -> bool {
            Object::instanceof(val)
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            RtcIceCandidateInit {
                obj: Object::unchecked_from_js(val),
            }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const RtcIceCandidateInit) }
        }
    }
};
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_266c398245a4e129: [u8; 105usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}'\0\0\0\0\0\0\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
