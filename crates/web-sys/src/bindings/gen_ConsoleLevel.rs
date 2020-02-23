use super::*;
use js_sys::Object;
#[allow(bad_style)]
#[derive(Copy, Clone, PartialEq, Debug)]
#[allow(clippy::all)]
pub enum ConsoleLevel {
    Log = 0,
    Warning = 1,
    Error = 2,
    #[doc(hidden)]
    __Nonexhaustive,
}
#[allow(clippy::all)]
impl ConsoleLevel {
    pub fn from_js_value(obj: &wasm_bindgen::JsValue) -> Option<ConsoleLevel> {
        obj.as_string().and_then(|obj_str| match obj_str.as_str() {
            "log" => Some(ConsoleLevel::Log),
            "warning" => Some(ConsoleLevel::Warning),
            "error" => Some(ConsoleLevel::Error),
            _ => None,
        })
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::describe::WasmDescribe for ConsoleLevel {
    fn describe() {
        wasm_bindgen::JsValue::describe()
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::IntoWasmAbi for ConsoleLevel {
    type Abi = <wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi;
    #[inline]
    fn into_abi(self) -> Self::Abi {
        wasm_bindgen::JsValue::from(self).into_abi()
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::FromWasmAbi for ConsoleLevel {
    type Abi = <wasm_bindgen::JsValue as wasm_bindgen::convert::FromWasmAbi>::Abi;
    unsafe fn from_abi(js: Self::Abi) -> Self {
        ConsoleLevel::from_js_value(&wasm_bindgen::JsValue::from_abi(js))
            .unwrap_or(ConsoleLevel::__Nonexhaustive)
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::OptionIntoWasmAbi for ConsoleLevel {
    #[inline]
    fn none() -> Self::Abi {
        Object::none()
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::OptionFromWasmAbi for ConsoleLevel {
    #[inline]
    fn is_none(abi: &Self::Abi) -> bool {
        Object::is_none(abi)
    }
}
#[allow(clippy::all)]
impl From<ConsoleLevel> for wasm_bindgen::JsValue {
    fn from(obj: ConsoleLevel) -> wasm_bindgen::JsValue {
        match obj {
            ConsoleLevel::Log => wasm_bindgen::JsValue::from_str("log"),
            ConsoleLevel::Warning => wasm_bindgen::JsValue::from_str("warning"),
            ConsoleLevel::Error => wasm_bindgen::JsValue::from_str("error"),
            ConsoleLevel::__Nonexhaustive => {
                panic!("attempted to convert invalid ConsoleLevel into JSValue")
            }
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_7da6fcb8fa1cf80b: [u8; 108usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}*\0\0\0\0\0\x01\0\0\x03\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
