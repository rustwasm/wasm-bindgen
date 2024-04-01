#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = GainOptions)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `GainOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GainOptions`*"]
    pub type GainOptions;
    #[wasm_bindgen(method, setter = "channelCount")]
    fn channel_count_shim(this: &GainOptions, val: u32);
    #[cfg(feature = "ChannelCountMode")]
    #[wasm_bindgen(method, setter = "channelCountMode")]
    fn channel_count_mode_shim(this: &GainOptions, val: ChannelCountMode);
    #[cfg(feature = "ChannelInterpretation")]
    #[wasm_bindgen(method, setter = "channelInterpretation")]
    fn channel_interpretation_shim(this: &GainOptions, val: ChannelInterpretation);
    #[wasm_bindgen(method, setter = "gain")]
    fn gain_shim(this: &GainOptions, val: f32);
}
impl GainOptions {
    #[doc = "Construct a new `GainOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GainOptions`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `channelCount` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GainOptions`*"]
    pub fn channel_count(&mut self, val: u32) -> &mut Self {
        self.channel_count_shim(val);
        self
    }
    #[cfg(feature = "ChannelCountMode")]
    #[doc = "Change the `channelCountMode` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ChannelCountMode`, `GainOptions`*"]
    pub fn channel_count_mode(&mut self, val: ChannelCountMode) -> &mut Self {
        self.channel_count_mode_shim(val);
        self
    }
    #[cfg(feature = "ChannelInterpretation")]
    #[doc = "Change the `channelInterpretation` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ChannelInterpretation`, `GainOptions`*"]
    pub fn channel_interpretation(&mut self, val: ChannelInterpretation) -> &mut Self {
        self.channel_interpretation_shim(val);
        self
    }
    #[doc = "Change the `gain` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GainOptions`*"]
    pub fn gain(&mut self, val: f32) -> &mut Self {
        self.gain_shim(val);
        self
    }
}
impl Default for GainOptions {
    fn default() -> Self {
        Self::new()
    }
}
