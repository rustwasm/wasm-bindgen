use super::*;
use js_sys::Object;
#[allow(bad_style)]
#[derive(Copy, Clone, PartialEq, Debug)]
#[allow(clippy::all)]
pub enum OscillatorType {
    Sine = 0,
    Square = 1,
    Sawtooth = 2,
    Triangle = 3,
    Custom = 4,
    #[doc(hidden)]
    __Nonexhaustive,
}
#[allow(clippy::all)]
impl OscillatorType {
    pub fn from_js_value(obj: &wasm_bindgen::JsValue) -> Option<OscillatorType> {
        obj.as_string().and_then(|obj_str| match obj_str.as_str() {
            "sine" => Some(OscillatorType::Sine),
            "square" => Some(OscillatorType::Square),
            "sawtooth" => Some(OscillatorType::Sawtooth),
            "triangle" => Some(OscillatorType::Triangle),
            "custom" => Some(OscillatorType::Custom),
            _ => None,
        })
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::describe::WasmDescribe for OscillatorType {
    fn describe() {
        wasm_bindgen::JsValue::describe()
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::IntoWasmAbi for OscillatorType {
    type Abi = <wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi;
    #[inline]
    fn into_abi(self) -> Self::Abi {
        wasm_bindgen::JsValue::from(self).into_abi()
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::FromWasmAbi for OscillatorType {
    type Abi = <wasm_bindgen::JsValue as wasm_bindgen::convert::FromWasmAbi>::Abi;
    unsafe fn from_abi(js: Self::Abi) -> Self {
        OscillatorType::from_js_value(&wasm_bindgen::JsValue::from_abi(js))
            .unwrap_or(OscillatorType::__Nonexhaustive)
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::OptionIntoWasmAbi for OscillatorType {
    #[inline]
    fn none() -> Self::Abi {
        Object::none()
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::OptionFromWasmAbi for OscillatorType {
    #[inline]
    fn is_none(abi: &Self::Abi) -> bool {
        Object::is_none(abi)
    }
}
#[allow(clippy::all)]
impl From<OscillatorType> for wasm_bindgen::JsValue {
    fn from(obj: OscillatorType) -> wasm_bindgen::JsValue {
        match obj {
            OscillatorType::Sine => wasm_bindgen::JsValue::from_str("sine"),
            OscillatorType::Square => wasm_bindgen::JsValue::from_str("square"),
            OscillatorType::Sawtooth => wasm_bindgen::JsValue::from_str("sawtooth"),
            OscillatorType::Triangle => wasm_bindgen::JsValue::from_str("triangle"),
            OscillatorType::Custom => wasm_bindgen::JsValue::from_str("custom"),
            OscillatorType::__Nonexhaustive => {
                panic!("attempted to convert invalid OscillatorType into JSValue")
            }
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_3fbd1a36d35346f9: [u8; 108usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}*\0\0\0\0\0\x01\0\0\x03\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
