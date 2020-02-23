use super::*;
use js_sys::Object;
#[allow(bad_style)]
#[derive(Copy, Clone, PartialEq, Debug)]
#[allow(clippy::all)]
pub enum OverSampleType {
    None = 0,
    N2x = 1,
    N4x = 2,
    #[doc(hidden)]
    __Nonexhaustive,
}
#[allow(clippy::all)]
impl OverSampleType {
    pub fn from_js_value(obj: &wasm_bindgen::JsValue) -> Option<OverSampleType> {
        obj.as_string().and_then(|obj_str| match obj_str.as_str() {
            "none" => Some(OverSampleType::None),
            "2x" => Some(OverSampleType::N2x),
            "4x" => Some(OverSampleType::N4x),
            _ => None,
        })
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::describe::WasmDescribe for OverSampleType {
    fn describe() {
        wasm_bindgen::JsValue::describe()
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::IntoWasmAbi for OverSampleType {
    type Abi = <wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi;
    #[inline]
    fn into_abi(self) -> Self::Abi {
        wasm_bindgen::JsValue::from(self).into_abi()
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::FromWasmAbi for OverSampleType {
    type Abi = <wasm_bindgen::JsValue as wasm_bindgen::convert::FromWasmAbi>::Abi;
    unsafe fn from_abi(js: Self::Abi) -> Self {
        OverSampleType::from_js_value(&wasm_bindgen::JsValue::from_abi(js))
            .unwrap_or(OverSampleType::__Nonexhaustive)
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::OptionIntoWasmAbi for OverSampleType {
    #[inline]
    fn none() -> Self::Abi {
        Object::none()
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::OptionFromWasmAbi for OverSampleType {
    #[inline]
    fn is_none(abi: &Self::Abi) -> bool {
        Object::is_none(abi)
    }
}
#[allow(clippy::all)]
impl From<OverSampleType> for wasm_bindgen::JsValue {
    fn from(obj: OverSampleType) -> wasm_bindgen::JsValue {
        match obj {
            OverSampleType::None => wasm_bindgen::JsValue::from_str("none"),
            OverSampleType::N2x => wasm_bindgen::JsValue::from_str("2x"),
            OverSampleType::N4x => wasm_bindgen::JsValue::from_str("4x"),
            OverSampleType::__Nonexhaustive => {
                panic!("attempted to convert invalid OverSampleType into JSValue")
            }
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_df8a01d5ca0001b6: [u8; 108usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}*\0\0\0\0\0\x01\0\0\x03\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
