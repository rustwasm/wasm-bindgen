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
    #[wasm_bindgen(method, setter = "channelCount")]
    fn channel_count_shim(this: &AudioWorkletNodeOptions, val: u32);
    #[cfg(feature = "ChannelCountMode")]
    #[wasm_bindgen(method, setter = "channelCountMode")]
    fn channel_count_mode_shim(this: &AudioWorkletNodeOptions, val: ChannelCountMode);
    #[cfg(feature = "ChannelInterpretation")]
    #[wasm_bindgen(method, setter = "channelInterpretation")]
    fn channel_interpretation_shim(this: &AudioWorkletNodeOptions, val: ChannelInterpretation);
    #[wasm_bindgen(method, setter = "numberOfInputs")]
    fn number_of_inputs_shim(this: &AudioWorkletNodeOptions, val: u32);
    #[wasm_bindgen(method, setter = "numberOfOutputs")]
    fn number_of_outputs_shim(this: &AudioWorkletNodeOptions, val: u32);
    #[wasm_bindgen(method, setter = "outputChannelCount")]
    fn output_channel_count_shim(this: &AudioWorkletNodeOptions, val: &::wasm_bindgen::JsValue);
    #[wasm_bindgen(method, setter = "processorOptions")]
    fn processor_options_shim(this: &AudioWorkletNodeOptions, val: Option<&::js_sys::Object>);
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
        self.channel_count_shim(val);
        self
    }
    #[cfg(feature = "ChannelCountMode")]
    #[doc = "Change the `channelCountMode` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioWorkletNodeOptions`, `ChannelCountMode`*"]
    pub fn channel_count_mode(&mut self, val: ChannelCountMode) -> &mut Self {
        self.channel_count_mode_shim(val);
        self
    }
    #[cfg(feature = "ChannelInterpretation")]
    #[doc = "Change the `channelInterpretation` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioWorkletNodeOptions`, `ChannelInterpretation`*"]
    pub fn channel_interpretation(&mut self, val: ChannelInterpretation) -> &mut Self {
        self.channel_interpretation_shim(val);
        self
    }
    #[doc = "Change the `numberOfInputs` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioWorkletNodeOptions`*"]
    pub fn number_of_inputs(&mut self, val: u32) -> &mut Self {
        self.number_of_inputs_shim(val);
        self
    }
    #[doc = "Change the `numberOfOutputs` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioWorkletNodeOptions`*"]
    pub fn number_of_outputs(&mut self, val: u32) -> &mut Self {
        self.number_of_outputs_shim(val);
        self
    }
    #[doc = "Change the `outputChannelCount` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioWorkletNodeOptions`*"]
    pub fn output_channel_count(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.output_channel_count_shim(val);
        self
    }
    #[doc = "Change the `processorOptions` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioWorkletNodeOptions`*"]
    pub fn processor_options(&mut self, val: Option<&::js_sys::Object>) -> &mut Self {
        self.processor_options_shim(val);
        self
    }
}
impl Default for AudioWorkletNodeOptions {
    fn default() -> Self {
        Self::new()
    }
}
