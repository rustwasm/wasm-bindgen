#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = RTCTransportStats)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `RtcTransportStats` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcTransportStats`*"]
    pub type RtcTransportStats;
    #[wasm_bindgen(method, setter = "id")]
    fn id_shim(this: &RtcTransportStats, val: &str);
    #[wasm_bindgen(method, setter = "timestamp")]
    fn timestamp_shim(this: &RtcTransportStats, val: f64);
    #[cfg(feature = "RtcStatsType")]
    #[wasm_bindgen(method, setter = "type")]
    fn type__shim(this: &RtcTransportStats, val: RtcStatsType);
    #[wasm_bindgen(method, setter = "bytesReceived")]
    fn bytes_received_shim(this: &RtcTransportStats, val: u32);
    #[wasm_bindgen(method, setter = "bytesSent")]
    fn bytes_sent_shim(this: &RtcTransportStats, val: u32);
}
impl RtcTransportStats {
    #[doc = "Construct a new `RtcTransportStats`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcTransportStats`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `id` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcTransportStats`*"]
    pub fn id(&mut self, val: &str) -> &mut Self {
        self.id_shim(val);
        self
    }
    #[doc = "Change the `timestamp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcTransportStats`*"]
    pub fn timestamp(&mut self, val: f64) -> &mut Self {
        self.timestamp_shim(val);
        self
    }
    #[cfg(feature = "RtcStatsType")]
    #[doc = "Change the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcStatsType`, `RtcTransportStats`*"]
    pub fn type_(&mut self, val: RtcStatsType) -> &mut Self {
        self.type__shim(val);
        self
    }
    #[doc = "Change the `bytesReceived` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcTransportStats`*"]
    pub fn bytes_received(&mut self, val: u32) -> &mut Self {
        self.bytes_received_shim(val);
        self
    }
    #[doc = "Change the `bytesSent` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcTransportStats`*"]
    pub fn bytes_sent(&mut self, val: u32) -> &mut Self {
        self.bytes_sent_shim(val);
        self
    }
}
impl Default for RtcTransportStats {
    fn default() -> Self {
        Self::new()
    }
}
