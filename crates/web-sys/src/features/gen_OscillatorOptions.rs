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
    #[wasm_bindgen(method, getter = "channelCount")]
    fn channel_count_shim(this: &OscillatorOptions) -> u32;
    #[wasm_bindgen(method, setter = "channelCount")]
    fn set_channel_count_shim(this: &OscillatorOptions, val: u32);
    #[cfg(feature = "ChannelCountMode")]
    #[wasm_bindgen(method, getter = "channelCountMode")]
    fn channel_count_mode_shim(this: &OscillatorOptions) -> ChannelCountMode;
    #[cfg(feature = "ChannelCountMode")]
    #[wasm_bindgen(method, setter = "channelCountMode")]
    fn set_channel_count_mode_shim(this: &OscillatorOptions, val: ChannelCountMode);
    #[cfg(feature = "ChannelInterpretation")]
    #[wasm_bindgen(method, getter = "channelInterpretation")]
    fn channel_interpretation_shim(this: &OscillatorOptions) -> ChannelInterpretation;
    #[cfg(feature = "ChannelInterpretation")]
    #[wasm_bindgen(method, setter = "channelInterpretation")]
    fn set_channel_interpretation_shim(this: &OscillatorOptions, val: ChannelInterpretation);
    #[wasm_bindgen(method, getter = "detune")]
    fn detune_shim(this: &OscillatorOptions) -> f32;
    #[wasm_bindgen(method, setter = "detune")]
    fn set_detune_shim(this: &OscillatorOptions, val: f32);
    #[wasm_bindgen(method, getter = "frequency")]
    fn frequency_shim(this: &OscillatorOptions) -> f32;
    #[wasm_bindgen(method, setter = "frequency")]
    fn set_frequency_shim(this: &OscillatorOptions, val: f32);
    #[cfg(feature = "PeriodicWave")]
    #[wasm_bindgen(method, getter = "periodicWave")]
    fn periodic_wave_shim(this: &OscillatorOptions) -> &PeriodicWave;
    #[cfg(feature = "PeriodicWave")]
    #[wasm_bindgen(method, setter = "periodicWave")]
    fn set_periodic_wave_shim(this: &OscillatorOptions, val: &PeriodicWave);
    #[cfg(feature = "OscillatorType")]
    #[wasm_bindgen(method, getter = "type")]
    fn type__shim(this: &OscillatorOptions) -> OscillatorType;
    #[cfg(feature = "OscillatorType")]
    #[wasm_bindgen(method, setter = "type")]
    fn set_type__shim(this: &OscillatorOptions, val: OscillatorType);
}
#[doc = "The trait to access properties on the `OscillatorOptions` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `OscillatorOptions`*"]
pub trait OscillatorOptionsGetters {
    #[doc = "Get the `channelCount` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OscillatorOptions`*"]
    fn channel_count(&self) -> u32;
    #[cfg(feature = "ChannelCountMode")]
    #[doc = "Get the `channelCountMode` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ChannelCountMode`, `OscillatorOptions`*"]
    fn channel_count_mode(&self) -> ChannelCountMode;
    #[cfg(feature = "ChannelInterpretation")]
    #[doc = "Get the `channelInterpretation` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ChannelInterpretation`, `OscillatorOptions`*"]
    fn channel_interpretation(&self) -> ChannelInterpretation;
    #[doc = "Get the `detune` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OscillatorOptions`*"]
    fn detune(&self) -> f32;
    #[doc = "Get the `frequency` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OscillatorOptions`*"]
    fn frequency(&self) -> f32;
    #[cfg(feature = "PeriodicWave")]
    #[doc = "Get the `periodicWave` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OscillatorOptions`, `PeriodicWave`*"]
    fn periodic_wave(&self) -> &PeriodicWave;
    #[cfg(feature = "OscillatorType")]
    #[doc = "Get the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OscillatorOptions`, `OscillatorType`*"]
    fn type_(&self) -> OscillatorType;
}
impl OscillatorOptionsGetters for OscillatorOptions {
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
    fn detune(&self) -> f32 {
        self.detune_shim()
    }
    fn frequency(&self) -> f32 {
        self.frequency_shim()
    }
    #[cfg(feature = "PeriodicWave")]
    fn periodic_wave(&self) -> &PeriodicWave {
        self.periodic_wave_shim()
    }
    #[cfg(feature = "OscillatorType")]
    fn type_(&self) -> OscillatorType {
        self.type__shim()
    }
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
        self.set_channel_count_shim(val);
        self
    }
    #[cfg(feature = "ChannelCountMode")]
    #[doc = "Change the `channelCountMode` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ChannelCountMode`, `OscillatorOptions`*"]
    pub fn channel_count_mode(&mut self, val: ChannelCountMode) -> &mut Self {
        self.set_channel_count_mode_shim(val);
        self
    }
    #[cfg(feature = "ChannelInterpretation")]
    #[doc = "Change the `channelInterpretation` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ChannelInterpretation`, `OscillatorOptions`*"]
    pub fn channel_interpretation(&mut self, val: ChannelInterpretation) -> &mut Self {
        self.set_channel_interpretation_shim(val);
        self
    }
    #[doc = "Change the `detune` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OscillatorOptions`*"]
    pub fn detune(&mut self, val: f32) -> &mut Self {
        self.set_detune_shim(val);
        self
    }
    #[doc = "Change the `frequency` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OscillatorOptions`*"]
    pub fn frequency(&mut self, val: f32) -> &mut Self {
        self.set_frequency_shim(val);
        self
    }
    #[cfg(feature = "PeriodicWave")]
    #[doc = "Change the `periodicWave` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OscillatorOptions`, `PeriodicWave`*"]
    pub fn periodic_wave(&mut self, val: &PeriodicWave) -> &mut Self {
        self.set_periodic_wave_shim(val);
        self
    }
    #[cfg(feature = "OscillatorType")]
    #[doc = "Change the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OscillatorOptions`, `OscillatorType`*"]
    pub fn type_(&mut self, val: OscillatorType) -> &mut Self {
        self.set_type__shim(val);
        self
    }
}
impl Default for OscillatorOptions {
    fn default() -> Self {
        Self::new()
    }
}
