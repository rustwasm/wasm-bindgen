#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = OfflineAudioCompletionEventInit)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `OfflineAudioCompletionEventInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OfflineAudioCompletionEventInit`*"]
    pub type OfflineAudioCompletionEventInit;
    #[wasm_bindgen(method, getter = "bubbles")]
    fn bubbles_shim(this: &OfflineAudioCompletionEventInit) -> bool;
    #[wasm_bindgen(method, setter = "bubbles")]
    fn set_bubbles_shim(this: &OfflineAudioCompletionEventInit, val: bool);
    #[wasm_bindgen(method, getter = "cancelable")]
    fn cancelable_shim(this: &OfflineAudioCompletionEventInit) -> bool;
    #[wasm_bindgen(method, setter = "cancelable")]
    fn set_cancelable_shim(this: &OfflineAudioCompletionEventInit, val: bool);
    #[wasm_bindgen(method, getter = "composed")]
    fn composed_shim(this: &OfflineAudioCompletionEventInit) -> bool;
    #[wasm_bindgen(method, setter = "composed")]
    fn set_composed_shim(this: &OfflineAudioCompletionEventInit, val: bool);
    #[cfg(feature = "AudioBuffer")]
    #[wasm_bindgen(method, getter = "renderedBuffer")]
    fn rendered_buffer_shim(this: &OfflineAudioCompletionEventInit) -> AudioBuffer;
    #[cfg(feature = "AudioBuffer")]
    #[wasm_bindgen(method, setter = "renderedBuffer")]
    fn set_rendered_buffer_shim(this: &OfflineAudioCompletionEventInit, val: &AudioBuffer);
}
#[doc = "The trait to access properties on the `OfflineAudioCompletionEventInit` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `OfflineAudioCompletionEventInit`*"]
pub trait OfflineAudioCompletionEventInitGetters {
    #[doc = "Get the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OfflineAudioCompletionEventInit`*"]
    fn bubbles(&self) -> bool;
    #[doc = "Get the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OfflineAudioCompletionEventInit`*"]
    fn cancelable(&self) -> bool;
    #[doc = "Get the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OfflineAudioCompletionEventInit`*"]
    fn composed(&self) -> bool;
    #[cfg(feature = "AudioBuffer")]
    #[doc = "Get the `renderedBuffer` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioBuffer`, `OfflineAudioCompletionEventInit`*"]
    fn rendered_buffer(&self) -> AudioBuffer;
}
impl OfflineAudioCompletionEventInitGetters for OfflineAudioCompletionEventInit {
    fn bubbles(&self) -> bool {
        self.bubbles_shim()
    }
    fn cancelable(&self) -> bool {
        self.cancelable_shim()
    }
    fn composed(&self) -> bool {
        self.composed_shim()
    }
    #[cfg(feature = "AudioBuffer")]
    fn rendered_buffer(&self) -> AudioBuffer {
        self.rendered_buffer_shim()
    }
}
impl OfflineAudioCompletionEventInit {
    #[cfg(feature = "AudioBuffer")]
    #[doc = "Construct a new `OfflineAudioCompletionEventInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioBuffer`, `OfflineAudioCompletionEventInit`*"]
    pub fn new(rendered_buffer: &AudioBuffer) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        Self::rendered_buffer(&mut ret, rendered_buffer);
        ret
    }
    #[doc = "Change the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OfflineAudioCompletionEventInit`*"]
    pub fn bubbles(&mut self, val: bool) -> &mut Self {
        self.set_bubbles_shim(val);
        self
    }
    #[doc = "Change the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OfflineAudioCompletionEventInit`*"]
    pub fn cancelable(&mut self, val: bool) -> &mut Self {
        self.set_cancelable_shim(val);
        self
    }
    #[doc = "Change the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OfflineAudioCompletionEventInit`*"]
    pub fn composed(&mut self, val: bool) -> &mut Self {
        self.set_composed_shim(val);
        self
    }
    #[cfg(feature = "AudioBuffer")]
    #[doc = "Change the `renderedBuffer` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioBuffer`, `OfflineAudioCompletionEventInit`*"]
    pub fn rendered_buffer(&mut self, val: &AudioBuffer) -> &mut Self {
        self.set_rendered_buffer_shim(val);
        self
    }
}
