use super::*;
use js_sys::Object;
#[allow(bad_style)]
#[derive(Copy, Clone, PartialEq, Debug)]
#[allow(clippy::all)]
pub enum RtcStatsIceCandidateType {
    Host = 0,
    Serverreflexive = 1,
    Peerreflexive = 2,
    Relayed = 3,
    #[doc(hidden)]
    __Nonexhaustive,
}
#[allow(clippy::all)]
impl RtcStatsIceCandidateType {
    pub fn from_js_value(obj: &wasm_bindgen::JsValue) -> Option<RtcStatsIceCandidateType> {
        obj.as_string().and_then(|obj_str| match obj_str.as_str() {
            "host" => Some(RtcStatsIceCandidateType::Host),
            "serverreflexive" => Some(RtcStatsIceCandidateType::Serverreflexive),
            "peerreflexive" => Some(RtcStatsIceCandidateType::Peerreflexive),
            "relayed" => Some(RtcStatsIceCandidateType::Relayed),
            _ => None,
        })
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::describe::WasmDescribe for RtcStatsIceCandidateType {
    fn describe() {
        wasm_bindgen::JsValue::describe()
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::IntoWasmAbi for RtcStatsIceCandidateType {
    type Abi = <wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi;
    #[inline]
    fn into_abi(self) -> Self::Abi {
        wasm_bindgen::JsValue::from(self).into_abi()
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::FromWasmAbi for RtcStatsIceCandidateType {
    type Abi = <wasm_bindgen::JsValue as wasm_bindgen::convert::FromWasmAbi>::Abi;
    unsafe fn from_abi(js: Self::Abi) -> Self {
        RtcStatsIceCandidateType::from_js_value(&wasm_bindgen::JsValue::from_abi(js))
            .unwrap_or(RtcStatsIceCandidateType::__Nonexhaustive)
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::OptionIntoWasmAbi for RtcStatsIceCandidateType {
    #[inline]
    fn none() -> Self::Abi {
        Object::none()
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::OptionFromWasmAbi for RtcStatsIceCandidateType {
    #[inline]
    fn is_none(abi: &Self::Abi) -> bool {
        Object::is_none(abi)
    }
}
#[allow(clippy::all)]
impl From<RtcStatsIceCandidateType> for wasm_bindgen::JsValue {
    fn from(obj: RtcStatsIceCandidateType) -> wasm_bindgen::JsValue {
        match obj {
            RtcStatsIceCandidateType::Host => wasm_bindgen::JsValue::from_str("host"),
            RtcStatsIceCandidateType::Serverreflexive => {
                wasm_bindgen::JsValue::from_str("serverreflexive")
            }
            RtcStatsIceCandidateType::Peerreflexive => {
                wasm_bindgen::JsValue::from_str("peerreflexive")
            }
            RtcStatsIceCandidateType::Relayed => wasm_bindgen::JsValue::from_str("relayed"),
            RtcStatsIceCandidateType::__Nonexhaustive => {
                panic!("attempted to convert invalid RtcStatsIceCandidateType into JSValue")
            }
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_e062202dee23bfb3: [u8; 108usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}*\0\0\0\0\0\x01\0\0\x03\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
