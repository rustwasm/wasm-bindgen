use super::*;
use js_sys::Object;
#[allow(bad_style)]
#[derive(Copy, Clone, PartialEq, Debug)]
#[allow(clippy::all)]
pub enum TextTrackKind {
    Subtitles = 0,
    Captions = 1,
    Descriptions = 2,
    Chapters = 3,
    Metadata = 4,
    #[doc(hidden)]
    __Nonexhaustive,
}
#[allow(clippy::all)]
impl TextTrackKind {
    pub fn from_js_value(obj: &wasm_bindgen::JsValue) -> Option<TextTrackKind> {
        obj.as_string().and_then(|obj_str| match obj_str.as_str() {
            "subtitles" => Some(TextTrackKind::Subtitles),
            "captions" => Some(TextTrackKind::Captions),
            "descriptions" => Some(TextTrackKind::Descriptions),
            "chapters" => Some(TextTrackKind::Chapters),
            "metadata" => Some(TextTrackKind::Metadata),
            _ => None,
        })
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::describe::WasmDescribe for TextTrackKind {
    fn describe() {
        wasm_bindgen::JsValue::describe()
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::IntoWasmAbi for TextTrackKind {
    type Abi = <wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi;
    #[inline]
    fn into_abi(self) -> Self::Abi {
        wasm_bindgen::JsValue::from(self).into_abi()
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::FromWasmAbi for TextTrackKind {
    type Abi = <wasm_bindgen::JsValue as wasm_bindgen::convert::FromWasmAbi>::Abi;
    unsafe fn from_abi(js: Self::Abi) -> Self {
        TextTrackKind::from_js_value(&wasm_bindgen::JsValue::from_abi(js))
            .unwrap_or(TextTrackKind::__Nonexhaustive)
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::OptionIntoWasmAbi for TextTrackKind {
    #[inline]
    fn none() -> Self::Abi {
        Object::none()
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::OptionFromWasmAbi for TextTrackKind {
    #[inline]
    fn is_none(abi: &Self::Abi) -> bool {
        Object::is_none(abi)
    }
}
#[allow(clippy::all)]
impl From<TextTrackKind> for wasm_bindgen::JsValue {
    fn from(obj: TextTrackKind) -> wasm_bindgen::JsValue {
        match obj {
            TextTrackKind::Subtitles => wasm_bindgen::JsValue::from_str("subtitles"),
            TextTrackKind::Captions => wasm_bindgen::JsValue::from_str("captions"),
            TextTrackKind::Descriptions => wasm_bindgen::JsValue::from_str("descriptions"),
            TextTrackKind::Chapters => wasm_bindgen::JsValue::from_str("chapters"),
            TextTrackKind::Metadata => wasm_bindgen::JsValue::from_str("metadata"),
            TextTrackKind::__Nonexhaustive => {
                panic!("attempted to convert invalid TextTrackKind into JSValue")
            }
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_45c94992fd32df46: [u8; 108usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}*\0\0\0\0\0\x01\0\0\x03\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
