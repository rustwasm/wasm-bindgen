use super::*;
#[allow(bad_style)]
#[derive(Copy, Clone, PartialEq, Debug)]
#[allow(clippy::all)]
pub enum FontFaceLoadStatus {
    Unloaded = 0,
    Loading = 1,
    Loaded = 2,
    Error = 3,
    #[doc(hidden)]
    __Nonexhaustive,
}
#[allow(clippy::all)]
impl FontFaceLoadStatus {
    pub fn from_js_value(obj: &wasm_bindgen::JsValue) -> Option<FontFaceLoadStatus> {
        obj.as_string().and_then(|obj_str| match obj_str.as_str() {
            "unloaded" => Some(FontFaceLoadStatus::Unloaded),
            "loading" => Some(FontFaceLoadStatus::Loading),
            "loaded" => Some(FontFaceLoadStatus::Loaded),
            "error" => Some(FontFaceLoadStatus::Error),
            _ => None,
        })
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::describe::WasmDescribe for FontFaceLoadStatus {
    fn describe() {
        wasm_bindgen::JsValue::describe()
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::IntoWasmAbi for FontFaceLoadStatus {
    type Abi = <wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi;
    #[inline]
    fn into_abi(self) -> Self::Abi {
        wasm_bindgen::JsValue::from(self).into_abi()
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::FromWasmAbi for FontFaceLoadStatus {
    type Abi = <wasm_bindgen::JsValue as wasm_bindgen::convert::FromWasmAbi>::Abi;
    unsafe fn from_abi(js: Self::Abi) -> Self {
        FontFaceLoadStatus::from_js_value(&wasm_bindgen::JsValue::from_abi(js))
            .unwrap_or(FontFaceLoadStatus::__Nonexhaustive)
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::OptionIntoWasmAbi for FontFaceLoadStatus {
    #[inline]
    fn none() -> Self::Abi {
        Object::none()
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::OptionFromWasmAbi for FontFaceLoadStatus {
    #[inline]
    fn is_none(abi: &Self::Abi) -> bool {
        Object::is_none(abi)
    }
}
#[allow(clippy::all)]
impl From<FontFaceLoadStatus> for wasm_bindgen::JsValue {
    fn from(obj: FontFaceLoadStatus) -> wasm_bindgen::JsValue {
        match obj {
            FontFaceLoadStatus::Unloaded => wasm_bindgen::JsValue::from_str("unloaded"),
            FontFaceLoadStatus::Loading => wasm_bindgen::JsValue::from_str("loading"),
            FontFaceLoadStatus::Loaded => wasm_bindgen::JsValue::from_str("loaded"),
            FontFaceLoadStatus::Error => wasm_bindgen::JsValue::from_str("error"),
            FontFaceLoadStatus::__Nonexhaustive => {
                panic!("attempted to convert invalid FontFaceLoadStatus into JSValue")
            }
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_9844468125e4cc72: [u8; 108usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}*\0\0\0\0\0\x01\0\0\x03\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
