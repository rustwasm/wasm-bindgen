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
    #[wasm_bindgen(method, setter = "id")]
    fn id_shim(this: &RtcrtpContributingSourceStats, val: &str);
    #[wasm_bindgen(method, setter = "timestamp")]
    fn timestamp_shim(this: &RtcrtpContributingSourceStats, val: f64);
    #[cfg(feature = "RtcStatsType")]
    #[wasm_bindgen(method, setter = "type")]
    fn type__shim(this: &RtcrtpContributingSourceStats, val: RtcStatsType);
    #[wasm_bindgen(method, setter = "contributorSsrc")]
    fn contributor_ssrc_shim(this: &RtcrtpContributingSourceStats, val: u32);
    #[wasm_bindgen(method, setter = "inboundRtpStreamId")]
    fn inbound_rtp_stream_id_shim(this: &RtcrtpContributingSourceStats, val: &str);
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
        self.id_shim(val);
        self
    }
    #[doc = "Change the `timestamp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcrtpContributingSourceStats`*"]
    pub fn timestamp(&mut self, val: f64) -> &mut Self {
        self.timestamp_shim(val);
        self
    }
    #[cfg(feature = "RtcStatsType")]
    #[doc = "Change the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcStatsType`, `RtcrtpContributingSourceStats`*"]
    pub fn type_(&mut self, val: RtcStatsType) -> &mut Self {
        self.type__shim(val);
        self
    }
    #[doc = "Change the `contributorSsrc` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcrtpContributingSourceStats`*"]
    pub fn contributor_ssrc(&mut self, val: u32) -> &mut Self {
        self.contributor_ssrc_shim(val);
        self
    }
    #[doc = "Change the `inboundRtpStreamId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcrtpContributingSourceStats`*"]
    pub fn inbound_rtp_stream_id(&mut self, val: &str) -> &mut Self {
        self.inbound_rtp_stream_id_shim(val);
        self
    }
}
impl Default for RtcrtpContributingSourceStats {
    fn default() -> Self {
        Self::new()
    }
}
