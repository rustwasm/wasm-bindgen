#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = RTCIceCandidateStats)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `RtcIceCandidateStats` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidateStats`*"]
    pub type RtcIceCandidateStats;
    #[wasm_bindgen(method, getter = "id")]
    fn id_shim(this: &RtcIceCandidateStats) -> String;
    #[wasm_bindgen(method, setter = "id")]
    fn set_id_shim(this: &RtcIceCandidateStats, val: &str);
    #[wasm_bindgen(method, getter = "timestamp")]
    fn timestamp_shim(this: &RtcIceCandidateStats) -> f64;
    #[wasm_bindgen(method, setter = "timestamp")]
    fn set_timestamp_shim(this: &RtcIceCandidateStats, val: f64);
    #[cfg(feature = "RtcStatsType")]
    #[wasm_bindgen(method, getter = "type")]
    fn type__shim(this: &RtcIceCandidateStats) -> RtcStatsType;
    #[cfg(feature = "RtcStatsType")]
    #[wasm_bindgen(method, setter = "type")]
    fn set_type__shim(this: &RtcIceCandidateStats, val: RtcStatsType);
    #[wasm_bindgen(method, getter = "candidateId")]
    fn candidate_id_shim(this: &RtcIceCandidateStats) -> String;
    #[wasm_bindgen(method, setter = "candidateId")]
    fn set_candidate_id_shim(this: &RtcIceCandidateStats, val: &str);
    #[cfg(feature = "RtcStatsIceCandidateType")]
    #[wasm_bindgen(method, getter = "candidateType")]
    fn candidate_type_shim(this: &RtcIceCandidateStats) -> RtcStatsIceCandidateType;
    #[cfg(feature = "RtcStatsIceCandidateType")]
    #[wasm_bindgen(method, setter = "candidateType")]
    fn set_candidate_type_shim(this: &RtcIceCandidateStats, val: RtcStatsIceCandidateType);
    #[wasm_bindgen(method, getter = "componentId")]
    fn component_id_shim(this: &RtcIceCandidateStats) -> String;
    #[wasm_bindgen(method, setter = "componentId")]
    fn set_component_id_shim(this: &RtcIceCandidateStats, val: &str);
    #[wasm_bindgen(method, getter = "ipAddress")]
    fn ip_address_shim(this: &RtcIceCandidateStats) -> String;
    #[wasm_bindgen(method, setter = "ipAddress")]
    fn set_ip_address_shim(this: &RtcIceCandidateStats, val: &str);
    #[wasm_bindgen(method, getter = "portNumber")]
    fn port_number_shim(this: &RtcIceCandidateStats) -> i32;
    #[wasm_bindgen(method, setter = "portNumber")]
    fn set_port_number_shim(this: &RtcIceCandidateStats, val: i32);
    #[wasm_bindgen(method, getter = "transport")]
    fn transport_shim(this: &RtcIceCandidateStats) -> String;
    #[wasm_bindgen(method, setter = "transport")]
    fn set_transport_shim(this: &RtcIceCandidateStats, val: &str);
}
#[doc = "The trait to access properties on the `RtcIceCandidateStats` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `RtcIceCandidateStats`*"]
pub trait RtcIceCandidateStatsGetters {
    #[doc = "Get the `id` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidateStats`*"]
    fn id(&self) -> String;
    #[doc = "Get the `timestamp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidateStats`*"]
    fn timestamp(&self) -> f64;
    #[cfg(feature = "RtcStatsType")]
    #[doc = "Get the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidateStats`, `RtcStatsType`*"]
    fn type_(&self) -> RtcStatsType;
    #[doc = "Get the `candidateId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidateStats`*"]
    fn candidate_id(&self) -> String;
    #[cfg(feature = "RtcStatsIceCandidateType")]
    #[doc = "Get the `candidateType` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidateStats`, `RtcStatsIceCandidateType`*"]
    fn candidate_type(&self) -> RtcStatsIceCandidateType;
    #[doc = "Get the `componentId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidateStats`*"]
    fn component_id(&self) -> String;
    #[doc = "Get the `ipAddress` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidateStats`*"]
    fn ip_address(&self) -> String;
    #[doc = "Get the `portNumber` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidateStats`*"]
    fn port_number(&self) -> i32;
    #[doc = "Get the `transport` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidateStats`*"]
    fn transport(&self) -> String;
}
impl RtcIceCandidateStatsGetters for RtcIceCandidateStats {
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
    fn candidate_id(&self) -> String {
        self.candidate_id_shim()
    }
    #[cfg(feature = "RtcStatsIceCandidateType")]
    fn candidate_type(&self) -> RtcStatsIceCandidateType {
        self.candidate_type_shim()
    }
    fn component_id(&self) -> String {
        self.component_id_shim()
    }
    fn ip_address(&self) -> String {
        self.ip_address_shim()
    }
    fn port_number(&self) -> i32 {
        self.port_number_shim()
    }
    fn transport(&self) -> String {
        self.transport_shim()
    }
}
impl RtcIceCandidateStats {
    #[doc = "Construct a new `RtcIceCandidateStats`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidateStats`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `id` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidateStats`*"]
    pub fn id(&mut self, val: &str) -> &mut Self {
        self.set_id_shim(val);
        self
    }
    #[doc = "Change the `timestamp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidateStats`*"]
    pub fn timestamp(&mut self, val: f64) -> &mut Self {
        self.set_timestamp_shim(val);
        self
    }
    #[cfg(feature = "RtcStatsType")]
    #[doc = "Change the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidateStats`, `RtcStatsType`*"]
    pub fn type_(&mut self, val: RtcStatsType) -> &mut Self {
        self.set_type__shim(val);
        self
    }
    #[doc = "Change the `candidateId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidateStats`*"]
    pub fn candidate_id(&mut self, val: &str) -> &mut Self {
        self.set_candidate_id_shim(val);
        self
    }
    #[cfg(feature = "RtcStatsIceCandidateType")]
    #[doc = "Change the `candidateType` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidateStats`, `RtcStatsIceCandidateType`*"]
    pub fn candidate_type(&mut self, val: RtcStatsIceCandidateType) -> &mut Self {
        self.set_candidate_type_shim(val);
        self
    }
    #[doc = "Change the `componentId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidateStats`*"]
    pub fn component_id(&mut self, val: &str) -> &mut Self {
        self.set_component_id_shim(val);
        self
    }
    #[doc = "Change the `ipAddress` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidateStats`*"]
    pub fn ip_address(&mut self, val: &str) -> &mut Self {
        self.set_ip_address_shim(val);
        self
    }
    #[doc = "Change the `portNumber` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidateStats`*"]
    pub fn port_number(&mut self, val: i32) -> &mut Self {
        self.set_port_number_shim(val);
        self
    }
    #[doc = "Change the `transport` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidateStats`*"]
    pub fn transport(&mut self, val: &str) -> &mut Self {
        self.set_transport_shim(val);
        self
    }
}
impl Default for RtcIceCandidateStats {
    fn default() -> Self {
        Self::new()
    }
}
