#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = OscillatorOptions)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `OscillatorOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OscillatorOptions`*"]
    pub type OscillatorOptions;
    #[wasm_bindgen(method, setter = "channelCount")]
    fn channel_count_shim(this: &OscillatorOptions, val: u32);
    #[cfg(feature = "ChannelCountMode")]
    #[wasm_bindgen(method, setter = "channelCountMode")]
    fn channel_count_mode_shim(this: &OscillatorOptions, val: ChannelCountMode);
    #[cfg(feature = "ChannelInterpretation")]
    #[wasm_bindgen(method, setter = "channelInterpretation")]
    fn channel_interpretation_shim(this: &OscillatorOptions, val: ChannelInterpretation);
    #[wasm_bindgen(method, setter = "detune")]
    fn detune_shim(this: &OscillatorOptions, val: f32);
    #[wasm_bindgen(method, setter = "frequency")]
    fn frequency_shim(this: &OscillatorOptions, val: f32);
    #[cfg(feature = "PeriodicWave")]
    #[wasm_bindgen(method, setter = "periodicWave")]
    fn periodic_wave_shim(this: &OscillatorOptions, val: &PeriodicWave);
    #[cfg(feature = "OscillatorType")]
    #[wasm_bindgen(method, setter = "type")]
    fn type__shim(this: &OscillatorOptions, val: OscillatorType);
}
impl OscillatorOptions {
    #[doc = "Construct a new `OscillatorOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OscillatorOptions`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `channelCount` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OscillatorOptions`*"]
    pub fn channel_count(&mut self, val: u32) -> &mut Self {
        self.channel_count_shim(val);
        self
    }
    #[cfg(feature = "ChannelCountMode")]
    #[doc = "Change the `channelCountMode` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ChannelCountMode`, `OscillatorOptions`*"]
    pub fn channel_count_mode(&mut self, val: ChannelCountMode) -> &mut Self {
        self.channel_count_mode_shim(val);
        self
    }
    #[cfg(feature = "ChannelInterpretation")]
    #[doc = "Change the `channelInterpretation` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ChannelInterpretation`, `OscillatorOptions`*"]
    pub fn channel_interpretation(&mut self, val: ChannelInterpretation) -> &mut Self {
        self.channel_interpretation_shim(val);
        self
    }
    #[doc = "Change the `detune` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OscillatorOptions`*"]
    pub fn detune(&mut self, val: f32) -> &mut Self {
        self.detune_shim(val);
        self
    }
    #[doc = "Change the `frequency` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OscillatorOptions`*"]
    pub fn frequency(&mut self, val: f32) -> &mut Self {
        self.frequency_shim(val);
        self
    }
    #[cfg(feature = "PeriodicWave")]
    #[doc = "Change the `periodicWave` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OscillatorOptions`, `PeriodicWave`*"]
    pub fn periodic_wave(&mut self, val: &PeriodicWave) -> &mut Self {
        self.periodic_wave_shim(val);
        self
    }
    #[cfg(feature = "OscillatorType")]
    #[doc = "Change the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OscillatorOptions`, `OscillatorType`*"]
    pub fn type_(&mut self, val: OscillatorType) -> &mut Self {
        self.type__shim(val);
        self
    }
}
impl Default for OscillatorOptions {
    fn default() -> Self {
        Self::new()
    }
}
