#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = AudioWorkletNodeOptions)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `AudioWorkletNodeOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioWorkletNodeOptions`*"]
    pub type AudioWorkletNodeOptions;
    #[doc = "Get the `channelCount` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioWorkletNodeOptions`*"]
    #[wasm_bindgen(method, getter = "channelCount")]
    pub fn get_channel_count(this: &AudioWorkletNodeOptions) -> Option<u32>;
    #[wasm_bindgen(method, setter = "channelCount")]
    fn set_channel_count(this: &AudioWorkletNodeOptions, val: u32);
    #[cfg(feature = "ChannelCountMode")]
    #[doc = "Get the `channelCountMode` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioWorkletNodeOptions`, `ChannelCountMode`*"]
    #[wasm_bindgen(method, getter = "channelCountMode")]
    pub fn get_channel_count_mode(this: &AudioWorkletNodeOptions) -> Option<ChannelCountMode>;
    #[cfg(feature = "ChannelCountMode")]
    #[wasm_bindgen(method, setter = "channelCountMode")]
    fn set_channel_count_mode(this: &AudioWorkletNodeOptions, val: ChannelCountMode);
    #[cfg(feature = "ChannelInterpretation")]
    #[doc = "Get the `channelInterpretation` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioWorkletNodeOptions`, `ChannelInterpretation`*"]
    #[wasm_bindgen(method, getter = "channelInterpretation")]
    pub fn get_channel_interpretation(
        this: &AudioWorkletNodeOptions,
    ) -> Option<ChannelInterpretation>;
    #[cfg(feature = "ChannelInterpretation")]
    #[wasm_bindgen(method, setter = "channelInterpretation")]
    fn set_channel_interpretation(this: &AudioWorkletNodeOptions, val: ChannelInterpretation);
    #[doc = "Get the `numberOfInputs` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioWorkletNodeOptions`*"]
    #[wasm_bindgen(method, getter = "numberOfInputs")]
    pub fn get_number_of_inputs(this: &AudioWorkletNodeOptions) -> Option<u32>;
    #[wasm_bindgen(method, setter = "numberOfInputs")]
    fn set_number_of_inputs(this: &AudioWorkletNodeOptions, val: u32);
    #[doc = "Get the `numberOfOutputs` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioWorkletNodeOptions`*"]
    #[wasm_bindgen(method, getter = "numberOfOutputs")]
    pub fn get_number_of_outputs(this: &AudioWorkletNodeOptions) -> Option<u32>;
    #[wasm_bindgen(method, setter = "numberOfOutputs")]
    fn set_number_of_outputs(this: &AudioWorkletNodeOptions, val: u32);
    #[doc = "Get the `outputChannelCount` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioWorkletNodeOptions`*"]
    #[wasm_bindgen(method, getter = "outputChannelCount")]
    pub fn get_output_channel_count(this: &AudioWorkletNodeOptions) -> Option<::js_sys::Array>;
    #[wasm_bindgen(method, setter = "outputChannelCount")]
    fn set_output_channel_count(this: &AudioWorkletNodeOptions, val: &::wasm_bindgen::JsValue);
    #[doc = "Get the `processorOptions` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioWorkletNodeOptions`*"]
    #[wasm_bindgen(method, getter = "processorOptions")]
    pub fn get_processor_options(this: &AudioWorkletNodeOptions) -> Option<::js_sys::Object>;
    #[wasm_bindgen(method, setter = "processorOptions")]
    fn set_processor_options(this: &AudioWorkletNodeOptions, val: Option<&::js_sys::Object>);
}
impl AudioWorkletNodeOptions {
    #[doc = "Construct a new `AudioWorkletNodeOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioWorkletNodeOptions`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `channelCount` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioWorkletNodeOptions`*"]
    pub fn channel_count(&mut self, val: u32) -> &mut Self {
        self.set_channel_count(val);
        self
    }
    #[cfg(feature = "ChannelCountMode")]
    #[doc = "Change the `channelCountMode` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioWorkletNodeOptions`, `ChannelCountMode`*"]
    pub fn channel_count_mode(&mut self, val: ChannelCountMode) -> &mut Self {
        self.set_channel_count_mode(val);
        self
    }
    #[cfg(feature = "ChannelInterpretation")]
    #[doc = "Change the `channelInterpretation` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioWorkletNodeOptions`, `ChannelInterpretation`*"]
    pub fn channel_interpretation(&mut self, val: ChannelInterpretation) -> &mut Self {
        self.set_channel_interpretation(val);
        self
    }
    #[doc = "Change the `numberOfInputs` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioWorkletNodeOptions`*"]
    pub fn number_of_inputs(&mut self, val: u32) -> &mut Self {
        self.set_number_of_inputs(val);
        self
    }
    #[doc = "Change the `numberOfOutputs` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioWorkletNodeOptions`*"]
    pub fn number_of_outputs(&mut self, val: u32) -> &mut Self {
        self.set_number_of_outputs(val);
        self
    }
    #[doc = "Change the `outputChannelCount` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioWorkletNodeOptions`*"]
    pub fn output_channel_count(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_output_channel_count(val);
        self
    }
    #[doc = "Change the `processorOptions` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioWorkletNodeOptions`*"]
    pub fn processor_options(&mut self, val: Option<&::js_sys::Object>) -> &mut Self {
        self.set_processor_options(val);
        self
    }
}
impl Default for AudioWorkletNodeOptions {
    fn default() -> Self {
        Self::new()
    }
}
