#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = BiquadFilterOptions)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `BiquadFilterOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BiquadFilterOptions`*"]
    pub type BiquadFilterOptions;
    #[wasm_bindgen(method, getter = "channelCount")]
    fn channel_count_shim(this: &BiquadFilterOptions) -> u32;
    #[wasm_bindgen(method, setter = "channelCount")]
    fn set_channel_count_shim(this: &BiquadFilterOptions, val: u32);
    #[cfg(feature = "ChannelCountMode")]
    #[wasm_bindgen(method, getter = "channelCountMode")]
    fn channel_count_mode_shim(this: &BiquadFilterOptions) -> ChannelCountMode;
    #[cfg(feature = "ChannelCountMode")]
    #[wasm_bindgen(method, setter = "channelCountMode")]
    fn set_channel_count_mode_shim(this: &BiquadFilterOptions, val: ChannelCountMode);
    #[cfg(feature = "ChannelInterpretation")]
    #[wasm_bindgen(method, getter = "channelInterpretation")]
    fn channel_interpretation_shim(this: &BiquadFilterOptions) -> ChannelInterpretation;
    #[cfg(feature = "ChannelInterpretation")]
    #[wasm_bindgen(method, setter = "channelInterpretation")]
    fn set_channel_interpretation_shim(this: &BiquadFilterOptions, val: ChannelInterpretation);
    #[wasm_bindgen(method, getter = "Q")]
    fn q_shim(this: &BiquadFilterOptions) -> f32;
    #[wasm_bindgen(method, setter = "Q")]
    fn set_q_shim(this: &BiquadFilterOptions, val: f32);
    #[wasm_bindgen(method, getter = "detune")]
    fn detune_shim(this: &BiquadFilterOptions) -> f32;
    #[wasm_bindgen(method, setter = "detune")]
    fn set_detune_shim(this: &BiquadFilterOptions, val: f32);
    #[wasm_bindgen(method, getter = "frequency")]
    fn frequency_shim(this: &BiquadFilterOptions) -> f32;
    #[wasm_bindgen(method, setter = "frequency")]
    fn set_frequency_shim(this: &BiquadFilterOptions, val: f32);
    #[wasm_bindgen(method, getter = "gain")]
    fn gain_shim(this: &BiquadFilterOptions) -> f32;
    #[wasm_bindgen(method, setter = "gain")]
    fn set_gain_shim(this: &BiquadFilterOptions, val: f32);
    #[cfg(feature = "BiquadFilterType")]
    #[wasm_bindgen(method, getter = "type")]
    fn type__shim(this: &BiquadFilterOptions) -> BiquadFilterType;
    #[cfg(feature = "BiquadFilterType")]
    #[wasm_bindgen(method, setter = "type")]
    fn set_type__shim(this: &BiquadFilterOptions, val: BiquadFilterType);
}
#[doc = "The trait to access properties on the `BiquadFilterOptions` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `BiquadFilterOptions`*"]
pub trait BiquadFilterOptionsGetters {
    #[doc = "Get the `channelCount` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BiquadFilterOptions`*"]
    fn channel_count(&self) -> u32;
    #[cfg(feature = "ChannelCountMode")]
    #[doc = "Get the `channelCountMode` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BiquadFilterOptions`, `ChannelCountMode`*"]
    fn channel_count_mode(&self) -> ChannelCountMode;
    #[cfg(feature = "ChannelInterpretation")]
    #[doc = "Get the `channelInterpretation` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BiquadFilterOptions`, `ChannelInterpretation`*"]
    fn channel_interpretation(&self) -> ChannelInterpretation;
    #[doc = "Get the `Q` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BiquadFilterOptions`*"]
    fn q(&self) -> f32;
    #[doc = "Get the `detune` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BiquadFilterOptions`*"]
    fn detune(&self) -> f32;
    #[doc = "Get the `frequency` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BiquadFilterOptions`*"]
    fn frequency(&self) -> f32;
    #[doc = "Get the `gain` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BiquadFilterOptions`*"]
    fn gain(&self) -> f32;
    #[cfg(feature = "BiquadFilterType")]
    #[doc = "Get the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BiquadFilterOptions`, `BiquadFilterType`*"]
    fn type_(&self) -> BiquadFilterType;
}
impl BiquadFilterOptionsGetters for BiquadFilterOptions {
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
    fn q(&self) -> f32 {
        self.q_shim()
    }
    fn detune(&self) -> f32 {
        self.detune_shim()
    }
    fn frequency(&self) -> f32 {
        self.frequency_shim()
    }
    fn gain(&self) -> f32 {
        self.gain_shim()
    }
    #[cfg(feature = "BiquadFilterType")]
    fn type_(&self) -> BiquadFilterType {
        self.type__shim()
    }
}
impl BiquadFilterOptions {
    #[doc = "Construct a new `BiquadFilterOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BiquadFilterOptions`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `channelCount` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BiquadFilterOptions`*"]
    pub fn channel_count(&mut self, val: u32) -> &mut Self {
        self.set_channel_count_shim(val);
        self
    }
    #[cfg(feature = "ChannelCountMode")]
    #[doc = "Change the `channelCountMode` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BiquadFilterOptions`, `ChannelCountMode`*"]
    pub fn channel_count_mode(&mut self, val: ChannelCountMode) -> &mut Self {
        self.set_channel_count_mode_shim(val);
        self
    }
    #[cfg(feature = "ChannelInterpretation")]
    #[doc = "Change the `channelInterpretation` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BiquadFilterOptions`, `ChannelInterpretation`*"]
    pub fn channel_interpretation(&mut self, val: ChannelInterpretation) -> &mut Self {
        self.set_channel_interpretation_shim(val);
        self
    }
    #[doc = "Change the `Q` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BiquadFilterOptions`*"]
    pub fn q(&mut self, val: f32) -> &mut Self {
        self.set_q_shim(val);
        self
    }
    #[doc = "Change the `detune` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BiquadFilterOptions`*"]
    pub fn detune(&mut self, val: f32) -> &mut Self {
        self.set_detune_shim(val);
        self
    }
    #[doc = "Change the `frequency` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BiquadFilterOptions`*"]
    pub fn frequency(&mut self, val: f32) -> &mut Self {
        self.set_frequency_shim(val);
        self
    }
    #[doc = "Change the `gain` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BiquadFilterOptions`*"]
    pub fn gain(&mut self, val: f32) -> &mut Self {
        self.set_gain_shim(val);
        self
    }
    #[cfg(feature = "BiquadFilterType")]
    #[doc = "Change the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BiquadFilterOptions`, `BiquadFilterType`*"]
    pub fn type_(&mut self, val: BiquadFilterType) -> &mut Self {
        self.set_type__shim(val);
        self
    }
}
impl Default for BiquadFilterOptions {
    fn default() -> Self {
        Self::new()
    }
}
