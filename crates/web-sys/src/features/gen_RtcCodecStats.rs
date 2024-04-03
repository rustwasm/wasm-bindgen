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
    #[wasm_bindgen(method, setter = "id")]
    fn id_shim(this: &RtcCodecStats, val: &str);
    #[wasm_bindgen(method, setter = "timestamp")]
    fn timestamp_shim(this: &RtcCodecStats, val: f64);
    #[cfg(feature = "RtcStatsType")]
    #[wasm_bindgen(method, setter = "type")]
    fn type__shim(this: &RtcCodecStats, val: RtcStatsType);
    #[wasm_bindgen(method, setter = "channels")]
    fn channels_shim(this: &RtcCodecStats, val: u32);
    #[wasm_bindgen(method, setter = "clockRate")]
    fn clock_rate_shim(this: &RtcCodecStats, val: u32);
    #[wasm_bindgen(method, setter = "codec")]
    fn codec_shim(this: &RtcCodecStats, val: &str);
    #[wasm_bindgen(method, setter = "parameters")]
    fn parameters_shim(this: &RtcCodecStats, val: &str);
    #[wasm_bindgen(method, setter = "payloadType")]
    fn payload_type_shim(this: &RtcCodecStats, val: u32);
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
        self.id_shim(val);
        self
    }
    #[doc = "Change the `timestamp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcCodecStats`*"]
    pub fn timestamp(&mut self, val: f64) -> &mut Self {
        self.timestamp_shim(val);
        self
    }
    #[cfg(feature = "RtcStatsType")]
    #[doc = "Change the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcCodecStats`, `RtcStatsType`*"]
    pub fn type_(&mut self, val: RtcStatsType) -> &mut Self {
        self.type__shim(val);
        self
    }
    #[doc = "Change the `channels` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcCodecStats`*"]
    pub fn channels(&mut self, val: u32) -> &mut Self {
        self.channels_shim(val);
        self
    }
    #[doc = "Change the `clockRate` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcCodecStats`*"]
    pub fn clock_rate(&mut self, val: u32) -> &mut Self {
        self.clock_rate_shim(val);
        self
    }
    #[doc = "Change the `codec` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcCodecStats`*"]
    pub fn codec(&mut self, val: &str) -> &mut Self {
        self.codec_shim(val);
        self
    }
    #[doc = "Change the `parameters` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcCodecStats`*"]
    pub fn parameters(&mut self, val: &str) -> &mut Self {
        self.parameters_shim(val);
        self
    }
    #[doc = "Change the `payloadType` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcCodecStats`*"]
    pub fn payload_type(&mut self, val: u32) -> &mut Self {
        self.payload_type_shim(val);
        self
    }
}
impl Default for RtcCodecStats {
    fn default() -> Self {
        Self::new()
    }
}
