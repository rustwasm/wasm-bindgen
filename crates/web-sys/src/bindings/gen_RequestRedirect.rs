use super::*;
use js_sys::Object;
#[allow(bad_style)]
#[derive(Copy, Clone, PartialEq, Debug)]
#[allow(clippy::all)]
pub enum RequestRedirect {
    Follow = 0,
    Error = 1,
    Manual = 2,
    #[doc(hidden)]
    __Nonexhaustive,
}
#[allow(clippy::all)]
impl RequestRedirect {
    pub fn from_js_value(obj: &wasm_bindgen::JsValue) -> Option<RequestRedirect> {
        obj.as_string().and_then(|obj_str| match obj_str.as_str() {
            "follow" => Some(RequestRedirect::Follow),
            "error" => Some(RequestRedirect::Error),
            "manual" => Some(RequestRedirect::Manual),
            _ => None,
        })
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::describe::WasmDescribe for RequestRedirect {
    fn describe() {
        wasm_bindgen::JsValue::describe()
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::IntoWasmAbi for RequestRedirect {
    type Abi = <wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi;
    #[inline]
    fn into_abi(self) -> Self::Abi {
        wasm_bindgen::JsValue::from(self).into_abi()
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::FromWasmAbi for RequestRedirect {
    type Abi = <wasm_bindgen::JsValue as wasm_bindgen::convert::FromWasmAbi>::Abi;
    unsafe fn from_abi(js: Self::Abi) -> Self {
        RequestRedirect::from_js_value(&wasm_bindgen::JsValue::from_abi(js))
            .unwrap_or(RequestRedirect::__Nonexhaustive)
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::OptionIntoWasmAbi for RequestRedirect {
    #[inline]
    fn none() -> Self::Abi {
        Object::none()
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::OptionFromWasmAbi for RequestRedirect {
    #[inline]
    fn is_none(abi: &Self::Abi) -> bool {
        Object::is_none(abi)
    }
}
#[allow(clippy::all)]
impl From<RequestRedirect> for wasm_bindgen::JsValue {
    fn from(obj: RequestRedirect) -> wasm_bindgen::JsValue {
        match obj {
            RequestRedirect::Follow => wasm_bindgen::JsValue::from_str("follow"),
            RequestRedirect::Error => wasm_bindgen::JsValue::from_str("error"),
            RequestRedirect::Manual => wasm_bindgen::JsValue::from_str("manual"),
            RequestRedirect::__Nonexhaustive => {
                panic!("attempted to convert invalid RequestRedirect into JSValue")
            }
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_e9c60ebd6bad2fd6: [u8; 108usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}*\0\0\0\0\0\x01\0\0\x03\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
