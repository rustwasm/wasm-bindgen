#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = RTCRTPContributingSourceStats)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `RtcrtpContributingSourceStats` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcrtpContributingSourceStats`*"]
    pub type RtcrtpContributingSourceStats;
    #[wasm_bindgen(method, getter = "id")]
    fn id_shim(this: &RtcrtpContributingSourceStats) -> &str;
    #[wasm_bindgen(method, setter = "id")]
    fn set_id_shim(this: &RtcrtpContributingSourceStats, val: &str);
    #[wasm_bindgen(method, getter = "timestamp")]
    fn timestamp_shim(this: &RtcrtpContributingSourceStats) -> f64;
    #[wasm_bindgen(method, setter = "timestamp")]
    fn set_timestamp_shim(this: &RtcrtpContributingSourceStats, val: f64);
    #[cfg(feature = "RtcStatsType")]
    #[wasm_bindgen(method, getter = "type")]
    fn type__shim(this: &RtcrtpContributingSourceStats) -> RtcStatsType;
    #[cfg(feature = "RtcStatsType")]
    #[wasm_bindgen(method, setter = "type")]
    fn set_type__shim(this: &RtcrtpContributingSourceStats, val: RtcStatsType);
    #[wasm_bindgen(method, getter = "contributorSsrc")]
    fn contributor_ssrc_shim(this: &RtcrtpContributingSourceStats) -> u32;
    #[wasm_bindgen(method, setter = "contributorSsrc")]
    fn set_contributor_ssrc_shim(this: &RtcrtpContributingSourceStats, val: u32);
    #[wasm_bindgen(method, getter = "inboundRtpStreamId")]
    fn inbound_rtp_stream_id_shim(this: &RtcrtpContributingSourceStats) -> &str;
    #[wasm_bindgen(method, setter = "inboundRtpStreamId")]
    fn set_inbound_rtp_stream_id_shim(this: &RtcrtpContributingSourceStats, val: &str);
}
#[doc = "The trait to access properties on the `RtcrtpContributingSourceStats` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `RtcrtpContributingSourceStats`*"]
pub trait RtcrtpContributingSourceStatsGetters {
    #[doc = "Get the `id` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcrtpContributingSourceStats`*"]
    fn id(&self) -> &str;
    #[doc = "Get the `timestamp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcrtpContributingSourceStats`*"]
    fn timestamp(&self) -> f64;
    #[cfg(feature = "RtcStatsType")]
    #[doc = "Get the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcStatsType`, `RtcrtpContributingSourceStats`*"]
    fn type_(&self) -> RtcStatsType;
    #[doc = "Get the `contributorSsrc` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcrtpContributingSourceStats`*"]
    fn contributor_ssrc(&self) -> u32;
    #[doc = "Get the `inboundRtpStreamId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcrtpContributingSourceStats`*"]
    fn inbound_rtp_stream_id(&self) -> &str;
}
impl RtcrtpContributingSourceStatsGetters for RtcrtpContributingSourceStats {
    fn id(&self) -> &str {
        self.id_shim()
    }
    fn timestamp(&self) -> f64 {
        self.timestamp_shim()
    }
    #[cfg(feature = "RtcStatsType")]
    fn type_(&self) -> RtcStatsType {
        self.type__shim()
    }
    fn contributor_ssrc(&self) -> u32 {
        self.contributor_ssrc_shim()
    }
    fn inbound_rtp_stream_id(&self) -> &str {
        self.inbound_rtp_stream_id_shim()
    }
}
impl RtcrtpContributingSourceStats {
    #[doc = "Construct a new `RtcrtpContributingSourceStats`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcrtpContributingSourceStats`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `id` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcrtpContributingSourceStats`*"]
    pub fn id(&mut self, val: &str) -> &mut Self {
        self.set_id_shim(val);
        self
    }
    #[doc = "Change the `timestamp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcrtpContributingSourceStats`*"]
    pub fn timestamp(&mut self, val: f64) -> &mut Self {
        self.set_timestamp_shim(val);
        self
    }
    #[cfg(feature = "RtcStatsType")]
    #[doc = "Change the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcStatsType`, `RtcrtpContributingSourceStats`*"]
    pub fn type_(&mut self, val: RtcStatsType) -> &mut Self {
        self.set_type__shim(val);
        self
    }
    #[doc = "Change the `contributorSsrc` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcrtpContributingSourceStats`*"]
    pub fn contributor_ssrc(&mut self, val: u32) -> &mut Self {
        self.set_contributor_ssrc_shim(val);
        self
    }
    #[doc = "Change the `inboundRtpStreamId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcrtpContributingSourceStats`*"]
    pub fn inbound_rtp_stream_id(&mut self, val: &str) -> &mut Self {
        self.set_inbound_rtp_stream_id_shim(val);
        self
    }
}
impl Default for RtcrtpContributingSourceStats {
    fn default() -> Self {
        Self::new()
    }
}
