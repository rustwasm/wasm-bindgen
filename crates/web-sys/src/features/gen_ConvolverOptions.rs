#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = ConvolverOptions)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ConvolverOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConvolverOptions`*"]
    pub type ConvolverOptions;
    #[wasm_bindgen(method, getter = "channelCount")]
    fn channel_count_shim(this: &ConvolverOptions) -> u32;
    #[wasm_bindgen(method, setter = "channelCount")]
    fn set_channel_count_shim(this: &ConvolverOptions, val: u32);
    #[cfg(feature = "ChannelCountMode")]
    #[wasm_bindgen(method, getter = "channelCountMode")]
    fn channel_count_mode_shim(this: &ConvolverOptions) -> ChannelCountMode;
    #[cfg(feature = "ChannelCountMode")]
    #[wasm_bindgen(method, setter = "channelCountMode")]
    fn set_channel_count_mode_shim(this: &ConvolverOptions, val: ChannelCountMode);
    #[cfg(feature = "ChannelInterpretation")]
    #[wasm_bindgen(method, getter = "channelInterpretation")]
    fn channel_interpretation_shim(this: &ConvolverOptions) -> ChannelInterpretation;
    #[cfg(feature = "ChannelInterpretation")]
    #[wasm_bindgen(method, setter = "channelInterpretation")]
    fn set_channel_interpretation_shim(this: &ConvolverOptions, val: ChannelInterpretation);
    #[cfg(feature = "AudioBuffer")]
    #[wasm_bindgen(method, getter = "buffer")]
    fn buffer_shim(this: &ConvolverOptions) -> Option<&AudioBuffer>;
    #[cfg(feature = "AudioBuffer")]
    #[wasm_bindgen(method, setter = "buffer")]
    fn set_buffer_shim(this: &ConvolverOptions, val: Option<&AudioBuffer>);
    #[wasm_bindgen(method, getter = "disableNormalization")]
    fn disable_normalization_shim(this: &ConvolverOptions) -> bool;
    #[wasm_bindgen(method, setter = "disableNormalization")]
    fn set_disable_normalization_shim(this: &ConvolverOptions, val: bool);
}
#[doc = "The trait to access properties on the `ConvolverOptions` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `ConvolverOptions`*"]
pub trait ConvolverOptionsGetters {
    #[doc = "Get the `channelCount` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConvolverOptions`*"]
    fn channel_count(&self) -> u32;
    #[cfg(feature = "ChannelCountMode")]
    #[doc = "Get the `channelCountMode` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ChannelCountMode`, `ConvolverOptions`*"]
    fn channel_count_mode(&self) -> ChannelCountMode;
    #[cfg(feature = "ChannelInterpretation")]
    #[doc = "Get the `channelInterpretation` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ChannelInterpretation`, `ConvolverOptions`*"]
    fn channel_interpretation(&self) -> ChannelInterpretation;
    #[cfg(feature = "AudioBuffer")]
    #[doc = "Get the `buffer` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioBuffer`, `ConvolverOptions`*"]
    fn buffer(&self) -> Option<&AudioBuffer>;
    #[doc = "Get the `disableNormalization` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConvolverOptions`*"]
    fn disable_normalization(&self) -> bool;
}
impl ConvolverOptionsGetters for ConvolverOptions {
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
    #[cfg(feature = "AudioBuffer")]
    fn buffer(&self) -> Option<&AudioBuffer> {
        self.buffer_shim()
    }
    fn disable_normalization(&self) -> bool {
        self.disable_normalization_shim()
    }
}
impl ConvolverOptions {
    #[doc = "Construct a new `ConvolverOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConvolverOptions`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `channelCount` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConvolverOptions`*"]
    pub fn channel_count(&mut self, val: u32) -> &mut Self {
        self.set_channel_count_shim(val);
        self
    }
    #[cfg(feature = "ChannelCountMode")]
    #[doc = "Change the `channelCountMode` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ChannelCountMode`, `ConvolverOptions`*"]
    pub fn channel_count_mode(&mut self, val: ChannelCountMode) -> &mut Self {
        self.set_channel_count_mode_shim(val);
        self
    }
    #[cfg(feature = "ChannelInterpretation")]
    #[doc = "Change the `channelInterpretation` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ChannelInterpretation`, `ConvolverOptions`*"]
    pub fn channel_interpretation(&mut self, val: ChannelInterpretation) -> &mut Self {
        self.set_channel_interpretation_shim(val);
        self
    }
    #[cfg(feature = "AudioBuffer")]
    #[doc = "Change the `buffer` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioBuffer`, `ConvolverOptions`*"]
    pub fn buffer(&mut self, val: Option<&AudioBuffer>) -> &mut Self {
        self.set_buffer_shim(val);
        self
    }
    #[doc = "Change the `disableNormalization` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConvolverOptions`*"]
    pub fn disable_normalization(&mut self, val: bool) -> &mut Self {
        self.set_disable_normalization_shim(val);
        self
    }
}
impl Default for ConvolverOptions {
    fn default() -> Self {
        Self::new()
    }
}
