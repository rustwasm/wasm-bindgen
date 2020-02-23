use super::*;
use js_sys::Object;
#[allow(bad_style)]
#[derive(Copy, Clone, PartialEq, Debug)]
#[allow(clippy::all)]
pub enum IdbTransactionMode {
    Readonly = 0,
    Readwrite = 1,
    Readwriteflush = 2,
    Cleanup = 3,
    Versionchange = 4,
    #[doc(hidden)]
    __Nonexhaustive,
}
#[allow(clippy::all)]
impl IdbTransactionMode {
    pub fn from_js_value(obj: &wasm_bindgen::JsValue) -> Option<IdbTransactionMode> {
        obj.as_string().and_then(|obj_str| match obj_str.as_str() {
            "readonly" => Some(IdbTransactionMode::Readonly),
            "readwrite" => Some(IdbTransactionMode::Readwrite),
            "readwriteflush" => Some(IdbTransactionMode::Readwriteflush),
            "cleanup" => Some(IdbTransactionMode::Cleanup),
            "versionchange" => Some(IdbTransactionMode::Versionchange),
            _ => None,
        })
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::describe::WasmDescribe for IdbTransactionMode {
    fn describe() {
        wasm_bindgen::JsValue::describe()
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::IntoWasmAbi for IdbTransactionMode {
    type Abi = <wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi;
    #[inline]
    fn into_abi(self) -> Self::Abi {
        wasm_bindgen::JsValue::from(self).into_abi()
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::FromWasmAbi for IdbTransactionMode {
    type Abi = <wasm_bindgen::JsValue as wasm_bindgen::convert::FromWasmAbi>::Abi;
    unsafe fn from_abi(js: Self::Abi) -> Self {
        IdbTransactionMode::from_js_value(&wasm_bindgen::JsValue::from_abi(js))
            .unwrap_or(IdbTransactionMode::__Nonexhaustive)
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::OptionIntoWasmAbi for IdbTransactionMode {
    #[inline]
    fn none() -> Self::Abi {
        Object::none()
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::OptionFromWasmAbi for IdbTransactionMode {
    #[inline]
    fn is_none(abi: &Self::Abi) -> bool {
        Object::is_none(abi)
    }
}
#[allow(clippy::all)]
impl From<IdbTransactionMode> for wasm_bindgen::JsValue {
    fn from(obj: IdbTransactionMode) -> wasm_bindgen::JsValue {
        match obj {
            IdbTransactionMode::Readonly => wasm_bindgen::JsValue::from_str("readonly"),
            IdbTransactionMode::Readwrite => wasm_bindgen::JsValue::from_str("readwrite"),
            IdbTransactionMode::Readwriteflush => wasm_bindgen::JsValue::from_str("readwriteflush"),
            IdbTransactionMode::Cleanup => wasm_bindgen::JsValue::from_str("cleanup"),
            IdbTransactionMode::Versionchange => wasm_bindgen::JsValue::from_str("versionchange"),
            IdbTransactionMode::__Nonexhaustive => {
                panic!("attempted to convert invalid IdbTransactionMode into JSValue")
            }
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_dae2aa92535da264: [u8; 108usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}*\0\0\0\0\0\x01\0\0\x03\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
