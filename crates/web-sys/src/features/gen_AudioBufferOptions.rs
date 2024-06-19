#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = AudioBufferOptions)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `AudioBufferOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioBufferOptions`*"]
    pub type AudioBufferOptions;
    #[doc = "Get the `length` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioBufferOptions`*"]
    #[wasm_bindgen(method, getter = "length")]
    pub fn get_length(this: &AudioBufferOptions) -> u32;
    #[wasm_bindgen(method, setter = "length")]
    fn set_length(this: &AudioBufferOptions, val: u32);
    #[doc = "Get the `numberOfChannels` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioBufferOptions`*"]
    #[wasm_bindgen(method, getter = "numberOfChannels")]
    pub fn get_number_of_channels(this: &AudioBufferOptions) -> Option<u32>;
    #[wasm_bindgen(method, setter = "numberOfChannels")]
    fn set_number_of_channels(this: &AudioBufferOptions, val: u32);
    #[doc = "Get the `sampleRate` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioBufferOptions`*"]
    #[wasm_bindgen(method, getter = "sampleRate")]
    pub fn get_sample_rate(this: &AudioBufferOptions) -> f32;
    #[wasm_bindgen(method, setter = "sampleRate")]
    fn set_sample_rate(this: &AudioBufferOptions, val: f32);
}
impl AudioBufferOptions {
    #[doc = "Construct a new `AudioBufferOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioBufferOptions`*"]
    pub fn new(length: u32, sample_rate: f32) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.length(length);
        ret.sample_rate(sample_rate);
        ret
    }
    #[doc = "Change the `length` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioBufferOptions`*"]
    pub fn length(&mut self, val: u32) -> &mut Self {
        self.set_length(val);
        self
    }
    #[doc = "Change the `numberOfChannels` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioBufferOptions`*"]
    pub fn number_of_channels(&mut self, val: u32) -> &mut Self {
        self.set_number_of_channels(val);
        self
    }
    #[doc = "Change the `sampleRate` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioBufferOptions`*"]
    pub fn sample_rate(&mut self, val: f32) -> &mut Self {
        self.set_sample_rate(val);
        self
    }
}
