use super::*;
#[allow(bad_style)]
#[derive(Copy, Clone, PartialEq, Debug)]
#[allow(clippy::all)]
pub enum RtcStatsType {
    InboundRtp = 0,
    OutboundRtp = 1,
    Csrc = 2,
    Session = 3,
    Track = 4,
    Transport = 5,
    CandidatePair = 6,
    LocalCandidate = 7,
    RemoteCandidate = 8,
    #[doc(hidden)]
    __Nonexhaustive,
}
#[allow(clippy::all)]
impl RtcStatsType {
    pub fn from_js_value(obj: &wasm_bindgen::JsValue) -> Option<RtcStatsType> {
        obj.as_string().and_then(|obj_str| match obj_str.as_str() {
            "inbound-rtp" => Some(RtcStatsType::InboundRtp),
            "outbound-rtp" => Some(RtcStatsType::OutboundRtp),
            "csrc" => Some(RtcStatsType::Csrc),
            "session" => Some(RtcStatsType::Session),
            "track" => Some(RtcStatsType::Track),
            "transport" => Some(RtcStatsType::Transport),
            "candidate-pair" => Some(RtcStatsType::CandidatePair),
            "local-candidate" => Some(RtcStatsType::LocalCandidate),
            "remote-candidate" => Some(RtcStatsType::RemoteCandidate),
            _ => None,
        })
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::describe::WasmDescribe for RtcStatsType {
    fn describe() {
        wasm_bindgen::JsValue::describe()
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::IntoWasmAbi for RtcStatsType {
    type Abi = <wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi;
    #[inline]
    fn into_abi(self) -> Self::Abi {
        wasm_bindgen::JsValue::from(self).into_abi()
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::FromWasmAbi for RtcStatsType {
    type Abi = <wasm_bindgen::JsValue as wasm_bindgen::convert::FromWasmAbi>::Abi;
    unsafe fn from_abi(js: Self::Abi) -> Self {
        RtcStatsType::from_js_value(&wasm_bindgen::JsValue::from_abi(js))
            .unwrap_or(RtcStatsType::__Nonexhaustive)
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::OptionIntoWasmAbi for RtcStatsType {
    #[inline]
    fn none() -> Self::Abi {
        Object::none()
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::OptionFromWasmAbi for RtcStatsType {
    #[inline]
    fn is_none(abi: &Self::Abi) -> bool {
        Object::is_none(abi)
    }
}
#[allow(clippy::all)]
impl From<RtcStatsType> for wasm_bindgen::JsValue {
    fn from(obj: RtcStatsType) -> wasm_bindgen::JsValue {
        match obj {
            RtcStatsType::InboundRtp => wasm_bindgen::JsValue::from_str("inbound-rtp"),
            RtcStatsType::OutboundRtp => wasm_bindgen::JsValue::from_str("outbound-rtp"),
            RtcStatsType::Csrc => wasm_bindgen::JsValue::from_str("csrc"),
            RtcStatsType::Session => wasm_bindgen::JsValue::from_str("session"),
            RtcStatsType::Track => wasm_bindgen::JsValue::from_str("track"),
            RtcStatsType::Transport => wasm_bindgen::JsValue::from_str("transport"),
            RtcStatsType::CandidatePair => wasm_bindgen::JsValue::from_str("candidate-pair"),
            RtcStatsType::LocalCandidate => wasm_bindgen::JsValue::from_str("local-candidate"),
            RtcStatsType::RemoteCandidate => wasm_bindgen::JsValue::from_str("remote-candidate"),
            RtcStatsType::__Nonexhaustive => {
                panic!("attempted to convert invalid RtcStatsType into JSValue")
            }
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_d825069b6d94025f: [u8; 108usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}*\0\0\0\0\0\x01\0\0\x03\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
