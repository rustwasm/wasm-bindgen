use super::*;
#[allow(bad_style)]
#[derive(Copy, Clone, PartialEq, Debug)]
#[allow(clippy::all)]
pub enum WebGlPowerPreference {
    Default = 0,
    LowPower = 1,
    HighPerformance = 2,
    #[doc(hidden)]
    __Nonexhaustive,
}
#[allow(clippy::all)]
impl WebGlPowerPreference {
    pub fn from_js_value(obj: &wasm_bindgen::JsValue) -> Option<WebGlPowerPreference> {
        obj.as_string().and_then(|obj_str| match obj_str.as_str() {
            "default" => Some(WebGlPowerPreference::Default),
            "low-power" => Some(WebGlPowerPreference::LowPower),
            "high-performance" => Some(WebGlPowerPreference::HighPerformance),
            _ => None,
        })
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::describe::WasmDescribe for WebGlPowerPreference {
    fn describe() {
        wasm_bindgen::JsValue::describe()
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::IntoWasmAbi for WebGlPowerPreference {
    type Abi = <wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi;
    #[inline]
    fn into_abi(self) -> Self::Abi {
        wasm_bindgen::JsValue::from(self).into_abi()
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::FromWasmAbi for WebGlPowerPreference {
    type Abi = <wasm_bindgen::JsValue as wasm_bindgen::convert::FromWasmAbi>::Abi;
    unsafe fn from_abi(js: Self::Abi) -> Self {
        WebGlPowerPreference::from_js_value(&wasm_bindgen::JsValue::from_abi(js))
            .unwrap_or(WebGlPowerPreference::__Nonexhaustive)
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::OptionIntoWasmAbi for WebGlPowerPreference {
    #[inline]
    fn none() -> Self::Abi {
        Object::none()
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::OptionFromWasmAbi for WebGlPowerPreference {
    #[inline]
    fn is_none(abi: &Self::Abi) -> bool {
        Object::is_none(abi)
    }
}
#[allow(clippy::all)]
impl From<WebGlPowerPreference> for wasm_bindgen::JsValue {
    fn from(obj: WebGlPowerPreference) -> wasm_bindgen::JsValue {
        match obj {
            WebGlPowerPreference::Default => wasm_bindgen::JsValue::from_str("default"),
            WebGlPowerPreference::LowPower => wasm_bindgen::JsValue::from_str("low-power"),
            WebGlPowerPreference::HighPerformance => {
                wasm_bindgen::JsValue::from_str("high-performance")
            }
            WebGlPowerPreference::__Nonexhaustive => {
                panic!("attempted to convert invalid WebGlPowerPreference into JSValue")
            }
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_de841f93a34b872f: [u8; 108usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}*\0\0\0\0\0\x01\0\0\x03\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
