use super::*;
use js_sys::Object;
#[allow(bad_style)]
#[derive(Copy, Clone, PartialEq, Debug)]
#[allow(clippy::all)]
pub enum PcImplIceConnectionState {
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
impl PcImplIceConnectionState {
    pub fn from_js_value(obj: &wasm_bindgen::JsValue) -> Option<PcImplIceConnectionState> {
        obj.as_string().and_then(|obj_str| match obj_str.as_str() {
            "new" => Some(PcImplIceConnectionState::New),
            "checking" => Some(PcImplIceConnectionState::Checking),
            "connected" => Some(PcImplIceConnectionState::Connected),
            "completed" => Some(PcImplIceConnectionState::Completed),
            "failed" => Some(PcImplIceConnectionState::Failed),
            "disconnected" => Some(PcImplIceConnectionState::Disconnected),
            "closed" => Some(PcImplIceConnectionState::Closed),
            _ => None,
        })
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::describe::WasmDescribe for PcImplIceConnectionState {
    fn describe() {
        wasm_bindgen::JsValue::describe()
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::IntoWasmAbi for PcImplIceConnectionState {
    type Abi = <wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi;
    #[inline]
    fn into_abi(self) -> Self::Abi {
        wasm_bindgen::JsValue::from(self).into_abi()
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::FromWasmAbi for PcImplIceConnectionState {
    type Abi = <wasm_bindgen::JsValue as wasm_bindgen::convert::FromWasmAbi>::Abi;
    unsafe fn from_abi(js: Self::Abi) -> Self {
        PcImplIceConnectionState::from_js_value(&wasm_bindgen::JsValue::from_abi(js))
            .unwrap_or(PcImplIceConnectionState::__Nonexhaustive)
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::OptionIntoWasmAbi for PcImplIceConnectionState {
    #[inline]
    fn none() -> Self::Abi {
        Object::none()
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::OptionFromWasmAbi for PcImplIceConnectionState {
    #[inline]
    fn is_none(abi: &Self::Abi) -> bool {
        Object::is_none(abi)
    }
}
#[allow(clippy::all)]
impl From<PcImplIceConnectionState> for wasm_bindgen::JsValue {
    fn from(obj: PcImplIceConnectionState) -> wasm_bindgen::JsValue {
        match obj {
            PcImplIceConnectionState::New => wasm_bindgen::JsValue::from_str("new"),
            PcImplIceConnectionState::Checking => wasm_bindgen::JsValue::from_str("checking"),
            PcImplIceConnectionState::Connected => wasm_bindgen::JsValue::from_str("connected"),
            PcImplIceConnectionState::Completed => wasm_bindgen::JsValue::from_str("completed"),
            PcImplIceConnectionState::Failed => wasm_bindgen::JsValue::from_str("failed"),
            PcImplIceConnectionState::Disconnected => {
                wasm_bindgen::JsValue::from_str("disconnected")
            }
            PcImplIceConnectionState::Closed => wasm_bindgen::JsValue::from_str("closed"),
            PcImplIceConnectionState::__Nonexhaustive => {
                panic!("attempted to convert invalid PcImplIceConnectionState into JSValue")
            }
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_3a7d2f24a1cc42df: [u8; 108usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}*\0\0\0\0\0\x01\0\0\x03\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
