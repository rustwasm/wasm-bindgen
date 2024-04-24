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
    #[wasm_bindgen(method, getter = "channelCount")]
    fn channel_count_shim(this: &IirFilterOptions) -> u32;
    #[wasm_bindgen(method, setter = "channelCount")]
    fn set_channel_count_shim(this: &IirFilterOptions, val: u32);
    #[cfg(feature = "ChannelCountMode")]
    #[wasm_bindgen(method, getter = "channelCountMode")]
    fn channel_count_mode_shim(this: &IirFilterOptions) -> ChannelCountMode;
    #[cfg(feature = "ChannelCountMode")]
    #[wasm_bindgen(method, setter = "channelCountMode")]
    fn set_channel_count_mode_shim(this: &IirFilterOptions, val: ChannelCountMode);
    #[cfg(feature = "ChannelInterpretation")]
    #[wasm_bindgen(method, getter = "channelInterpretation")]
    fn channel_interpretation_shim(this: &IirFilterOptions) -> ChannelInterpretation;
    #[cfg(feature = "ChannelInterpretation")]
    #[wasm_bindgen(method, setter = "channelInterpretation")]
    fn set_channel_interpretation_shim(this: &IirFilterOptions, val: ChannelInterpretation);
    #[wasm_bindgen(method, getter = "feedback")]
    fn feedback_shim(this: &IirFilterOptions) -> &::wasm_bindgen::JsValue;
    #[wasm_bindgen(method, setter = "feedback")]
    fn set_feedback_shim(this: &IirFilterOptions, val: &::wasm_bindgen::JsValue);
    #[wasm_bindgen(method, getter = "feedforward")]
    fn feedforward_shim(this: &IirFilterOptions) -> &::wasm_bindgen::JsValue;
    #[wasm_bindgen(method, setter = "feedforward")]
    fn set_feedforward_shim(this: &IirFilterOptions, val: &::wasm_bindgen::JsValue);
}
#[doc = "The trait to access properties on the `IirFilterOptions` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `IirFilterOptions`*"]
pub trait IirFilterOptionsGetters {
    #[doc = "Get the `channelCount` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IirFilterOptions`*"]
    fn channel_count(&self) -> u32;
    #[cfg(feature = "ChannelCountMode")]
    #[doc = "Get the `channelCountMode` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ChannelCountMode`, `IirFilterOptions`*"]
    fn channel_count_mode(&self) -> ChannelCountMode;
    #[cfg(feature = "ChannelInterpretation")]
    #[doc = "Get the `channelInterpretation` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ChannelInterpretation`, `IirFilterOptions`*"]
    fn channel_interpretation(&self) -> ChannelInterpretation;
    #[doc = "Get the `feedback` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IirFilterOptions`*"]
    fn feedback(&self) -> &::wasm_bindgen::JsValue;
    #[doc = "Get the `feedforward` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IirFilterOptions`*"]
    fn feedforward(&self) -> &::wasm_bindgen::JsValue;
}
impl IirFilterOptionsGetters for IirFilterOptions {
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
    fn feedback(&self) -> &::wasm_bindgen::JsValue {
        self.feedback_shim()
    }
    fn feedforward(&self) -> &::wasm_bindgen::JsValue {
        self.feedforward_shim()
    }
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
        self.set_channel_count_shim(val);
        self
    }
    #[cfg(feature = "ChannelCountMode")]
    #[doc = "Change the `channelCountMode` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ChannelCountMode`, `IirFilterOptions`*"]
    pub fn channel_count_mode(&mut self, val: ChannelCountMode) -> &mut Self {
        self.set_channel_count_mode_shim(val);
        self
    }
    #[cfg(feature = "ChannelInterpretation")]
    #[doc = "Change the `channelInterpretation` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ChannelInterpretation`, `IirFilterOptions`*"]
    pub fn channel_interpretation(&mut self, val: ChannelInterpretation) -> &mut Self {
        self.set_channel_interpretation_shim(val);
        self
    }
    #[doc = "Change the `feedback` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IirFilterOptions`*"]
    pub fn feedback(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_feedback_shim(val);
        self
    }
    #[doc = "Change the `feedforward` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IirFilterOptions`*"]
    pub fn feedforward(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_feedforward_shim(val);
        self
    }
}
