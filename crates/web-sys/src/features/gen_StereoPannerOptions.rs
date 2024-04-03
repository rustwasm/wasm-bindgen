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
    #[wasm_bindgen(method, setter = "channelCount")]
    fn channel_count_shim(this: &StereoPannerOptions, val: u32);
    #[cfg(feature = "ChannelCountMode")]
    #[wasm_bindgen(method, setter = "channelCountMode")]
    fn channel_count_mode_shim(this: &StereoPannerOptions, val: ChannelCountMode);
    #[cfg(feature = "ChannelInterpretation")]
    #[wasm_bindgen(method, setter = "channelInterpretation")]
    fn channel_interpretation_shim(this: &StereoPannerOptions, val: ChannelInterpretation);
    #[wasm_bindgen(method, setter = "pan")]
    fn pan_shim(this: &StereoPannerOptions, val: f32);
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
        self.channel_count_shim(val);
        self
    }
    #[cfg(feature = "ChannelCountMode")]
    #[doc = "Change the `channelCountMode` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ChannelCountMode`, `StereoPannerOptions`*"]
    pub fn channel_count_mode(&mut self, val: ChannelCountMode) -> &mut Self {
        self.channel_count_mode_shim(val);
        self
    }
    #[cfg(feature = "ChannelInterpretation")]
    #[doc = "Change the `channelInterpretation` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ChannelInterpretation`, `StereoPannerOptions`*"]
    pub fn channel_interpretation(&mut self, val: ChannelInterpretation) -> &mut Self {
        self.channel_interpretation_shim(val);
        self
    }
    #[doc = "Change the `pan` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `StereoPannerOptions`*"]
    pub fn pan(&mut self, val: f32) -> &mut Self {
        self.pan_shim(val);
        self
    }
}
impl Default for StereoPannerOptions {
    fn default() -> Self {
        Self::new()
    }
}
