use super::*;
#[allow(bad_style)]
#[derive(Copy, Clone, PartialEq, Debug)]
#[allow(clippy::all)]
pub enum TcpSocketBinaryType {
    Arraybuffer = 0,
    String = 1,
    #[doc(hidden)]
    __Nonexhaustive,
}
#[allow(clippy::all)]
impl TcpSocketBinaryType {
    pub fn from_js_value(obj: &wasm_bindgen::JsValue) -> Option<TcpSocketBinaryType> {
        obj.as_string().and_then(|obj_str| match obj_str.as_str() {
            "arraybuffer" => Some(TcpSocketBinaryType::Arraybuffer),
            "string" => Some(TcpSocketBinaryType::String),
            _ => None,
        })
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::describe::WasmDescribe for TcpSocketBinaryType {
    fn describe() {
        wasm_bindgen::JsValue::describe()
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::IntoWasmAbi for TcpSocketBinaryType {
    type Abi = <wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi;
    #[inline]
    fn into_abi(self) -> Self::Abi {
        wasm_bindgen::JsValue::from(self).into_abi()
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::FromWasmAbi for TcpSocketBinaryType {
    type Abi = <wasm_bindgen::JsValue as wasm_bindgen::convert::FromWasmAbi>::Abi;
    unsafe fn from_abi(js: Self::Abi) -> Self {
        TcpSocketBinaryType::from_js_value(&wasm_bindgen::JsValue::from_abi(js))
            .unwrap_or(TcpSocketBinaryType::__Nonexhaustive)
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::OptionIntoWasmAbi for TcpSocketBinaryType {
    #[inline]
    fn none() -> Self::Abi {
        Object::none()
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::OptionFromWasmAbi for TcpSocketBinaryType {
    #[inline]
    fn is_none(abi: &Self::Abi) -> bool {
        Object::is_none(abi)
    }
}
#[allow(clippy::all)]
impl From<TcpSocketBinaryType> for wasm_bindgen::JsValue {
    fn from(obj: TcpSocketBinaryType) -> wasm_bindgen::JsValue {
        match obj {
            TcpSocketBinaryType::Arraybuffer => wasm_bindgen::JsValue::from_str("arraybuffer"),
            TcpSocketBinaryType::String => wasm_bindgen::JsValue::from_str("string"),
            TcpSocketBinaryType::__Nonexhaustive => {
                panic!("attempted to convert invalid TcpSocketBinaryType into JSValue")
            }
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_f485f8a753a6e293: [u8; 108usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}*\0\0\0\0\0\x01\0\0\x03\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
