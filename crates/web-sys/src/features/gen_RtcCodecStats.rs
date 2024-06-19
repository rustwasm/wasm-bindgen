#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = RTCCodecStats)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `RtcCodecStats` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcCodecStats`*"]
    pub type RtcCodecStats;
    #[doc = "Get the `id` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcCodecStats`*"]
    #[wasm_bindgen(method, getter = "id")]
    pub fn get_id(this: &RtcCodecStats) -> Option<String>;
    #[wasm_bindgen(method, setter = "id")]
    fn set_id(this: &RtcCodecStats, val: &str);
    #[doc = "Get the `timestamp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcCodecStats`*"]
    #[wasm_bindgen(method, getter = "timestamp")]
    pub fn get_timestamp(this: &RtcCodecStats) -> Option<f64>;
    #[wasm_bindgen(method, setter = "timestamp")]
    fn set_timestamp(this: &RtcCodecStats, val: f64);
    #[cfg(feature = "RtcStatsType")]
    #[doc = "Get the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcCodecStats`, `RtcStatsType`*"]
    #[wasm_bindgen(method, getter = "type")]
    pub fn get_type(this: &RtcCodecStats) -> Option<RtcStatsType>;
    #[cfg(feature = "RtcStatsType")]
    #[wasm_bindgen(method, setter = "type")]
    fn set_type(this: &RtcCodecStats, val: RtcStatsType);
    #[doc = "Get the `channels` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcCodecStats`*"]
    #[wasm_bindgen(method, getter = "channels")]
    pub fn get_channels(this: &RtcCodecStats) -> Option<u32>;
    #[wasm_bindgen(method, setter = "channels")]
    fn set_channels(this: &RtcCodecStats, val: u32);
    #[doc = "Get the `clockRate` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcCodecStats`*"]
    #[wasm_bindgen(method, getter = "clockRate")]
    pub fn get_clock_rate(this: &RtcCodecStats) -> Option<u32>;
    #[wasm_bindgen(method, setter = "clockRate")]
    fn set_clock_rate(this: &RtcCodecStats, val: u32);
    #[doc = "Get the `codec` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcCodecStats`*"]
    #[wasm_bindgen(method, getter = "codec")]
    pub fn get_codec(this: &RtcCodecStats) -> Option<String>;
    #[wasm_bindgen(method, setter = "codec")]
    fn set_codec(this: &RtcCodecStats, val: &str);
    #[doc = "Get the `parameters` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcCodecStats`*"]
    #[wasm_bindgen(method, getter = "parameters")]
    pub fn get_parameters(this: &RtcCodecStats) -> Option<String>;
    #[wasm_bindgen(method, setter = "parameters")]
    fn set_parameters(this: &RtcCodecStats, val: &str);
    #[doc = "Get the `payloadType` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcCodecStats`*"]
    #[wasm_bindgen(method, getter = "payloadType")]
    pub fn get_payload_type(this: &RtcCodecStats) -> Option<u32>;
    #[wasm_bindgen(method, setter = "payloadType")]
    fn set_payload_type(this: &RtcCodecStats, val: u32);
}
impl RtcCodecStats {
    #[doc = "Construct a new `RtcCodecStats`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcCodecStats`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `id` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcCodecStats`*"]
    pub fn id(&mut self, val: &str) -> &mut Self {
        self.set_id(val);
        self
    }
    #[doc = "Change the `timestamp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcCodecStats`*"]
    pub fn timestamp(&mut self, val: f64) -> &mut Self {
        self.set_timestamp(val);
        self
    }
    #[cfg(feature = "RtcStatsType")]
    #[doc = "Change the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcCodecStats`, `RtcStatsType`*"]
    pub fn type_(&mut self, val: RtcStatsType) -> &mut Self {
        self.set_type(val);
        self
    }
    #[doc = "Change the `channels` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcCodecStats`*"]
    pub fn channels(&mut self, val: u32) -> &mut Self {
        self.set_channels(val);
        self
    }
    #[doc = "Change the `clockRate` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcCodecStats`*"]
    pub fn clock_rate(&mut self, val: u32) -> &mut Self {
        self.set_clock_rate(val);
        self
    }
    #[doc = "Change the `codec` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcCodecStats`*"]
    pub fn codec(&mut self, val: &str) -> &mut Self {
        self.set_codec(val);
        self
    }
    #[doc = "Change the `parameters` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcCodecStats`*"]
    pub fn parameters(&mut self, val: &str) -> &mut Self {
        self.set_parameters(val);
        self
    }
    #[doc = "Change the `payloadType` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcCodecStats`*"]
    pub fn payload_type(&mut self, val: u32) -> &mut Self {
        self.set_payload_type(val);
        self
    }
}
impl Default for RtcCodecStats {
    fn default() -> Self {
        Self::new()
    }
}
