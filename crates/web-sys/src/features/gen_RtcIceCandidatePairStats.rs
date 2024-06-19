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
    #[doc = "Get the `id` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidatePairStats`*"]
    #[wasm_bindgen(method, getter = "id")]
    pub fn get_id(this: &RtcIceCandidatePairStats) -> Option<String>;
    #[wasm_bindgen(method, setter = "id")]
    fn set_id(this: &RtcIceCandidatePairStats, val: &str);
    #[doc = "Get the `timestamp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidatePairStats`*"]
    #[wasm_bindgen(method, getter = "timestamp")]
    pub fn get_timestamp(this: &RtcIceCandidatePairStats) -> Option<f64>;
    #[wasm_bindgen(method, setter = "timestamp")]
    fn set_timestamp(this: &RtcIceCandidatePairStats, val: f64);
    #[cfg(feature = "RtcStatsType")]
    #[doc = "Get the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidatePairStats`, `RtcStatsType`*"]
    #[wasm_bindgen(method, getter = "type")]
    pub fn get_type(this: &RtcIceCandidatePairStats) -> Option<RtcStatsType>;
    #[cfg(feature = "RtcStatsType")]
    #[wasm_bindgen(method, setter = "type")]
    fn set_type(this: &RtcIceCandidatePairStats, val: RtcStatsType);
    #[doc = "Get the `bytesReceived` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidatePairStats`*"]
    #[wasm_bindgen(method, getter = "bytesReceived")]
    pub fn get_bytes_received(this: &RtcIceCandidatePairStats) -> Option<f64>;
    #[wasm_bindgen(method, setter = "bytesReceived")]
    fn set_bytes_received(this: &RtcIceCandidatePairStats, val: f64);
    #[doc = "Get the `bytesSent` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidatePairStats`*"]
    #[wasm_bindgen(method, getter = "bytesSent")]
    pub fn get_bytes_sent(this: &RtcIceCandidatePairStats) -> Option<f64>;
    #[wasm_bindgen(method, setter = "bytesSent")]
    fn set_bytes_sent(this: &RtcIceCandidatePairStats, val: f64);
    #[doc = "Get the `componentId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidatePairStats`*"]
    #[wasm_bindgen(method, getter = "componentId")]
    pub fn get_component_id(this: &RtcIceCandidatePairStats) -> Option<u32>;
    #[wasm_bindgen(method, setter = "componentId")]
    fn set_component_id(this: &RtcIceCandidatePairStats, val: u32);
    #[doc = "Get the `lastPacketReceivedTimestamp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidatePairStats`*"]
    #[wasm_bindgen(method, getter = "lastPacketReceivedTimestamp")]
    pub fn get_last_packet_received_timestamp(this: &RtcIceCandidatePairStats) -> Option<f64>;
    #[wasm_bindgen(method, setter = "lastPacketReceivedTimestamp")]
    fn set_last_packet_received_timestamp(this: &RtcIceCandidatePairStats, val: f64);
    #[doc = "Get the `lastPacketSentTimestamp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidatePairStats`*"]
    #[wasm_bindgen(method, getter = "lastPacketSentTimestamp")]
    pub fn get_last_packet_sent_timestamp(this: &RtcIceCandidatePairStats) -> Option<f64>;
    #[wasm_bindgen(method, setter = "lastPacketSentTimestamp")]
    fn set_last_packet_sent_timestamp(this: &RtcIceCandidatePairStats, val: f64);
    #[doc = "Get the `localCandidateId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidatePairStats`*"]
    #[wasm_bindgen(method, getter = "localCandidateId")]
    pub fn get_local_candidate_id(this: &RtcIceCandidatePairStats) -> Option<String>;
    #[wasm_bindgen(method, setter = "localCandidateId")]
    fn set_local_candidate_id(this: &RtcIceCandidatePairStats, val: &str);
    #[doc = "Get the `nominated` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidatePairStats`*"]
    #[wasm_bindgen(method, getter = "nominated")]
    pub fn get_nominated(this: &RtcIceCandidatePairStats) -> Option<bool>;
    #[wasm_bindgen(method, setter = "nominated")]
    fn set_nominated(this: &RtcIceCandidatePairStats, val: bool);
    #[doc = "Get the `priority` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidatePairStats`*"]
    #[wasm_bindgen(method, getter = "priority")]
    pub fn get_priority(this: &RtcIceCandidatePairStats) -> Option<f64>;
    #[wasm_bindgen(method, setter = "priority")]
    fn set_priority(this: &RtcIceCandidatePairStats, val: f64);
    #[doc = "Get the `readable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidatePairStats`*"]
    #[wasm_bindgen(method, getter = "readable")]
    pub fn get_readable(this: &RtcIceCandidatePairStats) -> Option<bool>;
    #[wasm_bindgen(method, setter = "readable")]
    fn set_readable(this: &RtcIceCandidatePairStats, val: bool);
    #[doc = "Get the `remoteCandidateId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidatePairStats`*"]
    #[wasm_bindgen(method, getter = "remoteCandidateId")]
    pub fn get_remote_candidate_id(this: &RtcIceCandidatePairStats) -> Option<String>;
    #[wasm_bindgen(method, setter = "remoteCandidateId")]
    fn set_remote_candidate_id(this: &RtcIceCandidatePairStats, val: &str);
    #[doc = "Get the `selected` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidatePairStats`*"]
    #[wasm_bindgen(method, getter = "selected")]
    pub fn get_selected(this: &RtcIceCandidatePairStats) -> Option<bool>;
    #[wasm_bindgen(method, setter = "selected")]
    fn set_selected(this: &RtcIceCandidatePairStats, val: bool);
    #[cfg(feature = "RtcStatsIceCandidatePairState")]
    #[doc = "Get the `state` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidatePairStats`, `RtcStatsIceCandidatePairState`*"]
    #[wasm_bindgen(method, getter = "state")]
    pub fn get_state(this: &RtcIceCandidatePairStats) -> Option<RtcStatsIceCandidatePairState>;
    #[cfg(feature = "RtcStatsIceCandidatePairState")]
    #[wasm_bindgen(method, setter = "state")]
    fn set_state(this: &RtcIceCandidatePairStats, val: RtcStatsIceCandidatePairState);
    #[doc = "Get the `transportId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidatePairStats`*"]
    #[wasm_bindgen(method, getter = "transportId")]
    pub fn get_transport_id(this: &RtcIceCandidatePairStats) -> Option<String>;
    #[wasm_bindgen(method, setter = "transportId")]
    fn set_transport_id(this: &RtcIceCandidatePairStats, val: &str);
    #[doc = "Get the `writable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidatePairStats`*"]
    #[wasm_bindgen(method, getter = "writable")]
    pub fn get_writable(this: &RtcIceCandidatePairStats) -> Option<bool>;
    #[wasm_bindgen(method, setter = "writable")]
    fn set_writable(this: &RtcIceCandidatePairStats, val: bool);
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
        self.set_id(val);
        self
    }
    #[doc = "Change the `timestamp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidatePairStats`*"]
    pub fn timestamp(&mut self, val: f64) -> &mut Self {
        self.set_timestamp(val);
        self
    }
    #[cfg(feature = "RtcStatsType")]
    #[doc = "Change the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidatePairStats`, `RtcStatsType`*"]
    pub fn type_(&mut self, val: RtcStatsType) -> &mut Self {
        self.set_type(val);
        self
    }
    #[doc = "Change the `bytesReceived` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidatePairStats`*"]
    pub fn bytes_received(&mut self, val: f64) -> &mut Self {
        self.set_bytes_received(val);
        self
    }
    #[doc = "Change the `bytesSent` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidatePairStats`*"]
    pub fn bytes_sent(&mut self, val: f64) -> &mut Self {
        self.set_bytes_sent(val);
        self
    }
    #[doc = "Change the `componentId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidatePairStats`*"]
    pub fn component_id(&mut self, val: u32) -> &mut Self {
        self.set_component_id(val);
        self
    }
    #[doc = "Change the `lastPacketReceivedTimestamp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidatePairStats`*"]
    pub fn last_packet_received_timestamp(&mut self, val: f64) -> &mut Self {
        self.set_last_packet_received_timestamp(val);
        self
    }
    #[doc = "Change the `lastPacketSentTimestamp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidatePairStats`*"]
    pub fn last_packet_sent_timestamp(&mut self, val: f64) -> &mut Self {
        self.set_last_packet_sent_timestamp(val);
        self
    }
    #[doc = "Change the `localCandidateId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidatePairStats`*"]
    pub fn local_candidate_id(&mut self, val: &str) -> &mut Self {
        self.set_local_candidate_id(val);
        self
    }
    #[doc = "Change the `nominated` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidatePairStats`*"]
    pub fn nominated(&mut self, val: bool) -> &mut Self {
        self.set_nominated(val);
        self
    }
    #[doc = "Change the `priority` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidatePairStats`*"]
    pub fn priority(&mut self, val: f64) -> &mut Self {
        self.set_priority(val);
        self
    }
    #[doc = "Change the `readable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidatePairStats`*"]
    pub fn readable(&mut self, val: bool) -> &mut Self {
        self.set_readable(val);
        self
    }
    #[doc = "Change the `remoteCandidateId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidatePairStats`*"]
    pub fn remote_candidate_id(&mut self, val: &str) -> &mut Self {
        self.set_remote_candidate_id(val);
        self
    }
    #[doc = "Change the `selected` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidatePairStats`*"]
    pub fn selected(&mut self, val: bool) -> &mut Self {
        self.set_selected(val);
        self
    }
    #[cfg(feature = "RtcStatsIceCandidatePairState")]
    #[doc = "Change the `state` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidatePairStats`, `RtcStatsIceCandidatePairState`*"]
    pub fn state(&mut self, val: RtcStatsIceCandidatePairState) -> &mut Self {
        self.set_state(val);
        self
    }
    #[doc = "Change the `transportId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidatePairStats`*"]
    pub fn transport_id(&mut self, val: &str) -> &mut Self {
        self.set_transport_id(val);
        self
    }
    #[doc = "Change the `writable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidatePairStats`*"]
    pub fn writable(&mut self, val: bool) -> &mut Self {
        self.set_writable(val);
        self
    }
}
impl Default for RtcIceCandidatePairStats {
    fn default() -> Self {
        Self::new()
    }
}
