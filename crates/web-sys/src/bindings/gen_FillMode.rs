use super::*;
use js_sys::Object;
#[allow(bad_style)]
#[derive(Copy, Clone, PartialEq, Debug)]
#[allow(clippy::all)]
pub enum FillMode {
    None = 0,
    Forwards = 1,
    Backwards = 2,
    Both = 3,
    Auto = 4,
    #[doc(hidden)]
    __Nonexhaustive,
}
#[allow(clippy::all)]
impl FillMode {
    pub fn from_js_value(obj: &wasm_bindgen::JsValue) -> Option<FillMode> {
        obj.as_string().and_then(|obj_str| match obj_str.as_str() {
            "none" => Some(FillMode::None),
            "forwards" => Some(FillMode::Forwards),
            "backwards" => Some(FillMode::Backwards),
            "both" => Some(FillMode::Both),
            "auto" => Some(FillMode::Auto),
            _ => None,
        })
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::describe::WasmDescribe for FillMode {
    fn describe() {
        wasm_bindgen::JsValue::describe()
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::IntoWasmAbi for FillMode {
    type Abi = <wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi;
    #[inline]
    fn into_abi(self) -> Self::Abi {
        wasm_bindgen::JsValue::from(self).into_abi()
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::FromWasmAbi for FillMode {
    type Abi = <wasm_bindgen::JsValue as wasm_bindgen::convert::FromWasmAbi>::Abi;
    unsafe fn from_abi(js: Self::Abi) -> Self {
        FillMode::from_js_value(&wasm_bindgen::JsValue::from_abi(js))
            .unwrap_or(FillMode::__Nonexhaustive)
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::OptionIntoWasmAbi for FillMode {
    #[inline]
    fn none() -> Self::Abi {
        Object::none()
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::OptionFromWasmAbi for FillMode {
    #[inline]
    fn is_none(abi: &Self::Abi) -> bool {
        Object::is_none(abi)
    }
}
#[allow(clippy::all)]
impl From<FillMode> for wasm_bindgen::JsValue {
    fn from(obj: FillMode) -> wasm_bindgen::JsValue {
        match obj {
            FillMode::None => wasm_bindgen::JsValue::from_str("none"),
            FillMode::Forwards => wasm_bindgen::JsValue::from_str("forwards"),
            FillMode::Backwards => wasm_bindgen::JsValue::from_str("backwards"),
            FillMode::Both => wasm_bindgen::JsValue::from_str("both"),
            FillMode::Auto => wasm_bindgen::JsValue::from_str("auto"),
            FillMode::__Nonexhaustive => {
                panic!("attempted to convert invalid FillMode into JSValue")
            }
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_e14b244f30c45522: [u8; 108usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}*\0\0\0\0\0\x01\0\0\x03\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
