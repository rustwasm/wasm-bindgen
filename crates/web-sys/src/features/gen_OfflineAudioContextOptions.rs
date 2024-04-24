#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = OfflineAudioContextOptions)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `OfflineAudioContextOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OfflineAudioContextOptions`*"]
    pub type OfflineAudioContextOptions;
    #[wasm_bindgen(method, getter = "length")]
    fn length_shim(this: &OfflineAudioContextOptions) -> u32;
    #[wasm_bindgen(method, setter = "length")]
    fn set_length_shim(this: &OfflineAudioContextOptions, val: u32);
    #[wasm_bindgen(method, getter = "numberOfChannels")]
    fn number_of_channels_shim(this: &OfflineAudioContextOptions) -> u32;
    #[wasm_bindgen(method, setter = "numberOfChannels")]
    fn set_number_of_channels_shim(this: &OfflineAudioContextOptions, val: u32);
    #[wasm_bindgen(method, getter = "sampleRate")]
    fn sample_rate_shim(this: &OfflineAudioContextOptions) -> f32;
    #[wasm_bindgen(method, setter = "sampleRate")]
    fn set_sample_rate_shim(this: &OfflineAudioContextOptions, val: f32);
}
#[doc = "The trait to access properties on the `OfflineAudioContextOptions` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `OfflineAudioContextOptions`*"]
pub trait OfflineAudioContextOptionsGetters {
    #[doc = "Get the `length` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OfflineAudioContextOptions`*"]
    fn length(&self) -> u32;
    #[doc = "Get the `numberOfChannels` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OfflineAudioContextOptions`*"]
    fn number_of_channels(&self) -> u32;
    #[doc = "Get the `sampleRate` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OfflineAudioContextOptions`*"]
    fn sample_rate(&self) -> f32;
}
impl OfflineAudioContextOptionsGetters for OfflineAudioContextOptions {
    fn length(&self) -> u32 {
        self.length_shim()
    }
    fn number_of_channels(&self) -> u32 {
        self.number_of_channels_shim()
    }
    fn sample_rate(&self) -> f32 {
        self.sample_rate_shim()
    }
}
impl OfflineAudioContextOptions {
    #[doc = "Construct a new `OfflineAudioContextOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OfflineAudioContextOptions`*"]
    pub fn new(length: u32, sample_rate: f32) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        Self::length(&mut ret, length);
        Self::sample_rate(&mut ret, sample_rate);
        ret
    }
    #[doc = "Change the `length` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OfflineAudioContextOptions`*"]
    pub fn length(&mut self, val: u32) -> &mut Self {
        self.set_length_shim(val);
        self
    }
    #[doc = "Change the `numberOfChannels` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OfflineAudioContextOptions`*"]
    pub fn number_of_channels(&mut self, val: u32) -> &mut Self {
        self.set_number_of_channels_shim(val);
        self
    }
    #[doc = "Change the `sampleRate` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OfflineAudioContextOptions`*"]
    pub fn sample_rate(&mut self, val: f32) -> &mut Self {
        self.set_sample_rate_shim(val);
        self
    }
}
