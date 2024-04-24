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
    #[wasm_bindgen(method, getter = "bubbles")]
    fn bubbles_shim(this: &SpeechRecognitionErrorInit) -> bool;
    #[wasm_bindgen(method, setter = "bubbles")]
    fn set_bubbles_shim(this: &SpeechRecognitionErrorInit, val: bool);
    #[wasm_bindgen(method, getter = "cancelable")]
    fn cancelable_shim(this: &SpeechRecognitionErrorInit) -> bool;
    #[wasm_bindgen(method, setter = "cancelable")]
    fn set_cancelable_shim(this: &SpeechRecognitionErrorInit, val: bool);
    #[wasm_bindgen(method, getter = "composed")]
    fn composed_shim(this: &SpeechRecognitionErrorInit) -> bool;
    #[wasm_bindgen(method, setter = "composed")]
    fn set_composed_shim(this: &SpeechRecognitionErrorInit, val: bool);
    #[cfg(feature = "SpeechRecognitionErrorCode")]
    #[wasm_bindgen(method, getter = "error")]
    fn error_shim(this: &SpeechRecognitionErrorInit) -> SpeechRecognitionErrorCode;
    #[cfg(feature = "SpeechRecognitionErrorCode")]
    #[wasm_bindgen(method, setter = "error")]
    fn set_error_shim(this: &SpeechRecognitionErrorInit, val: SpeechRecognitionErrorCode);
    #[wasm_bindgen(method, getter = "message")]
    fn message_shim(this: &SpeechRecognitionErrorInit) -> String;
    #[wasm_bindgen(method, setter = "message")]
    fn set_message_shim(this: &SpeechRecognitionErrorInit, val: &str);
}
#[doc = "The trait to access properties on the `SpeechRecognitionErrorInit` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `SpeechRecognitionErrorInit`*"]
pub trait SpeechRecognitionErrorInitGetters {
    #[doc = "Get the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechRecognitionErrorInit`*"]
    fn bubbles(&self) -> bool;
    #[doc = "Get the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechRecognitionErrorInit`*"]
    fn cancelable(&self) -> bool;
    #[doc = "Get the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechRecognitionErrorInit`*"]
    fn composed(&self) -> bool;
    #[cfg(feature = "SpeechRecognitionErrorCode")]
    #[doc = "Get the `error` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechRecognitionErrorCode`, `SpeechRecognitionErrorInit`*"]
    fn error(&self) -> SpeechRecognitionErrorCode;
    #[doc = "Get the `message` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechRecognitionErrorInit`*"]
    fn message(&self) -> String;
}
impl SpeechRecognitionErrorInitGetters for SpeechRecognitionErrorInit {
    fn bubbles(&self) -> bool {
        self.bubbles_shim()
    }
    fn cancelable(&self) -> bool {
        self.cancelable_shim()
    }
    fn composed(&self) -> bool {
        self.composed_shim()
    }
    #[cfg(feature = "SpeechRecognitionErrorCode")]
    fn error(&self) -> SpeechRecognitionErrorCode {
        self.error_shim()
    }
    fn message(&self) -> String {
        self.message_shim()
    }
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
        self.set_bubbles_shim(val);
        self
    }
    #[doc = "Change the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechRecognitionErrorInit`*"]
    pub fn cancelable(&mut self, val: bool) -> &mut Self {
        self.set_cancelable_shim(val);
        self
    }
    #[doc = "Change the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechRecognitionErrorInit`*"]
    pub fn composed(&mut self, val: bool) -> &mut Self {
        self.set_composed_shim(val);
        self
    }
    #[cfg(feature = "SpeechRecognitionErrorCode")]
    #[doc = "Change the `error` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechRecognitionErrorCode`, `SpeechRecognitionErrorInit`*"]
    pub fn error(&mut self, val: SpeechRecognitionErrorCode) -> &mut Self {
        self.set_error_shim(val);
        self
    }
    #[doc = "Change the `message` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechRecognitionErrorInit`*"]
    pub fn message(&mut self, val: &str) -> &mut Self {
        self.set_message_shim(val);
        self
    }
}
impl Default for SpeechRecognitionErrorInit {
    fn default() -> Self {
        Self::new()
    }
}
