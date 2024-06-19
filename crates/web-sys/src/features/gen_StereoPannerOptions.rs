#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = StereoPannerOptions)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `StereoPannerOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `StereoPannerOptions`*"]
    pub type StereoPannerOptions;
    #[doc = "Get the `channelCount` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `StereoPannerOptions`*"]
    #[wasm_bindgen(method, getter = "channelCount")]
    pub fn get_channel_count(this: &StereoPannerOptions) -> Option<u32>;
    #[wasm_bindgen(method, setter = "channelCount")]
    fn set_channel_count(this: &StereoPannerOptions, val: u32);
    #[cfg(feature = "ChannelCountMode")]
    #[doc = "Get the `channelCountMode` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ChannelCountMode`, `StereoPannerOptions`*"]
    #[wasm_bindgen(method, getter = "channelCountMode")]
    pub fn get_channel_count_mode(this: &StereoPannerOptions) -> Option<ChannelCountMode>;
    #[cfg(feature = "ChannelCountMode")]
    #[wasm_bindgen(method, setter = "channelCountMode")]
    fn set_channel_count_mode(this: &StereoPannerOptions, val: ChannelCountMode);
    #[cfg(feature = "ChannelInterpretation")]
    #[doc = "Get the `channelInterpretation` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ChannelInterpretation`, `StereoPannerOptions`*"]
    #[wasm_bindgen(method, getter = "channelInterpretation")]
    pub fn get_channel_interpretation(this: &StereoPannerOptions) -> Option<ChannelInterpretation>;
    #[cfg(feature = "ChannelInterpretation")]
    #[wasm_bindgen(method, setter = "channelInterpretation")]
    fn set_channel_interpretation(this: &StereoPannerOptions, val: ChannelInterpretation);
    #[doc = "Get the `pan` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `StereoPannerOptions`*"]
    #[wasm_bindgen(method, getter = "pan")]
    pub fn get_pan(this: &StereoPannerOptions) -> Option<f32>;
    #[wasm_bindgen(method, setter = "pan")]
    fn set_pan(this: &StereoPannerOptions, val: f32);
}
impl StereoPannerOptions {
    #[doc = "Construct a new `StereoPannerOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `StereoPannerOptions`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `channelCount` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `StereoPannerOptions`*"]
    pub fn channel_count(&mut self, val: u32) -> &mut Self {
        self.set_channel_count(val);
        self
    }
    #[cfg(feature = "ChannelCountMode")]
    #[doc = "Change the `channelCountMode` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ChannelCountMode`, `StereoPannerOptions`*"]
    pub fn channel_count_mode(&mut self, val: ChannelCountMode) -> &mut Self {
        self.set_channel_count_mode(val);
        self
    }
    #[cfg(feature = "ChannelInterpretation")]
    #[doc = "Change the `channelInterpretation` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ChannelInterpretation`, `StereoPannerOptions`*"]
    pub fn channel_interpretation(&mut self, val: ChannelInterpretation) -> &mut Self {
        self.set_channel_interpretation(val);
        self
    }
    #[doc = "Change the `pan` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `StereoPannerOptions`*"]
    pub fn pan(&mut self, val: f32) -> &mut Self {
        self.set_pan(val);
        self
    }
}
impl Default for StereoPannerOptions {
    fn default() -> Self {
        Self::new()
    }
}
