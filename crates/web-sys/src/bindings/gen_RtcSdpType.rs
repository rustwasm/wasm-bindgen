use super::*;
#[allow(bad_style)]
#[derive(Copy, Clone, PartialEq, Debug)]
#[allow(clippy::all)]
pub enum RtcSdpType {
    Offer = 0,
    Pranswer = 1,
    Answer = 2,
    Rollback = 3,
    #[doc(hidden)]
    __Nonexhaustive,
}
#[allow(clippy::all)]
impl RtcSdpType {
    pub fn from_js_value(obj: &wasm_bindgen::JsValue) -> Option<RtcSdpType> {
        obj.as_string().and_then(|obj_str| match obj_str.as_str() {
            "offer" => Some(RtcSdpType::Offer),
            "pranswer" => Some(RtcSdpType::Pranswer),
            "answer" => Some(RtcSdpType::Answer),
            "rollback" => Some(RtcSdpType::Rollback),
            _ => None,
        })
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::describe::WasmDescribe for RtcSdpType {
    fn describe() {
        wasm_bindgen::JsValue::describe()
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::IntoWasmAbi for RtcSdpType {
    type Abi = <wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi;
    #[inline]
    fn into_abi(self) -> Self::Abi {
        wasm_bindgen::JsValue::from(self).into_abi()
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::FromWasmAbi for RtcSdpType {
    type Abi = <wasm_bindgen::JsValue as wasm_bindgen::convert::FromWasmAbi>::Abi;
    unsafe fn from_abi(js: Self::Abi) -> Self {
        RtcSdpType::from_js_value(&wasm_bindgen::JsValue::from_abi(js))
            .unwrap_or(RtcSdpType::__Nonexhaustive)
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::OptionIntoWasmAbi for RtcSdpType {
    #[inline]
    fn none() -> Self::Abi {
        Object::none()
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::OptionFromWasmAbi for RtcSdpType {
    #[inline]
    fn is_none(abi: &Self::Abi) -> bool {
        Object::is_none(abi)
    }
}
#[allow(clippy::all)]
impl From<RtcSdpType> for wasm_bindgen::JsValue {
    fn from(obj: RtcSdpType) -> wasm_bindgen::JsValue {
        match obj {
            RtcSdpType::Offer => wasm_bindgen::JsValue::from_str("offer"),
            RtcSdpType::Pranswer => wasm_bindgen::JsValue::from_str("pranswer"),
            RtcSdpType::Answer => wasm_bindgen::JsValue::from_str("answer"),
            RtcSdpType::Rollback => wasm_bindgen::JsValue::from_str("rollback"),
            RtcSdpType::__Nonexhaustive => {
                panic!("attempted to convert invalid RtcSdpType into JSValue")
            }
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_61bb736ec8367069: [u8; 108usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}*\0\0\0\0\0\x01\0\0\x03\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
