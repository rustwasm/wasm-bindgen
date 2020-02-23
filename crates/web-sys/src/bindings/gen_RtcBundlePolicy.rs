use super::*;
#[allow(bad_style)]
#[derive(Copy, Clone, PartialEq, Debug)]
#[allow(clippy::all)]
pub enum RtcBundlePolicy {
    Balanced = 0,
    MaxCompat = 1,
    MaxBundle = 2,
    #[doc(hidden)]
    __Nonexhaustive,
}
#[allow(clippy::all)]
impl RtcBundlePolicy {
    pub fn from_js_value(obj: &wasm_bindgen::JsValue) -> Option<RtcBundlePolicy> {
        obj.as_string().and_then(|obj_str| match obj_str.as_str() {
            "balanced" => Some(RtcBundlePolicy::Balanced),
            "max-compat" => Some(RtcBundlePolicy::MaxCompat),
            "max-bundle" => Some(RtcBundlePolicy::MaxBundle),
            _ => None,
        })
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::describe::WasmDescribe for RtcBundlePolicy {
    fn describe() {
        wasm_bindgen::JsValue::describe()
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::IntoWasmAbi for RtcBundlePolicy {
    type Abi = <wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi;
    #[inline]
    fn into_abi(self) -> Self::Abi {
        wasm_bindgen::JsValue::from(self).into_abi()
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::FromWasmAbi for RtcBundlePolicy {
    type Abi = <wasm_bindgen::JsValue as wasm_bindgen::convert::FromWasmAbi>::Abi;
    unsafe fn from_abi(js: Self::Abi) -> Self {
        RtcBundlePolicy::from_js_value(&wasm_bindgen::JsValue::from_abi(js))
            .unwrap_or(RtcBundlePolicy::__Nonexhaustive)
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::OptionIntoWasmAbi for RtcBundlePolicy {
    #[inline]
    fn none() -> Self::Abi {
        Object::none()
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::OptionFromWasmAbi for RtcBundlePolicy {
    #[inline]
    fn is_none(abi: &Self::Abi) -> bool {
        Object::is_none(abi)
    }
}
#[allow(clippy::all)]
impl From<RtcBundlePolicy> for wasm_bindgen::JsValue {
    fn from(obj: RtcBundlePolicy) -> wasm_bindgen::JsValue {
        match obj {
            RtcBundlePolicy::Balanced => wasm_bindgen::JsValue::from_str("balanced"),
            RtcBundlePolicy::MaxCompat => wasm_bindgen::JsValue::from_str("max-compat"),
            RtcBundlePolicy::MaxBundle => wasm_bindgen::JsValue::from_str("max-bundle"),
            RtcBundlePolicy::__Nonexhaustive => {
                panic!("attempted to convert invalid RtcBundlePolicy into JSValue")
            }
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_f6bf54a87ca34615: [u8; 108usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}*\0\0\0\0\0\x01\0\0\x03\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
