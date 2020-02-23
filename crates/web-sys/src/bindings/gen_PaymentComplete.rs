use super::*;
use js_sys::Object;
#[allow(bad_style)]
#[derive(Copy, Clone, PartialEq, Debug)]
#[allow(clippy::all)]
pub enum PaymentComplete {
    Success = 0,
    Fail = 1,
    Unknown = 2,
    #[doc(hidden)]
    __Nonexhaustive,
}
#[allow(clippy::all)]
impl PaymentComplete {
    pub fn from_js_value(obj: &wasm_bindgen::JsValue) -> Option<PaymentComplete> {
        obj.as_string().and_then(|obj_str| match obj_str.as_str() {
            "success" => Some(PaymentComplete::Success),
            "fail" => Some(PaymentComplete::Fail),
            "unknown" => Some(PaymentComplete::Unknown),
            _ => None,
        })
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::describe::WasmDescribe for PaymentComplete {
    fn describe() {
        wasm_bindgen::JsValue::describe()
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::IntoWasmAbi for PaymentComplete {
    type Abi = <wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi;
    #[inline]
    fn into_abi(self) -> Self::Abi {
        wasm_bindgen::JsValue::from(self).into_abi()
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::FromWasmAbi for PaymentComplete {
    type Abi = <wasm_bindgen::JsValue as wasm_bindgen::convert::FromWasmAbi>::Abi;
    unsafe fn from_abi(js: Self::Abi) -> Self {
        PaymentComplete::from_js_value(&wasm_bindgen::JsValue::from_abi(js))
            .unwrap_or(PaymentComplete::__Nonexhaustive)
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::OptionIntoWasmAbi for PaymentComplete {
    #[inline]
    fn none() -> Self::Abi {
        Object::none()
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::OptionFromWasmAbi for PaymentComplete {
    #[inline]
    fn is_none(abi: &Self::Abi) -> bool {
        Object::is_none(abi)
    }
}
#[allow(clippy::all)]
impl From<PaymentComplete> for wasm_bindgen::JsValue {
    fn from(obj: PaymentComplete) -> wasm_bindgen::JsValue {
        match obj {
            PaymentComplete::Success => wasm_bindgen::JsValue::from_str("success"),
            PaymentComplete::Fail => wasm_bindgen::JsValue::from_str("fail"),
            PaymentComplete::Unknown => wasm_bindgen::JsValue::from_str("unknown"),
            PaymentComplete::__Nonexhaustive => {
                panic!("attempted to convert invalid PaymentComplete into JSValue")
            }
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_58a11e65008ac205: [u8; 108usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}*\0\0\0\0\0\x01\0\0\x03\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
