use super::*;
#[allow(bad_style)]
#[derive(Copy, Clone, PartialEq, Debug)]
#[allow(clippy::all)]
pub enum RtcDegradationPreference {
    MaintainFramerate = 0,
    MaintainResolution = 1,
    Balanced = 2,
    #[doc(hidden)]
    __Nonexhaustive,
}
#[allow(clippy::all)]
impl RtcDegradationPreference {
    pub fn from_js_value(obj: &wasm_bindgen::JsValue) -> Option<RtcDegradationPreference> {
        obj.as_string().and_then(|obj_str| match obj_str.as_str() {
            "maintain-framerate" => Some(RtcDegradationPreference::MaintainFramerate),
            "maintain-resolution" => Some(RtcDegradationPreference::MaintainResolution),
            "balanced" => Some(RtcDegradationPreference::Balanced),
            _ => None,
        })
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::describe::WasmDescribe for RtcDegradationPreference {
    fn describe() {
        wasm_bindgen::JsValue::describe()
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::IntoWasmAbi for RtcDegradationPreference {
    type Abi = <wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi;
    #[inline]
    fn into_abi(self) -> Self::Abi {
        wasm_bindgen::JsValue::from(self).into_abi()
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::FromWasmAbi for RtcDegradationPreference {
    type Abi = <wasm_bindgen::JsValue as wasm_bindgen::convert::FromWasmAbi>::Abi;
    unsafe fn from_abi(js: Self::Abi) -> Self {
        RtcDegradationPreference::from_js_value(&wasm_bindgen::JsValue::from_abi(js))
            .unwrap_or(RtcDegradationPreference::__Nonexhaustive)
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::OptionIntoWasmAbi for RtcDegradationPreference {
    #[inline]
    fn none() -> Self::Abi {
        Object::none()
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::OptionFromWasmAbi for RtcDegradationPreference {
    #[inline]
    fn is_none(abi: &Self::Abi) -> bool {
        Object::is_none(abi)
    }
}
#[allow(clippy::all)]
impl From<RtcDegradationPreference> for wasm_bindgen::JsValue {
    fn from(obj: RtcDegradationPreference) -> wasm_bindgen::JsValue {
        match obj {
            RtcDegradationPreference::MaintainFramerate => {
                wasm_bindgen::JsValue::from_str("maintain-framerate")
            }
            RtcDegradationPreference::MaintainResolution => {
                wasm_bindgen::JsValue::from_str("maintain-resolution")
            }
            RtcDegradationPreference::Balanced => wasm_bindgen::JsValue::from_str("balanced"),
            RtcDegradationPreference::__Nonexhaustive => {
                panic!("attempted to convert invalid RtcDegradationPreference into JSValue")
            }
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_6bf1c392e787846b: [u8; 108usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}*\0\0\0\0\0\x01\0\0\x03\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
