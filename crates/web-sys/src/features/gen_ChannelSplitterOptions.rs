#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = ChannelSplitterOptions)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ChannelSplitterOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ChannelSplitterOptions`*"]
    pub type ChannelSplitterOptions;
    #[wasm_bindgen(method, setter = "channelCount")]
    fn channel_count_shim(this: &ChannelSplitterOptions, val: u32);
    #[cfg(feature = "ChannelCountMode")]
    #[wasm_bindgen(method, setter = "channelCountMode")]
    fn channel_count_mode_shim(this: &ChannelSplitterOptions, val: ChannelCountMode);
    #[cfg(feature = "ChannelInterpretation")]
    #[wasm_bindgen(method, setter = "channelInterpretation")]
    fn channel_interpretation_shim(this: &ChannelSplitterOptions, val: ChannelInterpretation);
    #[wasm_bindgen(method, setter = "numberOfOutputs")]
    fn number_of_outputs_shim(this: &ChannelSplitterOptions, val: u32);
}
impl ChannelSplitterOptions {
    #[doc = "Construct a new `ChannelSplitterOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ChannelSplitterOptions`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `channelCount` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ChannelSplitterOptions`*"]
    pub fn channel_count(&mut self, val: u32) -> &mut Self {
        self.channel_count_shim(val);
        self
    }
    #[cfg(feature = "ChannelCountMode")]
    #[doc = "Change the `channelCountMode` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ChannelCountMode`, `ChannelSplitterOptions`*"]
    pub fn channel_count_mode(&mut self, val: ChannelCountMode) -> &mut Self {
        self.channel_count_mode_shim(val);
        self
    }
    #[cfg(feature = "ChannelInterpretation")]
    #[doc = "Change the `channelInterpretation` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ChannelInterpretation`, `ChannelSplitterOptions`*"]
    pub fn channel_interpretation(&mut self, val: ChannelInterpretation) -> &mut Self {
        self.channel_interpretation_shim(val);
        self
    }
    #[doc = "Change the `numberOfOutputs` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ChannelSplitterOptions`*"]
    pub fn number_of_outputs(&mut self, val: u32) -> &mut Self {
        self.number_of_outputs_shim(val);
        self
    }
}
impl Default for ChannelSplitterOptions {
    fn default() -> Self {
        Self::new()
    }
}
