use super::*;
#[allow(bad_style)]
#[derive(Copy, Clone, PartialEq, Debug)]
#[allow(clippy::all)]
pub enum ScrollBehavior {
    Auto = 0,
    Instant = 1,
    Smooth = 2,
    #[doc(hidden)]
    __Nonexhaustive,
}
#[allow(clippy::all)]
impl ScrollBehavior {
    pub fn from_js_value(obj: &wasm_bindgen::JsValue) -> Option<ScrollBehavior> {
        obj.as_string().and_then(|obj_str| match obj_str.as_str() {
            "auto" => Some(ScrollBehavior::Auto),
            "instant" => Some(ScrollBehavior::Instant),
            "smooth" => Some(ScrollBehavior::Smooth),
            _ => None,
        })
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::describe::WasmDescribe for ScrollBehavior {
    fn describe() {
        wasm_bindgen::JsValue::describe()
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::IntoWasmAbi for ScrollBehavior {
    type Abi = <wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi;
    #[inline]
    fn into_abi(self) -> Self::Abi {
        wasm_bindgen::JsValue::from(self).into_abi()
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::FromWasmAbi for ScrollBehavior {
    type Abi = <wasm_bindgen::JsValue as wasm_bindgen::convert::FromWasmAbi>::Abi;
    unsafe fn from_abi(js: Self::Abi) -> Self {
        ScrollBehavior::from_js_value(&wasm_bindgen::JsValue::from_abi(js))
            .unwrap_or(ScrollBehavior::__Nonexhaustive)
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::OptionIntoWasmAbi for ScrollBehavior {
    #[inline]
    fn none() -> Self::Abi {
        Object::none()
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::OptionFromWasmAbi for ScrollBehavior {
    #[inline]
    fn is_none(abi: &Self::Abi) -> bool {
        Object::is_none(abi)
    }
}
#[allow(clippy::all)]
impl From<ScrollBehavior> for wasm_bindgen::JsValue {
    fn from(obj: ScrollBehavior) -> wasm_bindgen::JsValue {
        match obj {
            ScrollBehavior::Auto => wasm_bindgen::JsValue::from_str("auto"),
            ScrollBehavior::Instant => wasm_bindgen::JsValue::from_str("instant"),
            ScrollBehavior::Smooth => wasm_bindgen::JsValue::from_str("smooth"),
            ScrollBehavior::__Nonexhaustive => {
                panic!("attempted to convert invalid ScrollBehavior into JSValue")
            }
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_570e82d0b3ca7f8d: [u8; 108usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}*\0\0\0\0\0\x01\0\0\x03\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
