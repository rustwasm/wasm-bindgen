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
    #[wasm_bindgen(method, setter = "bubbles")]
    fn bubbles_shim(this: &SpeechSynthesisErrorEventInit, val: bool);
    #[wasm_bindgen(method, setter = "cancelable")]
    fn cancelable_shim(this: &SpeechSynthesisErrorEventInit, val: bool);
    #[wasm_bindgen(method, setter = "composed")]
    fn composed_shim(this: &SpeechSynthesisErrorEventInit, val: bool);
    #[wasm_bindgen(method, setter = "charIndex")]
    fn char_index_shim(this: &SpeechSynthesisErrorEventInit, val: u32);
    #[wasm_bindgen(method, setter = "charLength")]
    fn char_length_shim(this: &SpeechSynthesisErrorEventInit, val: Option<u32>);
    #[wasm_bindgen(method, setter = "elapsedTime")]
    fn elapsed_time_shim(this: &SpeechSynthesisErrorEventInit, val: f32);
    #[wasm_bindgen(method, setter = "name")]
    fn name_shim(this: &SpeechSynthesisErrorEventInit, val: &str);
    #[cfg(feature = "SpeechSynthesisUtterance")]
    #[wasm_bindgen(method, setter = "utterance")]
    fn utterance_shim(this: &SpeechSynthesisErrorEventInit, val: &SpeechSynthesisUtterance);
    #[cfg(feature = "SpeechSynthesisErrorCode")]
    #[wasm_bindgen(method, setter = "error")]
    fn error_shim(this: &SpeechSynthesisErrorEventInit, val: SpeechSynthesisErrorCode);
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
        self.bubbles_shim(val);
        self
    }
    #[doc = "Change the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechSynthesisErrorEventInit`*"]
    pub fn cancelable(&mut self, val: bool) -> &mut Self {
        self.cancelable_shim(val);
        self
    }
    #[doc = "Change the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechSynthesisErrorEventInit`*"]
    pub fn composed(&mut self, val: bool) -> &mut Self {
        self.composed_shim(val);
        self
    }
    #[doc = "Change the `charIndex` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechSynthesisErrorEventInit`*"]
    pub fn char_index(&mut self, val: u32) -> &mut Self {
        self.char_index_shim(val);
        self
    }
    #[doc = "Change the `charLength` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechSynthesisErrorEventInit`*"]
    pub fn char_length(&mut self, val: Option<u32>) -> &mut Self {
        self.char_length_shim(val);
        self
    }
    #[doc = "Change the `elapsedTime` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechSynthesisErrorEventInit`*"]
    pub fn elapsed_time(&mut self, val: f32) -> &mut Self {
        self.elapsed_time_shim(val);
        self
    }
    #[doc = "Change the `name` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechSynthesisErrorEventInit`*"]
    pub fn name(&mut self, val: &str) -> &mut Self {
        self.name_shim(val);
        self
    }
    #[cfg(feature = "SpeechSynthesisUtterance")]
    #[doc = "Change the `utterance` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechSynthesisErrorEventInit`, `SpeechSynthesisUtterance`*"]
    pub fn utterance(&mut self, val: &SpeechSynthesisUtterance) -> &mut Self {
        self.utterance_shim(val);
        self
    }
    #[cfg(feature = "SpeechSynthesisErrorCode")]
    #[doc = "Change the `error` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechSynthesisErrorCode`, `SpeechSynthesisErrorEventInit`*"]
    pub fn error(&mut self, val: SpeechSynthesisErrorCode) -> &mut Self {
        self.error_shim(val);
        self
    }
}
