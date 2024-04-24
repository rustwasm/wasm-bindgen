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
    #[wasm_bindgen(method, getter = "channelCount")]
    fn channel_count_shim(this: &AudioWorkletNodeOptions) -> u32;
    #[wasm_bindgen(method, setter = "channelCount")]
    fn set_channel_count_shim(this: &AudioWorkletNodeOptions, val: u32);
    #[cfg(feature = "ChannelCountMode")]
    #[wasm_bindgen(method, getter = "channelCountMode")]
    fn channel_count_mode_shim(this: &AudioWorkletNodeOptions) -> ChannelCountMode;
    #[cfg(feature = "ChannelCountMode")]
    #[wasm_bindgen(method, setter = "channelCountMode")]
    fn set_channel_count_mode_shim(this: &AudioWorkletNodeOptions, val: ChannelCountMode);
    #[cfg(feature = "ChannelInterpretation")]
    #[wasm_bindgen(method, getter = "channelInterpretation")]
    fn channel_interpretation_shim(this: &AudioWorkletNodeOptions) -> ChannelInterpretation;
    #[cfg(feature = "ChannelInterpretation")]
    #[wasm_bindgen(method, setter = "channelInterpretation")]
    fn set_channel_interpretation_shim(this: &AudioWorkletNodeOptions, val: ChannelInterpretation);
    #[wasm_bindgen(method, getter = "numberOfInputs")]
    fn number_of_inputs_shim(this: &AudioWorkletNodeOptions) -> u32;
    #[wasm_bindgen(method, setter = "numberOfInputs")]
    fn set_number_of_inputs_shim(this: &AudioWorkletNodeOptions, val: u32);
    #[wasm_bindgen(method, getter = "numberOfOutputs")]
    fn number_of_outputs_shim(this: &AudioWorkletNodeOptions) -> u32;
    #[wasm_bindgen(method, setter = "numberOfOutputs")]
    fn set_number_of_outputs_shim(this: &AudioWorkletNodeOptions, val: u32);
    #[wasm_bindgen(method, getter = "outputChannelCount")]
    fn output_channel_count_shim(this: &AudioWorkletNodeOptions) -> &::wasm_bindgen::JsValue;
    #[wasm_bindgen(method, setter = "outputChannelCount")]
    fn set_output_channel_count_shim(this: &AudioWorkletNodeOptions, val: &::wasm_bindgen::JsValue);
    #[wasm_bindgen(method, getter = "processorOptions")]
    fn processor_options_shim(this: &AudioWorkletNodeOptions) -> Option<&::js_sys::Object>;
    #[wasm_bindgen(method, setter = "processorOptions")]
    fn set_processor_options_shim(this: &AudioWorkletNodeOptions, val: Option<&::js_sys::Object>);
}
#[doc = "The trait to access properties on the `AudioWorkletNodeOptions` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `AudioWorkletNodeOptions`*"]
pub trait AudioWorkletNodeOptionsGetters {
    #[doc = "Get the `channelCount` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioWorkletNodeOptions`*"]
    fn channel_count(&self) -> u32;
    #[cfg(feature = "ChannelCountMode")]
    #[doc = "Get the `channelCountMode` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioWorkletNodeOptions`, `ChannelCountMode`*"]
    fn channel_count_mode(&self) -> ChannelCountMode;
    #[cfg(feature = "ChannelInterpretation")]
    #[doc = "Get the `channelInterpretation` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioWorkletNodeOptions`, `ChannelInterpretation`*"]
    fn channel_interpretation(&self) -> ChannelInterpretation;
    #[doc = "Get the `numberOfInputs` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioWorkletNodeOptions`*"]
    fn number_of_inputs(&self) -> u32;
    #[doc = "Get the `numberOfOutputs` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioWorkletNodeOptions`*"]
    fn number_of_outputs(&self) -> u32;
    #[doc = "Get the `outputChannelCount` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioWorkletNodeOptions`*"]
    fn output_channel_count(&self) -> &::wasm_bindgen::JsValue;
    #[doc = "Get the `processorOptions` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioWorkletNodeOptions`*"]
    fn processor_options(&self) -> Option<&::js_sys::Object>;
}
impl AudioWorkletNodeOptionsGetters for AudioWorkletNodeOptions {
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
    fn number_of_outputs(&self) -> u32 {
        self.number_of_outputs_shim()
    }
    fn output_channel_count(&self) -> &::wasm_bindgen::JsValue {
        self.output_channel_count_shim()
    }
    fn processor_options(&self) -> Option<&::js_sys::Object> {
        self.processor_options_shim()
    }
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
        self.set_channel_count_shim(val);
        self
    }
    #[cfg(feature = "ChannelCountMode")]
    #[doc = "Change the `channelCountMode` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioWorkletNodeOptions`, `ChannelCountMode`*"]
    pub fn channel_count_mode(&mut self, val: ChannelCountMode) -> &mut Self {
        self.set_channel_count_mode_shim(val);
        self
    }
    #[cfg(feature = "ChannelInterpretation")]
    #[doc = "Change the `channelInterpretation` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioWorkletNodeOptions`, `ChannelInterpretation`*"]
    pub fn channel_interpretation(&mut self, val: ChannelInterpretation) -> &mut Self {
        self.set_channel_interpretation_shim(val);
        self
    }
    #[doc = "Change the `numberOfInputs` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioWorkletNodeOptions`*"]
    pub fn number_of_inputs(&mut self, val: u32) -> &mut Self {
        self.set_number_of_inputs_shim(val);
        self
    }
    #[doc = "Change the `numberOfOutputs` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioWorkletNodeOptions`*"]
    pub fn number_of_outputs(&mut self, val: u32) -> &mut Self {
        self.set_number_of_outputs_shim(val);
        self
    }
    #[doc = "Change the `outputChannelCount` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioWorkletNodeOptions`*"]
    pub fn output_channel_count(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_output_channel_count_shim(val);
        self
    }
    #[doc = "Change the `processorOptions` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioWorkletNodeOptions`*"]
    pub fn processor_options(&mut self, val: Option<&::js_sys::Object>) -> &mut Self {
        self.set_processor_options_shim(val);
        self
    }
}
impl Default for AudioWorkletNodeOptions {
    fn default() -> Self {
        Self::new()
    }
}
