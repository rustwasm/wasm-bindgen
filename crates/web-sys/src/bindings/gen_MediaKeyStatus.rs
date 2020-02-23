use super::*;
#[allow(bad_style)]
#[derive(Copy, Clone, PartialEq, Debug)]
#[allow(clippy::all)]
pub enum MediaKeyStatus {
    Usable = 0,
    Expired = 1,
    Released = 2,
    OutputRestricted = 3,
    OutputDownscaled = 4,
    StatusPending = 5,
    InternalError = 6,
    #[doc(hidden)]
    __Nonexhaustive,
}
#[allow(clippy::all)]
impl MediaKeyStatus {
    pub fn from_js_value(obj: &wasm_bindgen::JsValue) -> Option<MediaKeyStatus> {
        obj.as_string().and_then(|obj_str| match obj_str.as_str() {
            "usable" => Some(MediaKeyStatus::Usable),
            "expired" => Some(MediaKeyStatus::Expired),
            "released" => Some(MediaKeyStatus::Released),
            "output-restricted" => Some(MediaKeyStatus::OutputRestricted),
            "output-downscaled" => Some(MediaKeyStatus::OutputDownscaled),
            "status-pending" => Some(MediaKeyStatus::StatusPending),
            "internal-error" => Some(MediaKeyStatus::InternalError),
            _ => None,
        })
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::describe::WasmDescribe for MediaKeyStatus {
    fn describe() {
        wasm_bindgen::JsValue::describe()
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::IntoWasmAbi for MediaKeyStatus {
    type Abi = <wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi;
    #[inline]
    fn into_abi(self) -> Self::Abi {
        wasm_bindgen::JsValue::from(self).into_abi()
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::FromWasmAbi for MediaKeyStatus {
    type Abi = <wasm_bindgen::JsValue as wasm_bindgen::convert::FromWasmAbi>::Abi;
    unsafe fn from_abi(js: Self::Abi) -> Self {
        MediaKeyStatus::from_js_value(&wasm_bindgen::JsValue::from_abi(js))
            .unwrap_or(MediaKeyStatus::__Nonexhaustive)
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::OptionIntoWasmAbi for MediaKeyStatus {
    #[inline]
    fn none() -> Self::Abi {
        Object::none()
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::OptionFromWasmAbi for MediaKeyStatus {
    #[inline]
    fn is_none(abi: &Self::Abi) -> bool {
        Object::is_none(abi)
    }
}
#[allow(clippy::all)]
impl From<MediaKeyStatus> for wasm_bindgen::JsValue {
    fn from(obj: MediaKeyStatus) -> wasm_bindgen::JsValue {
        match obj {
            MediaKeyStatus::Usable => wasm_bindgen::JsValue::from_str("usable"),
            MediaKeyStatus::Expired => wasm_bindgen::JsValue::from_str("expired"),
            MediaKeyStatus::Released => wasm_bindgen::JsValue::from_str("released"),
            MediaKeyStatus::OutputRestricted => {
                wasm_bindgen::JsValue::from_str("output-restricted")
            }
            MediaKeyStatus::OutputDownscaled => {
                wasm_bindgen::JsValue::from_str("output-downscaled")
            }
            MediaKeyStatus::StatusPending => wasm_bindgen::JsValue::from_str("status-pending"),
            MediaKeyStatus::InternalError => wasm_bindgen::JsValue::from_str("internal-error"),
            MediaKeyStatus::__Nonexhaustive => {
                panic!("attempted to convert invalid MediaKeyStatus into JSValue")
            }
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_ae042791b1a2f052: [u8; 108usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}*\0\0\0\0\0\x01\0\0\x03\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
