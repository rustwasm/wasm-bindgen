use super::*;
#[allow(bad_style)]
#[derive(Copy, Clone, PartialEq, Debug)]
#[allow(clippy::all)]
pub enum FlashClassification {
    Unclassified = 0,
    Unknown = 1,
    Allowed = 2,
    Denied = 3,
    #[doc(hidden)]
    __Nonexhaustive,
}
#[allow(clippy::all)]
impl FlashClassification {
    pub fn from_js_value(obj: &wasm_bindgen::JsValue) -> Option<FlashClassification> {
        obj.as_string().and_then(|obj_str| match obj_str.as_str() {
            "unclassified" => Some(FlashClassification::Unclassified),
            "unknown" => Some(FlashClassification::Unknown),
            "allowed" => Some(FlashClassification::Allowed),
            "denied" => Some(FlashClassification::Denied),
            _ => None,
        })
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::describe::WasmDescribe for FlashClassification {
    fn describe() {
        wasm_bindgen::JsValue::describe()
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::IntoWasmAbi for FlashClassification {
    type Abi = <wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi;
    #[inline]
    fn into_abi(self) -> Self::Abi {
        wasm_bindgen::JsValue::from(self).into_abi()
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::FromWasmAbi for FlashClassification {
    type Abi = <wasm_bindgen::JsValue as wasm_bindgen::convert::FromWasmAbi>::Abi;
    unsafe fn from_abi(js: Self::Abi) -> Self {
        FlashClassification::from_js_value(&wasm_bindgen::JsValue::from_abi(js))
            .unwrap_or(FlashClassification::__Nonexhaustive)
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::OptionIntoWasmAbi for FlashClassification {
    #[inline]
    fn none() -> Self::Abi {
        Object::none()
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::OptionFromWasmAbi for FlashClassification {
    #[inline]
    fn is_none(abi: &Self::Abi) -> bool {
        Object::is_none(abi)
    }
}
#[allow(clippy::all)]
impl From<FlashClassification> for wasm_bindgen::JsValue {
    fn from(obj: FlashClassification) -> wasm_bindgen::JsValue {
        match obj {
            FlashClassification::Unclassified => wasm_bindgen::JsValue::from_str("unclassified"),
            FlashClassification::Unknown => wasm_bindgen::JsValue::from_str("unknown"),
            FlashClassification::Allowed => wasm_bindgen::JsValue::from_str("allowed"),
            FlashClassification::Denied => wasm_bindgen::JsValue::from_str("denied"),
            FlashClassification::__Nonexhaustive => {
                panic!("attempted to convert invalid FlashClassification into JSValue")
            }
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_5f49211dae8d4ac7: [u8; 108usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}*\0\0\0\0\0\x01\0\0\x03\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
