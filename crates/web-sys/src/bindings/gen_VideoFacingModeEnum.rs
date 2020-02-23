use super::*;
use js_sys::Object;
#[allow(bad_style)]
#[derive(Copy, Clone, PartialEq, Debug)]
#[allow(clippy::all)]
pub enum VideoFacingModeEnum {
    User = 0,
    Environment = 1,
    Left = 2,
    Right = 3,
    #[doc(hidden)]
    __Nonexhaustive,
}
#[allow(clippy::all)]
impl VideoFacingModeEnum {
    pub fn from_js_value(obj: &wasm_bindgen::JsValue) -> Option<VideoFacingModeEnum> {
        obj.as_string().and_then(|obj_str| match obj_str.as_str() {
            "user" => Some(VideoFacingModeEnum::User),
            "environment" => Some(VideoFacingModeEnum::Environment),
            "left" => Some(VideoFacingModeEnum::Left),
            "right" => Some(VideoFacingModeEnum::Right),
            _ => None,
        })
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::describe::WasmDescribe for VideoFacingModeEnum {
    fn describe() {
        wasm_bindgen::JsValue::describe()
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::IntoWasmAbi for VideoFacingModeEnum {
    type Abi = <wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi;
    #[inline]
    fn into_abi(self) -> Self::Abi {
        wasm_bindgen::JsValue::from(self).into_abi()
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::FromWasmAbi for VideoFacingModeEnum {
    type Abi = <wasm_bindgen::JsValue as wasm_bindgen::convert::FromWasmAbi>::Abi;
    unsafe fn from_abi(js: Self::Abi) -> Self {
        VideoFacingModeEnum::from_js_value(&wasm_bindgen::JsValue::from_abi(js))
            .unwrap_or(VideoFacingModeEnum::__Nonexhaustive)
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::OptionIntoWasmAbi for VideoFacingModeEnum {
    #[inline]
    fn none() -> Self::Abi {
        Object::none()
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::OptionFromWasmAbi for VideoFacingModeEnum {
    #[inline]
    fn is_none(abi: &Self::Abi) -> bool {
        Object::is_none(abi)
    }
}
#[allow(clippy::all)]
impl From<VideoFacingModeEnum> for wasm_bindgen::JsValue {
    fn from(obj: VideoFacingModeEnum) -> wasm_bindgen::JsValue {
        match obj {
            VideoFacingModeEnum::User => wasm_bindgen::JsValue::from_str("user"),
            VideoFacingModeEnum::Environment => wasm_bindgen::JsValue::from_str("environment"),
            VideoFacingModeEnum::Left => wasm_bindgen::JsValue::from_str("left"),
            VideoFacingModeEnum::Right => wasm_bindgen::JsValue::from_str("right"),
            VideoFacingModeEnum::__Nonexhaustive => {
                panic!("attempted to convert invalid VideoFacingModeEnum into JSValue")
            }
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_2a73c76add45e65c: [u8; 108usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}*\0\0\0\0\0\x01\0\0\x03\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
