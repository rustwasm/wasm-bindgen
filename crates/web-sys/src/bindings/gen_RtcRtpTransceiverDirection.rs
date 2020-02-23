use super::*;
use js_sys::Object;
#[allow(bad_style)]
#[derive(Copy, Clone, PartialEq, Debug)]
#[allow(clippy::all)]
pub enum RtcRtpTransceiverDirection {
    Sendrecv = 0,
    Sendonly = 1,
    Recvonly = 2,
    Inactive = 3,
    #[doc(hidden)]
    __Nonexhaustive,
}
#[allow(clippy::all)]
impl RtcRtpTransceiverDirection {
    pub fn from_js_value(obj: &wasm_bindgen::JsValue) -> Option<RtcRtpTransceiverDirection> {
        obj.as_string().and_then(|obj_str| match obj_str.as_str() {
            "sendrecv" => Some(RtcRtpTransceiverDirection::Sendrecv),
            "sendonly" => Some(RtcRtpTransceiverDirection::Sendonly),
            "recvonly" => Some(RtcRtpTransceiverDirection::Recvonly),
            "inactive" => Some(RtcRtpTransceiverDirection::Inactive),
            _ => None,
        })
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::describe::WasmDescribe for RtcRtpTransceiverDirection {
    fn describe() {
        wasm_bindgen::JsValue::describe()
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::IntoWasmAbi for RtcRtpTransceiverDirection {
    type Abi = <wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi;
    #[inline]
    fn into_abi(self) -> Self::Abi {
        wasm_bindgen::JsValue::from(self).into_abi()
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::FromWasmAbi for RtcRtpTransceiverDirection {
    type Abi = <wasm_bindgen::JsValue as wasm_bindgen::convert::FromWasmAbi>::Abi;
    unsafe fn from_abi(js: Self::Abi) -> Self {
        RtcRtpTransceiverDirection::from_js_value(&wasm_bindgen::JsValue::from_abi(js))
            .unwrap_or(RtcRtpTransceiverDirection::__Nonexhaustive)
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::OptionIntoWasmAbi for RtcRtpTransceiverDirection {
    #[inline]
    fn none() -> Self::Abi {
        Object::none()
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::OptionFromWasmAbi for RtcRtpTransceiverDirection {
    #[inline]
    fn is_none(abi: &Self::Abi) -> bool {
        Object::is_none(abi)
    }
}
#[allow(clippy::all)]
impl From<RtcRtpTransceiverDirection> for wasm_bindgen::JsValue {
    fn from(obj: RtcRtpTransceiverDirection) -> wasm_bindgen::JsValue {
        match obj {
            RtcRtpTransceiverDirection::Sendrecv => wasm_bindgen::JsValue::from_str("sendrecv"),
            RtcRtpTransceiverDirection::Sendonly => wasm_bindgen::JsValue::from_str("sendonly"),
            RtcRtpTransceiverDirection::Recvonly => wasm_bindgen::JsValue::from_str("recvonly"),
            RtcRtpTransceiverDirection::Inactive => wasm_bindgen::JsValue::from_str("inactive"),
            RtcRtpTransceiverDirection::__Nonexhaustive => {
                panic!("attempted to convert invalid RtcRtpTransceiverDirection into JSValue")
            }
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_cf0141d208946fdc: [u8; 108usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}*\0\0\0\0\0\x01\0\0\x03\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
