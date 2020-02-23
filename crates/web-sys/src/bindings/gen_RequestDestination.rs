use super::*;
use js_sys::Object;
#[allow(bad_style)]
#[derive(Copy, Clone, PartialEq, Debug)]
#[allow(clippy::all)]
pub enum RequestDestination {
    None = 0,
    Audio = 1,
    Audioworklet = 2,
    Document = 3,
    Embed = 4,
    Font = 5,
    Image = 6,
    Manifest = 7,
    Object = 8,
    Paintworklet = 9,
    Report = 10,
    Script = 11,
    Sharedworker = 12,
    Style = 13,
    Track = 14,
    Video = 15,
    Worker = 16,
    Xslt = 17,
    #[doc(hidden)]
    __Nonexhaustive,
}
#[allow(clippy::all)]
impl RequestDestination {
    pub fn from_js_value(obj: &wasm_bindgen::JsValue) -> Option<RequestDestination> {
        obj.as_string().and_then(|obj_str| match obj_str.as_str() {
            "" => Some(RequestDestination::None),
            "audio" => Some(RequestDestination::Audio),
            "audioworklet" => Some(RequestDestination::Audioworklet),
            "document" => Some(RequestDestination::Document),
            "embed" => Some(RequestDestination::Embed),
            "font" => Some(RequestDestination::Font),
            "image" => Some(RequestDestination::Image),
            "manifest" => Some(RequestDestination::Manifest),
            "object" => Some(RequestDestination::Object),
            "paintworklet" => Some(RequestDestination::Paintworklet),
            "report" => Some(RequestDestination::Report),
            "script" => Some(RequestDestination::Script),
            "sharedworker" => Some(RequestDestination::Sharedworker),
            "style" => Some(RequestDestination::Style),
            "track" => Some(RequestDestination::Track),
            "video" => Some(RequestDestination::Video),
            "worker" => Some(RequestDestination::Worker),
            "xslt" => Some(RequestDestination::Xslt),
            _ => None,
        })
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::describe::WasmDescribe for RequestDestination {
    fn describe() {
        wasm_bindgen::JsValue::describe()
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::IntoWasmAbi for RequestDestination {
    type Abi = <wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi;
    #[inline]
    fn into_abi(self) -> Self::Abi {
        wasm_bindgen::JsValue::from(self).into_abi()
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::FromWasmAbi for RequestDestination {
    type Abi = <wasm_bindgen::JsValue as wasm_bindgen::convert::FromWasmAbi>::Abi;
    unsafe fn from_abi(js: Self::Abi) -> Self {
        RequestDestination::from_js_value(&wasm_bindgen::JsValue::from_abi(js))
            .unwrap_or(RequestDestination::__Nonexhaustive)
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::OptionIntoWasmAbi for RequestDestination {
    #[inline]
    fn none() -> Self::Abi {
        Object::none()
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::OptionFromWasmAbi for RequestDestination {
    #[inline]
    fn is_none(abi: &Self::Abi) -> bool {
        Object::is_none(abi)
    }
}
#[allow(clippy::all)]
impl From<RequestDestination> for wasm_bindgen::JsValue {
    fn from(obj: RequestDestination) -> wasm_bindgen::JsValue {
        match obj {
            RequestDestination::None => wasm_bindgen::JsValue::from_str(""),
            RequestDestination::Audio => wasm_bindgen::JsValue::from_str("audio"),
            RequestDestination::Audioworklet => wasm_bindgen::JsValue::from_str("audioworklet"),
            RequestDestination::Document => wasm_bindgen::JsValue::from_str("document"),
            RequestDestination::Embed => wasm_bindgen::JsValue::from_str("embed"),
            RequestDestination::Font => wasm_bindgen::JsValue::from_str("font"),
            RequestDestination::Image => wasm_bindgen::JsValue::from_str("image"),
            RequestDestination::Manifest => wasm_bindgen::JsValue::from_str("manifest"),
            RequestDestination::Object => wasm_bindgen::JsValue::from_str("object"),
            RequestDestination::Paintworklet => wasm_bindgen::JsValue::from_str("paintworklet"),
            RequestDestination::Report => wasm_bindgen::JsValue::from_str("report"),
            RequestDestination::Script => wasm_bindgen::JsValue::from_str("script"),
            RequestDestination::Sharedworker => wasm_bindgen::JsValue::from_str("sharedworker"),
            RequestDestination::Style => wasm_bindgen::JsValue::from_str("style"),
            RequestDestination::Track => wasm_bindgen::JsValue::from_str("track"),
            RequestDestination::Video => wasm_bindgen::JsValue::from_str("video"),
            RequestDestination::Worker => wasm_bindgen::JsValue::from_str("worker"),
            RequestDestination::Xslt => wasm_bindgen::JsValue::from_str("xslt"),
            RequestDestination::__Nonexhaustive => {
                panic!("attempted to convert invalid RequestDestination into JSValue")
            }
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_c270f0fa58d87414: [u8; 108usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}*\0\0\0\0\0\x01\0\0\x03\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
