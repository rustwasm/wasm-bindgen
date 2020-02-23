use super::*;
use js_sys::Object;
#[allow(bad_style)]
#[derive(Copy, Clone, PartialEq, Debug)]
#[allow(clippy::all)]
pub enum CssBoxType {
    Margin = 0,
    Border = 1,
    Padding = 2,
    Content = 3,
    #[doc(hidden)]
    __Nonexhaustive,
}
#[allow(clippy::all)]
impl CssBoxType {
    pub fn from_js_value(obj: &wasm_bindgen::JsValue) -> Option<CssBoxType> {
        obj.as_string().and_then(|obj_str| match obj_str.as_str() {
            "margin" => Some(CssBoxType::Margin),
            "border" => Some(CssBoxType::Border),
            "padding" => Some(CssBoxType::Padding),
            "content" => Some(CssBoxType::Content),
            _ => None,
        })
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::describe::WasmDescribe for CssBoxType {
    fn describe() {
        wasm_bindgen::JsValue::describe()
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::IntoWasmAbi for CssBoxType {
    type Abi = <wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi;
    #[inline]
    fn into_abi(self) -> Self::Abi {
        wasm_bindgen::JsValue::from(self).into_abi()
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::FromWasmAbi for CssBoxType {
    type Abi = <wasm_bindgen::JsValue as wasm_bindgen::convert::FromWasmAbi>::Abi;
    unsafe fn from_abi(js: Self::Abi) -> Self {
        CssBoxType::from_js_value(&wasm_bindgen::JsValue::from_abi(js))
            .unwrap_or(CssBoxType::__Nonexhaustive)
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::OptionIntoWasmAbi for CssBoxType {
    #[inline]
    fn none() -> Self::Abi {
        Object::none()
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::OptionFromWasmAbi for CssBoxType {
    #[inline]
    fn is_none(abi: &Self::Abi) -> bool {
        Object::is_none(abi)
    }
}
#[allow(clippy::all)]
impl From<CssBoxType> for wasm_bindgen::JsValue {
    fn from(obj: CssBoxType) -> wasm_bindgen::JsValue {
        match obj {
            CssBoxType::Margin => wasm_bindgen::JsValue::from_str("margin"),
            CssBoxType::Border => wasm_bindgen::JsValue::from_str("border"),
            CssBoxType::Padding => wasm_bindgen::JsValue::from_str("padding"),
            CssBoxType::Content => wasm_bindgen::JsValue::from_str("content"),
            CssBoxType::__Nonexhaustive => {
                panic!("attempted to convert invalid CssBoxType into JSValue")
            }
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_d98ede1bc3c1a740: [u8; 108usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}*\0\0\0\0\0\x01\0\0\x03\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
