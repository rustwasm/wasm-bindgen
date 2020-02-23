use super::*;
use js_sys::Object;
#[allow(bad_style)]
#[derive(Copy, Clone, PartialEq, Debug)]
#[allow(clippy::all)]
pub enum CanvasWindingRule {
    Nonzero = 0,
    Evenodd = 1,
    #[doc(hidden)]
    __Nonexhaustive,
}
#[allow(clippy::all)]
impl CanvasWindingRule {
    pub fn from_js_value(obj: &wasm_bindgen::JsValue) -> Option<CanvasWindingRule> {
        obj.as_string().and_then(|obj_str| match obj_str.as_str() {
            "nonzero" => Some(CanvasWindingRule::Nonzero),
            "evenodd" => Some(CanvasWindingRule::Evenodd),
            _ => None,
        })
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::describe::WasmDescribe for CanvasWindingRule {
    fn describe() {
        wasm_bindgen::JsValue::describe()
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::IntoWasmAbi for CanvasWindingRule {
    type Abi = <wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi;
    #[inline]
    fn into_abi(self) -> Self::Abi {
        wasm_bindgen::JsValue::from(self).into_abi()
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::FromWasmAbi for CanvasWindingRule {
    type Abi = <wasm_bindgen::JsValue as wasm_bindgen::convert::FromWasmAbi>::Abi;
    unsafe fn from_abi(js: Self::Abi) -> Self {
        CanvasWindingRule::from_js_value(&wasm_bindgen::JsValue::from_abi(js))
            .unwrap_or(CanvasWindingRule::__Nonexhaustive)
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::OptionIntoWasmAbi for CanvasWindingRule {
    #[inline]
    fn none() -> Self::Abi {
        Object::none()
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::OptionFromWasmAbi for CanvasWindingRule {
    #[inline]
    fn is_none(abi: &Self::Abi) -> bool {
        Object::is_none(abi)
    }
}
#[allow(clippy::all)]
impl From<CanvasWindingRule> for wasm_bindgen::JsValue {
    fn from(obj: CanvasWindingRule) -> wasm_bindgen::JsValue {
        match obj {
            CanvasWindingRule::Nonzero => wasm_bindgen::JsValue::from_str("nonzero"),
            CanvasWindingRule::Evenodd => wasm_bindgen::JsValue::from_str("evenodd"),
            CanvasWindingRule::__Nonexhaustive => {
                panic!("attempted to convert invalid CanvasWindingRule into JSValue")
            }
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_2958c1412b11784c: [u8; 108usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}*\0\0\0\0\0\x01\0\0\x03\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
