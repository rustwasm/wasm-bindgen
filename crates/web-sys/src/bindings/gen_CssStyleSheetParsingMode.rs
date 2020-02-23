use super::*;
#[allow(bad_style)]
#[derive(Copy, Clone, PartialEq, Debug)]
#[allow(clippy::all)]
pub enum CssStyleSheetParsingMode {
    Author = 0,
    User = 1,
    Agent = 2,
    #[doc(hidden)]
    __Nonexhaustive,
}
#[allow(clippy::all)]
impl CssStyleSheetParsingMode {
    pub fn from_js_value(obj: &wasm_bindgen::JsValue) -> Option<CssStyleSheetParsingMode> {
        obj.as_string().and_then(|obj_str| match obj_str.as_str() {
            "author" => Some(CssStyleSheetParsingMode::Author),
            "user" => Some(CssStyleSheetParsingMode::User),
            "agent" => Some(CssStyleSheetParsingMode::Agent),
            _ => None,
        })
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::describe::WasmDescribe for CssStyleSheetParsingMode {
    fn describe() {
        wasm_bindgen::JsValue::describe()
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::IntoWasmAbi for CssStyleSheetParsingMode {
    type Abi = <wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi;
    #[inline]
    fn into_abi(self) -> Self::Abi {
        wasm_bindgen::JsValue::from(self).into_abi()
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::FromWasmAbi for CssStyleSheetParsingMode {
    type Abi = <wasm_bindgen::JsValue as wasm_bindgen::convert::FromWasmAbi>::Abi;
    unsafe fn from_abi(js: Self::Abi) -> Self {
        CssStyleSheetParsingMode::from_js_value(&wasm_bindgen::JsValue::from_abi(js))
            .unwrap_or(CssStyleSheetParsingMode::__Nonexhaustive)
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::OptionIntoWasmAbi for CssStyleSheetParsingMode {
    #[inline]
    fn none() -> Self::Abi {
        Object::none()
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::OptionFromWasmAbi for CssStyleSheetParsingMode {
    #[inline]
    fn is_none(abi: &Self::Abi) -> bool {
        Object::is_none(abi)
    }
}
#[allow(clippy::all)]
impl From<CssStyleSheetParsingMode> for wasm_bindgen::JsValue {
    fn from(obj: CssStyleSheetParsingMode) -> wasm_bindgen::JsValue {
        match obj {
            CssStyleSheetParsingMode::Author => wasm_bindgen::JsValue::from_str("author"),
            CssStyleSheetParsingMode::User => wasm_bindgen::JsValue::from_str("user"),
            CssStyleSheetParsingMode::Agent => wasm_bindgen::JsValue::from_str("agent"),
            CssStyleSheetParsingMode::__Nonexhaustive => {
                panic!("attempted to convert invalid CssStyleSheetParsingMode into JSValue")
            }
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_32282370ef9e1604: [u8; 108usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}*\0\0\0\0\0\x01\0\0\x03\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
