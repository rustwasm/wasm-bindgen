#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = DynamicsCompressorOptions)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `DynamicsCompressorOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DynamicsCompressorOptions`*"]
    pub type DynamicsCompressorOptions;
    #[wasm_bindgen(method, setter = "channelCount")]
    fn channel_count_shim(this: &DynamicsCompressorOptions, val: u32);
    #[cfg(feature = "ChannelCountMode")]
    #[wasm_bindgen(method, setter = "channelCountMode")]
    fn channel_count_mode_shim(this: &DynamicsCompressorOptions, val: ChannelCountMode);
    #[cfg(feature = "ChannelInterpretation")]
    #[wasm_bindgen(method, setter = "channelInterpretation")]
    fn channel_interpretation_shim(this: &DynamicsCompressorOptions, val: ChannelInterpretation);
    #[wasm_bindgen(method, setter = "attack")]
    fn attack_shim(this: &DynamicsCompressorOptions, val: f32);
    #[wasm_bindgen(method, setter = "knee")]
    fn knee_shim(this: &DynamicsCompressorOptions, val: f32);
    #[wasm_bindgen(method, setter = "ratio")]
    fn ratio_shim(this: &DynamicsCompressorOptions, val: f32);
    #[wasm_bindgen(method, setter = "release")]
    fn release_shim(this: &DynamicsCompressorOptions, val: f32);
    #[wasm_bindgen(method, setter = "threshold")]
    fn threshold_shim(this: &DynamicsCompressorOptions, val: f32);
}
impl DynamicsCompressorOptions {
    #[doc = "Construct a new `DynamicsCompressorOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DynamicsCompressorOptions`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `channelCount` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DynamicsCompressorOptions`*"]
    pub fn channel_count(&mut self, val: u32) -> &mut Self {
        self.channel_count_shim(val);
        self
    }
    #[cfg(feature = "ChannelCountMode")]
    #[doc = "Change the `channelCountMode` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ChannelCountMode`, `DynamicsCompressorOptions`*"]
    pub fn channel_count_mode(&mut self, val: ChannelCountMode) -> &mut Self {
        self.channel_count_mode_shim(val);
        self
    }
    #[cfg(feature = "ChannelInterpretation")]
    #[doc = "Change the `channelInterpretation` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ChannelInterpretation`, `DynamicsCompressorOptions`*"]
    pub fn channel_interpretation(&mut self, val: ChannelInterpretation) -> &mut Self {
        self.channel_interpretation_shim(val);
        self
    }
    #[doc = "Change the `attack` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DynamicsCompressorOptions`*"]
    pub fn attack(&mut self, val: f32) -> &mut Self {
        self.attack_shim(val);
        self
    }
    #[doc = "Change the `knee` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DynamicsCompressorOptions`*"]
    pub fn knee(&mut self, val: f32) -> &mut Self {
        self.knee_shim(val);
        self
    }
    #[doc = "Change the `ratio` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DynamicsCompressorOptions`*"]
    pub fn ratio(&mut self, val: f32) -> &mut Self {
        self.ratio_shim(val);
        self
    }
    #[doc = "Change the `release` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DynamicsCompressorOptions`*"]
    pub fn release(&mut self, val: f32) -> &mut Self {
        self.release_shim(val);
        self
    }
    #[doc = "Change the `threshold` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DynamicsCompressorOptions`*"]
    pub fn threshold(&mut self, val: f32) -> &mut Self {
        self.threshold_shim(val);
        self
    }
}
impl Default for DynamicsCompressorOptions {
    fn default() -> Self {
        Self::new()
    }
}
