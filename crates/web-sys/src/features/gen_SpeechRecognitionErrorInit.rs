#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = SpeechRecognitionErrorInit)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SpeechRecognitionErrorInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechRecognitionErrorInit`*"]
    pub type SpeechRecognitionErrorInit;
    #[wasm_bindgen(method, setter = "bubbles")]
    fn bubbles_shim(this: &SpeechRecognitionErrorInit, val: bool);
    #[wasm_bindgen(method, setter = "cancelable")]
    fn cancelable_shim(this: &SpeechRecognitionErrorInit, val: bool);
    #[wasm_bindgen(method, setter = "composed")]
    fn composed_shim(this: &SpeechRecognitionErrorInit, val: bool);
    #[cfg(feature = "SpeechRecognitionErrorCode")]
    #[wasm_bindgen(method, setter = "error")]
    fn error_shim(this: &SpeechRecognitionErrorInit, val: SpeechRecognitionErrorCode);
    #[wasm_bindgen(method, setter = "message")]
    fn message_shim(this: &SpeechRecognitionErrorInit, val: &str);
}
impl SpeechRecognitionErrorInit {
    #[doc = "Construct a new `SpeechRecognitionErrorInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechRecognitionErrorInit`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechRecognitionErrorInit`*"]
    pub fn bubbles(&mut self, val: bool) -> &mut Self {
        self.bubbles_shim(val);
        self
    }
    #[doc = "Change the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechRecognitionErrorInit`*"]
    pub fn cancelable(&mut self, val: bool) -> &mut Self {
        self.cancelable_shim(val);
        self
    }
    #[doc = "Change the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechRecognitionErrorInit`*"]
    pub fn composed(&mut self, val: bool) -> &mut Self {
        self.composed_shim(val);
        self
    }
    #[cfg(feature = "SpeechRecognitionErrorCode")]
    #[doc = "Change the `error` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechRecognitionErrorCode`, `SpeechRecognitionErrorInit`*"]
    pub fn error(&mut self, val: SpeechRecognitionErrorCode) -> &mut Self {
        self.error_shim(val);
        self
    }
    #[doc = "Change the `message` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechRecognitionErrorInit`*"]
    pub fn message(&mut self, val: &str) -> &mut Self {
        self.message_shim(val);
        self
    }
}
impl Default for SpeechRecognitionErrorInit {
    fn default() -> Self {
        Self::new()
    }
}
