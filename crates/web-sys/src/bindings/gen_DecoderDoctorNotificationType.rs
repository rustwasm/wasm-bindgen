use super::*;
#[allow(bad_style)]
#[derive(Copy, Clone, PartialEq, Debug)]
#[allow(clippy::all)]
pub enum DecoderDoctorNotificationType {
    CannotPlay = 0,
    PlatformDecoderNotFound = 1,
    CanPlayButSomeMissingDecoders = 2,
    CannotInitializePulseaudio = 3,
    UnsupportedLibavcodec = 4,
    DecodeError = 5,
    DecodeWarning = 6,
    #[doc(hidden)]
    __Nonexhaustive,
}
#[allow(clippy::all)]
impl DecoderDoctorNotificationType {
    pub fn from_js_value(obj: &wasm_bindgen::JsValue) -> Option<DecoderDoctorNotificationType> {
        obj.as_string().and_then(|obj_str| match obj_str.as_str() {
            "cannot-play" => Some(DecoderDoctorNotificationType::CannotPlay),
            "platform-decoder-not-found" => {
                Some(DecoderDoctorNotificationType::PlatformDecoderNotFound)
            }
            "can-play-but-some-missing-decoders" => {
                Some(DecoderDoctorNotificationType::CanPlayButSomeMissingDecoders)
            }
            "cannot-initialize-pulseaudio" => {
                Some(DecoderDoctorNotificationType::CannotInitializePulseaudio)
            }
            "unsupported-libavcodec" => Some(DecoderDoctorNotificationType::UnsupportedLibavcodec),
            "decode-error" => Some(DecoderDoctorNotificationType::DecodeError),
            "decode-warning" => Some(DecoderDoctorNotificationType::DecodeWarning),
            _ => None,
        })
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::describe::WasmDescribe for DecoderDoctorNotificationType {
    fn describe() {
        wasm_bindgen::JsValue::describe()
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::IntoWasmAbi for DecoderDoctorNotificationType {
    type Abi = <wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi;
    #[inline]
    fn into_abi(self) -> Self::Abi {
        wasm_bindgen::JsValue::from(self).into_abi()
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::FromWasmAbi for DecoderDoctorNotificationType {
    type Abi = <wasm_bindgen::JsValue as wasm_bindgen::convert::FromWasmAbi>::Abi;
    unsafe fn from_abi(js: Self::Abi) -> Self {
        DecoderDoctorNotificationType::from_js_value(&wasm_bindgen::JsValue::from_abi(js))
            .unwrap_or(DecoderDoctorNotificationType::__Nonexhaustive)
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::OptionIntoWasmAbi for DecoderDoctorNotificationType {
    #[inline]
    fn none() -> Self::Abi {
        Object::none()
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::OptionFromWasmAbi for DecoderDoctorNotificationType {
    #[inline]
    fn is_none(abi: &Self::Abi) -> bool {
        Object::is_none(abi)
    }
}
#[allow(clippy::all)]
impl From<DecoderDoctorNotificationType> for wasm_bindgen::JsValue {
    fn from(obj: DecoderDoctorNotificationType) -> wasm_bindgen::JsValue {
        match obj {
            DecoderDoctorNotificationType::CannotPlay => {
                wasm_bindgen::JsValue::from_str("cannot-play")
            }
            DecoderDoctorNotificationType::PlatformDecoderNotFound => {
                wasm_bindgen::JsValue::from_str("platform-decoder-not-found")
            }
            DecoderDoctorNotificationType::CanPlayButSomeMissingDecoders => {
                wasm_bindgen::JsValue::from_str("can-play-but-some-missing-decoders")
            }
            DecoderDoctorNotificationType::CannotInitializePulseaudio => {
                wasm_bindgen::JsValue::from_str("cannot-initialize-pulseaudio")
            }
            DecoderDoctorNotificationType::UnsupportedLibavcodec => {
                wasm_bindgen::JsValue::from_str("unsupported-libavcodec")
            }
            DecoderDoctorNotificationType::DecodeError => {
                wasm_bindgen::JsValue::from_str("decode-error")
            }
            DecoderDoctorNotificationType::DecodeWarning => {
                wasm_bindgen::JsValue::from_str("decode-warning")
            }
            DecoderDoctorNotificationType::__Nonexhaustive => {
                panic!("attempted to convert invalid DecoderDoctorNotificationType into JSValue")
            }
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_b1f8f347aac90019: [u8; 108usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}*\0\0\0\0\0\x01\0\0\x03\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
