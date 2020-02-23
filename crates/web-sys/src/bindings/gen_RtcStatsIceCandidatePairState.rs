use super::*;
use js_sys::Object;
#[allow(bad_style)]
#[derive(Copy, Clone, PartialEq, Debug)]
#[allow(clippy::all)]
pub enum RtcStatsIceCandidatePairState {
    Frozen = 0,
    Waiting = 1,
    Inprogress = 2,
    Failed = 3,
    Succeeded = 4,
    Cancelled = 5,
    #[doc(hidden)]
    __Nonexhaustive,
}
#[allow(clippy::all)]
impl RtcStatsIceCandidatePairState {
    pub fn from_js_value(obj: &wasm_bindgen::JsValue) -> Option<RtcStatsIceCandidatePairState> {
        obj.as_string().and_then(|obj_str| match obj_str.as_str() {
            "frozen" => Some(RtcStatsIceCandidatePairState::Frozen),
            "waiting" => Some(RtcStatsIceCandidatePairState::Waiting),
            "inprogress" => Some(RtcStatsIceCandidatePairState::Inprogress),
            "failed" => Some(RtcStatsIceCandidatePairState::Failed),
            "succeeded" => Some(RtcStatsIceCandidatePairState::Succeeded),
            "cancelled" => Some(RtcStatsIceCandidatePairState::Cancelled),
            _ => None,
        })
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::describe::WasmDescribe for RtcStatsIceCandidatePairState {
    fn describe() {
        wasm_bindgen::JsValue::describe()
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::IntoWasmAbi for RtcStatsIceCandidatePairState {
    type Abi = <wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi;
    #[inline]
    fn into_abi(self) -> Self::Abi {
        wasm_bindgen::JsValue::from(self).into_abi()
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::FromWasmAbi for RtcStatsIceCandidatePairState {
    type Abi = <wasm_bindgen::JsValue as wasm_bindgen::convert::FromWasmAbi>::Abi;
    unsafe fn from_abi(js: Self::Abi) -> Self {
        RtcStatsIceCandidatePairState::from_js_value(&wasm_bindgen::JsValue::from_abi(js))
            .unwrap_or(RtcStatsIceCandidatePairState::__Nonexhaustive)
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::OptionIntoWasmAbi for RtcStatsIceCandidatePairState {
    #[inline]
    fn none() -> Self::Abi {
        Object::none()
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::OptionFromWasmAbi for RtcStatsIceCandidatePairState {
    #[inline]
    fn is_none(abi: &Self::Abi) -> bool {
        Object::is_none(abi)
    }
}
#[allow(clippy::all)]
impl From<RtcStatsIceCandidatePairState> for wasm_bindgen::JsValue {
    fn from(obj: RtcStatsIceCandidatePairState) -> wasm_bindgen::JsValue {
        match obj {
            RtcStatsIceCandidatePairState::Frozen => wasm_bindgen::JsValue::from_str("frozen"),
            RtcStatsIceCandidatePairState::Waiting => wasm_bindgen::JsValue::from_str("waiting"),
            RtcStatsIceCandidatePairState::Inprogress => {
                wasm_bindgen::JsValue::from_str("inprogress")
            }
            RtcStatsIceCandidatePairState::Failed => wasm_bindgen::JsValue::from_str("failed"),
            RtcStatsIceCandidatePairState::Succeeded => {
                wasm_bindgen::JsValue::from_str("succeeded")
            }
            RtcStatsIceCandidatePairState::Cancelled => {
                wasm_bindgen::JsValue::from_str("cancelled")
            }
            RtcStatsIceCandidatePairState::__Nonexhaustive => {
                panic!("attempted to convert invalid RtcStatsIceCandidatePairState into JSValue")
            }
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_ae9321ee2db55530: [u8; 108usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}*\0\0\0\0\0\x01\0\0\x03\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
