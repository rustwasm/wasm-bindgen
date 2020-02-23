use super::*;
#[allow(bad_style)]
#[derive(Copy, Clone, PartialEq, Debug)]
#[allow(clippy::all)]
pub enum RtcIceGatheringState {
    New = 0,
    Gathering = 1,
    Complete = 2,
    #[doc(hidden)]
    __Nonexhaustive,
}
#[allow(clippy::all)]
impl RtcIceGatheringState {
    pub fn from_js_value(obj: &wasm_bindgen::JsValue) -> Option<RtcIceGatheringState> {
        obj.as_string().and_then(|obj_str| match obj_str.as_str() {
            "new" => Some(RtcIceGatheringState::New),
            "gathering" => Some(RtcIceGatheringState::Gathering),
            "complete" => Some(RtcIceGatheringState::Complete),
            _ => None,
        })
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::describe::WasmDescribe for RtcIceGatheringState {
    fn describe() {
        wasm_bindgen::JsValue::describe()
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::IntoWasmAbi for RtcIceGatheringState {
    type Abi = <wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi;
    #[inline]
    fn into_abi(self) -> Self::Abi {
        wasm_bindgen::JsValue::from(self).into_abi()
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::FromWasmAbi for RtcIceGatheringState {
    type Abi = <wasm_bindgen::JsValue as wasm_bindgen::convert::FromWasmAbi>::Abi;
    unsafe fn from_abi(js: Self::Abi) -> Self {
        RtcIceGatheringState::from_js_value(&wasm_bindgen::JsValue::from_abi(js))
            .unwrap_or(RtcIceGatheringState::__Nonexhaustive)
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::OptionIntoWasmAbi for RtcIceGatheringState {
    #[inline]
    fn none() -> Self::Abi {
        Object::none()
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::OptionFromWasmAbi for RtcIceGatheringState {
    #[inline]
    fn is_none(abi: &Self::Abi) -> bool {
        Object::is_none(abi)
    }
}
#[allow(clippy::all)]
impl From<RtcIceGatheringState> for wasm_bindgen::JsValue {
    fn from(obj: RtcIceGatheringState) -> wasm_bindgen::JsValue {
        match obj {
            RtcIceGatheringState::New => wasm_bindgen::JsValue::from_str("new"),
            RtcIceGatheringState::Gathering => wasm_bindgen::JsValue::from_str("gathering"),
            RtcIceGatheringState::Complete => wasm_bindgen::JsValue::from_str("complete"),
            RtcIceGatheringState::__Nonexhaustive => {
                panic!("attempted to convert invalid RtcIceGatheringState into JSValue")
            }
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_2bafa447678b9b99: [u8; 108usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}*\0\0\0\0\0\x01\0\0\x03\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
