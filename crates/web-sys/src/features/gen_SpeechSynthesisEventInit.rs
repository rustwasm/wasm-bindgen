#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = SpeechSynthesisEventInit)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SpeechSynthesisEventInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechSynthesisEventInit`*"]
    pub type SpeechSynthesisEventInit;
    #[wasm_bindgen(method, getter = "bubbles")]
    fn bubbles_shim(this: &SpeechSynthesisEventInit) -> bool;
    #[wasm_bindgen(method, setter = "bubbles")]
    fn set_bubbles_shim(this: &SpeechSynthesisEventInit, val: bool);
    #[wasm_bindgen(method, getter = "cancelable")]
    fn cancelable_shim(this: &SpeechSynthesisEventInit) -> bool;
    #[wasm_bindgen(method, setter = "cancelable")]
    fn set_cancelable_shim(this: &SpeechSynthesisEventInit, val: bool);
    #[wasm_bindgen(method, getter = "composed")]
    fn composed_shim(this: &SpeechSynthesisEventInit) -> bool;
    #[wasm_bindgen(method, setter = "composed")]
    fn set_composed_shim(this: &SpeechSynthesisEventInit, val: bool);
    #[wasm_bindgen(method, getter = "charIndex")]
    fn char_index_shim(this: &SpeechSynthesisEventInit) -> u32;
    #[wasm_bindgen(method, setter = "charIndex")]
    fn set_char_index_shim(this: &SpeechSynthesisEventInit, val: u32);
    #[wasm_bindgen(method, getter = "charLength")]
    fn char_length_shim(this: &SpeechSynthesisEventInit) -> Option<u32>;
    #[wasm_bindgen(method, setter = "charLength")]
    fn set_char_length_shim(this: &SpeechSynthesisEventInit, val: Option<u32>);
    #[wasm_bindgen(method, getter = "elapsedTime")]
    fn elapsed_time_shim(this: &SpeechSynthesisEventInit) -> f32;
    #[wasm_bindgen(method, setter = "elapsedTime")]
    fn set_elapsed_time_shim(this: &SpeechSynthesisEventInit, val: f32);
    #[wasm_bindgen(method, getter = "name")]
    fn name_shim(this: &SpeechSynthesisEventInit) -> String;
    #[wasm_bindgen(method, setter = "name")]
    fn set_name_shim(this: &SpeechSynthesisEventInit, val: &str);
    #[cfg(feature = "SpeechSynthesisUtterance")]
    #[wasm_bindgen(method, getter = "utterance")]
    fn utterance_shim(this: &SpeechSynthesisEventInit) -> SpeechSynthesisUtterance;
    #[cfg(feature = "SpeechSynthesisUtterance")]
    #[wasm_bindgen(method, setter = "utterance")]
    fn set_utterance_shim(this: &SpeechSynthesisEventInit, val: &SpeechSynthesisUtterance);
}
#[doc = "The trait to access properties on the `SpeechSynthesisEventInit` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `SpeechSynthesisEventInit`*"]
pub trait SpeechSynthesisEventInitGetters {
    #[doc = "Get the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechSynthesisEventInit`*"]
    fn bubbles(&self) -> bool;
    #[doc = "Get the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechSynthesisEventInit`*"]
    fn cancelable(&self) -> bool;
    #[doc = "Get the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechSynthesisEventInit`*"]
    fn composed(&self) -> bool;
    #[doc = "Get the `charIndex` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechSynthesisEventInit`*"]
    fn char_index(&self) -> u32;
    #[doc = "Get the `charLength` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechSynthesisEventInit`*"]
    fn char_length(&self) -> Option<u32>;
    #[doc = "Get the `elapsedTime` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechSynthesisEventInit`*"]
    fn elapsed_time(&self) -> f32;
    #[doc = "Get the `name` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechSynthesisEventInit`*"]
    fn name(&self) -> String;
    #[cfg(feature = "SpeechSynthesisUtterance")]
    #[doc = "Get the `utterance` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechSynthesisEventInit`, `SpeechSynthesisUtterance`*"]
    fn utterance(&self) -> SpeechSynthesisUtterance;
}
impl SpeechSynthesisEventInitGetters for SpeechSynthesisEventInit {
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
}
impl SpeechSynthesisEventInit {
    #[cfg(feature = "SpeechSynthesisUtterance")]
    #[doc = "Construct a new `SpeechSynthesisEventInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechSynthesisEventInit`, `SpeechSynthesisUtterance`*"]
    pub fn new(utterance: &SpeechSynthesisUtterance) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.utterance(utterance);
        ret
    }
    #[doc = "Change the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechSynthesisEventInit`*"]
    pub fn bubbles(&mut self, val: bool) -> &mut Self {
        self.set_bubbles_shim(val);
        self
    }
    #[doc = "Change the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechSynthesisEventInit`*"]
    pub fn cancelable(&mut self, val: bool) -> &mut Self {
        self.set_cancelable_shim(val);
        self
    }
    #[doc = "Change the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechSynthesisEventInit`*"]
    pub fn composed(&mut self, val: bool) -> &mut Self {
        self.set_composed_shim(val);
        self
    }
    #[doc = "Change the `charIndex` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechSynthesisEventInit`*"]
    pub fn char_index(&mut self, val: u32) -> &mut Self {
        self.set_char_index_shim(val);
        self
    }
    #[doc = "Change the `charLength` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechSynthesisEventInit`*"]
    pub fn char_length(&mut self, val: Option<u32>) -> &mut Self {
        self.set_char_length_shim(val);
        self
    }
    #[doc = "Change the `elapsedTime` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechSynthesisEventInit`*"]
    pub fn elapsed_time(&mut self, val: f32) -> &mut Self {
        self.set_elapsed_time_shim(val);
        self
    }
    #[doc = "Change the `name` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechSynthesisEventInit`*"]
    pub fn name(&mut self, val: &str) -> &mut Self {
        self.set_name_shim(val);
        self
    }
    #[cfg(feature = "SpeechSynthesisUtterance")]
    #[doc = "Change the `utterance` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechSynthesisEventInit`, `SpeechSynthesisUtterance`*"]
    pub fn utterance(&mut self, val: &SpeechSynthesisUtterance) -> &mut Self {
        self.set_utterance_shim(val);
        self
    }
}
