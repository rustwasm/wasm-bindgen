#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = ChannelMergerOptions)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ChannelMergerOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ChannelMergerOptions`*"]
    pub type ChannelMergerOptions;
    #[wasm_bindgen(method, getter = "channelCount")]
    fn channel_count_shim(this: &ChannelMergerOptions) -> u32;
    #[wasm_bindgen(method, setter = "channelCount")]
    fn set_channel_count_shim(this: &ChannelMergerOptions, val: u32);
    #[cfg(feature = "ChannelCountMode")]
    #[wasm_bindgen(method, getter = "channelCountMode")]
    fn channel_count_mode_shim(this: &ChannelMergerOptions) -> ChannelCountMode;
    #[cfg(feature = "ChannelCountMode")]
    #[wasm_bindgen(method, setter = "channelCountMode")]
    fn set_channel_count_mode_shim(this: &ChannelMergerOptions, val: ChannelCountMode);
    #[cfg(feature = "ChannelInterpretation")]
    #[wasm_bindgen(method, getter = "channelInterpretation")]
    fn channel_interpretation_shim(this: &ChannelMergerOptions) -> ChannelInterpretation;
    #[cfg(feature = "ChannelInterpretation")]
    #[wasm_bindgen(method, setter = "channelInterpretation")]
    fn set_channel_interpretation_shim(this: &ChannelMergerOptions, val: ChannelInterpretation);
    #[wasm_bindgen(method, getter = "numberOfInputs")]
    fn number_of_inputs_shim(this: &ChannelMergerOptions) -> u32;
    #[wasm_bindgen(method, setter = "numberOfInputs")]
    fn set_number_of_inputs_shim(this: &ChannelMergerOptions, val: u32);
}
#[doc = "The trait to access properties on the `ChannelMergerOptions` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `ChannelMergerOptions`*"]
pub trait ChannelMergerOptionsGetters {
    #[doc = "Get the `channelCount` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ChannelMergerOptions`*"]
    fn channel_count(&self) -> u32;
    #[cfg(feature = "ChannelCountMode")]
    #[doc = "Get the `channelCountMode` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ChannelCountMode`, `ChannelMergerOptions`*"]
    fn channel_count_mode(&self) -> ChannelCountMode;
    #[cfg(feature = "ChannelInterpretation")]
    #[doc = "Get the `channelInterpretation` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ChannelInterpretation`, `ChannelMergerOptions`*"]
    fn channel_interpretation(&self) -> ChannelInterpretation;
    #[doc = "Get the `numberOfInputs` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ChannelMergerOptions`*"]
    fn number_of_inputs(&self) -> u32;
}
impl ChannelMergerOptionsGetters for ChannelMergerOptions {
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
    fn number_of_inputs(&self) -> u32 {
        self.number_of_inputs_shim()
    }
}
impl ChannelMergerOptions {
    #[doc = "Construct a new `ChannelMergerOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ChannelMergerOptions`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `channelCount` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ChannelMergerOptions`*"]
    pub fn channel_count(&mut self, val: u32) -> &mut Self {
        self.set_channel_count_shim(val);
        self
    }
    #[cfg(feature = "ChannelCountMode")]
    #[doc = "Change the `channelCountMode` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ChannelCountMode`, `ChannelMergerOptions`*"]
    pub fn channel_count_mode(&mut self, val: ChannelCountMode) -> &mut Self {
        self.set_channel_count_mode_shim(val);
        self
    }
    #[cfg(feature = "ChannelInterpretation")]
    #[doc = "Change the `channelInterpretation` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ChannelInterpretation`, `ChannelMergerOptions`*"]
    pub fn channel_interpretation(&mut self, val: ChannelInterpretation) -> &mut Self {
        self.set_channel_interpretation_shim(val);
        self
    }
    #[doc = "Change the `numberOfInputs` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ChannelMergerOptions`*"]
    pub fn number_of_inputs(&mut self, val: u32) -> &mut Self {
        self.set_number_of_inputs_shim(val);
        self
    }
}
impl Default for ChannelMergerOptions {
    fn default() -> Self {
        Self::new()
    }
}
