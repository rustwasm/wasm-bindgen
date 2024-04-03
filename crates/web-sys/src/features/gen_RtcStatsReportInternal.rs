#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = RTCStatsReportInternal)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `RtcStatsReportInternal` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcStatsReportInternal`*"]
    pub type RtcStatsReportInternal;
    #[wasm_bindgen(method, setter = "closed")]
    fn closed_shim(this: &RtcStatsReportInternal, val: bool);
    #[wasm_bindgen(method, setter = "codecStats")]
    fn codec_stats_shim(this: &RtcStatsReportInternal, val: &::wasm_bindgen::JsValue);
    #[wasm_bindgen(method, setter = "iceCandidatePairStats")]
    fn ice_candidate_pair_stats_shim(this: &RtcStatsReportInternal, val: &::wasm_bindgen::JsValue);
    #[wasm_bindgen(method, setter = "iceCandidateStats")]
    fn ice_candidate_stats_shim(this: &RtcStatsReportInternal, val: &::wasm_bindgen::JsValue);
    #[wasm_bindgen(method, setter = "iceComponentStats")]
    fn ice_component_stats_shim(this: &RtcStatsReportInternal, val: &::wasm_bindgen::JsValue);
    #[wasm_bindgen(method, setter = "iceRestarts")]
    fn ice_restarts_shim(this: &RtcStatsReportInternal, val: u32);
    #[wasm_bindgen(method, setter = "iceRollbacks")]
    fn ice_rollbacks_shim(this: &RtcStatsReportInternal, val: u32);
    #[wasm_bindgen(method, setter = "inboundRTPStreamStats")]
    fn inbound_rtp_stream_stats_shim(this: &RtcStatsReportInternal, val: &::wasm_bindgen::JsValue);
    #[wasm_bindgen(method, setter = "localSdp")]
    fn local_sdp_shim(this: &RtcStatsReportInternal, val: &str);
    #[wasm_bindgen(method, setter = "mediaStreamStats")]
    fn media_stream_stats_shim(this: &RtcStatsReportInternal, val: &::wasm_bindgen::JsValue);
    #[wasm_bindgen(method, setter = "mediaStreamTrackStats")]
    fn media_stream_track_stats_shim(this: &RtcStatsReportInternal, val: &::wasm_bindgen::JsValue);
    #[wasm_bindgen(method, setter = "offerer")]
    fn offerer_shim(this: &RtcStatsReportInternal, val: bool);
    #[wasm_bindgen(method, setter = "outboundRTPStreamStats")]
    fn outbound_rtp_stream_stats_shim(this: &RtcStatsReportInternal, val: &::wasm_bindgen::JsValue);
    #[wasm_bindgen(method, setter = "pcid")]
    fn pcid_shim(this: &RtcStatsReportInternal, val: &str);
    #[wasm_bindgen(method, setter = "rawLocalCandidates")]
    fn raw_local_candidates_shim(this: &RtcStatsReportInternal, val: &::wasm_bindgen::JsValue);
    #[wasm_bindgen(method, setter = "rawRemoteCandidates")]
    fn raw_remote_candidates_shim(this: &RtcStatsReportInternal, val: &::wasm_bindgen::JsValue);
    #[wasm_bindgen(method, setter = "remoteSdp")]
    fn remote_sdp_shim(this: &RtcStatsReportInternal, val: &str);
    #[wasm_bindgen(method, setter = "rtpContributingSourceStats")]
    fn rtp_contributing_source_stats_shim(
        this: &RtcStatsReportInternal,
        val: &::wasm_bindgen::JsValue,
    );
    #[wasm_bindgen(method, setter = "timestamp")]
    fn timestamp_shim(this: &RtcStatsReportInternal, val: f64);
    #[wasm_bindgen(method, setter = "transportStats")]
    fn transport_stats_shim(this: &RtcStatsReportInternal, val: &::wasm_bindgen::JsValue);
    #[wasm_bindgen(method, setter = "trickledIceCandidateStats")]
    fn trickled_ice_candidate_stats_shim(
        this: &RtcStatsReportInternal,
        val: &::wasm_bindgen::JsValue,
    );
}
impl RtcStatsReportInternal {
    #[doc = "Construct a new `RtcStatsReportInternal`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcStatsReportInternal`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `closed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcStatsReportInternal`*"]
    pub fn closed(&mut self, val: bool) -> &mut Self {
        self.closed_shim(val);
        self
    }
    #[doc = "Change the `codecStats` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcStatsReportInternal`*"]
    pub fn codec_stats(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.codec_stats_shim(val);
        self
    }
    #[doc = "Change the `iceCandidatePairStats` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcStatsReportInternal`*"]
    pub fn ice_candidate_pair_stats(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.ice_candidate_pair_stats_shim(val);
        self
    }
    #[doc = "Change the `iceCandidateStats` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcStatsReportInternal`*"]
    pub fn ice_candidate_stats(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.ice_candidate_stats_shim(val);
        self
    }
    #[doc = "Change the `iceComponentStats` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcStatsReportInternal`*"]
    pub fn ice_component_stats(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.ice_component_stats_shim(val);
        self
    }
    #[doc = "Change the `iceRestarts` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcStatsReportInternal`*"]
    pub fn ice_restarts(&mut self, val: u32) -> &mut Self {
        self.ice_restarts_shim(val);
        self
    }
    #[doc = "Change the `iceRollbacks` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcStatsReportInternal`*"]
    pub fn ice_rollbacks(&mut self, val: u32) -> &mut Self {
        self.ice_rollbacks_shim(val);
        self
    }
    #[doc = "Change the `inboundRTPStreamStats` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcStatsReportInternal`*"]
    pub fn inbound_rtp_stream_stats(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.inbound_rtp_stream_stats_shim(val);
        self
    }
    #[doc = "Change the `localSdp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcStatsReportInternal`*"]
    pub fn local_sdp(&mut self, val: &str) -> &mut Self {
        self.local_sdp_shim(val);
        self
    }
    #[doc = "Change the `mediaStreamStats` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcStatsReportInternal`*"]
    pub fn media_stream_stats(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.media_stream_stats_shim(val);
        self
    }
    #[doc = "Change the `mediaStreamTrackStats` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcStatsReportInternal`*"]
    pub fn media_stream_track_stats(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.media_stream_track_stats_shim(val);
        self
    }
    #[doc = "Change the `offerer` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcStatsReportInternal`*"]
    pub fn offerer(&mut self, val: bool) -> &mut Self {
        self.offerer_shim(val);
        self
    }
    #[doc = "Change the `outboundRTPStreamStats` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcStatsReportInternal`*"]
    pub fn outbound_rtp_stream_stats(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.outbound_rtp_stream_stats_shim(val);
        self
    }
    #[doc = "Change the `pcid` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcStatsReportInternal`*"]
    pub fn pcid(&mut self, val: &str) -> &mut Self {
        self.pcid_shim(val);
        self
    }
    #[doc = "Change the `rawLocalCandidates` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcStatsReportInternal`*"]
    pub fn raw_local_candidates(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.raw_local_candidates_shim(val);
        self
    }
    #[doc = "Change the `rawRemoteCandidates` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcStatsReportInternal`*"]
    pub fn raw_remote_candidates(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.raw_remote_candidates_shim(val);
        self
    }
    #[doc = "Change the `remoteSdp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcStatsReportInternal`*"]
    pub fn remote_sdp(&mut self, val: &str) -> &mut Self {
        self.remote_sdp_shim(val);
        self
    }
    #[doc = "Change the `rtpContributingSourceStats` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcStatsReportInternal`*"]
    pub fn rtp_contributing_source_stats(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.rtp_contributing_source_stats_shim(val);
        self
    }
    #[doc = "Change the `timestamp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcStatsReportInternal`*"]
    pub fn timestamp(&mut self, val: f64) -> &mut Self {
        self.timestamp_shim(val);
        self
    }
    #[doc = "Change the `transportStats` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcStatsReportInternal`*"]
    pub fn transport_stats(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.transport_stats_shim(val);
        self
    }
    #[doc = "Change the `trickledIceCandidateStats` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcStatsReportInternal`*"]
    pub fn trickled_ice_candidate_stats(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.trickled_ice_candidate_stats_shim(val);
        self
    }
}
impl Default for RtcStatsReportInternal {
    fn default() -> Self {
        Self::new()
    }
}
