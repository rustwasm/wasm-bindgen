use super::*;
#[allow(bad_style)]
#[derive(Copy, Clone, PartialEq, Debug)]
#[allow(clippy::all)]
pub enum ScrollLogicalPosition {
    Start = 0,
    Center = 1,
    End = 2,
    Nearest = 3,
    #[doc(hidden)]
    __Nonexhaustive,
}
#[allow(clippy::all)]
impl ScrollLogicalPosition {
    pub fn from_js_value(obj: &wasm_bindgen::JsValue) -> Option<ScrollLogicalPosition> {
        obj.as_string().and_then(|obj_str| match obj_str.as_str() {
            "start" => Some(ScrollLogicalPosition::Start),
            "center" => Some(ScrollLogicalPosition::Center),
            "end" => Some(ScrollLogicalPosition::End),
            "nearest" => Some(ScrollLogicalPosition::Nearest),
            _ => None,
        })
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::describe::WasmDescribe for ScrollLogicalPosition {
    fn describe() {
        wasm_bindgen::JsValue::describe()
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::IntoWasmAbi for ScrollLogicalPosition {
    type Abi = <wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi;
    #[inline]
    fn into_abi(self) -> Self::Abi {
        wasm_bindgen::JsValue::from(self).into_abi()
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::FromWasmAbi for ScrollLogicalPosition {
    type Abi = <wasm_bindgen::JsValue as wasm_bindgen::convert::FromWasmAbi>::Abi;
    unsafe fn from_abi(js: Self::Abi) -> Self {
        ScrollLogicalPosition::from_js_value(&wasm_bindgen::JsValue::from_abi(js))
            .unwrap_or(ScrollLogicalPosition::__Nonexhaustive)
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::OptionIntoWasmAbi for ScrollLogicalPosition {
    #[inline]
    fn none() -> Self::Abi {
        Object::none()
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::OptionFromWasmAbi for ScrollLogicalPosition {
    #[inline]
    fn is_none(abi: &Self::Abi) -> bool {
        Object::is_none(abi)
    }
}
#[allow(clippy::all)]
impl From<ScrollLogicalPosition> for wasm_bindgen::JsValue {
    fn from(obj: ScrollLogicalPosition) -> wasm_bindgen::JsValue {
        match obj {
            ScrollLogicalPosition::Start => wasm_bindgen::JsValue::from_str("start"),
            ScrollLogicalPosition::Center => wasm_bindgen::JsValue::from_str("center"),
            ScrollLogicalPosition::End => wasm_bindgen::JsValue::from_str("end"),
            ScrollLogicalPosition::Nearest => wasm_bindgen::JsValue::from_str("nearest"),
            ScrollLogicalPosition::__Nonexhaustive => {
                panic!("attempted to convert invalid ScrollLogicalPosition into JSValue")
            }
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_f48dc5159dea8b6c: [u8; 108usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}*\0\0\0\0\0\x01\0\0\x03\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
