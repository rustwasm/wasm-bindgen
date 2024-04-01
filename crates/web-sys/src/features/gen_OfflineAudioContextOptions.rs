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
    #[wasm_bindgen(method, setter = "length")]
    fn length_shim(this: &OfflineAudioContextOptions, val: u32);
    #[wasm_bindgen(method, setter = "numberOfChannels")]
    fn number_of_channels_shim(this: &OfflineAudioContextOptions, val: u32);
    #[wasm_bindgen(method, setter = "sampleRate")]
    fn sample_rate_shim(this: &OfflineAudioContextOptions, val: f32);
}
impl OfflineAudioContextOptions {
    #[doc = "Construct a new `OfflineAudioContextOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OfflineAudioContextOptions`*"]
    pub fn new(length: u32, sample_rate: f32) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.length(length);
        ret.sample_rate(sample_rate);
        ret
    }
    #[doc = "Change the `length` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OfflineAudioContextOptions`*"]
    pub fn length(&mut self, val: u32) -> &mut Self {
        self.length_shim(val);
        self
    }
    #[doc = "Change the `numberOfChannels` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OfflineAudioContextOptions`*"]
    pub fn number_of_channels(&mut self, val: u32) -> &mut Self {
        self.number_of_channels_shim(val);
        self
    }
    #[doc = "Change the `sampleRate` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OfflineAudioContextOptions`*"]
    pub fn sample_rate(&mut self, val: f32) -> &mut Self {
        self.sample_rate_shim(val);
        self
    }
}
