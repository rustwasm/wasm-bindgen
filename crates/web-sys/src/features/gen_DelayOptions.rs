#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = DelayOptions)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `DelayOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DelayOptions`*"]
    pub type DelayOptions;
    #[wasm_bindgen(method, getter = "channelCount")]
    fn channel_count_shim(this: &DelayOptions) -> u32;
    #[wasm_bindgen(method, setter = "channelCount")]
    fn set_channel_count_shim(this: &DelayOptions, val: u32);
    #[cfg(feature = "ChannelCountMode")]
    #[wasm_bindgen(method, getter = "channelCountMode")]
    fn channel_count_mode_shim(this: &DelayOptions) -> ChannelCountMode;
    #[cfg(feature = "ChannelCountMode")]
    #[wasm_bindgen(method, setter = "channelCountMode")]
    fn set_channel_count_mode_shim(this: &DelayOptions, val: ChannelCountMode);
    #[cfg(feature = "ChannelInterpretation")]
    #[wasm_bindgen(method, getter = "channelInterpretation")]
    fn channel_interpretation_shim(this: &DelayOptions) -> ChannelInterpretation;
    #[cfg(feature = "ChannelInterpretation")]
    #[wasm_bindgen(method, setter = "channelInterpretation")]
    fn set_channel_interpretation_shim(this: &DelayOptions, val: ChannelInterpretation);
    #[wasm_bindgen(method, getter = "delayTime")]
    fn delay_time_shim(this: &DelayOptions) -> f64;
    #[wasm_bindgen(method, setter = "delayTime")]
    fn set_delay_time_shim(this: &DelayOptions, val: f64);
    #[wasm_bindgen(method, getter = "maxDelayTime")]
    fn max_delay_time_shim(this: &DelayOptions) -> f64;
    #[wasm_bindgen(method, setter = "maxDelayTime")]
    fn set_max_delay_time_shim(this: &DelayOptions, val: f64);
}
#[doc = "The trait to access properties on the `DelayOptions` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `DelayOptions`*"]
pub trait DelayOptionsGetters {
    #[doc = "Get the `channelCount` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DelayOptions`*"]
    fn channel_count(&self) -> u32;
    #[cfg(feature = "ChannelCountMode")]
    #[doc = "Get the `channelCountMode` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ChannelCountMode`, `DelayOptions`*"]
    fn channel_count_mode(&self) -> ChannelCountMode;
    #[cfg(feature = "ChannelInterpretation")]
    #[doc = "Get the `channelInterpretation` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ChannelInterpretation`, `DelayOptions`*"]
    fn channel_interpretation(&self) -> ChannelInterpretation;
    #[doc = "Get the `delayTime` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DelayOptions`*"]
    fn delay_time(&self) -> f64;
    #[doc = "Get the `maxDelayTime` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DelayOptions`*"]
    fn max_delay_time(&self) -> f64;
}
impl DelayOptionsGetters for DelayOptions {
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
    fn delay_time(&self) -> f64 {
        self.delay_time_shim()
    }
    fn max_delay_time(&self) -> f64 {
        self.max_delay_time_shim()
    }
}
impl DelayOptions {
    #[doc = "Construct a new `DelayOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DelayOptions`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `channelCount` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DelayOptions`*"]
    pub fn channel_count(&mut self, val: u32) -> &mut Self {
        self.set_channel_count_shim(val);
        self
    }
    #[cfg(feature = "ChannelCountMode")]
    #[doc = "Change the `channelCountMode` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ChannelCountMode`, `DelayOptions`*"]
    pub fn channel_count_mode(&mut self, val: ChannelCountMode) -> &mut Self {
        self.set_channel_count_mode_shim(val);
        self
    }
    #[cfg(feature = "ChannelInterpretation")]
    #[doc = "Change the `channelInterpretation` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ChannelInterpretation`, `DelayOptions`*"]
    pub fn channel_interpretation(&mut self, val: ChannelInterpretation) -> &mut Self {
        self.set_channel_interpretation_shim(val);
        self
    }
    #[doc = "Change the `delayTime` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DelayOptions`*"]
    pub fn delay_time(&mut self, val: f64) -> &mut Self {
        self.set_delay_time_shim(val);
        self
    }
    #[doc = "Change the `maxDelayTime` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DelayOptions`*"]
    pub fn max_delay_time(&mut self, val: f64) -> &mut Self {
        self.set_max_delay_time_shim(val);
        self
    }
}
impl Default for DelayOptions {
    fn default() -> Self {
        Self::new()
    }
}
