use super::*;
#[allow(bad_style)]
#[derive(Copy, Clone, PartialEq, Debug)]
#[allow(clippy::all)]
pub enum SocketReadyState {
    Opening = 0,
    Open = 1,
    Closing = 2,
    Closed = 3,
    Halfclosed = 4,
    #[doc(hidden)]
    __Nonexhaustive,
}
#[allow(clippy::all)]
impl SocketReadyState {
    pub fn from_js_value(obj: &wasm_bindgen::JsValue) -> Option<SocketReadyState> {
        obj.as_string().and_then(|obj_str| match obj_str.as_str() {
            "opening" => Some(SocketReadyState::Opening),
            "open" => Some(SocketReadyState::Open),
            "closing" => Some(SocketReadyState::Closing),
            "closed" => Some(SocketReadyState::Closed),
            "halfclosed" => Some(SocketReadyState::Halfclosed),
            _ => None,
        })
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::describe::WasmDescribe for SocketReadyState {
    fn describe() {
        wasm_bindgen::JsValue::describe()
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::IntoWasmAbi for SocketReadyState {
    type Abi = <wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi;
    #[inline]
    fn into_abi(self) -> Self::Abi {
        wasm_bindgen::JsValue::from(self).into_abi()
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::FromWasmAbi for SocketReadyState {
    type Abi = <wasm_bindgen::JsValue as wasm_bindgen::convert::FromWasmAbi>::Abi;
    unsafe fn from_abi(js: Self::Abi) -> Self {
        SocketReadyState::from_js_value(&wasm_bindgen::JsValue::from_abi(js))
            .unwrap_or(SocketReadyState::__Nonexhaustive)
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::OptionIntoWasmAbi for SocketReadyState {
    #[inline]
    fn none() -> Self::Abi {
        Object::none()
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::OptionFromWasmAbi for SocketReadyState {
    #[inline]
    fn is_none(abi: &Self::Abi) -> bool {
        Object::is_none(abi)
    }
}
#[allow(clippy::all)]
impl From<SocketReadyState> for wasm_bindgen::JsValue {
    fn from(obj: SocketReadyState) -> wasm_bindgen::JsValue {
        match obj {
            SocketReadyState::Opening => wasm_bindgen::JsValue::from_str("opening"),
            SocketReadyState::Open => wasm_bindgen::JsValue::from_str("open"),
            SocketReadyState::Closing => wasm_bindgen::JsValue::from_str("closing"),
            SocketReadyState::Closed => wasm_bindgen::JsValue::from_str("closed"),
            SocketReadyState::Halfclosed => wasm_bindgen::JsValue::from_str("halfclosed"),
            SocketReadyState::__Nonexhaustive => {
                panic!("attempted to convert invalid SocketReadyState into JSValue")
            }
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_2f09b81284e2d0e1: [u8; 108usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}*\0\0\0\0\0\x01\0\0\x03\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
