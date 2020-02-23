use super::*;
#[allow(bad_style)]
#[derive(Copy, Clone, PartialEq, Debug)]
#[allow(clippy::all)]
pub enum ResponseType {
    Basic = 0,
    Cors = 1,
    Default = 2,
    Error = 3,
    Opaque = 4,
    Opaqueredirect = 5,
    #[doc(hidden)]
    __Nonexhaustive,
}
#[allow(clippy::all)]
impl ResponseType {
    pub fn from_js_value(obj: &wasm_bindgen::JsValue) -> Option<ResponseType> {
        obj.as_string().and_then(|obj_str| match obj_str.as_str() {
            "basic" => Some(ResponseType::Basic),
            "cors" => Some(ResponseType::Cors),
            "default" => Some(ResponseType::Default),
            "error" => Some(ResponseType::Error),
            "opaque" => Some(ResponseType::Opaque),
            "opaqueredirect" => Some(ResponseType::Opaqueredirect),
            _ => None,
        })
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::describe::WasmDescribe for ResponseType {
    fn describe() {
        wasm_bindgen::JsValue::describe()
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::IntoWasmAbi for ResponseType {
    type Abi = <wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi;
    #[inline]
    fn into_abi(self) -> Self::Abi {
        wasm_bindgen::JsValue::from(self).into_abi()
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::FromWasmAbi for ResponseType {
    type Abi = <wasm_bindgen::JsValue as wasm_bindgen::convert::FromWasmAbi>::Abi;
    unsafe fn from_abi(js: Self::Abi) -> Self {
        ResponseType::from_js_value(&wasm_bindgen::JsValue::from_abi(js))
            .unwrap_or(ResponseType::__Nonexhaustive)
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::OptionIntoWasmAbi for ResponseType {
    #[inline]
    fn none() -> Self::Abi {
        Object::none()
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::OptionFromWasmAbi for ResponseType {
    #[inline]
    fn is_none(abi: &Self::Abi) -> bool {
        Object::is_none(abi)
    }
}
#[allow(clippy::all)]
impl From<ResponseType> for wasm_bindgen::JsValue {
    fn from(obj: ResponseType) -> wasm_bindgen::JsValue {
        match obj {
            ResponseType::Basic => wasm_bindgen::JsValue::from_str("basic"),
            ResponseType::Cors => wasm_bindgen::JsValue::from_str("cors"),
            ResponseType::Default => wasm_bindgen::JsValue::from_str("default"),
            ResponseType::Error => wasm_bindgen::JsValue::from_str("error"),
            ResponseType::Opaque => wasm_bindgen::JsValue::from_str("opaque"),
            ResponseType::Opaqueredirect => wasm_bindgen::JsValue::from_str("opaqueredirect"),
            ResponseType::__Nonexhaustive => {
                panic!("attempted to convert invalid ResponseType into JSValue")
            }
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_8b275a0adb0ceeca: [u8; 108usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}*\0\0\0\0\0\x01\0\0\x03\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
