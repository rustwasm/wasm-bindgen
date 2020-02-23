use super::*;
use js_sys::Object;
#[allow(bad_style)]
#[derive(Copy, Clone, PartialEq, Debug)]
#[allow(clippy::all)]
pub enum SupportedType {
    TextHtml = 0,
    TextXml = 1,
    ApplicationXml = 2,
    ApplicationXhtmlXml = 3,
    ImageSvgXml = 4,
    #[doc(hidden)]
    __Nonexhaustive,
}
#[allow(clippy::all)]
impl SupportedType {
    pub fn from_js_value(obj: &wasm_bindgen::JsValue) -> Option<SupportedType> {
        obj.as_string().and_then(|obj_str| match obj_str.as_str() {
            "text/html" => Some(SupportedType::TextHtml),
            "text/xml" => Some(SupportedType::TextXml),
            "application/xml" => Some(SupportedType::ApplicationXml),
            "application/xhtml+xml" => Some(SupportedType::ApplicationXhtmlXml),
            "image/svg+xml" => Some(SupportedType::ImageSvgXml),
            _ => None,
        })
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::describe::WasmDescribe for SupportedType {
    fn describe() {
        wasm_bindgen::JsValue::describe()
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::IntoWasmAbi for SupportedType {
    type Abi = <wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi;
    #[inline]
    fn into_abi(self) -> Self::Abi {
        wasm_bindgen::JsValue::from(self).into_abi()
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::FromWasmAbi for SupportedType {
    type Abi = <wasm_bindgen::JsValue as wasm_bindgen::convert::FromWasmAbi>::Abi;
    unsafe fn from_abi(js: Self::Abi) -> Self {
        SupportedType::from_js_value(&wasm_bindgen::JsValue::from_abi(js))
            .unwrap_or(SupportedType::__Nonexhaustive)
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::OptionIntoWasmAbi for SupportedType {
    #[inline]
    fn none() -> Self::Abi {
        Object::none()
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::OptionFromWasmAbi for SupportedType {
    #[inline]
    fn is_none(abi: &Self::Abi) -> bool {
        Object::is_none(abi)
    }
}
#[allow(clippy::all)]
impl From<SupportedType> for wasm_bindgen::JsValue {
    fn from(obj: SupportedType) -> wasm_bindgen::JsValue {
        match obj {
            SupportedType::TextHtml => wasm_bindgen::JsValue::from_str("text/html"),
            SupportedType::TextXml => wasm_bindgen::JsValue::from_str("text/xml"),
            SupportedType::ApplicationXml => wasm_bindgen::JsValue::from_str("application/xml"),
            SupportedType::ApplicationXhtmlXml => {
                wasm_bindgen::JsValue::from_str("application/xhtml+xml")
            }
            SupportedType::ImageSvgXml => wasm_bindgen::JsValue::from_str("image/svg+xml"),
            SupportedType::__Nonexhaustive => {
                panic!("attempted to convert invalid SupportedType into JSValue")
            }
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_32a682e056445704: [u8; 108usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}*\0\0\0\0\0\x01\0\0\x03\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
