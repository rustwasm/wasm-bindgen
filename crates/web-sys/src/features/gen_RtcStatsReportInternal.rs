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
    #[wasm_bindgen(method, getter = "closed")]
    fn closed_shim(this: &RtcStatsReportInternal) -> bool;
    #[wasm_bindgen(method, setter = "closed")]
    fn set_closed_shim(this: &RtcStatsReportInternal, val: bool);
    #[wasm_bindgen(method, getter = "codecStats")]
    fn codec_stats_shim(this: &RtcStatsReportInternal) -> ::js_sys::Array;
    #[wasm_bindgen(method, setter = "codecStats")]
    fn set_codec_stats_shim(this: &RtcStatsReportInternal, val: &::wasm_bindgen::JsValue);
    #[wasm_bindgen(method, getter = "iceCandidatePairStats")]
    fn ice_candidate_pair_stats_shim(this: &RtcStatsReportInternal) -> ::js_sys::Array;
    #[wasm_bindgen(method, setter = "iceCandidatePairStats")]
    fn set_ice_candidate_pair_stats_shim(
        this: &RtcStatsReportInternal,
        val: &::wasm_bindgen::JsValue,
    );
    #[wasm_bindgen(method, getter = "iceCandidateStats")]
    fn ice_candidate_stats_shim(this: &RtcStatsReportInternal) -> ::js_sys::Array;
    #[wasm_bindgen(method, setter = "iceCandidateStats")]
    fn set_ice_candidate_stats_shim(this: &RtcStatsReportInternal, val: &::wasm_bindgen::JsValue);
    #[wasm_bindgen(method, getter = "iceComponentStats")]
    fn ice_component_stats_shim(this: &RtcStatsReportInternal) -> ::js_sys::Array;
    #[wasm_bindgen(method, setter = "iceComponentStats")]
    fn set_ice_component_stats_shim(this: &RtcStatsReportInternal, val: &::wasm_bindgen::JsValue);
    #[wasm_bindgen(method, getter = "iceRestarts")]
    fn ice_restarts_shim(this: &RtcStatsReportInternal) -> u32;
    #[wasm_bindgen(method, setter = "iceRestarts")]
    fn set_ice_restarts_shim(this: &RtcStatsReportInternal, val: u32);
    #[wasm_bindgen(method, getter = "iceRollbacks")]
    fn ice_rollbacks_shim(this: &RtcStatsReportInternal) -> u32;
    #[wasm_bindgen(method, setter = "iceRollbacks")]
    fn set_ice_rollbacks_shim(this: &RtcStatsReportInternal, val: u32);
    #[wasm_bindgen(method, getter = "inboundRTPStreamStats")]
    fn inbound_rtp_stream_stats_shim(this: &RtcStatsReportInternal) -> ::js_sys::Array;
    #[wasm_bindgen(method, setter = "inboundRTPStreamStats")]
    fn set_inbound_rtp_stream_stats_shim(
        this: &RtcStatsReportInternal,
        val: &::wasm_bindgen::JsValue,
    );
    #[wasm_bindgen(method, getter = "localSdp")]
    fn local_sdp_shim(this: &RtcStatsReportInternal) -> String;
    #[wasm_bindgen(method, setter = "localSdp")]
    fn set_local_sdp_shim(this: &RtcStatsReportInternal, val: &str);
    #[wasm_bindgen(method, getter = "mediaStreamStats")]
    fn media_stream_stats_shim(this: &RtcStatsReportInternal) -> ::js_sys::Array;
    #[wasm_bindgen(method, setter = "mediaStreamStats")]
    fn set_media_stream_stats_shim(this: &RtcStatsReportInternal, val: &::wasm_bindgen::JsValue);
    #[wasm_bindgen(method, getter = "mediaStreamTrackStats")]
    fn media_stream_track_stats_shim(this: &RtcStatsReportInternal) -> ::js_sys::Array;
    #[wasm_bindgen(method, setter = "mediaStreamTrackStats")]
    fn set_media_stream_track_stats_shim(
        this: &RtcStatsReportInternal,
        val: &::wasm_bindgen::JsValue,
    );
    #[wasm_bindgen(method, getter = "offerer")]
    fn offerer_shim(this: &RtcStatsReportInternal) -> bool;
    #[wasm_bindgen(method, setter = "offerer")]
    fn set_offerer_shim(this: &RtcStatsReportInternal, val: bool);
    #[wasm_bindgen(method, getter = "outboundRTPStreamStats")]
    fn outbound_rtp_stream_stats_shim(this: &RtcStatsReportInternal) -> ::js_sys::Array;
    #[wasm_bindgen(method, setter = "outboundRTPStreamStats")]
    fn set_outbound_rtp_stream_stats_shim(
        this: &RtcStatsReportInternal,
        val: &::wasm_bindgen::JsValue,
    );
    #[wasm_bindgen(method, getter = "pcid")]
    fn pcid_shim(this: &RtcStatsReportInternal) -> String;
    #[wasm_bindgen(method, setter = "pcid")]
    fn set_pcid_shim(this: &RtcStatsReportInternal, val: &str);
    #[wasm_bindgen(method, getter = "rawLocalCandidates")]
    fn raw_local_candidates_shim(this: &RtcStatsReportInternal) -> ::js_sys::Array;
    #[wasm_bindgen(method, setter = "rawLocalCandidates")]
    fn set_raw_local_candidates_shim(this: &RtcStatsReportInternal, val: &::wasm_bindgen::JsValue);
    #[wasm_bindgen(method, getter = "rawRemoteCandidates")]
    fn raw_remote_candidates_shim(this: &RtcStatsReportInternal) -> ::js_sys::Array;
    #[wasm_bindgen(method, setter = "rawRemoteCandidates")]
    fn set_raw_remote_candidates_shim(this: &RtcStatsReportInternal, val: &::wasm_bindgen::JsValue);
    #[wasm_bindgen(method, getter = "remoteSdp")]
    fn remote_sdp_shim(this: &RtcStatsReportInternal) -> String;
    #[wasm_bindgen(method, setter = "remoteSdp")]
    fn set_remote_sdp_shim(this: &RtcStatsReportInternal, val: &str);
    #[wasm_bindgen(method, getter = "rtpContributingSourceStats")]
    fn rtp_contributing_source_stats_shim(this: &RtcStatsReportInternal) -> ::js_sys::Array;
    #[wasm_bindgen(method, setter = "rtpContributingSourceStats")]
    fn set_rtp_contributing_source_stats_shim(
        this: &RtcStatsReportInternal,
        val: &::wasm_bindgen::JsValue,
    );
    #[wasm_bindgen(method, getter = "timestamp")]
    fn timestamp_shim(this: &RtcStatsReportInternal) -> f64;
    #[wasm_bindgen(method, setter = "timestamp")]
    fn set_timestamp_shim(this: &RtcStatsReportInternal, val: f64);
    #[wasm_bindgen(method, getter = "transportStats")]
    fn transport_stats_shim(this: &RtcStatsReportInternal) -> ::js_sys::Array;
    #[wasm_bindgen(method, setter = "transportStats")]
    fn set_transport_stats_shim(this: &RtcStatsReportInternal, val: &::wasm_bindgen::JsValue);
    #[wasm_bindgen(method, getter = "trickledIceCandidateStats")]
    fn trickled_ice_candidate_stats_shim(this: &RtcStatsReportInternal) -> ::js_sys::Array;
    #[wasm_bindgen(method, setter = "trickledIceCandidateStats")]
    fn set_trickled_ice_candidate_stats_shim(
        this: &RtcStatsReportInternal,
        val: &::wasm_bindgen::JsValue,
    );
}
#[doc = "The trait to access properties on the `RtcStatsReportInternal` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `RtcStatsReportInternal`*"]
pub trait RtcStatsReportInternalGetters {
    #[doc = "Get the `closed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcStatsReportInternal`*"]
    fn closed(&self) -> bool;
    #[doc = "Get the `codecStats` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcStatsReportInternal`*"]
    fn codec_stats(&self) -> ::js_sys::Array;
    #[doc = "Get the `iceCandidatePairStats` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcStatsReportInternal`*"]
    fn ice_candidate_pair_stats(&self) -> ::js_sys::Array;
    #[doc = "Get the `iceCandidateStats` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcStatsReportInternal`*"]
    fn ice_candidate_stats(&self) -> ::js_sys::Array;
    #[doc = "Get the `iceComponentStats` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcStatsReportInternal`*"]
    fn ice_component_stats(&self) -> ::js_sys::Array;
    #[doc = "Get the `iceRestarts` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcStatsReportInternal`*"]
    fn ice_restarts(&self) -> u32;
    #[doc = "Get the `iceRollbacks` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcStatsReportInternal`*"]
    fn ice_rollbacks(&self) -> u32;
    #[doc = "Get the `inboundRTPStreamStats` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcStatsReportInternal`*"]
    fn inbound_rtp_stream_stats(&self) -> ::js_sys::Array;
    #[doc = "Get the `localSdp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcStatsReportInternal`*"]
    fn local_sdp(&self) -> String;
    #[doc = "Get the `mediaStreamStats` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcStatsReportInternal`*"]
    fn media_stream_stats(&self) -> ::js_sys::Array;
    #[doc = "Get the `mediaStreamTrackStats` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcStatsReportInternal`*"]
    fn media_stream_track_stats(&self) -> ::js_sys::Array;
    #[doc = "Get the `offerer` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcStatsReportInternal`*"]
    fn offerer(&self) -> bool;
    #[doc = "Get the `outboundRTPStreamStats` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcStatsReportInternal`*"]
    fn outbound_rtp_stream_stats(&self) -> ::js_sys::Array;
    #[doc = "Get the `pcid` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcStatsReportInternal`*"]
    fn pcid(&self) -> String;
    #[doc = "Get the `rawLocalCandidates` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcStatsReportInternal`*"]
    fn raw_local_candidates(&self) -> ::js_sys::Array;
    #[doc = "Get the `rawRemoteCandidates` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcStatsReportInternal`*"]
    fn raw_remote_candidates(&self) -> ::js_sys::Array;
    #[doc = "Get the `remoteSdp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcStatsReportInternal`*"]
    fn remote_sdp(&self) -> String;
    #[doc = "Get the `rtpContributingSourceStats` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcStatsReportInternal`*"]
    fn rtp_contributing_source_stats(&self) -> ::js_sys::Array;
    #[doc = "Get the `timestamp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcStatsReportInternal`*"]
    fn timestamp(&self) -> f64;
    #[doc = "Get the `transportStats` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcStatsReportInternal`*"]
    fn transport_stats(&self) -> ::js_sys::Array;
    #[doc = "Get the `trickledIceCandidateStats` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcStatsReportInternal`*"]
    fn trickled_ice_candidate_stats(&self) -> ::js_sys::Array;
}
impl RtcStatsReportInternalGetters for RtcStatsReportInternal {
    fn closed(&self) -> bool {
        self.closed_shim()
    }
    fn codec_stats(&self) -> ::js_sys::Array {
        self.codec_stats_shim()
    }
    fn ice_candidate_pair_stats(&self) -> ::js_sys::Array {
        self.ice_candidate_pair_stats_shim()
    }
    fn ice_candidate_stats(&self) -> ::js_sys::Array {
        self.ice_candidate_stats_shim()
    }
    fn ice_component_stats(&self) -> ::js_sys::Array {
        self.ice_component_stats_shim()
    }
    fn ice_restarts(&self) -> u32 {
        self.ice_restarts_shim()
    }
    fn ice_rollbacks(&self) -> u32 {
        self.ice_rollbacks_shim()
    }
    fn inbound_rtp_stream_stats(&self) -> ::js_sys::Array {
        self.inbound_rtp_stream_stats_shim()
    }
    fn local_sdp(&self) -> String {
        self.local_sdp_shim()
    }
    fn media_stream_stats(&self) -> ::js_sys::Array {
        self.media_stream_stats_shim()
    }
    fn media_stream_track_stats(&self) -> ::js_sys::Array {
        self.media_stream_track_stats_shim()
    }
    fn offerer(&self) -> bool {
        self.offerer_shim()
    }
    fn outbound_rtp_stream_stats(&self) -> ::js_sys::Array {
        self.outbound_rtp_stream_stats_shim()
    }
    fn pcid(&self) -> String {
        self.pcid_shim()
    }
    fn raw_local_candidates(&self) -> ::js_sys::Array {
        self.raw_local_candidates_shim()
    }
    fn raw_remote_candidates(&self) -> ::js_sys::Array {
        self.raw_remote_candidates_shim()
    }
    fn remote_sdp(&self) -> String {
        self.remote_sdp_shim()
    }
    fn rtp_contributing_source_stats(&self) -> ::js_sys::Array {
        self.rtp_contributing_source_stats_shim()
    }
    fn timestamp(&self) -> f64 {
        self.timestamp_shim()
    }
    fn transport_stats(&self) -> ::js_sys::Array {
        self.transport_stats_shim()
    }
    fn trickled_ice_candidate_stats(&self) -> ::js_sys::Array {
        self.trickled_ice_candidate_stats_shim()
    }
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
        self.set_closed_shim(val);
        self
    }
    #[doc = "Change the `codecStats` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcStatsReportInternal`*"]
    pub fn codec_stats(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_codec_stats_shim(val);
        self
    }
    #[doc = "Change the `iceCandidatePairStats` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcStatsReportInternal`*"]
    pub fn ice_candidate_pair_stats(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_ice_candidate_pair_stats_shim(val);
        self
    }
    #[doc = "Change the `iceCandidateStats` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcStatsReportInternal`*"]
    pub fn ice_candidate_stats(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_ice_candidate_stats_shim(val);
        self
    }
    #[doc = "Change the `iceComponentStats` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcStatsReportInternal`*"]
    pub fn ice_component_stats(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_ice_component_stats_shim(val);
        self
    }
    #[doc = "Change the `iceRestarts` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcStatsReportInternal`*"]
    pub fn ice_restarts(&mut self, val: u32) -> &mut Self {
        self.set_ice_restarts_shim(val);
        self
    }
    #[doc = "Change the `iceRollbacks` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcStatsReportInternal`*"]
    pub fn ice_rollbacks(&mut self, val: u32) -> &mut Self {
        self.set_ice_rollbacks_shim(val);
        self
    }
    #[doc = "Change the `inboundRTPStreamStats` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcStatsReportInternal`*"]
    pub fn inbound_rtp_stream_stats(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_inbound_rtp_stream_stats_shim(val);
        self
    }
    #[doc = "Change the `localSdp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcStatsReportInternal`*"]
    pub fn local_sdp(&mut self, val: &str) -> &mut Self {
        self.set_local_sdp_shim(val);
        self
    }
    #[doc = "Change the `mediaStreamStats` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcStatsReportInternal`*"]
    pub fn media_stream_stats(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_media_stream_stats_shim(val);
        self
    }
    #[doc = "Change the `mediaStreamTrackStats` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcStatsReportInternal`*"]
    pub fn media_stream_track_stats(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_media_stream_track_stats_shim(val);
        self
    }
    #[doc = "Change the `offerer` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcStatsReportInternal`*"]
    pub fn offerer(&mut self, val: bool) -> &mut Self {
        self.set_offerer_shim(val);
        self
    }
    #[doc = "Change the `outboundRTPStreamStats` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcStatsReportInternal`*"]
    pub fn outbound_rtp_stream_stats(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_outbound_rtp_stream_stats_shim(val);
        self
    }
    #[doc = "Change the `pcid` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcStatsReportInternal`*"]
    pub fn pcid(&mut self, val: &str) -> &mut Self {
        self.set_pcid_shim(val);
        self
    }
    #[doc = "Change the `rawLocalCandidates` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcStatsReportInternal`*"]
    pub fn raw_local_candidates(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_raw_local_candidates_shim(val);
        self
    }
    #[doc = "Change the `rawRemoteCandidates` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcStatsReportInternal`*"]
    pub fn raw_remote_candidates(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_raw_remote_candidates_shim(val);
        self
    }
    #[doc = "Change the `remoteSdp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcStatsReportInternal`*"]
    pub fn remote_sdp(&mut self, val: &str) -> &mut Self {
        self.set_remote_sdp_shim(val);
        self
    }
    #[doc = "Change the `rtpContributingSourceStats` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcStatsReportInternal`*"]
    pub fn rtp_contributing_source_stats(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_rtp_contributing_source_stats_shim(val);
        self
    }
    #[doc = "Change the `timestamp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcStatsReportInternal`*"]
    pub fn timestamp(&mut self, val: f64) -> &mut Self {
        self.set_timestamp_shim(val);
        self
    }
    #[doc = "Change the `transportStats` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcStatsReportInternal`*"]
    pub fn transport_stats(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_transport_stats_shim(val);
        self
    }
    #[doc = "Change the `trickledIceCandidateStats` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcStatsReportInternal`*"]
    pub fn trickled_ice_candidate_stats(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_trickled_ice_candidate_stats_shim(val);
        self
    }
}
impl Default for RtcStatsReportInternal {
    fn default() -> Self {
        Self::new()
    }
}
