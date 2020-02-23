use super::*;
#[allow(bad_style)]
#[derive(Copy, Clone, PartialEq, Debug)]
#[allow(clippy::all)]
pub enum ChannelPixelLayoutDataType {
    Uint8 = 0,
    Int8 = 1,
    Uint16 = 2,
    Int16 = 3,
    Uint32 = 4,
    Int32 = 5,
    Float32 = 6,
    Float64 = 7,
    #[doc(hidden)]
    __Nonexhaustive,
}
#[allow(clippy::all)]
impl ChannelPixelLayoutDataType {
    pub fn from_js_value(obj: &wasm_bindgen::JsValue) -> Option<ChannelPixelLayoutDataType> {
        obj.as_string().and_then(|obj_str| match obj_str.as_str() {
            "uint8" => Some(ChannelPixelLayoutDataType::Uint8),
            "int8" => Some(ChannelPixelLayoutDataType::Int8),
            "uint16" => Some(ChannelPixelLayoutDataType::Uint16),
            "int16" => Some(ChannelPixelLayoutDataType::Int16),
            "uint32" => Some(ChannelPixelLayoutDataType::Uint32),
            "int32" => Some(ChannelPixelLayoutDataType::Int32),
            "float32" => Some(ChannelPixelLayoutDataType::Float32),
            "float64" => Some(ChannelPixelLayoutDataType::Float64),
            _ => None,
        })
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::describe::WasmDescribe for ChannelPixelLayoutDataType {
    fn describe() {
        wasm_bindgen::JsValue::describe()
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::IntoWasmAbi for ChannelPixelLayoutDataType {
    type Abi = <wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi;
    #[inline]
    fn into_abi(self) -> Self::Abi {
        wasm_bindgen::JsValue::from(self).into_abi()
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::FromWasmAbi for ChannelPixelLayoutDataType {
    type Abi = <wasm_bindgen::JsValue as wasm_bindgen::convert::FromWasmAbi>::Abi;
    unsafe fn from_abi(js: Self::Abi) -> Self {
        ChannelPixelLayoutDataType::from_js_value(&wasm_bindgen::JsValue::from_abi(js))
            .unwrap_or(ChannelPixelLayoutDataType::__Nonexhaustive)
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::OptionIntoWasmAbi for ChannelPixelLayoutDataType {
    #[inline]
    fn none() -> Self::Abi {
        Object::none()
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::OptionFromWasmAbi for ChannelPixelLayoutDataType {
    #[inline]
    fn is_none(abi: &Self::Abi) -> bool {
        Object::is_none(abi)
    }
}
#[allow(clippy::all)]
impl From<ChannelPixelLayoutDataType> for wasm_bindgen::JsValue {
    fn from(obj: ChannelPixelLayoutDataType) -> wasm_bindgen::JsValue {
        match obj {
            ChannelPixelLayoutDataType::Uint8 => wasm_bindgen::JsValue::from_str("uint8"),
            ChannelPixelLayoutDataType::Int8 => wasm_bindgen::JsValue::from_str("int8"),
            ChannelPixelLayoutDataType::Uint16 => wasm_bindgen::JsValue::from_str("uint16"),
            ChannelPixelLayoutDataType::Int16 => wasm_bindgen::JsValue::from_str("int16"),
            ChannelPixelLayoutDataType::Uint32 => wasm_bindgen::JsValue::from_str("uint32"),
            ChannelPixelLayoutDataType::Int32 => wasm_bindgen::JsValue::from_str("int32"),
            ChannelPixelLayoutDataType::Float32 => wasm_bindgen::JsValue::from_str("float32"),
            ChannelPixelLayoutDataType::Float64 => wasm_bindgen::JsValue::from_str("float64"),
            ChannelPixelLayoutDataType::__Nonexhaustive => {
                panic!("attempted to convert invalid ChannelPixelLayoutDataType into JSValue")
            }
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_16618c36cd495a6c: [u8; 108usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}*\0\0\0\0\0\x01\0\0\x03\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
