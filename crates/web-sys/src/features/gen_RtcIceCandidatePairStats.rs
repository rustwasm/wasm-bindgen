#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = RTCIceCandidatePairStats)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `RtcIceCandidatePairStats` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidatePairStats`*"]
    pub type RtcIceCandidatePairStats;
    #[wasm_bindgen(method, setter = "id")]
    fn id_shim(this: &RtcIceCandidatePairStats, val: &str);
    #[wasm_bindgen(method, setter = "timestamp")]
    fn timestamp_shim(this: &RtcIceCandidatePairStats, val: f64);
    #[cfg(feature = "RtcStatsType")]
    #[wasm_bindgen(method, setter = "type")]
    fn type__shim(this: &RtcIceCandidatePairStats, val: RtcStatsType);
    #[wasm_bindgen(method, setter = "bytesReceived")]
    fn bytes_received_shim(this: &RtcIceCandidatePairStats, val: f64);
    #[wasm_bindgen(method, setter = "bytesSent")]
    fn bytes_sent_shim(this: &RtcIceCandidatePairStats, val: f64);
    #[wasm_bindgen(method, setter = "componentId")]
    fn component_id_shim(this: &RtcIceCandidatePairStats, val: u32);
    #[wasm_bindgen(method, setter = "lastPacketReceivedTimestamp")]
    fn last_packet_received_timestamp_shim(this: &RtcIceCandidatePairStats, val: f64);
    #[wasm_bindgen(method, setter = "lastPacketSentTimestamp")]
    fn last_packet_sent_timestamp_shim(this: &RtcIceCandidatePairStats, val: f64);
    #[wasm_bindgen(method, setter = "localCandidateId")]
    fn local_candidate_id_shim(this: &RtcIceCandidatePairStats, val: &str);
    #[wasm_bindgen(method, setter = "nominated")]
    fn nominated_shim(this: &RtcIceCandidatePairStats, val: bool);
    #[wasm_bindgen(method, setter = "priority")]
    fn priority_shim(this: &RtcIceCandidatePairStats, val: f64);
    #[wasm_bindgen(method, setter = "readable")]
    fn readable_shim(this: &RtcIceCandidatePairStats, val: bool);
    #[wasm_bindgen(method, setter = "remoteCandidateId")]
    fn remote_candidate_id_shim(this: &RtcIceCandidatePairStats, val: &str);
    #[wasm_bindgen(method, setter = "selected")]
    fn selected_shim(this: &RtcIceCandidatePairStats, val: bool);
    #[cfg(feature = "RtcStatsIceCandidatePairState")]
    #[wasm_bindgen(method, setter = "state")]
    fn state_shim(this: &RtcIceCandidatePairStats, val: RtcStatsIceCandidatePairState);
    #[wasm_bindgen(method, setter = "transportId")]
    fn transport_id_shim(this: &RtcIceCandidatePairStats, val: &str);
    #[wasm_bindgen(method, setter = "writable")]
    fn writable_shim(this: &RtcIceCandidatePairStats, val: bool);
}
impl RtcIceCandidatePairStats {
    #[doc = "Construct a new `RtcIceCandidatePairStats`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidatePairStats`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `id` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidatePairStats`*"]
    pub fn id(&mut self, val: &str) -> &mut Self {
        self.id_shim(val);
        self
    }
    #[doc = "Change the `timestamp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidatePairStats`*"]
    pub fn timestamp(&mut self, val: f64) -> &mut Self {
        self.timestamp_shim(val);
        self
    }
    #[cfg(feature = "RtcStatsType")]
    #[doc = "Change the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidatePairStats`, `RtcStatsType`*"]
    pub fn type_(&mut self, val: RtcStatsType) -> &mut Self {
        self.type__shim(val);
        self
    }
    #[doc = "Change the `bytesReceived` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidatePairStats`*"]
    pub fn bytes_received(&mut self, val: f64) -> &mut Self {
        self.bytes_received_shim(val);
        self
    }
    #[doc = "Change the `bytesSent` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidatePairStats`*"]
    pub fn bytes_sent(&mut self, val: f64) -> &mut Self {
        self.bytes_sent_shim(val);
        self
    }
    #[doc = "Change the `componentId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidatePairStats`*"]
    pub fn component_id(&mut self, val: u32) -> &mut Self {
        self.component_id_shim(val);
        self
    }
    #[doc = "Change the `lastPacketReceivedTimestamp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidatePairStats`*"]
    pub fn last_packet_received_timestamp(&mut self, val: f64) -> &mut Self {
        self.last_packet_received_timestamp_shim(val);
        self
    }
    #[doc = "Change the `lastPacketSentTimestamp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidatePairStats`*"]
    pub fn last_packet_sent_timestamp(&mut self, val: f64) -> &mut Self {
        self.last_packet_sent_timestamp_shim(val);
        self
    }
    #[doc = "Change the `localCandidateId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidatePairStats`*"]
    pub fn local_candidate_id(&mut self, val: &str) -> &mut Self {
        self.local_candidate_id_shim(val);
        self
    }
    #[doc = "Change the `nominated` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidatePairStats`*"]
    pub fn nominated(&mut self, val: bool) -> &mut Self {
        self.nominated_shim(val);
        self
    }
    #[doc = "Change the `priority` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidatePairStats`*"]
    pub fn priority(&mut self, val: f64) -> &mut Self {
        self.priority_shim(val);
        self
    }
    #[doc = "Change the `readable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidatePairStats`*"]
    pub fn readable(&mut self, val: bool) -> &mut Self {
        self.readable_shim(val);
        self
    }
    #[doc = "Change the `remoteCandidateId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidatePairStats`*"]
    pub fn remote_candidate_id(&mut self, val: &str) -> &mut Self {
        self.remote_candidate_id_shim(val);
        self
    }
    #[doc = "Change the `selected` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidatePairStats`*"]
    pub fn selected(&mut self, val: bool) -> &mut Self {
        self.selected_shim(val);
        self
    }
    #[cfg(feature = "RtcStatsIceCandidatePairState")]
    #[doc = "Change the `state` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidatePairStats`, `RtcStatsIceCandidatePairState`*"]
    pub fn state(&mut self, val: RtcStatsIceCandidatePairState) -> &mut Self {
        self.state_shim(val);
        self
    }
    #[doc = "Change the `transportId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidatePairStats`*"]
    pub fn transport_id(&mut self, val: &str) -> &mut Self {
        self.transport_id_shim(val);
        self
    }
    #[doc = "Change the `writable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidatePairStats`*"]
    pub fn writable(&mut self, val: bool) -> &mut Self {
        self.writable_shim(val);
        self
    }
}
impl Default for RtcIceCandidatePairStats {
    fn default() -> Self {
        Self::new()
    }
}
