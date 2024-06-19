#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = SpeechSynthesisErrorEventInit)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SpeechSynthesisErrorEventInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechSynthesisErrorEventInit`*"]
    pub type SpeechSynthesisErrorEventInit;
    #[doc = "Get the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechSynthesisErrorEventInit`*"]
    #[wasm_bindgen(method, getter = "bubbles")]
    pub fn get_bubbles(this: &SpeechSynthesisErrorEventInit) -> Option<bool>;
    #[wasm_bindgen(method, setter = "bubbles")]
    fn set_bubbles(this: &SpeechSynthesisErrorEventInit, val: bool);
    #[doc = "Get the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechSynthesisErrorEventInit`*"]
    #[wasm_bindgen(method, getter = "cancelable")]
    pub fn get_cancelable(this: &SpeechSynthesisErrorEventInit) -> Option<bool>;
    #[wasm_bindgen(method, setter = "cancelable")]
    fn set_cancelable(this: &SpeechSynthesisErrorEventInit, val: bool);
    #[doc = "Get the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechSynthesisErrorEventInit`*"]
    #[wasm_bindgen(method, getter = "composed")]
    pub fn get_composed(this: &SpeechSynthesisErrorEventInit) -> Option<bool>;
    #[wasm_bindgen(method, setter = "composed")]
    fn set_composed(this: &SpeechSynthesisErrorEventInit, val: bool);
    #[doc = "Get the `charIndex` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechSynthesisErrorEventInit`*"]
    #[wasm_bindgen(method, getter = "charIndex")]
    pub fn get_char_index(this: &SpeechSynthesisErrorEventInit) -> Option<u32>;
    #[wasm_bindgen(method, setter = "charIndex")]
    fn set_char_index(this: &SpeechSynthesisErrorEventInit, val: u32);
    #[doc = "Get the `charLength` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechSynthesisErrorEventInit`*"]
    #[wasm_bindgen(method, getter = "charLength")]
    pub fn get_char_length(this: &SpeechSynthesisErrorEventInit) -> Option<u32>;
    #[wasm_bindgen(method, setter = "charLength")]
    fn set_char_length(this: &SpeechSynthesisErrorEventInit, val: Option<u32>);
    #[doc = "Get the `elapsedTime` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechSynthesisErrorEventInit`*"]
    #[wasm_bindgen(method, getter = "elapsedTime")]
    pub fn get_elapsed_time(this: &SpeechSynthesisErrorEventInit) -> Option<f32>;
    #[wasm_bindgen(method, setter = "elapsedTime")]
    fn set_elapsed_time(this: &SpeechSynthesisErrorEventInit, val: f32);
    #[doc = "Get the `name` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechSynthesisErrorEventInit`*"]
    #[wasm_bindgen(method, getter = "name")]
    pub fn get_name(this: &SpeechSynthesisErrorEventInit) -> Option<String>;
    #[wasm_bindgen(method, setter = "name")]
    fn set_name(this: &SpeechSynthesisErrorEventInit, val: &str);
    #[cfg(feature = "SpeechSynthesisUtterance")]
    #[doc = "Get the `utterance` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechSynthesisErrorEventInit`, `SpeechSynthesisUtterance`*"]
    #[wasm_bindgen(method, getter = "utterance")]
    pub fn get_utterance(this: &SpeechSynthesisErrorEventInit) -> SpeechSynthesisUtterance;
    #[cfg(feature = "SpeechSynthesisUtterance")]
    #[wasm_bindgen(method, setter = "utterance")]
    fn set_utterance(this: &SpeechSynthesisErrorEventInit, val: &SpeechSynthesisUtterance);
    #[cfg(feature = "SpeechSynthesisErrorCode")]
    #[doc = "Get the `error` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechSynthesisErrorCode`, `SpeechSynthesisErrorEventInit`*"]
    #[wasm_bindgen(method, getter = "error")]
    pub fn get_error(this: &SpeechSynthesisErrorEventInit) -> SpeechSynthesisErrorCode;
    #[cfg(feature = "SpeechSynthesisErrorCode")]
    #[wasm_bindgen(method, setter = "error")]
    fn set_error(this: &SpeechSynthesisErrorEventInit, val: SpeechSynthesisErrorCode);
}
impl SpeechSynthesisErrorEventInit {
    #[cfg(all(
        feature = "SpeechSynthesisErrorCode",
        feature = "SpeechSynthesisUtterance",
    ))]
    #[doc = "Construct a new `SpeechSynthesisErrorEventInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechSynthesisErrorCode`, `SpeechSynthesisErrorEventInit`, `SpeechSynthesisUtterance`*"]
    pub fn new(utterance: &SpeechSynthesisUtterance, error: SpeechSynthesisErrorCode) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.utterance(utterance);
        ret.error(error);
        ret
    }
    #[doc = "Change the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechSynthesisErrorEventInit`*"]
    pub fn bubbles(&mut self, val: bool) -> &mut Self {
        self.set_bubbles(val);
        self
    }
    #[doc = "Change the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechSynthesisErrorEventInit`*"]
    pub fn cancelable(&mut self, val: bool) -> &mut Self {
        self.set_cancelable(val);
        self
    }
    #[doc = "Change the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechSynthesisErrorEventInit`*"]
    pub fn composed(&mut self, val: bool) -> &mut Self {
        self.set_composed(val);
        self
    }
    #[doc = "Change the `charIndex` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechSynthesisErrorEventInit`*"]
    pub fn char_index(&mut self, val: u32) -> &mut Self {
        self.set_char_index(val);
        self
    }
    #[doc = "Change the `charLength` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechSynthesisErrorEventInit`*"]
    pub fn char_length(&mut self, val: Option<u32>) -> &mut Self {
        self.set_char_length(val);
        self
    }
    #[doc = "Change the `elapsedTime` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechSynthesisErrorEventInit`*"]
    pub fn elapsed_time(&mut self, val: f32) -> &mut Self {
        self.set_elapsed_time(val);
        self
    }
    #[doc = "Change the `name` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechSynthesisErrorEventInit`*"]
    pub fn name(&mut self, val: &str) -> &mut Self {
        self.set_name(val);
        self
    }
    #[cfg(feature = "SpeechSynthesisUtterance")]
    #[doc = "Change the `utterance` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechSynthesisErrorEventInit`, `SpeechSynthesisUtterance`*"]
    pub fn utterance(&mut self, val: &SpeechSynthesisUtterance) -> &mut Self {
        self.set_utterance(val);
        self
    }
    #[cfg(feature = "SpeechSynthesisErrorCode")]
    #[doc = "Change the `error` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechSynthesisErrorCode`, `SpeechSynthesisErrorEventInit`*"]
    pub fn error(&mut self, val: SpeechSynthesisErrorCode) -> &mut Self {
        self.set_error(val);
        self
    }
}
