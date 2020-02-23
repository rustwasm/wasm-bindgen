use super::*;
#[allow(bad_style)]
#[derive(Copy, Clone, PartialEq, Debug)]
#[allow(clippy::all)]
pub enum XmlHttpRequestResponseType {
    None = 0,
    Arraybuffer = 1,
    Blob = 2,
    Document = 3,
    Json = 4,
    Text = 5,
    #[doc(hidden)]
    __Nonexhaustive,
}
#[allow(clippy::all)]
impl XmlHttpRequestResponseType {
    pub fn from_js_value(obj: &wasm_bindgen::JsValue) -> Option<XmlHttpRequestResponseType> {
        obj.as_string().and_then(|obj_str| match obj_str.as_str() {
            "" => Some(XmlHttpRequestResponseType::None),
            "arraybuffer" => Some(XmlHttpRequestResponseType::Arraybuffer),
            "blob" => Some(XmlHttpRequestResponseType::Blob),
            "document" => Some(XmlHttpRequestResponseType::Document),
            "json" => Some(XmlHttpRequestResponseType::Json),
            "text" => Some(XmlHttpRequestResponseType::Text),
            _ => None,
        })
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::describe::WasmDescribe for XmlHttpRequestResponseType {
    fn describe() {
        wasm_bindgen::JsValue::describe()
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::IntoWasmAbi for XmlHttpRequestResponseType {
    type Abi = <wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi;
    #[inline]
    fn into_abi(self) -> Self::Abi {
        wasm_bindgen::JsValue::from(self).into_abi()
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::FromWasmAbi for XmlHttpRequestResponseType {
    type Abi = <wasm_bindgen::JsValue as wasm_bindgen::convert::FromWasmAbi>::Abi;
    unsafe fn from_abi(js: Self::Abi) -> Self {
        XmlHttpRequestResponseType::from_js_value(&wasm_bindgen::JsValue::from_abi(js))
            .unwrap_or(XmlHttpRequestResponseType::__Nonexhaustive)
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::OptionIntoWasmAbi for XmlHttpRequestResponseType {
    #[inline]
    fn none() -> Self::Abi {
        Object::none()
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::OptionFromWasmAbi for XmlHttpRequestResponseType {
    #[inline]
    fn is_none(abi: &Self::Abi) -> bool {
        Object::is_none(abi)
    }
}
#[allow(clippy::all)]
impl From<XmlHttpRequestResponseType> for wasm_bindgen::JsValue {
    fn from(obj: XmlHttpRequestResponseType) -> wasm_bindgen::JsValue {
        match obj {
            XmlHttpRequestResponseType::None => wasm_bindgen::JsValue::from_str(""),
            XmlHttpRequestResponseType::Arraybuffer => {
                wasm_bindgen::JsValue::from_str("arraybuffer")
            }
            XmlHttpRequestResponseType::Blob => wasm_bindgen::JsValue::from_str("blob"),
            XmlHttpRequestResponseType::Document => wasm_bindgen::JsValue::from_str("document"),
            XmlHttpRequestResponseType::Json => wasm_bindgen::JsValue::from_str("json"),
            XmlHttpRequestResponseType::Text => wasm_bindgen::JsValue::from_str("text"),
            XmlHttpRequestResponseType::__Nonexhaustive => {
                panic!("attempted to convert invalid XmlHttpRequestResponseType into JSValue")
            }
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_cf56334b854d86c5: [u8; 108usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}*\0\0\0\0\0\x01\0\0\x03\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
