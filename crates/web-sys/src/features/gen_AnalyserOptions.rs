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
    #[doc = "Get the `channelCount` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnalyserOptions`*"]
    #[wasm_bindgen(method, getter = "channelCount")]
    pub fn get_channel_count(this: &AnalyserOptions) -> Option<u32>;
    #[wasm_bindgen(method, setter = "channelCount")]
    fn set_channel_count(this: &AnalyserOptions, val: u32);
    #[cfg(feature = "ChannelCountMode")]
    #[doc = "Get the `channelCountMode` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnalyserOptions`, `ChannelCountMode`*"]
    #[wasm_bindgen(method, getter = "channelCountMode")]
    pub fn get_channel_count_mode(this: &AnalyserOptions) -> Option<ChannelCountMode>;
    #[cfg(feature = "ChannelCountMode")]
    #[wasm_bindgen(method, setter = "channelCountMode")]
    fn set_channel_count_mode(this: &AnalyserOptions, val: ChannelCountMode);
    #[cfg(feature = "ChannelInterpretation")]
    #[doc = "Get the `channelInterpretation` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnalyserOptions`, `ChannelInterpretation`*"]
    #[wasm_bindgen(method, getter = "channelInterpretation")]
    pub fn get_channel_interpretation(this: &AnalyserOptions) -> Option<ChannelInterpretation>;
    #[cfg(feature = "ChannelInterpretation")]
    #[wasm_bindgen(method, setter = "channelInterpretation")]
    fn set_channel_interpretation(this: &AnalyserOptions, val: ChannelInterpretation);
    #[doc = "Get the `fftSize` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnalyserOptions`*"]
    #[wasm_bindgen(method, getter = "fftSize")]
    pub fn get_fft_size(this: &AnalyserOptions) -> Option<u32>;
    #[wasm_bindgen(method, setter = "fftSize")]
    fn set_fft_size(this: &AnalyserOptions, val: u32);
    #[doc = "Get the `maxDecibels` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnalyserOptions`*"]
    #[wasm_bindgen(method, getter = "maxDecibels")]
    pub fn get_max_decibels(this: &AnalyserOptions) -> Option<f64>;
    #[wasm_bindgen(method, setter = "maxDecibels")]
    fn set_max_decibels(this: &AnalyserOptions, val: f64);
    #[doc = "Get the `minDecibels` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnalyserOptions`*"]
    #[wasm_bindgen(method, getter = "minDecibels")]
    pub fn get_min_decibels(this: &AnalyserOptions) -> Option<f64>;
    #[wasm_bindgen(method, setter = "minDecibels")]
    fn set_min_decibels(this: &AnalyserOptions, val: f64);
    #[doc = "Get the `smoothingTimeConstant` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnalyserOptions`*"]
    #[wasm_bindgen(method, getter = "smoothingTimeConstant")]
    pub fn get_smoothing_time_constant(this: &AnalyserOptions) -> Option<f64>;
    #[wasm_bindgen(method, setter = "smoothingTimeConstant")]
    fn set_smoothing_time_constant(this: &AnalyserOptions, val: f64);
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
        self.set_channel_count(val);
        self
    }
    #[cfg(feature = "ChannelCountMode")]
    #[doc = "Change the `channelCountMode` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnalyserOptions`, `ChannelCountMode`*"]
    pub fn channel_count_mode(&mut self, val: ChannelCountMode) -> &mut Self {
        self.set_channel_count_mode(val);
        self
    }
    #[cfg(feature = "ChannelInterpretation")]
    #[doc = "Change the `channelInterpretation` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnalyserOptions`, `ChannelInterpretation`*"]
    pub fn channel_interpretation(&mut self, val: ChannelInterpretation) -> &mut Self {
        self.set_channel_interpretation(val);
        self
    }
    #[doc = "Change the `fftSize` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnalyserOptions`*"]
    pub fn fft_size(&mut self, val: u32) -> &mut Self {
        self.set_fft_size(val);
        self
    }
    #[doc = "Change the `maxDecibels` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnalyserOptions`*"]
    pub fn max_decibels(&mut self, val: f64) -> &mut Self {
        self.set_max_decibels(val);
        self
    }
    #[doc = "Change the `minDecibels` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnalyserOptions`*"]
    pub fn min_decibels(&mut self, val: f64) -> &mut Self {
        self.set_min_decibels(val);
        self
    }
    #[doc = "Change the `smoothingTimeConstant` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnalyserOptions`*"]
    pub fn smoothing_time_constant(&mut self, val: f64) -> &mut Self {
        self.set_smoothing_time_constant(val);
        self
    }
}
impl Default for AnalyserOptions {
    fn default() -> Self {
        Self::new()
    }
}
