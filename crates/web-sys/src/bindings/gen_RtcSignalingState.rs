use super::*;
use js_sys::Object;
#[allow(bad_style)]
#[derive(Copy, Clone, PartialEq, Debug)]
#[allow(clippy::all)]
pub enum RtcSignalingState {
    Stable = 0,
    HaveLocalOffer = 1,
    HaveRemoteOffer = 2,
    HaveLocalPranswer = 3,
    HaveRemotePranswer = 4,
    Closed = 5,
    #[doc(hidden)]
    __Nonexhaustive,
}
#[allow(clippy::all)]
impl RtcSignalingState {
    pub fn from_js_value(obj: &wasm_bindgen::JsValue) -> Option<RtcSignalingState> {
        obj.as_string().and_then(|obj_str| match obj_str.as_str() {
            "stable" => Some(RtcSignalingState::Stable),
            "have-local-offer" => Some(RtcSignalingState::HaveLocalOffer),
            "have-remote-offer" => Some(RtcSignalingState::HaveRemoteOffer),
            "have-local-pranswer" => Some(RtcSignalingState::HaveLocalPranswer),
            "have-remote-pranswer" => Some(RtcSignalingState::HaveRemotePranswer),
            "closed" => Some(RtcSignalingState::Closed),
            _ => None,
        })
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::describe::WasmDescribe for RtcSignalingState {
    fn describe() {
        wasm_bindgen::JsValue::describe()
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::IntoWasmAbi for RtcSignalingState {
    type Abi = <wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi;
    #[inline]
    fn into_abi(self) -> Self::Abi {
        wasm_bindgen::JsValue::from(self).into_abi()
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::FromWasmAbi for RtcSignalingState {
    type Abi = <wasm_bindgen::JsValue as wasm_bindgen::convert::FromWasmAbi>::Abi;
    unsafe fn from_abi(js: Self::Abi) -> Self {
        RtcSignalingState::from_js_value(&wasm_bindgen::JsValue::from_abi(js))
            .unwrap_or(RtcSignalingState::__Nonexhaustive)
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::OptionIntoWasmAbi for RtcSignalingState {
    #[inline]
    fn none() -> Self::Abi {
        Object::none()
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::OptionFromWasmAbi for RtcSignalingState {
    #[inline]
    fn is_none(abi: &Self::Abi) -> bool {
        Object::is_none(abi)
    }
}
#[allow(clippy::all)]
impl From<RtcSignalingState> for wasm_bindgen::JsValue {
    fn from(obj: RtcSignalingState) -> wasm_bindgen::JsValue {
        match obj {
            RtcSignalingState::Stable => wasm_bindgen::JsValue::from_str("stable"),
            RtcSignalingState::HaveLocalOffer => {
                wasm_bindgen::JsValue::from_str("have-local-offer")
            }
            RtcSignalingState::HaveRemoteOffer => {
                wasm_bindgen::JsValue::from_str("have-remote-offer")
            }
            RtcSignalingState::HaveLocalPranswer => {
                wasm_bindgen::JsValue::from_str("have-local-pranswer")
            }
            RtcSignalingState::HaveRemotePranswer => {
                wasm_bindgen::JsValue::from_str("have-remote-pranswer")
            }
            RtcSignalingState::Closed => wasm_bindgen::JsValue::from_str("closed"),
            RtcSignalingState::__Nonexhaustive => {
                panic!("attempted to convert invalid RtcSignalingState into JSValue")
            }
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_a05492095bae0c3e: [u8; 108usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}*\0\0\0\0\0\x01\0\0\x03\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
