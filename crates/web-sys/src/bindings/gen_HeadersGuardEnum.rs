use super::*;
#[allow(bad_style)]
#[derive(Copy, Clone, PartialEq, Debug)]
#[allow(clippy::all)]
pub enum HeadersGuardEnum {
    None = 0,
    Request = 1,
    RequestNoCors = 2,
    Response = 3,
    Immutable = 4,
    #[doc(hidden)]
    __Nonexhaustive,
}
#[allow(clippy::all)]
impl HeadersGuardEnum {
    pub fn from_js_value(obj: &wasm_bindgen::JsValue) -> Option<HeadersGuardEnum> {
        obj.as_string().and_then(|obj_str| match obj_str.as_str() {
            "none" => Some(HeadersGuardEnum::None),
            "request" => Some(HeadersGuardEnum::Request),
            "request-no-cors" => Some(HeadersGuardEnum::RequestNoCors),
            "response" => Some(HeadersGuardEnum::Response),
            "immutable" => Some(HeadersGuardEnum::Immutable),
            _ => None,
        })
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::describe::WasmDescribe for HeadersGuardEnum {
    fn describe() {
        wasm_bindgen::JsValue::describe()
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::IntoWasmAbi for HeadersGuardEnum {
    type Abi = <wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi;
    #[inline]
    fn into_abi(self) -> Self::Abi {
        wasm_bindgen::JsValue::from(self).into_abi()
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::FromWasmAbi for HeadersGuardEnum {
    type Abi = <wasm_bindgen::JsValue as wasm_bindgen::convert::FromWasmAbi>::Abi;
    unsafe fn from_abi(js: Self::Abi) -> Self {
        HeadersGuardEnum::from_js_value(&wasm_bindgen::JsValue::from_abi(js))
            .unwrap_or(HeadersGuardEnum::__Nonexhaustive)
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::OptionIntoWasmAbi for HeadersGuardEnum {
    #[inline]
    fn none() -> Self::Abi {
        Object::none()
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::OptionFromWasmAbi for HeadersGuardEnum {
    #[inline]
    fn is_none(abi: &Self::Abi) -> bool {
        Object::is_none(abi)
    }
}
#[allow(clippy::all)]
impl From<HeadersGuardEnum> for wasm_bindgen::JsValue {
    fn from(obj: HeadersGuardEnum) -> wasm_bindgen::JsValue {
        match obj {
            HeadersGuardEnum::None => wasm_bindgen::JsValue::from_str("none"),
            HeadersGuardEnum::Request => wasm_bindgen::JsValue::from_str("request"),
            HeadersGuardEnum::RequestNoCors => wasm_bindgen::JsValue::from_str("request-no-cors"),
            HeadersGuardEnum::Response => wasm_bindgen::JsValue::from_str("response"),
            HeadersGuardEnum::Immutable => wasm_bindgen::JsValue::from_str("immutable"),
            HeadersGuardEnum::__Nonexhaustive => {
                panic!("attempted to convert invalid HeadersGuardEnum into JSValue")
            }
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_0bd15c5279b00721: [u8; 108usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}*\0\0\0\0\0\x01\0\0\x03\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
