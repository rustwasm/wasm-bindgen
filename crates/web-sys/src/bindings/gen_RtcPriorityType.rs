use super::*;
#[allow(bad_style)]
#[derive(Copy, Clone, PartialEq, Debug)]
#[allow(clippy::all)]
pub enum RtcPriorityType {
    VeryLow = 0,
    Low = 1,
    Medium = 2,
    High = 3,
    #[doc(hidden)]
    __Nonexhaustive,
}
#[allow(clippy::all)]
impl RtcPriorityType {
    pub fn from_js_value(obj: &wasm_bindgen::JsValue) -> Option<RtcPriorityType> {
        obj.as_string().and_then(|obj_str| match obj_str.as_str() {
            "very-low" => Some(RtcPriorityType::VeryLow),
            "low" => Some(RtcPriorityType::Low),
            "medium" => Some(RtcPriorityType::Medium),
            "high" => Some(RtcPriorityType::High),
            _ => None,
        })
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::describe::WasmDescribe for RtcPriorityType {
    fn describe() {
        wasm_bindgen::JsValue::describe()
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::IntoWasmAbi for RtcPriorityType {
    type Abi = <wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi;
    #[inline]
    fn into_abi(self) -> Self::Abi {
        wasm_bindgen::JsValue::from(self).into_abi()
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::FromWasmAbi for RtcPriorityType {
    type Abi = <wasm_bindgen::JsValue as wasm_bindgen::convert::FromWasmAbi>::Abi;
    unsafe fn from_abi(js: Self::Abi) -> Self {
        RtcPriorityType::from_js_value(&wasm_bindgen::JsValue::from_abi(js))
            .unwrap_or(RtcPriorityType::__Nonexhaustive)
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::OptionIntoWasmAbi for RtcPriorityType {
    #[inline]
    fn none() -> Self::Abi {
        Object::none()
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::OptionFromWasmAbi for RtcPriorityType {
    #[inline]
    fn is_none(abi: &Self::Abi) -> bool {
        Object::is_none(abi)
    }
}
#[allow(clippy::all)]
impl From<RtcPriorityType> for wasm_bindgen::JsValue {
    fn from(obj: RtcPriorityType) -> wasm_bindgen::JsValue {
        match obj {
            RtcPriorityType::VeryLow => wasm_bindgen::JsValue::from_str("very-low"),
            RtcPriorityType::Low => wasm_bindgen::JsValue::from_str("low"),
            RtcPriorityType::Medium => wasm_bindgen::JsValue::from_str("medium"),
            RtcPriorityType::High => wasm_bindgen::JsValue::from_str("high"),
            RtcPriorityType::__Nonexhaustive => {
                panic!("attempted to convert invalid RtcPriorityType into JSValue")
            }
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_f91fae74884b1bb9: [u8; 108usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}*\0\0\0\0\0\x01\0\0\x03\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
