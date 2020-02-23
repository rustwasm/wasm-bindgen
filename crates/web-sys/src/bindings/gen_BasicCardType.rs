use super::*;
use js_sys::Object;
#[allow(bad_style)]
#[derive(Copy, Clone, PartialEq, Debug)]
#[allow(clippy::all)]
pub enum BasicCardType {
    Credit = 0,
    Debit = 1,
    Prepaid = 2,
    #[doc(hidden)]
    __Nonexhaustive,
}
#[allow(clippy::all)]
impl BasicCardType {
    pub fn from_js_value(obj: &wasm_bindgen::JsValue) -> Option<BasicCardType> {
        obj.as_string().and_then(|obj_str| match obj_str.as_str() {
            "credit" => Some(BasicCardType::Credit),
            "debit" => Some(BasicCardType::Debit),
            "prepaid" => Some(BasicCardType::Prepaid),
            _ => None,
        })
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::describe::WasmDescribe for BasicCardType {
    fn describe() {
        wasm_bindgen::JsValue::describe()
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::IntoWasmAbi for BasicCardType {
    type Abi = <wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi;
    #[inline]
    fn into_abi(self) -> Self::Abi {
        wasm_bindgen::JsValue::from(self).into_abi()
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::FromWasmAbi for BasicCardType {
    type Abi = <wasm_bindgen::JsValue as wasm_bindgen::convert::FromWasmAbi>::Abi;
    unsafe fn from_abi(js: Self::Abi) -> Self {
        BasicCardType::from_js_value(&wasm_bindgen::JsValue::from_abi(js))
            .unwrap_or(BasicCardType::__Nonexhaustive)
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::OptionIntoWasmAbi for BasicCardType {
    #[inline]
    fn none() -> Self::Abi {
        Object::none()
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::OptionFromWasmAbi for BasicCardType {
    #[inline]
    fn is_none(abi: &Self::Abi) -> bool {
        Object::is_none(abi)
    }
}
#[allow(clippy::all)]
impl From<BasicCardType> for wasm_bindgen::JsValue {
    fn from(obj: BasicCardType) -> wasm_bindgen::JsValue {
        match obj {
            BasicCardType::Credit => wasm_bindgen::JsValue::from_str("credit"),
            BasicCardType::Debit => wasm_bindgen::JsValue::from_str("debit"),
            BasicCardType::Prepaid => wasm_bindgen::JsValue::from_str("prepaid"),
            BasicCardType::__Nonexhaustive => {
                panic!("attempted to convert invalid BasicCardType into JSValue")
            }
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_7f222e026de649ae: [u8; 108usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}*\0\0\0\0\0\x01\0\0\x03\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
