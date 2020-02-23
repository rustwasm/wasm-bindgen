use super::*;
#[allow(bad_style)]
#[derive(Copy, Clone, PartialEq, Debug)]
#[allow(clippy::all)]
pub enum MediaKeySystemStatus {
    Available = 0,
    ApiDisabled = 1,
    CdmDisabled = 2,
    CdmNotSupported = 3,
    CdmNotInstalled = 4,
    CdmCreated = 5,
    #[doc(hidden)]
    __Nonexhaustive,
}
#[allow(clippy::all)]
impl MediaKeySystemStatus {
    pub fn from_js_value(obj: &wasm_bindgen::JsValue) -> Option<MediaKeySystemStatus> {
        obj.as_string().and_then(|obj_str| match obj_str.as_str() {
            "available" => Some(MediaKeySystemStatus::Available),
            "api-disabled" => Some(MediaKeySystemStatus::ApiDisabled),
            "cdm-disabled" => Some(MediaKeySystemStatus::CdmDisabled),
            "cdm-not-supported" => Some(MediaKeySystemStatus::CdmNotSupported),
            "cdm-not-installed" => Some(MediaKeySystemStatus::CdmNotInstalled),
            "cdm-created" => Some(MediaKeySystemStatus::CdmCreated),
            _ => None,
        })
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::describe::WasmDescribe for MediaKeySystemStatus {
    fn describe() {
        wasm_bindgen::JsValue::describe()
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::IntoWasmAbi for MediaKeySystemStatus {
    type Abi = <wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi;
    #[inline]
    fn into_abi(self) -> Self::Abi {
        wasm_bindgen::JsValue::from(self).into_abi()
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::FromWasmAbi for MediaKeySystemStatus {
    type Abi = <wasm_bindgen::JsValue as wasm_bindgen::convert::FromWasmAbi>::Abi;
    unsafe fn from_abi(js: Self::Abi) -> Self {
        MediaKeySystemStatus::from_js_value(&wasm_bindgen::JsValue::from_abi(js))
            .unwrap_or(MediaKeySystemStatus::__Nonexhaustive)
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::OptionIntoWasmAbi for MediaKeySystemStatus {
    #[inline]
    fn none() -> Self::Abi {
        Object::none()
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::OptionFromWasmAbi for MediaKeySystemStatus {
    #[inline]
    fn is_none(abi: &Self::Abi) -> bool {
        Object::is_none(abi)
    }
}
#[allow(clippy::all)]
impl From<MediaKeySystemStatus> for wasm_bindgen::JsValue {
    fn from(obj: MediaKeySystemStatus) -> wasm_bindgen::JsValue {
        match obj {
            MediaKeySystemStatus::Available => wasm_bindgen::JsValue::from_str("available"),
            MediaKeySystemStatus::ApiDisabled => wasm_bindgen::JsValue::from_str("api-disabled"),
            MediaKeySystemStatus::CdmDisabled => wasm_bindgen::JsValue::from_str("cdm-disabled"),
            MediaKeySystemStatus::CdmNotSupported => {
                wasm_bindgen::JsValue::from_str("cdm-not-supported")
            }
            MediaKeySystemStatus::CdmNotInstalled => {
                wasm_bindgen::JsValue::from_str("cdm-not-installed")
            }
            MediaKeySystemStatus::CdmCreated => wasm_bindgen::JsValue::from_str("cdm-created"),
            MediaKeySystemStatus::__Nonexhaustive => {
                panic!("attempted to convert invalid MediaKeySystemStatus into JSValue")
            }
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_7e440caed1b8abd9: [u8; 108usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}*\0\0\0\0\0\x01\0\0\x03\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
