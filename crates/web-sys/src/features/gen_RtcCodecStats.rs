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
    #[wasm_bindgen(method, getter = "id")]
    fn id_shim(this: &RtcCodecStats) -> String;
    #[wasm_bindgen(method, setter = "id")]
    fn set_id_shim(this: &RtcCodecStats, val: &str);
    #[wasm_bindgen(method, getter = "timestamp")]
    fn timestamp_shim(this: &RtcCodecStats) -> f64;
    #[wasm_bindgen(method, setter = "timestamp")]
    fn set_timestamp_shim(this: &RtcCodecStats, val: f64);
    #[cfg(feature = "RtcStatsType")]
    #[wasm_bindgen(method, getter = "type")]
    fn type__shim(this: &RtcCodecStats) -> RtcStatsType;
    #[cfg(feature = "RtcStatsType")]
    #[wasm_bindgen(method, setter = "type")]
    fn set_type__shim(this: &RtcCodecStats, val: RtcStatsType);
    #[wasm_bindgen(method, getter = "channels")]
    fn channels_shim(this: &RtcCodecStats) -> u32;
    #[wasm_bindgen(method, setter = "channels")]
    fn set_channels_shim(this: &RtcCodecStats, val: u32);
    #[wasm_bindgen(method, getter = "clockRate")]
    fn clock_rate_shim(this: &RtcCodecStats) -> u32;
    #[wasm_bindgen(method, setter = "clockRate")]
    fn set_clock_rate_shim(this: &RtcCodecStats, val: u32);
    #[wasm_bindgen(method, getter = "codec")]
    fn codec_shim(this: &RtcCodecStats) -> String;
    #[wasm_bindgen(method, setter = "codec")]
    fn set_codec_shim(this: &RtcCodecStats, val: &str);
    #[wasm_bindgen(method, getter = "parameters")]
    fn parameters_shim(this: &RtcCodecStats) -> String;
    #[wasm_bindgen(method, setter = "parameters")]
    fn set_parameters_shim(this: &RtcCodecStats, val: &str);
    #[wasm_bindgen(method, getter = "payloadType")]
    fn payload_type_shim(this: &RtcCodecStats) -> u32;
    #[wasm_bindgen(method, setter = "payloadType")]
    fn set_payload_type_shim(this: &RtcCodecStats, val: u32);
}
#[doc = "The trait to access properties on the `RtcCodecStats` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `RtcCodecStats`*"]
pub trait RtcCodecStatsGetters {
    #[doc = "Get the `id` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcCodecStats`*"]
    fn id(&self) -> String;
    #[doc = "Get the `timestamp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcCodecStats`*"]
    fn timestamp(&self) -> f64;
    #[cfg(feature = "RtcStatsType")]
    #[doc = "Get the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcCodecStats`, `RtcStatsType`*"]
    fn type_(&self) -> RtcStatsType;
    #[doc = "Get the `channels` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcCodecStats`*"]
    fn channels(&self) -> u32;
    #[doc = "Get the `clockRate` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcCodecStats`*"]
    fn clock_rate(&self) -> u32;
    #[doc = "Get the `codec` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcCodecStats`*"]
    fn codec(&self) -> String;
    #[doc = "Get the `parameters` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcCodecStats`*"]
    fn parameters(&self) -> String;
    #[doc = "Get the `payloadType` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcCodecStats`*"]
    fn payload_type(&self) -> u32;
}
impl RtcCodecStatsGetters for RtcCodecStats {
    fn id(&self) -> String {
        self.id_shim()
    }
    fn timestamp(&self) -> f64 {
        self.timestamp_shim()
    }
    #[cfg(feature = "RtcStatsType")]
    fn type_(&self) -> RtcStatsType {
        self.type__shim()
    }
    fn channels(&self) -> u32 {
        self.channels_shim()
    }
    fn clock_rate(&self) -> u32 {
        self.clock_rate_shim()
    }
    fn codec(&self) -> String {
        self.codec_shim()
    }
    fn parameters(&self) -> String {
        self.parameters_shim()
    }
    fn payload_type(&self) -> u32 {
        self.payload_type_shim()
    }
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
        self.set_id_shim(val);
        self
    }
    #[doc = "Change the `timestamp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcCodecStats`*"]
    pub fn timestamp(&mut self, val: f64) -> &mut Self {
        self.set_timestamp_shim(val);
        self
    }
    #[cfg(feature = "RtcStatsType")]
    #[doc = "Change the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcCodecStats`, `RtcStatsType`*"]
    pub fn type_(&mut self, val: RtcStatsType) -> &mut Self {
        self.set_type__shim(val);
        self
    }
    #[doc = "Change the `channels` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcCodecStats`*"]
    pub fn channels(&mut self, val: u32) -> &mut Self {
        self.set_channels_shim(val);
        self
    }
    #[doc = "Change the `clockRate` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcCodecStats`*"]
    pub fn clock_rate(&mut self, val: u32) -> &mut Self {
        self.set_clock_rate_shim(val);
        self
    }
    #[doc = "Change the `codec` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcCodecStats`*"]
    pub fn codec(&mut self, val: &str) -> &mut Self {
        self.set_codec_shim(val);
        self
    }
    #[doc = "Change the `parameters` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcCodecStats`*"]
    pub fn parameters(&mut self, val: &str) -> &mut Self {
        self.set_parameters_shim(val);
        self
    }
    #[doc = "Change the `payloadType` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcCodecStats`*"]
    pub fn payload_type(&mut self, val: u32) -> &mut Self {
        self.set_payload_type_shim(val);
        self
    }
}
impl Default for RtcCodecStats {
    fn default() -> Self {
        Self::new()
    }
}
