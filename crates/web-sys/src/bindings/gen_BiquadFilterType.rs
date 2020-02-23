use super::*;
use js_sys::Object;
#[allow(bad_style)]
#[derive(Copy, Clone, PartialEq, Debug)]
#[allow(clippy::all)]
pub enum BiquadFilterType {
    Lowpass = 0,
    Highpass = 1,
    Bandpass = 2,
    Lowshelf = 3,
    Highshelf = 4,
    Peaking = 5,
    Notch = 6,
    Allpass = 7,
    #[doc(hidden)]
    __Nonexhaustive,
}
#[allow(clippy::all)]
impl BiquadFilterType {
    pub fn from_js_value(obj: &wasm_bindgen::JsValue) -> Option<BiquadFilterType> {
        obj.as_string().and_then(|obj_str| match obj_str.as_str() {
            "lowpass" => Some(BiquadFilterType::Lowpass),
            "highpass" => Some(BiquadFilterType::Highpass),
            "bandpass" => Some(BiquadFilterType::Bandpass),
            "lowshelf" => Some(BiquadFilterType::Lowshelf),
            "highshelf" => Some(BiquadFilterType::Highshelf),
            "peaking" => Some(BiquadFilterType::Peaking),
            "notch" => Some(BiquadFilterType::Notch),
            "allpass" => Some(BiquadFilterType::Allpass),
            _ => None,
        })
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::describe::WasmDescribe for BiquadFilterType {
    fn describe() {
        wasm_bindgen::JsValue::describe()
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::IntoWasmAbi for BiquadFilterType {
    type Abi = <wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi;
    #[inline]
    fn into_abi(self) -> Self::Abi {
        wasm_bindgen::JsValue::from(self).into_abi()
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::FromWasmAbi for BiquadFilterType {
    type Abi = <wasm_bindgen::JsValue as wasm_bindgen::convert::FromWasmAbi>::Abi;
    unsafe fn from_abi(js: Self::Abi) -> Self {
        BiquadFilterType::from_js_value(&wasm_bindgen::JsValue::from_abi(js))
            .unwrap_or(BiquadFilterType::__Nonexhaustive)
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::OptionIntoWasmAbi for BiquadFilterType {
    #[inline]
    fn none() -> Self::Abi {
        Object::none()
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::OptionFromWasmAbi for BiquadFilterType {
    #[inline]
    fn is_none(abi: &Self::Abi) -> bool {
        Object::is_none(abi)
    }
}
#[allow(clippy::all)]
impl From<BiquadFilterType> for wasm_bindgen::JsValue {
    fn from(obj: BiquadFilterType) -> wasm_bindgen::JsValue {
        match obj {
            BiquadFilterType::Lowpass => wasm_bindgen::JsValue::from_str("lowpass"),
            BiquadFilterType::Highpass => wasm_bindgen::JsValue::from_str("highpass"),
            BiquadFilterType::Bandpass => wasm_bindgen::JsValue::from_str("bandpass"),
            BiquadFilterType::Lowshelf => wasm_bindgen::JsValue::from_str("lowshelf"),
            BiquadFilterType::Highshelf => wasm_bindgen::JsValue::from_str("highshelf"),
            BiquadFilterType::Peaking => wasm_bindgen::JsValue::from_str("peaking"),
            BiquadFilterType::Notch => wasm_bindgen::JsValue::from_str("notch"),
            BiquadFilterType::Allpass => wasm_bindgen::JsValue::from_str("allpass"),
            BiquadFilterType::__Nonexhaustive => {
                panic!("attempted to convert invalid BiquadFilterType into JSValue")
            }
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_96ae945abc9e82a0: [u8; 108usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}*\0\0\0\0\0\x01\0\0\x03\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
