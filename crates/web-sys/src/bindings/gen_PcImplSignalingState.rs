use super::*;
#[allow(bad_style)]
#[derive(Copy, Clone, PartialEq, Debug)]
#[allow(clippy::all)]
pub enum PcImplSignalingState {
    SignalingInvalid = 0,
    SignalingStable = 1,
    SignalingHaveLocalOffer = 2,
    SignalingHaveRemoteOffer = 3,
    SignalingHaveLocalPranswer = 4,
    SignalingHaveRemotePranswer = 5,
    SignalingClosed = 6,
    #[doc(hidden)]
    __Nonexhaustive,
}
#[allow(clippy::all)]
impl PcImplSignalingState {
    pub fn from_js_value(obj: &wasm_bindgen::JsValue) -> Option<PcImplSignalingState> {
        obj.as_string().and_then(|obj_str| match obj_str.as_str() {
            "SignalingInvalid" => Some(PcImplSignalingState::SignalingInvalid),
            "SignalingStable" => Some(PcImplSignalingState::SignalingStable),
            "SignalingHaveLocalOffer" => Some(PcImplSignalingState::SignalingHaveLocalOffer),
            "SignalingHaveRemoteOffer" => Some(PcImplSignalingState::SignalingHaveRemoteOffer),
            "SignalingHaveLocalPranswer" => Some(PcImplSignalingState::SignalingHaveLocalPranswer),
            "SignalingHaveRemotePranswer" => {
                Some(PcImplSignalingState::SignalingHaveRemotePranswer)
            }
            "SignalingClosed" => Some(PcImplSignalingState::SignalingClosed),
            _ => None,
        })
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::describe::WasmDescribe for PcImplSignalingState {
    fn describe() {
        wasm_bindgen::JsValue::describe()
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::IntoWasmAbi for PcImplSignalingState {
    type Abi = <wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi;
    #[inline]
    fn into_abi(self) -> Self::Abi {
        wasm_bindgen::JsValue::from(self).into_abi()
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::FromWasmAbi for PcImplSignalingState {
    type Abi = <wasm_bindgen::JsValue as wasm_bindgen::convert::FromWasmAbi>::Abi;
    unsafe fn from_abi(js: Self::Abi) -> Self {
        PcImplSignalingState::from_js_value(&wasm_bindgen::JsValue::from_abi(js))
            .unwrap_or(PcImplSignalingState::__Nonexhaustive)
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::OptionIntoWasmAbi for PcImplSignalingState {
    #[inline]
    fn none() -> Self::Abi {
        Object::none()
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::OptionFromWasmAbi for PcImplSignalingState {
    #[inline]
    fn is_none(abi: &Self::Abi) -> bool {
        Object::is_none(abi)
    }
}
#[allow(clippy::all)]
impl From<PcImplSignalingState> for wasm_bindgen::JsValue {
    fn from(obj: PcImplSignalingState) -> wasm_bindgen::JsValue {
        match obj {
            PcImplSignalingState::SignalingInvalid => {
                wasm_bindgen::JsValue::from_str("SignalingInvalid")
            }
            PcImplSignalingState::SignalingStable => {
                wasm_bindgen::JsValue::from_str("SignalingStable")
            }
            PcImplSignalingState::SignalingHaveLocalOffer => {
                wasm_bindgen::JsValue::from_str("SignalingHaveLocalOffer")
            }
            PcImplSignalingState::SignalingHaveRemoteOffer => {
                wasm_bindgen::JsValue::from_str("SignalingHaveRemoteOffer")
            }
            PcImplSignalingState::SignalingHaveLocalPranswer => {
                wasm_bindgen::JsValue::from_str("SignalingHaveLocalPranswer")
            }
            PcImplSignalingState::SignalingHaveRemotePranswer => {
                wasm_bindgen::JsValue::from_str("SignalingHaveRemotePranswer")
            }
            PcImplSignalingState::SignalingClosed => {
                wasm_bindgen::JsValue::from_str("SignalingClosed")
            }
            PcImplSignalingState::__Nonexhaustive => {
                panic!("attempted to convert invalid PcImplSignalingState into JSValue")
            }
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_bbfcd960cca380b4: [u8; 108usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}*\0\0\0\0\0\x01\0\0\x03\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
