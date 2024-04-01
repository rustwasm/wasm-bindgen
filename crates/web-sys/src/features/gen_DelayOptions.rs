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
    #[wasm_bindgen(method, setter = "channelCount")]
    fn channel_count_shim(this: &DelayOptions, val: u32);
    #[cfg(feature = "ChannelCountMode")]
    #[wasm_bindgen(method, setter = "channelCountMode")]
    fn channel_count_mode_shim(this: &DelayOptions, val: ChannelCountMode);
    #[cfg(feature = "ChannelInterpretation")]
    #[wasm_bindgen(method, setter = "channelInterpretation")]
    fn channel_interpretation_shim(this: &DelayOptions, val: ChannelInterpretation);
    #[wasm_bindgen(method, setter = "delayTime")]
    fn delay_time_shim(this: &DelayOptions, val: f64);
    #[wasm_bindgen(method, setter = "maxDelayTime")]
    fn max_delay_time_shim(this: &DelayOptions, val: f64);
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
        self.channel_count_shim(val);
        self
    }
    #[cfg(feature = "ChannelCountMode")]
    #[doc = "Change the `channelCountMode` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ChannelCountMode`, `DelayOptions`*"]
    pub fn channel_count_mode(&mut self, val: ChannelCountMode) -> &mut Self {
        self.channel_count_mode_shim(val);
        self
    }
    #[cfg(feature = "ChannelInterpretation")]
    #[doc = "Change the `channelInterpretation` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ChannelInterpretation`, `DelayOptions`*"]
    pub fn channel_interpretation(&mut self, val: ChannelInterpretation) -> &mut Self {
        self.channel_interpretation_shim(val);
        self
    }
    #[doc = "Change the `delayTime` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DelayOptions`*"]
    pub fn delay_time(&mut self, val: f64) -> &mut Self {
        self.delay_time_shim(val);
        self
    }
    #[doc = "Change the `maxDelayTime` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DelayOptions`*"]
    pub fn max_delay_time(&mut self, val: f64) -> &mut Self {
        self.max_delay_time_shim(val);
        self
    }
}
impl Default for DelayOptions {
    fn default() -> Self {
        Self::new()
    }
}
