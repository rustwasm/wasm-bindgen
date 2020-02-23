use super::*;
use js_sys::Object;
#[allow(bad_style)]
#[derive(Copy, Clone, PartialEq, Debug)]
#[allow(clippy::all)]
pub enum FrameType {
    Auxiliary = 0,
    TopLevel = 1,
    Nested = 2,
    None = 3,
    #[doc(hidden)]
    __Nonexhaustive,
}
#[allow(clippy::all)]
impl FrameType {
    pub fn from_js_value(obj: &wasm_bindgen::JsValue) -> Option<FrameType> {
        obj.as_string().and_then(|obj_str| match obj_str.as_str() {
            "auxiliary" => Some(FrameType::Auxiliary),
            "top-level" => Some(FrameType::TopLevel),
            "nested" => Some(FrameType::Nested),
            "none" => Some(FrameType::None),
            _ => None,
        })
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::describe::WasmDescribe for FrameType {
    fn describe() {
        wasm_bindgen::JsValue::describe()
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::IntoWasmAbi for FrameType {
    type Abi = <wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi;
    #[inline]
    fn into_abi(self) -> Self::Abi {
        wasm_bindgen::JsValue::from(self).into_abi()
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::FromWasmAbi for FrameType {
    type Abi = <wasm_bindgen::JsValue as wasm_bindgen::convert::FromWasmAbi>::Abi;
    unsafe fn from_abi(js: Self::Abi) -> Self {
        FrameType::from_js_value(&wasm_bindgen::JsValue::from_abi(js))
            .unwrap_or(FrameType::__Nonexhaustive)
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::OptionIntoWasmAbi for FrameType {
    #[inline]
    fn none() -> Self::Abi {
        Object::none()
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::OptionFromWasmAbi for FrameType {
    #[inline]
    fn is_none(abi: &Self::Abi) -> bool {
        Object::is_none(abi)
    }
}
#[allow(clippy::all)]
impl From<FrameType> for wasm_bindgen::JsValue {
    fn from(obj: FrameType) -> wasm_bindgen::JsValue {
        match obj {
            FrameType::Auxiliary => wasm_bindgen::JsValue::from_str("auxiliary"),
            FrameType::TopLevel => wasm_bindgen::JsValue::from_str("top-level"),
            FrameType::Nested => wasm_bindgen::JsValue::from_str("nested"),
            FrameType::None => wasm_bindgen::JsValue::from_str("none"),
            FrameType::__Nonexhaustive => {
                panic!("attempted to convert invalid FrameType into JSValue")
            }
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_9d36ae4668a823ca: [u8; 108usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}*\0\0\0\0\0\x01\0\0\x03\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
