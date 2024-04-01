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
    #[wasm_bindgen(method, setter = "id")]
    fn id_shim(this: &RtcIceCandidateStats, val: &str);
    #[wasm_bindgen(method, setter = "timestamp")]
    fn timestamp_shim(this: &RtcIceCandidateStats, val: f64);
    #[cfg(feature = "RtcStatsType")]
    #[wasm_bindgen(method, setter = "type")]
    fn type__shim(this: &RtcIceCandidateStats, val: RtcStatsType);
    #[wasm_bindgen(method, setter = "candidateId")]
    fn candidate_id_shim(this: &RtcIceCandidateStats, val: &str);
    #[cfg(feature = "RtcStatsIceCandidateType")]
    #[wasm_bindgen(method, setter = "candidateType")]
    fn candidate_type_shim(this: &RtcIceCandidateStats, val: RtcStatsIceCandidateType);
    #[wasm_bindgen(method, setter = "componentId")]
    fn component_id_shim(this: &RtcIceCandidateStats, val: &str);
    #[wasm_bindgen(method, setter = "ipAddress")]
    fn ip_address_shim(this: &RtcIceCandidateStats, val: &str);
    #[wasm_bindgen(method, setter = "portNumber")]
    fn port_number_shim(this: &RtcIceCandidateStats, val: i32);
    #[wasm_bindgen(method, setter = "transport")]
    fn transport_shim(this: &RtcIceCandidateStats, val: &str);
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
        self.id_shim(val);
        self
    }
    #[doc = "Change the `timestamp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidateStats`*"]
    pub fn timestamp(&mut self, val: f64) -> &mut Self {
        self.timestamp_shim(val);
        self
    }
    #[cfg(feature = "RtcStatsType")]
    #[doc = "Change the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidateStats`, `RtcStatsType`*"]
    pub fn type_(&mut self, val: RtcStatsType) -> &mut Self {
        self.type__shim(val);
        self
    }
    #[doc = "Change the `candidateId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidateStats`*"]
    pub fn candidate_id(&mut self, val: &str) -> &mut Self {
        self.candidate_id_shim(val);
        self
    }
    #[cfg(feature = "RtcStatsIceCandidateType")]
    #[doc = "Change the `candidateType` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidateStats`, `RtcStatsIceCandidateType`*"]
    pub fn candidate_type(&mut self, val: RtcStatsIceCandidateType) -> &mut Self {
        self.candidate_type_shim(val);
        self
    }
    #[doc = "Change the `componentId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidateStats`*"]
    pub fn component_id(&mut self, val: &str) -> &mut Self {
        self.component_id_shim(val);
        self
    }
    #[doc = "Change the `ipAddress` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidateStats`*"]
    pub fn ip_address(&mut self, val: &str) -> &mut Self {
        self.ip_address_shim(val);
        self
    }
    #[doc = "Change the `portNumber` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidateStats`*"]
    pub fn port_number(&mut self, val: i32) -> &mut Self {
        self.port_number_shim(val);
        self
    }
    #[doc = "Change the `transport` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidateStats`*"]
    pub fn transport(&mut self, val: &str) -> &mut Self {
        self.transport_shim(val);
        self
    }
}
impl Default for RtcIceCandidateStats {
    fn default() -> Self {
        Self::new()
    }
}
