#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = AnalyserOptions)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `AnalyserOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnalyserOptions`*"]
    pub type AnalyserOptions;
    #[wasm_bindgen(method, getter = "channelCount")]
    fn channel_count_shim(this: &AnalyserOptions) -> u32;
    #[wasm_bindgen(method, setter = "channelCount")]
    fn set_channel_count_shim(this: &AnalyserOptions, val: u32);
    #[cfg(feature = "ChannelCountMode")]
    #[wasm_bindgen(method, getter = "channelCountMode")]
    fn channel_count_mode_shim(this: &AnalyserOptions) -> ChannelCountMode;
    #[cfg(feature = "ChannelCountMode")]
    #[wasm_bindgen(method, setter = "channelCountMode")]
    fn set_channel_count_mode_shim(this: &AnalyserOptions, val: ChannelCountMode);
    #[cfg(feature = "ChannelInterpretation")]
    #[wasm_bindgen(method, getter = "channelInterpretation")]
    fn channel_interpretation_shim(this: &AnalyserOptions) -> ChannelInterpretation;
    #[cfg(feature = "ChannelInterpretation")]
    #[wasm_bindgen(method, setter = "channelInterpretation")]
    fn set_channel_interpretation_shim(this: &AnalyserOptions, val: ChannelInterpretation);
    #[wasm_bindgen(method, getter = "fftSize")]
    fn fft_size_shim(this: &AnalyserOptions) -> u32;
    #[wasm_bindgen(method, setter = "fftSize")]
    fn set_fft_size_shim(this: &AnalyserOptions, val: u32);
    #[wasm_bindgen(method, getter = "maxDecibels")]
    fn max_decibels_shim(this: &AnalyserOptions) -> f64;
    #[wasm_bindgen(method, setter = "maxDecibels")]
    fn set_max_decibels_shim(this: &AnalyserOptions, val: f64);
    #[wasm_bindgen(method, getter = "minDecibels")]
    fn min_decibels_shim(this: &AnalyserOptions) -> f64;
    #[wasm_bindgen(method, setter = "minDecibels")]
    fn set_min_decibels_shim(this: &AnalyserOptions, val: f64);
    #[wasm_bindgen(method, getter = "smoothingTimeConstant")]
    fn smoothing_time_constant_shim(this: &AnalyserOptions) -> f64;
    #[wasm_bindgen(method, setter = "smoothingTimeConstant")]
    fn set_smoothing_time_constant_shim(this: &AnalyserOptions, val: f64);
}
#[doc = "The trait to access properties on the `AnalyserOptions` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `AnalyserOptions`*"]
pub trait AnalyserOptionsGetters {
    #[doc = "Get the `channelCount` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnalyserOptions`*"]
    fn channel_count(&self) -> u32;
    #[cfg(feature = "ChannelCountMode")]
    #[doc = "Get the `channelCountMode` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnalyserOptions`, `ChannelCountMode`*"]
    fn channel_count_mode(&self) -> ChannelCountMode;
    #[cfg(feature = "ChannelInterpretation")]
    #[doc = "Get the `channelInterpretation` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnalyserOptions`, `ChannelInterpretation`*"]
    fn channel_interpretation(&self) -> ChannelInterpretation;
    #[doc = "Get the `fftSize` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnalyserOptions`*"]
    fn fft_size(&self) -> u32;
    #[doc = "Get the `maxDecibels` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnalyserOptions`*"]
    fn max_decibels(&self) -> f64;
    #[doc = "Get the `minDecibels` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnalyserOptions`*"]
    fn min_decibels(&self) -> f64;
    #[doc = "Get the `smoothingTimeConstant` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnalyserOptions`*"]
    fn smoothing_time_constant(&self) -> f64;
}
impl AnalyserOptionsGetters for AnalyserOptions {
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
    fn fft_size(&self) -> u32 {
        self.fft_size_shim()
    }
    fn max_decibels(&self) -> f64 {
        self.max_decibels_shim()
    }
    fn min_decibels(&self) -> f64 {
        self.min_decibels_shim()
    }
    fn smoothing_time_constant(&self) -> f64 {
        self.smoothing_time_constant_shim()
    }
}
impl AnalyserOptions {
    #[doc = "Construct a new `AnalyserOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnalyserOptions`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `channelCount` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnalyserOptions`*"]
    pub fn channel_count(&mut self, val: u32) -> &mut Self {
        self.set_channel_count_shim(val);
        self
    }
    #[cfg(feature = "ChannelCountMode")]
    #[doc = "Change the `channelCountMode` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnalyserOptions`, `ChannelCountMode`*"]
    pub fn channel_count_mode(&mut self, val: ChannelCountMode) -> &mut Self {
        self.set_channel_count_mode_shim(val);
        self
    }
    #[cfg(feature = "ChannelInterpretation")]
    #[doc = "Change the `channelInterpretation` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnalyserOptions`, `ChannelInterpretation`*"]
    pub fn channel_interpretation(&mut self, val: ChannelInterpretation) -> &mut Self {
        self.set_channel_interpretation_shim(val);
        self
    }
    #[doc = "Change the `fftSize` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnalyserOptions`*"]
    pub fn fft_size(&mut self, val: u32) -> &mut Self {
        self.set_fft_size_shim(val);
        self
    }
    #[doc = "Change the `maxDecibels` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnalyserOptions`*"]
    pub fn max_decibels(&mut self, val: f64) -> &mut Self {
        self.set_max_decibels_shim(val);
        self
    }
    #[doc = "Change the `minDecibels` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnalyserOptions`*"]
    pub fn min_decibels(&mut self, val: f64) -> &mut Self {
        self.set_min_decibels_shim(val);
        self
    }
    #[doc = "Change the `smoothingTimeConstant` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnalyserOptions`*"]
    pub fn smoothing_time_constant(&mut self, val: f64) -> &mut Self {
        self.set_smoothing_time_constant_shim(val);
        self
    }
}
impl Default for AnalyserOptions {
    fn default() -> Self {
        Self::new()
    }
}
