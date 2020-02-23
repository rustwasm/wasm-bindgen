use super::*;
use js_sys::Object;
#[allow(bad_style)]
#[derive(Copy, Clone, PartialEq, Debug)]
#[allow(clippy::all)]
pub enum EndingTypes {
    Transparent = 0,
    Native = 1,
    #[doc(hidden)]
    __Nonexhaustive,
}
#[allow(clippy::all)]
impl EndingTypes {
    pub fn from_js_value(obj: &wasm_bindgen::JsValue) -> Option<EndingTypes> {
        obj.as_string().and_then(|obj_str| match obj_str.as_str() {
            "transparent" => Some(EndingTypes::Transparent),
            "native" => Some(EndingTypes::Native),
            _ => None,
        })
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::describe::WasmDescribe for EndingTypes {
    fn describe() {
        wasm_bindgen::JsValue::describe()
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::IntoWasmAbi for EndingTypes {
    type Abi = <wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi;
    #[inline]
    fn into_abi(self) -> Self::Abi {
        wasm_bindgen::JsValue::from(self).into_abi()
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::FromWasmAbi for EndingTypes {
    type Abi = <wasm_bindgen::JsValue as wasm_bindgen::convert::FromWasmAbi>::Abi;
    unsafe fn from_abi(js: Self::Abi) -> Self {
        EndingTypes::from_js_value(&wasm_bindgen::JsValue::from_abi(js))
            .unwrap_or(EndingTypes::__Nonexhaustive)
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::OptionIntoWasmAbi for EndingTypes {
    #[inline]
    fn none() -> Self::Abi {
        Object::none()
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::OptionFromWasmAbi for EndingTypes {
    #[inline]
    fn is_none(abi: &Self::Abi) -> bool {
        Object::is_none(abi)
    }
}
#[allow(clippy::all)]
impl From<EndingTypes> for wasm_bindgen::JsValue {
    fn from(obj: EndingTypes) -> wasm_bindgen::JsValue {
        match obj {
            EndingTypes::Transparent => wasm_bindgen::JsValue::from_str("transparent"),
            EndingTypes::Native => wasm_bindgen::JsValue::from_str("native"),
            EndingTypes::__Nonexhaustive => {
                panic!("attempted to convert invalid EndingTypes into JSValue")
            }
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_c807ad9d75290bc6: [u8; 108usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}*\0\0\0\0\0\x01\0\0\x03\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
