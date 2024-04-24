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
    #[wasm_bindgen(method, getter = "id")]
    fn id_shim(this: &RtcIceCandidatePairStats) -> String;
    #[wasm_bindgen(method, setter = "id")]
    fn set_id_shim(this: &RtcIceCandidatePairStats, val: &str);
    #[wasm_bindgen(method, getter = "timestamp")]
    fn timestamp_shim(this: &RtcIceCandidatePairStats) -> f64;
    #[wasm_bindgen(method, setter = "timestamp")]
    fn set_timestamp_shim(this: &RtcIceCandidatePairStats, val: f64);
    #[cfg(feature = "RtcStatsType")]
    #[wasm_bindgen(method, getter = "type")]
    fn type__shim(this: &RtcIceCandidatePairStats) -> RtcStatsType;
    #[cfg(feature = "RtcStatsType")]
    #[wasm_bindgen(method, setter = "type")]
    fn set_type__shim(this: &RtcIceCandidatePairStats, val: RtcStatsType);
    #[wasm_bindgen(method, getter = "bytesReceived")]
    fn bytes_received_shim(this: &RtcIceCandidatePairStats) -> f64;
    #[wasm_bindgen(method, setter = "bytesReceived")]
    fn set_bytes_received_shim(this: &RtcIceCandidatePairStats, val: f64);
    #[wasm_bindgen(method, getter = "bytesSent")]
    fn bytes_sent_shim(this: &RtcIceCandidatePairStats) -> f64;
    #[wasm_bindgen(method, setter = "bytesSent")]
    fn set_bytes_sent_shim(this: &RtcIceCandidatePairStats, val: f64);
    #[wasm_bindgen(method, getter = "componentId")]
    fn component_id_shim(this: &RtcIceCandidatePairStats) -> u32;
    #[wasm_bindgen(method, setter = "componentId")]
    fn set_component_id_shim(this: &RtcIceCandidatePairStats, val: u32);
    #[wasm_bindgen(method, getter = "lastPacketReceivedTimestamp")]
    fn last_packet_received_timestamp_shim(this: &RtcIceCandidatePairStats) -> f64;
    #[wasm_bindgen(method, setter = "lastPacketReceivedTimestamp")]
    fn set_last_packet_received_timestamp_shim(this: &RtcIceCandidatePairStats, val: f64);
    #[wasm_bindgen(method, getter = "lastPacketSentTimestamp")]
    fn last_packet_sent_timestamp_shim(this: &RtcIceCandidatePairStats) -> f64;
    #[wasm_bindgen(method, setter = "lastPacketSentTimestamp")]
    fn set_last_packet_sent_timestamp_shim(this: &RtcIceCandidatePairStats, val: f64);
    #[wasm_bindgen(method, getter = "localCandidateId")]
    fn local_candidate_id_shim(this: &RtcIceCandidatePairStats) -> String;
    #[wasm_bindgen(method, setter = "localCandidateId")]
    fn set_local_candidate_id_shim(this: &RtcIceCandidatePairStats, val: &str);
    #[wasm_bindgen(method, getter = "nominated")]
    fn nominated_shim(this: &RtcIceCandidatePairStats) -> bool;
    #[wasm_bindgen(method, setter = "nominated")]
    fn set_nominated_shim(this: &RtcIceCandidatePairStats, val: bool);
    #[wasm_bindgen(method, getter = "priority")]
    fn priority_shim(this: &RtcIceCandidatePairStats) -> f64;
    #[wasm_bindgen(method, setter = "priority")]
    fn set_priority_shim(this: &RtcIceCandidatePairStats, val: f64);
    #[wasm_bindgen(method, getter = "readable")]
    fn readable_shim(this: &RtcIceCandidatePairStats) -> bool;
    #[wasm_bindgen(method, setter = "readable")]
    fn set_readable_shim(this: &RtcIceCandidatePairStats, val: bool);
    #[wasm_bindgen(method, getter = "remoteCandidateId")]
    fn remote_candidate_id_shim(this: &RtcIceCandidatePairStats) -> String;
    #[wasm_bindgen(method, setter = "remoteCandidateId")]
    fn set_remote_candidate_id_shim(this: &RtcIceCandidatePairStats, val: &str);
    #[wasm_bindgen(method, getter = "selected")]
    fn selected_shim(this: &RtcIceCandidatePairStats) -> bool;
    #[wasm_bindgen(method, setter = "selected")]
    fn set_selected_shim(this: &RtcIceCandidatePairStats, val: bool);
    #[cfg(feature = "RtcStatsIceCandidatePairState")]
    #[wasm_bindgen(method, getter = "state")]
    fn state_shim(this: &RtcIceCandidatePairStats) -> RtcStatsIceCandidatePairState;
    #[cfg(feature = "RtcStatsIceCandidatePairState")]
    #[wasm_bindgen(method, setter = "state")]
    fn set_state_shim(this: &RtcIceCandidatePairStats, val: RtcStatsIceCandidatePairState);
    #[wasm_bindgen(method, getter = "transportId")]
    fn transport_id_shim(this: &RtcIceCandidatePairStats) -> String;
    #[wasm_bindgen(method, setter = "transportId")]
    fn set_transport_id_shim(this: &RtcIceCandidatePairStats, val: &str);
    #[wasm_bindgen(method, getter = "writable")]
    fn writable_shim(this: &RtcIceCandidatePairStats) -> bool;
    #[wasm_bindgen(method, setter = "writable")]
    fn set_writable_shim(this: &RtcIceCandidatePairStats, val: bool);
}
#[doc = "The trait to access properties on the `RtcIceCandidatePairStats` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `RtcIceCandidatePairStats`*"]
pub trait RtcIceCandidatePairStatsGetters {
    #[doc = "Get the `id` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidatePairStats`*"]
    fn id(&self) -> String;
    #[doc = "Get the `timestamp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidatePairStats`*"]
    fn timestamp(&self) -> f64;
    #[cfg(feature = "RtcStatsType")]
    #[doc = "Get the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidatePairStats`, `RtcStatsType`*"]
    fn type_(&self) -> RtcStatsType;
    #[doc = "Get the `bytesReceived` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidatePairStats`*"]
    fn bytes_received(&self) -> f64;
    #[doc = "Get the `bytesSent` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidatePairStats`*"]
    fn bytes_sent(&self) -> f64;
    #[doc = "Get the `componentId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidatePairStats`*"]
    fn component_id(&self) -> u32;
    #[doc = "Get the `lastPacketReceivedTimestamp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidatePairStats`*"]
    fn last_packet_received_timestamp(&self) -> f64;
    #[doc = "Get the `lastPacketSentTimestamp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidatePairStats`*"]
    fn last_packet_sent_timestamp(&self) -> f64;
    #[doc = "Get the `localCandidateId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidatePairStats`*"]
    fn local_candidate_id(&self) -> String;
    #[doc = "Get the `nominated` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidatePairStats`*"]
    fn nominated(&self) -> bool;
    #[doc = "Get the `priority` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidatePairStats`*"]
    fn priority(&self) -> f64;
    #[doc = "Get the `readable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidatePairStats`*"]
    fn readable(&self) -> bool;
    #[doc = "Get the `remoteCandidateId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidatePairStats`*"]
    fn remote_candidate_id(&self) -> String;
    #[doc = "Get the `selected` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidatePairStats`*"]
    fn selected(&self) -> bool;
    #[cfg(feature = "RtcStatsIceCandidatePairState")]
    #[doc = "Get the `state` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidatePairStats`, `RtcStatsIceCandidatePairState`*"]
    fn state(&self) -> RtcStatsIceCandidatePairState;
    #[doc = "Get the `transportId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidatePairStats`*"]
    fn transport_id(&self) -> String;
    #[doc = "Get the `writable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidatePairStats`*"]
    fn writable(&self) -> bool;
}
impl RtcIceCandidatePairStatsGetters for RtcIceCandidatePairStats {
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
    fn bytes_received(&self) -> f64 {
        self.bytes_received_shim()
    }
    fn bytes_sent(&self) -> f64 {
        self.bytes_sent_shim()
    }
    fn component_id(&self) -> u32 {
        self.component_id_shim()
    }
    fn last_packet_received_timestamp(&self) -> f64 {
        self.last_packet_received_timestamp_shim()
    }
    fn last_packet_sent_timestamp(&self) -> f64 {
        self.last_packet_sent_timestamp_shim()
    }
    fn local_candidate_id(&self) -> String {
        self.local_candidate_id_shim()
    }
    fn nominated(&self) -> bool {
        self.nominated_shim()
    }
    fn priority(&self) -> f64 {
        self.priority_shim()
    }
    fn readable(&self) -> bool {
        self.readable_shim()
    }
    fn remote_candidate_id(&self) -> String {
        self.remote_candidate_id_shim()
    }
    fn selected(&self) -> bool {
        self.selected_shim()
    }
    #[cfg(feature = "RtcStatsIceCandidatePairState")]
    fn state(&self) -> RtcStatsIceCandidatePairState {
        self.state_shim()
    }
    fn transport_id(&self) -> String {
        self.transport_id_shim()
    }
    fn writable(&self) -> bool {
        self.writable_shim()
    }
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
        self.set_id_shim(val);
        self
    }
    #[doc = "Change the `timestamp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidatePairStats`*"]
    pub fn timestamp(&mut self, val: f64) -> &mut Self {
        self.set_timestamp_shim(val);
        self
    }
    #[cfg(feature = "RtcStatsType")]
    #[doc = "Change the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidatePairStats`, `RtcStatsType`*"]
    pub fn type_(&mut self, val: RtcStatsType) -> &mut Self {
        self.set_type__shim(val);
        self
    }
    #[doc = "Change the `bytesReceived` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidatePairStats`*"]
    pub fn bytes_received(&mut self, val: f64) -> &mut Self {
        self.set_bytes_received_shim(val);
        self
    }
    #[doc = "Change the `bytesSent` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidatePairStats`*"]
    pub fn bytes_sent(&mut self, val: f64) -> &mut Self {
        self.set_bytes_sent_shim(val);
        self
    }
    #[doc = "Change the `componentId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidatePairStats`*"]
    pub fn component_id(&mut self, val: u32) -> &mut Self {
        self.set_component_id_shim(val);
        self
    }
    #[doc = "Change the `lastPacketReceivedTimestamp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidatePairStats`*"]
    pub fn last_packet_received_timestamp(&mut self, val: f64) -> &mut Self {
        self.set_last_packet_received_timestamp_shim(val);
        self
    }
    #[doc = "Change the `lastPacketSentTimestamp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidatePairStats`*"]
    pub fn last_packet_sent_timestamp(&mut self, val: f64) -> &mut Self {
        self.set_last_packet_sent_timestamp_shim(val);
        self
    }
    #[doc = "Change the `localCandidateId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidatePairStats`*"]
    pub fn local_candidate_id(&mut self, val: &str) -> &mut Self {
        self.set_local_candidate_id_shim(val);
        self
    }
    #[doc = "Change the `nominated` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidatePairStats`*"]
    pub fn nominated(&mut self, val: bool) -> &mut Self {
        self.set_nominated_shim(val);
        self
    }
    #[doc = "Change the `priority` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidatePairStats`*"]
    pub fn priority(&mut self, val: f64) -> &mut Self {
        self.set_priority_shim(val);
        self
    }
    #[doc = "Change the `readable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidatePairStats`*"]
    pub fn readable(&mut self, val: bool) -> &mut Self {
        self.set_readable_shim(val);
        self
    }
    #[doc = "Change the `remoteCandidateId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidatePairStats`*"]
    pub fn remote_candidate_id(&mut self, val: &str) -> &mut Self {
        self.set_remote_candidate_id_shim(val);
        self
    }
    #[doc = "Change the `selected` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidatePairStats`*"]
    pub fn selected(&mut self, val: bool) -> &mut Self {
        self.set_selected_shim(val);
        self
    }
    #[cfg(feature = "RtcStatsIceCandidatePairState")]
    #[doc = "Change the `state` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidatePairStats`, `RtcStatsIceCandidatePairState`*"]
    pub fn state(&mut self, val: RtcStatsIceCandidatePairState) -> &mut Self {
        self.set_state_shim(val);
        self
    }
    #[doc = "Change the `transportId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidatePairStats`*"]
    pub fn transport_id(&mut self, val: &str) -> &mut Self {
        self.set_transport_id_shim(val);
        self
    }
    #[doc = "Change the `writable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidatePairStats`*"]
    pub fn writable(&mut self, val: bool) -> &mut Self {
        self.set_writable_shim(val);
        self
    }
}
impl Default for RtcIceCandidatePairStats {
    fn default() -> Self {
        Self::new()
    }
}
