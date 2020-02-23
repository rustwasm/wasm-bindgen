use super::*;
use js_sys::Object;
#[allow(bad_style)]
#[derive(Copy, Clone, PartialEq, Debug)]
#[allow(clippy::all)]
pub enum PresentationConnectionClosedReason {
    Error = 0,
    Closed = 1,
    Wentaway = 2,
    #[doc(hidden)]
    __Nonexhaustive,
}
#[allow(clippy::all)]
impl PresentationConnectionClosedReason {
    pub fn from_js_value(
        obj: &wasm_bindgen::JsValue,
    ) -> Option<PresentationConnectionClosedReason> {
        obj.as_string().and_then(|obj_str| match obj_str.as_str() {
            "error" => Some(PresentationConnectionClosedReason::Error),
            "closed" => Some(PresentationConnectionClosedReason::Closed),
            "wentaway" => Some(PresentationConnectionClosedReason::Wentaway),
            _ => None,
        })
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::describe::WasmDescribe for PresentationConnectionClosedReason {
    fn describe() {
        wasm_bindgen::JsValue::describe()
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::IntoWasmAbi for PresentationConnectionClosedReason {
    type Abi = <wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi;
    #[inline]
    fn into_abi(self) -> Self::Abi {
        wasm_bindgen::JsValue::from(self).into_abi()
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::FromWasmAbi for PresentationConnectionClosedReason {
    type Abi = <wasm_bindgen::JsValue as wasm_bindgen::convert::FromWasmAbi>::Abi;
    unsafe fn from_abi(js: Self::Abi) -> Self {
        PresentationConnectionClosedReason::from_js_value(&wasm_bindgen::JsValue::from_abi(js))
            .unwrap_or(PresentationConnectionClosedReason::__Nonexhaustive)
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::OptionIntoWasmAbi for PresentationConnectionClosedReason {
    #[inline]
    fn none() -> Self::Abi {
        Object::none()
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::OptionFromWasmAbi for PresentationConnectionClosedReason {
    #[inline]
    fn is_none(abi: &Self::Abi) -> bool {
        Object::is_none(abi)
    }
}
#[allow(clippy::all)]
impl From<PresentationConnectionClosedReason> for wasm_bindgen::JsValue {
    fn from(obj: PresentationConnectionClosedReason) -> wasm_bindgen::JsValue {
        match obj {
            PresentationConnectionClosedReason::Error => wasm_bindgen::JsValue::from_str("error"),
            PresentationConnectionClosedReason::Closed => wasm_bindgen::JsValue::from_str("closed"),
            PresentationConnectionClosedReason::Wentaway => {
                wasm_bindgen::JsValue::from_str("wentaway")
            }
            PresentationConnectionClosedReason::__Nonexhaustive => panic!(
                "attempted to convert invalid PresentationConnectionClosedReason into JSValue"
            ),
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_44f8c109982350a6: [u8; 108usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}*\0\0\0\0\0\x01\0\0\x03\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
