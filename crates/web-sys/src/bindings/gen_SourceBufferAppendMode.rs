use super::*;
#[allow(bad_style)]
#[derive(Copy, Clone, PartialEq, Debug)]
#[allow(clippy::all)]
pub enum SourceBufferAppendMode {
    Segments = 0,
    Sequence = 1,
    #[doc(hidden)]
    __Nonexhaustive,
}
#[allow(clippy::all)]
impl SourceBufferAppendMode {
    pub fn from_js_value(obj: &wasm_bindgen::JsValue) -> Option<SourceBufferAppendMode> {
        obj.as_string().and_then(|obj_str| match obj_str.as_str() {
            "segments" => Some(SourceBufferAppendMode::Segments),
            "sequence" => Some(SourceBufferAppendMode::Sequence),
            _ => None,
        })
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::describe::WasmDescribe for SourceBufferAppendMode {
    fn describe() {
        wasm_bindgen::JsValue::describe()
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::IntoWasmAbi for SourceBufferAppendMode {
    type Abi = <wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi;
    #[inline]
    fn into_abi(self) -> Self::Abi {
        wasm_bindgen::JsValue::from(self).into_abi()
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::FromWasmAbi for SourceBufferAppendMode {
    type Abi = <wasm_bindgen::JsValue as wasm_bindgen::convert::FromWasmAbi>::Abi;
    unsafe fn from_abi(js: Self::Abi) -> Self {
        SourceBufferAppendMode::from_js_value(&wasm_bindgen::JsValue::from_abi(js))
            .unwrap_or(SourceBufferAppendMode::__Nonexhaustive)
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::OptionIntoWasmAbi for SourceBufferAppendMode {
    #[inline]
    fn none() -> Self::Abi {
        Object::none()
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::OptionFromWasmAbi for SourceBufferAppendMode {
    #[inline]
    fn is_none(abi: &Self::Abi) -> bool {
        Object::is_none(abi)
    }
}
#[allow(clippy::all)]
impl From<SourceBufferAppendMode> for wasm_bindgen::JsValue {
    fn from(obj: SourceBufferAppendMode) -> wasm_bindgen::JsValue {
        match obj {
            SourceBufferAppendMode::Segments => wasm_bindgen::JsValue::from_str("segments"),
            SourceBufferAppendMode::Sequence => wasm_bindgen::JsValue::from_str("sequence"),
            SourceBufferAppendMode::__Nonexhaustive => {
                panic!("attempted to convert invalid SourceBufferAppendMode into JSValue")
            }
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_c8a7b446e8cf66c9: [u8; 108usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}*\0\0\0\0\0\x01\0\0\x03\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
