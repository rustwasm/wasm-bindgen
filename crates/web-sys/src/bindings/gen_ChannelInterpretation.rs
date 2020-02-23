use super::*;
#[allow(bad_style)]
#[derive(Copy, Clone, PartialEq, Debug)]
#[allow(clippy::all)]
pub enum ChannelInterpretation {
    Speakers = 0,
    Discrete = 1,
    #[doc(hidden)]
    __Nonexhaustive,
}
#[allow(clippy::all)]
impl ChannelInterpretation {
    pub fn from_js_value(obj: &wasm_bindgen::JsValue) -> Option<ChannelInterpretation> {
        obj.as_string().and_then(|obj_str| match obj_str.as_str() {
            "speakers" => Some(ChannelInterpretation::Speakers),
            "discrete" => Some(ChannelInterpretation::Discrete),
            _ => None,
        })
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::describe::WasmDescribe for ChannelInterpretation {
    fn describe() {
        wasm_bindgen::JsValue::describe()
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::IntoWasmAbi for ChannelInterpretation {
    type Abi = <wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi;
    #[inline]
    fn into_abi(self) -> Self::Abi {
        wasm_bindgen::JsValue::from(self).into_abi()
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::FromWasmAbi for ChannelInterpretation {
    type Abi = <wasm_bindgen::JsValue as wasm_bindgen::convert::FromWasmAbi>::Abi;
    unsafe fn from_abi(js: Self::Abi) -> Self {
        ChannelInterpretation::from_js_value(&wasm_bindgen::JsValue::from_abi(js))
            .unwrap_or(ChannelInterpretation::__Nonexhaustive)
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::OptionIntoWasmAbi for ChannelInterpretation {
    #[inline]
    fn none() -> Self::Abi {
        Object::none()
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::OptionFromWasmAbi for ChannelInterpretation {
    #[inline]
    fn is_none(abi: &Self::Abi) -> bool {
        Object::is_none(abi)
    }
}
#[allow(clippy::all)]
impl From<ChannelInterpretation> for wasm_bindgen::JsValue {
    fn from(obj: ChannelInterpretation) -> wasm_bindgen::JsValue {
        match obj {
            ChannelInterpretation::Speakers => wasm_bindgen::JsValue::from_str("speakers"),
            ChannelInterpretation::Discrete => wasm_bindgen::JsValue::from_str("discrete"),
            ChannelInterpretation::__Nonexhaustive => {
                panic!("attempted to convert invalid ChannelInterpretation into JSValue")
            }
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_d10f3b777dbc9757: [u8; 108usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}*\0\0\0\0\0\x01\0\0\x03\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
