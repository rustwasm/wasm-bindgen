use super::*;
#[allow(bad_style)]
#[derive(Copy, Clone, PartialEq, Debug)]
#[allow(clippy::all)]
pub enum SpeechRecognitionErrorCode {
    NoSpeech = 0,
    Aborted = 1,
    AudioCapture = 2,
    Network = 3,
    NotAllowed = 4,
    ServiceNotAllowed = 5,
    BadGrammar = 6,
    LanguageNotSupported = 7,
    #[doc(hidden)]
    __Nonexhaustive,
}
#[allow(clippy::all)]
impl SpeechRecognitionErrorCode {
    pub fn from_js_value(obj: &wasm_bindgen::JsValue) -> Option<SpeechRecognitionErrorCode> {
        obj.as_string().and_then(|obj_str| match obj_str.as_str() {
            "no-speech" => Some(SpeechRecognitionErrorCode::NoSpeech),
            "aborted" => Some(SpeechRecognitionErrorCode::Aborted),
            "audio-capture" => Some(SpeechRecognitionErrorCode::AudioCapture),
            "network" => Some(SpeechRecognitionErrorCode::Network),
            "not-allowed" => Some(SpeechRecognitionErrorCode::NotAllowed),
            "service-not-allowed" => Some(SpeechRecognitionErrorCode::ServiceNotAllowed),
            "bad-grammar" => Some(SpeechRecognitionErrorCode::BadGrammar),
            "language-not-supported" => Some(SpeechRecognitionErrorCode::LanguageNotSupported),
            _ => None,
        })
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::describe::WasmDescribe for SpeechRecognitionErrorCode {
    fn describe() {
        wasm_bindgen::JsValue::describe()
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::IntoWasmAbi for SpeechRecognitionErrorCode {
    type Abi = <wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi;
    #[inline]
    fn into_abi(self) -> Self::Abi {
        wasm_bindgen::JsValue::from(self).into_abi()
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::FromWasmAbi for SpeechRecognitionErrorCode {
    type Abi = <wasm_bindgen::JsValue as wasm_bindgen::convert::FromWasmAbi>::Abi;
    unsafe fn from_abi(js: Self::Abi) -> Self {
        SpeechRecognitionErrorCode::from_js_value(&wasm_bindgen::JsValue::from_abi(js))
            .unwrap_or(SpeechRecognitionErrorCode::__Nonexhaustive)
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::OptionIntoWasmAbi for SpeechRecognitionErrorCode {
    #[inline]
    fn none() -> Self::Abi {
        Object::none()
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::OptionFromWasmAbi for SpeechRecognitionErrorCode {
    #[inline]
    fn is_none(abi: &Self::Abi) -> bool {
        Object::is_none(abi)
    }
}
#[allow(clippy::all)]
impl From<SpeechRecognitionErrorCode> for wasm_bindgen::JsValue {
    fn from(obj: SpeechRecognitionErrorCode) -> wasm_bindgen::JsValue {
        match obj {
            SpeechRecognitionErrorCode::NoSpeech => wasm_bindgen::JsValue::from_str("no-speech"),
            SpeechRecognitionErrorCode::Aborted => wasm_bindgen::JsValue::from_str("aborted"),
            SpeechRecognitionErrorCode::AudioCapture => {
                wasm_bindgen::JsValue::from_str("audio-capture")
            }
            SpeechRecognitionErrorCode::Network => wasm_bindgen::JsValue::from_str("network"),
            SpeechRecognitionErrorCode::NotAllowed => {
                wasm_bindgen::JsValue::from_str("not-allowed")
            }
            SpeechRecognitionErrorCode::ServiceNotAllowed => {
                wasm_bindgen::JsValue::from_str("service-not-allowed")
            }
            SpeechRecognitionErrorCode::BadGrammar => {
                wasm_bindgen::JsValue::from_str("bad-grammar")
            }
            SpeechRecognitionErrorCode::LanguageNotSupported => {
                wasm_bindgen::JsValue::from_str("language-not-supported")
            }
            SpeechRecognitionErrorCode::__Nonexhaustive => {
                panic!("attempted to convert invalid SpeechRecognitionErrorCode into JSValue")
            }
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_1832b7fc675bd2ce: [u8; 108usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}*\0\0\0\0\0\x01\0\0\x03\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
