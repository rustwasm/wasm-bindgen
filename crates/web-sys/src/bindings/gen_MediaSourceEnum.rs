use super::*;
use js_sys::Object;
#[allow(bad_style)]
#[derive(Copy, Clone, PartialEq, Debug)]
#[allow(clippy::all)]
pub enum MediaSourceEnum {
    Camera = 0,
    Screen = 1,
    Application = 2,
    Window = 3,
    Browser = 4,
    Microphone = 5,
    AudioCapture = 6,
    Other = 7,
    #[doc(hidden)]
    __Nonexhaustive,
}
#[allow(clippy::all)]
impl MediaSourceEnum {
    pub fn from_js_value(obj: &wasm_bindgen::JsValue) -> Option<MediaSourceEnum> {
        obj.as_string().and_then(|obj_str| match obj_str.as_str() {
            "camera" => Some(MediaSourceEnum::Camera),
            "screen" => Some(MediaSourceEnum::Screen),
            "application" => Some(MediaSourceEnum::Application),
            "window" => Some(MediaSourceEnum::Window),
            "browser" => Some(MediaSourceEnum::Browser),
            "microphone" => Some(MediaSourceEnum::Microphone),
            "audioCapture" => Some(MediaSourceEnum::AudioCapture),
            "other" => Some(MediaSourceEnum::Other),
            _ => None,
        })
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::describe::WasmDescribe for MediaSourceEnum {
    fn describe() {
        wasm_bindgen::JsValue::describe()
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::IntoWasmAbi for MediaSourceEnum {
    type Abi = <wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi;
    #[inline]
    fn into_abi(self) -> Self::Abi {
        wasm_bindgen::JsValue::from(self).into_abi()
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::FromWasmAbi for MediaSourceEnum {
    type Abi = <wasm_bindgen::JsValue as wasm_bindgen::convert::FromWasmAbi>::Abi;
    unsafe fn from_abi(js: Self::Abi) -> Self {
        MediaSourceEnum::from_js_value(&wasm_bindgen::JsValue::from_abi(js))
            .unwrap_or(MediaSourceEnum::__Nonexhaustive)
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::OptionIntoWasmAbi for MediaSourceEnum {
    #[inline]
    fn none() -> Self::Abi {
        Object::none()
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::OptionFromWasmAbi for MediaSourceEnum {
    #[inline]
    fn is_none(abi: &Self::Abi) -> bool {
        Object::is_none(abi)
    }
}
#[allow(clippy::all)]
impl From<MediaSourceEnum> for wasm_bindgen::JsValue {
    fn from(obj: MediaSourceEnum) -> wasm_bindgen::JsValue {
        match obj {
            MediaSourceEnum::Camera => wasm_bindgen::JsValue::from_str("camera"),
            MediaSourceEnum::Screen => wasm_bindgen::JsValue::from_str("screen"),
            MediaSourceEnum::Application => wasm_bindgen::JsValue::from_str("application"),
            MediaSourceEnum::Window => wasm_bindgen::JsValue::from_str("window"),
            MediaSourceEnum::Browser => wasm_bindgen::JsValue::from_str("browser"),
            MediaSourceEnum::Microphone => wasm_bindgen::JsValue::from_str("microphone"),
            MediaSourceEnum::AudioCapture => wasm_bindgen::JsValue::from_str("audioCapture"),
            MediaSourceEnum::Other => wasm_bindgen::JsValue::from_str("other"),
            MediaSourceEnum::__Nonexhaustive => {
                panic!("attempted to convert invalid MediaSourceEnum into JSValue")
            }
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_71687aad3890468f: [u8; 108usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}*\0\0\0\0\0\x01\0\0\x03\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
