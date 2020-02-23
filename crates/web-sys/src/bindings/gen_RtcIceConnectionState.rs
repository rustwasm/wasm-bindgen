use super::*;
use js_sys::Object;
#[allow(bad_style)]
#[derive(Copy, Clone, PartialEq, Debug)]
#[allow(clippy::all)]
pub enum RtcIceConnectionState {
    New = 0,
    Checking = 1,
    Connected = 2,
    Completed = 3,
    Failed = 4,
    Disconnected = 5,
    Closed = 6,
    #[doc(hidden)]
    __Nonexhaustive,
}
#[allow(clippy::all)]
impl RtcIceConnectionState {
    pub fn from_js_value(obj: &wasm_bindgen::JsValue) -> Option<RtcIceConnectionState> {
        obj.as_string().and_then(|obj_str| match obj_str.as_str() {
            "new" => Some(RtcIceConnectionState::New),
            "checking" => Some(RtcIceConnectionState::Checking),
            "connected" => Some(RtcIceConnectionState::Connected),
            "completed" => Some(RtcIceConnectionState::Completed),
            "failed" => Some(RtcIceConnectionState::Failed),
            "disconnected" => Some(RtcIceConnectionState::Disconnected),
            "closed" => Some(RtcIceConnectionState::Closed),
            _ => None,
        })
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::describe::WasmDescribe for RtcIceConnectionState {
    fn describe() {
        wasm_bindgen::JsValue::describe()
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::IntoWasmAbi for RtcIceConnectionState {
    type Abi = <wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi;
    #[inline]
    fn into_abi(self) -> Self::Abi {
        wasm_bindgen::JsValue::from(self).into_abi()
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::FromWasmAbi for RtcIceConnectionState {
    type Abi = <wasm_bindgen::JsValue as wasm_bindgen::convert::FromWasmAbi>::Abi;
    unsafe fn from_abi(js: Self::Abi) -> Self {
        RtcIceConnectionState::from_js_value(&wasm_bindgen::JsValue::from_abi(js))
            .unwrap_or(RtcIceConnectionState::__Nonexhaustive)
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::OptionIntoWasmAbi for RtcIceConnectionState {
    #[inline]
    fn none() -> Self::Abi {
        Object::none()
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::OptionFromWasmAbi for RtcIceConnectionState {
    #[inline]
    fn is_none(abi: &Self::Abi) -> bool {
        Object::is_none(abi)
    }
}
#[allow(clippy::all)]
impl From<RtcIceConnectionState> for wasm_bindgen::JsValue {
    fn from(obj: RtcIceConnectionState) -> wasm_bindgen::JsValue {
        match obj {
            RtcIceConnectionState::New => wasm_bindgen::JsValue::from_str("new"),
            RtcIceConnectionState::Checking => wasm_bindgen::JsValue::from_str("checking"),
            RtcIceConnectionState::Connected => wasm_bindgen::JsValue::from_str("connected"),
            RtcIceConnectionState::Completed => wasm_bindgen::JsValue::from_str("completed"),
            RtcIceConnectionState::Failed => wasm_bindgen::JsValue::from_str("failed"),
            RtcIceConnectionState::Disconnected => wasm_bindgen::JsValue::from_str("disconnected"),
            RtcIceConnectionState::Closed => wasm_bindgen::JsValue::from_str("closed"),
            RtcIceConnectionState::__Nonexhaustive => {
                panic!("attempted to convert invalid RtcIceConnectionState into JSValue")
            }
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_134d5f8db8b77458: [u8; 108usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}*\0\0\0\0\0\x01\0\0\x03\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
