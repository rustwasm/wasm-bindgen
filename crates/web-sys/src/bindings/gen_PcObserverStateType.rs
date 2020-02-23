use super::*;
use js_sys::Object;
#[allow(bad_style)]
#[derive(Copy, Clone, PartialEq, Debug)]
#[allow(clippy::all)]
pub enum PcObserverStateType {
    None = 0,
    IceConnectionState = 1,
    IceGatheringState = 2,
    SignalingState = 3,
    #[doc(hidden)]
    __Nonexhaustive,
}
#[allow(clippy::all)]
impl PcObserverStateType {
    pub fn from_js_value(obj: &wasm_bindgen::JsValue) -> Option<PcObserverStateType> {
        obj.as_string().and_then(|obj_str| match obj_str.as_str() {
            "None" => Some(PcObserverStateType::None),
            "IceConnectionState" => Some(PcObserverStateType::IceConnectionState),
            "IceGatheringState" => Some(PcObserverStateType::IceGatheringState),
            "SignalingState" => Some(PcObserverStateType::SignalingState),
            _ => None,
        })
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::describe::WasmDescribe for PcObserverStateType {
    fn describe() {
        wasm_bindgen::JsValue::describe()
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::IntoWasmAbi for PcObserverStateType {
    type Abi = <wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi;
    #[inline]
    fn into_abi(self) -> Self::Abi {
        wasm_bindgen::JsValue::from(self).into_abi()
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::FromWasmAbi for PcObserverStateType {
    type Abi = <wasm_bindgen::JsValue as wasm_bindgen::convert::FromWasmAbi>::Abi;
    unsafe fn from_abi(js: Self::Abi) -> Self {
        PcObserverStateType::from_js_value(&wasm_bindgen::JsValue::from_abi(js))
            .unwrap_or(PcObserverStateType::__Nonexhaustive)
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::OptionIntoWasmAbi for PcObserverStateType {
    #[inline]
    fn none() -> Self::Abi {
        Object::none()
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::OptionFromWasmAbi for PcObserverStateType {
    #[inline]
    fn is_none(abi: &Self::Abi) -> bool {
        Object::is_none(abi)
    }
}
#[allow(clippy::all)]
impl From<PcObserverStateType> for wasm_bindgen::JsValue {
    fn from(obj: PcObserverStateType) -> wasm_bindgen::JsValue {
        match obj {
            PcObserverStateType::None => wasm_bindgen::JsValue::from_str("None"),
            PcObserverStateType::IceConnectionState => {
                wasm_bindgen::JsValue::from_str("IceConnectionState")
            }
            PcObserverStateType::IceGatheringState => {
                wasm_bindgen::JsValue::from_str("IceGatheringState")
            }
            PcObserverStateType::SignalingState => {
                wasm_bindgen::JsValue::from_str("SignalingState")
            }
            PcObserverStateType::__Nonexhaustive => {
                panic!("attempted to convert invalid PcObserverStateType into JSValue")
            }
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_3516edc142036b81: [u8; 108usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}*\0\0\0\0\0\x01\0\0\x03\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
