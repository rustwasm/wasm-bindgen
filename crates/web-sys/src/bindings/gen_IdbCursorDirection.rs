use super::*;
use js_sys::Object;
#[allow(bad_style)]
#[derive(Copy, Clone, PartialEq, Debug)]
#[allow(clippy::all)]
pub enum IdbCursorDirection {
    Next = 0,
    Nextunique = 1,
    Prev = 2,
    Prevunique = 3,
    #[doc(hidden)]
    __Nonexhaustive,
}
#[allow(clippy::all)]
impl IdbCursorDirection {
    pub fn from_js_value(obj: &wasm_bindgen::JsValue) -> Option<IdbCursorDirection> {
        obj.as_string().and_then(|obj_str| match obj_str.as_str() {
            "next" => Some(IdbCursorDirection::Next),
            "nextunique" => Some(IdbCursorDirection::Nextunique),
            "prev" => Some(IdbCursorDirection::Prev),
            "prevunique" => Some(IdbCursorDirection::Prevunique),
            _ => None,
        })
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::describe::WasmDescribe for IdbCursorDirection {
    fn describe() {
        wasm_bindgen::JsValue::describe()
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::IntoWasmAbi for IdbCursorDirection {
    type Abi = <wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi;
    #[inline]
    fn into_abi(self) -> Self::Abi {
        wasm_bindgen::JsValue::from(self).into_abi()
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::FromWasmAbi for IdbCursorDirection {
    type Abi = <wasm_bindgen::JsValue as wasm_bindgen::convert::FromWasmAbi>::Abi;
    unsafe fn from_abi(js: Self::Abi) -> Self {
        IdbCursorDirection::from_js_value(&wasm_bindgen::JsValue::from_abi(js))
            .unwrap_or(IdbCursorDirection::__Nonexhaustive)
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::OptionIntoWasmAbi for IdbCursorDirection {
    #[inline]
    fn none() -> Self::Abi {
        Object::none()
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::OptionFromWasmAbi for IdbCursorDirection {
    #[inline]
    fn is_none(abi: &Self::Abi) -> bool {
        Object::is_none(abi)
    }
}
#[allow(clippy::all)]
impl From<IdbCursorDirection> for wasm_bindgen::JsValue {
    fn from(obj: IdbCursorDirection) -> wasm_bindgen::JsValue {
        match obj {
            IdbCursorDirection::Next => wasm_bindgen::JsValue::from_str("next"),
            IdbCursorDirection::Nextunique => wasm_bindgen::JsValue::from_str("nextunique"),
            IdbCursorDirection::Prev => wasm_bindgen::JsValue::from_str("prev"),
            IdbCursorDirection::Prevunique => wasm_bindgen::JsValue::from_str("prevunique"),
            IdbCursorDirection::__Nonexhaustive => {
                panic!("attempted to convert invalid IdbCursorDirection into JSValue")
            }
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_b610994b6dc761e8: [u8; 108usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}*\0\0\0\0\0\x01\0\0\x03\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
