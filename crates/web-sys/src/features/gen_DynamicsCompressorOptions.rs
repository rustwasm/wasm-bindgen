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
    #[wasm_bindgen(method, getter = "channelCount")]
    fn channel_count_shim(this: &DynamicsCompressorOptions) -> u32;
    #[wasm_bindgen(method, setter = "channelCount")]
    fn set_channel_count_shim(this: &DynamicsCompressorOptions, val: u32);
    #[cfg(feature = "ChannelCountMode")]
    #[wasm_bindgen(method, getter = "channelCountMode")]
    fn channel_count_mode_shim(this: &DynamicsCompressorOptions) -> ChannelCountMode;
    #[cfg(feature = "ChannelCountMode")]
    #[wasm_bindgen(method, setter = "channelCountMode")]
    fn set_channel_count_mode_shim(this: &DynamicsCompressorOptions, val: ChannelCountMode);
    #[cfg(feature = "ChannelInterpretation")]
    #[wasm_bindgen(method, getter = "channelInterpretation")]
    fn channel_interpretation_shim(this: &DynamicsCompressorOptions) -> ChannelInterpretation;
    #[cfg(feature = "ChannelInterpretation")]
    #[wasm_bindgen(method, setter = "channelInterpretation")]
    fn set_channel_interpretation_shim(
        this: &DynamicsCompressorOptions,
        val: ChannelInterpretation,
    );
    #[wasm_bindgen(method, getter = "attack")]
    fn attack_shim(this: &DynamicsCompressorOptions) -> f32;
    #[wasm_bindgen(method, setter = "attack")]
    fn set_attack_shim(this: &DynamicsCompressorOptions, val: f32);
    #[wasm_bindgen(method, getter = "knee")]
    fn knee_shim(this: &DynamicsCompressorOptions) -> f32;
    #[wasm_bindgen(method, setter = "knee")]
    fn set_knee_shim(this: &DynamicsCompressorOptions, val: f32);
    #[wasm_bindgen(method, getter = "ratio")]
    fn ratio_shim(this: &DynamicsCompressorOptions) -> f32;
    #[wasm_bindgen(method, setter = "ratio")]
    fn set_ratio_shim(this: &DynamicsCompressorOptions, val: f32);
    #[wasm_bindgen(method, getter = "release")]
    fn release_shim(this: &DynamicsCompressorOptions) -> f32;
    #[wasm_bindgen(method, setter = "release")]
    fn set_release_shim(this: &DynamicsCompressorOptions, val: f32);
    #[wasm_bindgen(method, getter = "threshold")]
    fn threshold_shim(this: &DynamicsCompressorOptions) -> f32;
    #[wasm_bindgen(method, setter = "threshold")]
    fn set_threshold_shim(this: &DynamicsCompressorOptions, val: f32);
}
#[doc = "The trait to access properties on the `DynamicsCompressorOptions` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `DynamicsCompressorOptions`*"]
pub trait DynamicsCompressorOptionsGetters {
    #[doc = "Get the `channelCount` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DynamicsCompressorOptions`*"]
    fn channel_count(&self) -> u32;
    #[cfg(feature = "ChannelCountMode")]
    #[doc = "Get the `channelCountMode` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ChannelCountMode`, `DynamicsCompressorOptions`*"]
    fn channel_count_mode(&self) -> ChannelCountMode;
    #[cfg(feature = "ChannelInterpretation")]
    #[doc = "Get the `channelInterpretation` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ChannelInterpretation`, `DynamicsCompressorOptions`*"]
    fn channel_interpretation(&self) -> ChannelInterpretation;
    #[doc = "Get the `attack` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DynamicsCompressorOptions`*"]
    fn attack(&self) -> f32;
    #[doc = "Get the `knee` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DynamicsCompressorOptions`*"]
    fn knee(&self) -> f32;
    #[doc = "Get the `ratio` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DynamicsCompressorOptions`*"]
    fn ratio(&self) -> f32;
    #[doc = "Get the `release` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DynamicsCompressorOptions`*"]
    fn release(&self) -> f32;
    #[doc = "Get the `threshold` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DynamicsCompressorOptions`*"]
    fn threshold(&self) -> f32;
}
impl DynamicsCompressorOptionsGetters for DynamicsCompressorOptions {
    fn channel_count(&self) -> u32 {
        self.channel_count_shim()
    }
    #[cfg(feature = "ChannelCountMode")]
    fn channel_count_mode(&self) -> ChannelCountMode {
        self.channel_count_mode_shim()
    }
    #[cfg(feature = "ChannelInterpretation")]
    fn channel_interpretation(&self) -> ChannelInterpretation {
        self.channel_interpretation_shim()
    }
    fn attack(&self) -> f32 {
        self.attack_shim()
    }
    fn knee(&self) -> f32 {
        self.knee_shim()
    }
    fn ratio(&self) -> f32 {
        self.ratio_shim()
    }
    fn release(&self) -> f32 {
        self.release_shim()
    }
    fn threshold(&self) -> f32 {
        self.threshold_shim()
    }
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
        self.set_channel_count_shim(val);
        self
    }
    #[cfg(feature = "ChannelCountMode")]
    #[doc = "Change the `channelCountMode` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ChannelCountMode`, `DynamicsCompressorOptions`*"]
    pub fn channel_count_mode(&mut self, val: ChannelCountMode) -> &mut Self {
        self.set_channel_count_mode_shim(val);
        self
    }
    #[cfg(feature = "ChannelInterpretation")]
    #[doc = "Change the `channelInterpretation` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ChannelInterpretation`, `DynamicsCompressorOptions`*"]
    pub fn channel_interpretation(&mut self, val: ChannelInterpretation) -> &mut Self {
        self.set_channel_interpretation_shim(val);
        self
    }
    #[doc = "Change the `attack` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DynamicsCompressorOptions`*"]
    pub fn attack(&mut self, val: f32) -> &mut Self {
        self.set_attack_shim(val);
        self
    }
    #[doc = "Change the `knee` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DynamicsCompressorOptions`*"]
    pub fn knee(&mut self, val: f32) -> &mut Self {
        self.set_knee_shim(val);
        self
    }
    #[doc = "Change the `ratio` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DynamicsCompressorOptions`*"]
    pub fn ratio(&mut self, val: f32) -> &mut Self {
        self.set_ratio_shim(val);
        self
    }
    #[doc = "Change the `release` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DynamicsCompressorOptions`*"]
    pub fn release(&mut self, val: f32) -> &mut Self {
        self.set_release_shim(val);
        self
    }
    #[doc = "Change the `threshold` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DynamicsCompressorOptions`*"]
    pub fn threshold(&mut self, val: f32) -> &mut Self {
        self.set_threshold_shim(val);
        self
    }
}
impl Default for DynamicsCompressorOptions {
    fn default() -> Self {
        Self::new()
    }
}
