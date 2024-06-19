#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = IIRFilterOptions)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `IirFilterOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IirFilterOptions`*"]
    pub type IirFilterOptions;
    #[doc = "Get the `channelCount` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IirFilterOptions`*"]
    #[wasm_bindgen(method, getter = "channelCount")]
    pub fn get_channel_count(this: &IirFilterOptions) -> Option<u32>;
    #[wasm_bindgen(method, setter = "channelCount")]
    fn set_channel_count(this: &IirFilterOptions, val: u32);
    #[cfg(feature = "ChannelCountMode")]
    #[doc = "Get the `channelCountMode` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ChannelCountMode`, `IirFilterOptions`*"]
    #[wasm_bindgen(method, getter = "channelCountMode")]
    pub fn get_channel_count_mode(this: &IirFilterOptions) -> Option<ChannelCountMode>;
    #[cfg(feature = "ChannelCountMode")]
    #[wasm_bindgen(method, setter = "channelCountMode")]
    fn set_channel_count_mode(this: &IirFilterOptions, val: ChannelCountMode);
    #[cfg(feature = "ChannelInterpretation")]
    #[doc = "Get the `channelInterpretation` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ChannelInterpretation`, `IirFilterOptions`*"]
    #[wasm_bindgen(method, getter = "channelInterpretation")]
    pub fn get_channel_interpretation(this: &IirFilterOptions) -> Option<ChannelInterpretation>;
    #[cfg(feature = "ChannelInterpretation")]
    #[wasm_bindgen(method, setter = "channelInterpretation")]
    fn set_channel_interpretation(this: &IirFilterOptions, val: ChannelInterpretation);
    #[doc = "Get the `feedback` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IirFilterOptions`*"]
    #[wasm_bindgen(method, getter = "feedback")]
    pub fn get_feedback(this: &IirFilterOptions) -> ::js_sys::Array;
    #[wasm_bindgen(method, setter = "feedback")]
    fn set_feedback(this: &IirFilterOptions, val: &::wasm_bindgen::JsValue);
    #[doc = "Get the `feedforward` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IirFilterOptions`*"]
    #[wasm_bindgen(method, getter = "feedforward")]
    pub fn get_feedforward(this: &IirFilterOptions) -> ::js_sys::Array;
    #[wasm_bindgen(method, setter = "feedforward")]
    fn set_feedforward(this: &IirFilterOptions, val: &::wasm_bindgen::JsValue);
}
impl IirFilterOptions {
    #[doc = "Construct a new `IirFilterOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IirFilterOptions`*"]
    pub fn new(feedback: &::wasm_bindgen::JsValue, feedforward: &::wasm_bindgen::JsValue) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.feedback(feedback);
        ret.feedforward(feedforward);
        ret
    }
    #[doc = "Change the `channelCount` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IirFilterOptions`*"]
    pub fn channel_count(&mut self, val: u32) -> &mut Self {
        self.set_channel_count(val);
        self
    }
    #[cfg(feature = "ChannelCountMode")]
    #[doc = "Change the `channelCountMode` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ChannelCountMode`, `IirFilterOptions`*"]
    pub fn channel_count_mode(&mut self, val: ChannelCountMode) -> &mut Self {
        self.set_channel_count_mode(val);
        self
    }
    #[cfg(feature = "ChannelInterpretation")]
    #[doc = "Change the `channelInterpretation` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ChannelInterpretation`, `IirFilterOptions`*"]
    pub fn channel_interpretation(&mut self, val: ChannelInterpretation) -> &mut Self {
        self.set_channel_interpretation(val);
        self
    }
    #[doc = "Change the `feedback` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IirFilterOptions`*"]
    pub fn feedback(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_feedback(val);
        self
    }
    #[doc = "Change the `feedforward` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IirFilterOptions`*"]
    pub fn feedforward(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_feedforward(val);
        self
    }
}
