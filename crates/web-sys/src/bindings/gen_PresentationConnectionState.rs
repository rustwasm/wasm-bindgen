use super::*;
use js_sys::Object;
#[allow(bad_style)]
#[derive(Copy, Clone, PartialEq, Debug)]
#[allow(clippy::all)]
pub enum PresentationConnectionState {
    Connecting = 0,
    Connected = 1,
    Closed = 2,
    Terminated = 3,
    #[doc(hidden)]
    __Nonexhaustive,
}
#[allow(clippy::all)]
impl PresentationConnectionState {
    pub fn from_js_value(obj: &wasm_bindgen::JsValue) -> Option<PresentationConnectionState> {
        obj.as_string().and_then(|obj_str| match obj_str.as_str() {
            "connecting" => Some(PresentationConnectionState::Connecting),
            "connected" => Some(PresentationConnectionState::Connected),
            "closed" => Some(PresentationConnectionState::Closed),
            "terminated" => Some(PresentationConnectionState::Terminated),
            _ => None,
        })
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::describe::WasmDescribe for PresentationConnectionState {
    fn describe() {
        wasm_bindgen::JsValue::describe()
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::IntoWasmAbi for PresentationConnectionState {
    type Abi = <wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi;
    #[inline]
    fn into_abi(self) -> Self::Abi {
        wasm_bindgen::JsValue::from(self).into_abi()
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::FromWasmAbi for PresentationConnectionState {
    type Abi = <wasm_bindgen::JsValue as wasm_bindgen::convert::FromWasmAbi>::Abi;
    unsafe fn from_abi(js: Self::Abi) -> Self {
        PresentationConnectionState::from_js_value(&wasm_bindgen::JsValue::from_abi(js))
            .unwrap_or(PresentationConnectionState::__Nonexhaustive)
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::OptionIntoWasmAbi for PresentationConnectionState {
    #[inline]
    fn none() -> Self::Abi {
        Object::none()
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::OptionFromWasmAbi for PresentationConnectionState {
    #[inline]
    fn is_none(abi: &Self::Abi) -> bool {
        Object::is_none(abi)
    }
}
#[allow(clippy::all)]
impl From<PresentationConnectionState> for wasm_bindgen::JsValue {
    fn from(obj: PresentationConnectionState) -> wasm_bindgen::JsValue {
        match obj {
            PresentationConnectionState::Connecting => {
                wasm_bindgen::JsValue::from_str("connecting")
            }
            PresentationConnectionState::Connected => wasm_bindgen::JsValue::from_str("connected"),
            PresentationConnectionState::Closed => wasm_bindgen::JsValue::from_str("closed"),
            PresentationConnectionState::Terminated => {
                wasm_bindgen::JsValue::from_str("terminated")
            }
            PresentationConnectionState::__Nonexhaustive => {
                panic!("attempted to convert invalid PresentationConnectionState into JSValue")
            }
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_e63d05037f71b325: [u8; 108usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}*\0\0\0\0\0\x01\0\0\x03\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
