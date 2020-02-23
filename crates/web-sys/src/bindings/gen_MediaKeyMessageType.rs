use super::*;
#[allow(bad_style)]
#[derive(Copy, Clone, PartialEq, Debug)]
#[allow(clippy::all)]
pub enum MediaKeyMessageType {
    LicenseRequest = 0,
    LicenseRenewal = 1,
    LicenseRelease = 2,
    IndividualizationRequest = 3,
    #[doc(hidden)]
    __Nonexhaustive,
}
#[allow(clippy::all)]
impl MediaKeyMessageType {
    pub fn from_js_value(obj: &wasm_bindgen::JsValue) -> Option<MediaKeyMessageType> {
        obj.as_string().and_then(|obj_str| match obj_str.as_str() {
            "license-request" => Some(MediaKeyMessageType::LicenseRequest),
            "license-renewal" => Some(MediaKeyMessageType::LicenseRenewal),
            "license-release" => Some(MediaKeyMessageType::LicenseRelease),
            "individualization-request" => Some(MediaKeyMessageType::IndividualizationRequest),
            _ => None,
        })
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::describe::WasmDescribe for MediaKeyMessageType {
    fn describe() {
        wasm_bindgen::JsValue::describe()
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::IntoWasmAbi for MediaKeyMessageType {
    type Abi = <wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi;
    #[inline]
    fn into_abi(self) -> Self::Abi {
        wasm_bindgen::JsValue::from(self).into_abi()
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::FromWasmAbi for MediaKeyMessageType {
    type Abi = <wasm_bindgen::JsValue as wasm_bindgen::convert::FromWasmAbi>::Abi;
    unsafe fn from_abi(js: Self::Abi) -> Self {
        MediaKeyMessageType::from_js_value(&wasm_bindgen::JsValue::from_abi(js))
            .unwrap_or(MediaKeyMessageType::__Nonexhaustive)
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::OptionIntoWasmAbi for MediaKeyMessageType {
    #[inline]
    fn none() -> Self::Abi {
        Object::none()
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::OptionFromWasmAbi for MediaKeyMessageType {
    #[inline]
    fn is_none(abi: &Self::Abi) -> bool {
        Object::is_none(abi)
    }
}
#[allow(clippy::all)]
impl From<MediaKeyMessageType> for wasm_bindgen::JsValue {
    fn from(obj: MediaKeyMessageType) -> wasm_bindgen::JsValue {
        match obj {
            MediaKeyMessageType::LicenseRequest => {
                wasm_bindgen::JsValue::from_str("license-request")
            }
            MediaKeyMessageType::LicenseRenewal => {
                wasm_bindgen::JsValue::from_str("license-renewal")
            }
            MediaKeyMessageType::LicenseRelease => {
                wasm_bindgen::JsValue::from_str("license-release")
            }
            MediaKeyMessageType::IndividualizationRequest => {
                wasm_bindgen::JsValue::from_str("individualization-request")
            }
            MediaKeyMessageType::__Nonexhaustive => {
                panic!("attempted to convert invalid MediaKeyMessageType into JSValue")
            }
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_520e8fac5586d0f5: [u8; 108usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}*\0\0\0\0\0\x01\0\0\x03\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
