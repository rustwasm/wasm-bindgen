use super::*;
use js_sys::Object;
#[allow(bad_style)]
#[derive(Copy, Clone, PartialEq, Debug)]
#[allow(clippy::all)]
pub enum SpeechSynthesisErrorCode {
    Canceled = 0,
    Interrupted = 1,
    AudioBusy = 2,
    AudioHardware = 3,
    Network = 4,
    SynthesisUnavailable = 5,
    SynthesisFailed = 6,
    LanguageUnavailable = 7,
    VoiceUnavailable = 8,
    TextTooLong = 9,
    InvalidArgument = 10,
    #[doc(hidden)]
    __Nonexhaustive,
}
#[allow(clippy::all)]
impl SpeechSynthesisErrorCode {
    pub fn from_js_value(obj: &wasm_bindgen::JsValue) -> Option<SpeechSynthesisErrorCode> {
        obj.as_string().and_then(|obj_str| match obj_str.as_str() {
            "canceled" => Some(SpeechSynthesisErrorCode::Canceled),
            "interrupted" => Some(SpeechSynthesisErrorCode::Interrupted),
            "audio-busy" => Some(SpeechSynthesisErrorCode::AudioBusy),
            "audio-hardware" => Some(SpeechSynthesisErrorCode::AudioHardware),
            "network" => Some(SpeechSynthesisErrorCode::Network),
            "synthesis-unavailable" => Some(SpeechSynthesisErrorCode::SynthesisUnavailable),
            "synthesis-failed" => Some(SpeechSynthesisErrorCode::SynthesisFailed),
            "language-unavailable" => Some(SpeechSynthesisErrorCode::LanguageUnavailable),
            "voice-unavailable" => Some(SpeechSynthesisErrorCode::VoiceUnavailable),
            "text-too-long" => Some(SpeechSynthesisErrorCode::TextTooLong),
            "invalid-argument" => Some(SpeechSynthesisErrorCode::InvalidArgument),
            _ => None,
        })
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::describe::WasmDescribe for SpeechSynthesisErrorCode {
    fn describe() {
        wasm_bindgen::JsValue::describe()
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::IntoWasmAbi for SpeechSynthesisErrorCode {
    type Abi = <wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi;
    #[inline]
    fn into_abi(self) -> Self::Abi {
        wasm_bindgen::JsValue::from(self).into_abi()
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::FromWasmAbi for SpeechSynthesisErrorCode {
    type Abi = <wasm_bindgen::JsValue as wasm_bindgen::convert::FromWasmAbi>::Abi;
    unsafe fn from_abi(js: Self::Abi) -> Self {
        SpeechSynthesisErrorCode::from_js_value(&wasm_bindgen::JsValue::from_abi(js))
            .unwrap_or(SpeechSynthesisErrorCode::__Nonexhaustive)
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::OptionIntoWasmAbi for SpeechSynthesisErrorCode {
    #[inline]
    fn none() -> Self::Abi {
        Object::none()
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::OptionFromWasmAbi for SpeechSynthesisErrorCode {
    #[inline]
    fn is_none(abi: &Self::Abi) -> bool {
        Object::is_none(abi)
    }
}
#[allow(clippy::all)]
impl From<SpeechSynthesisErrorCode> for wasm_bindgen::JsValue {
    fn from(obj: SpeechSynthesisErrorCode) -> wasm_bindgen::JsValue {
        match obj {
            SpeechSynthesisErrorCode::Canceled => wasm_bindgen::JsValue::from_str("canceled"),
            SpeechSynthesisErrorCode::Interrupted => wasm_bindgen::JsValue::from_str("interrupted"),
            SpeechSynthesisErrorCode::AudioBusy => wasm_bindgen::JsValue::from_str("audio-busy"),
            SpeechSynthesisErrorCode::AudioHardware => {
                wasm_bindgen::JsValue::from_str("audio-hardware")
            }
            SpeechSynthesisErrorCode::Network => wasm_bindgen::JsValue::from_str("network"),
            SpeechSynthesisErrorCode::SynthesisUnavailable => {
                wasm_bindgen::JsValue::from_str("synthesis-unavailable")
            }
            SpeechSynthesisErrorCode::SynthesisFailed => {
                wasm_bindgen::JsValue::from_str("synthesis-failed")
            }
            SpeechSynthesisErrorCode::LanguageUnavailable => {
                wasm_bindgen::JsValue::from_str("language-unavailable")
            }
            SpeechSynthesisErrorCode::VoiceUnavailable => {
                wasm_bindgen::JsValue::from_str("voice-unavailable")
            }
            SpeechSynthesisErrorCode::TextTooLong => {
                wasm_bindgen::JsValue::from_str("text-too-long")
            }
            SpeechSynthesisErrorCode::InvalidArgument => {
                wasm_bindgen::JsValue::from_str("invalid-argument")
            }
            SpeechSynthesisErrorCode::__Nonexhaustive => {
                panic!("attempted to convert invalid SpeechSynthesisErrorCode into JSValue")
            }
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_bdb21924257beefb: [u8; 108usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}*\0\0\0\0\0\x01\0\0\x03\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
