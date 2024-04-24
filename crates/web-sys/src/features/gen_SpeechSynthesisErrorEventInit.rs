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
    #[wasm_bindgen(method, getter = "bubbles")]
    fn bubbles_shim(this: &SpeechSynthesisErrorEventInit) -> bool;
    #[wasm_bindgen(method, setter = "bubbles")]
    fn set_bubbles_shim(this: &SpeechSynthesisErrorEventInit, val: bool);
    #[wasm_bindgen(method, getter = "cancelable")]
    fn cancelable_shim(this: &SpeechSynthesisErrorEventInit) -> bool;
    #[wasm_bindgen(method, setter = "cancelable")]
    fn set_cancelable_shim(this: &SpeechSynthesisErrorEventInit, val: bool);
    #[wasm_bindgen(method, getter = "composed")]
    fn composed_shim(this: &SpeechSynthesisErrorEventInit) -> bool;
    #[wasm_bindgen(method, setter = "composed")]
    fn set_composed_shim(this: &SpeechSynthesisErrorEventInit, val: bool);
    #[wasm_bindgen(method, getter = "charIndex")]
    fn char_index_shim(this: &SpeechSynthesisErrorEventInit) -> u32;
    #[wasm_bindgen(method, setter = "charIndex")]
    fn set_char_index_shim(this: &SpeechSynthesisErrorEventInit, val: u32);
    #[wasm_bindgen(method, getter = "charLength")]
    fn char_length_shim(this: &SpeechSynthesisErrorEventInit) -> Option<u32>;
    #[wasm_bindgen(method, setter = "charLength")]
    fn set_char_length_shim(this: &SpeechSynthesisErrorEventInit, val: Option<u32>);
    #[wasm_bindgen(method, getter = "elapsedTime")]
    fn elapsed_time_shim(this: &SpeechSynthesisErrorEventInit) -> f32;
    #[wasm_bindgen(method, setter = "elapsedTime")]
    fn set_elapsed_time_shim(this: &SpeechSynthesisErrorEventInit, val: f32);
    #[wasm_bindgen(method, getter = "name")]
    fn name_shim(this: &SpeechSynthesisErrorEventInit) -> String;
    #[wasm_bindgen(method, setter = "name")]
    fn set_name_shim(this: &SpeechSynthesisErrorEventInit, val: &str);
    #[cfg(feature = "SpeechSynthesisUtterance")]
    #[wasm_bindgen(method, getter = "utterance")]
    fn utterance_shim(this: &SpeechSynthesisErrorEventInit) -> SpeechSynthesisUtterance;
    #[cfg(feature = "SpeechSynthesisUtterance")]
    #[wasm_bindgen(method, setter = "utterance")]
    fn set_utterance_shim(this: &SpeechSynthesisErrorEventInit, val: &SpeechSynthesisUtterance);
    #[cfg(feature = "SpeechSynthesisErrorCode")]
    #[wasm_bindgen(method, getter = "error")]
    fn error_shim(this: &SpeechSynthesisErrorEventInit) -> SpeechSynthesisErrorCode;
    #[cfg(feature = "SpeechSynthesisErrorCode")]
    #[wasm_bindgen(method, setter = "error")]
    fn set_error_shim(this: &SpeechSynthesisErrorEventInit, val: SpeechSynthesisErrorCode);
}
#[doc = "The trait to access properties on the `SpeechSynthesisErrorEventInit` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `SpeechSynthesisErrorEventInit`*"]
pub trait SpeechSynthesisErrorEventInitGetters {
    #[doc = "Get the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechSynthesisErrorEventInit`*"]
    fn bubbles(&self) -> bool;
    #[doc = "Get the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechSynthesisErrorEventInit`*"]
    fn cancelable(&self) -> bool;
    #[doc = "Get the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechSynthesisErrorEventInit`*"]
    fn composed(&self) -> bool;
    #[doc = "Get the `charIndex` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechSynthesisErrorEventInit`*"]
    fn char_index(&self) -> u32;
    #[doc = "Get the `charLength` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechSynthesisErrorEventInit`*"]
    fn char_length(&self) -> Option<u32>;
    #[doc = "Get the `elapsedTime` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechSynthesisErrorEventInit`*"]
    fn elapsed_time(&self) -> f32;
    #[doc = "Get the `name` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechSynthesisErrorEventInit`*"]
    fn name(&self) -> String;
    #[cfg(feature = "SpeechSynthesisUtterance")]
    #[doc = "Get the `utterance` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechSynthesisErrorEventInit`, `SpeechSynthesisUtterance`*"]
    fn utterance(&self) -> SpeechSynthesisUtterance;
    #[cfg(feature = "SpeechSynthesisErrorCode")]
    #[doc = "Get the `error` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechSynthesisErrorCode`, `SpeechSynthesisErrorEventInit`*"]
    fn error(&self) -> SpeechSynthesisErrorCode;
}
impl SpeechSynthesisErrorEventInitGetters for SpeechSynthesisErrorEventInit {
    fn bubbles(&self) -> bool {
        self.bubbles_shim()
    }
    fn cancelable(&self) -> bool {
        self.cancelable_shim()
    }
    fn composed(&self) -> bool {
        self.composed_shim()
    }
    fn char_index(&self) -> u32 {
        self.char_index_shim()
    }
    fn char_length(&self) -> Option<u32> {
        self.char_length_shim()
    }
    fn elapsed_time(&self) -> f32 {
        self.elapsed_time_shim()
    }
    fn name(&self) -> String {
        self.name_shim()
    }
    #[cfg(feature = "SpeechSynthesisUtterance")]
    fn utterance(&self) -> SpeechSynthesisUtterance {
        self.utterance_shim()
    }
    #[cfg(feature = "SpeechSynthesisErrorCode")]
    fn error(&self) -> SpeechSynthesisErrorCode {
        self.error_shim()
    }
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
        self.set_bubbles_shim(val);
        self
    }
    #[doc = "Change the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechSynthesisErrorEventInit`*"]
    pub fn cancelable(&mut self, val: bool) -> &mut Self {
        self.set_cancelable_shim(val);
        self
    }
    #[doc = "Change the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechSynthesisErrorEventInit`*"]
    pub fn composed(&mut self, val: bool) -> &mut Self {
        self.set_composed_shim(val);
        self
    }
    #[doc = "Change the `charIndex` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechSynthesisErrorEventInit`*"]
    pub fn char_index(&mut self, val: u32) -> &mut Self {
        self.set_char_index_shim(val);
        self
    }
    #[doc = "Change the `charLength` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechSynthesisErrorEventInit`*"]
    pub fn char_length(&mut self, val: Option<u32>) -> &mut Self {
        self.set_char_length_shim(val);
        self
    }
    #[doc = "Change the `elapsedTime` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechSynthesisErrorEventInit`*"]
    pub fn elapsed_time(&mut self, val: f32) -> &mut Self {
        self.set_elapsed_time_shim(val);
        self
    }
    #[doc = "Change the `name` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechSynthesisErrorEventInit`*"]
    pub fn name(&mut self, val: &str) -> &mut Self {
        self.set_name_shim(val);
        self
    }
    #[cfg(feature = "SpeechSynthesisUtterance")]
    #[doc = "Change the `utterance` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechSynthesisErrorEventInit`, `SpeechSynthesisUtterance`*"]
    pub fn utterance(&mut self, val: &SpeechSynthesisUtterance) -> &mut Self {
        self.set_utterance_shim(val);
        self
    }
    #[cfg(feature = "SpeechSynthesisErrorCode")]
    #[doc = "Change the `error` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechSynthesisErrorCode`, `SpeechSynthesisErrorEventInit`*"]
    pub fn error(&mut self, val: SpeechSynthesisErrorCode) -> &mut Self {
        self.set_error_shim(val);
        self
    }
}
