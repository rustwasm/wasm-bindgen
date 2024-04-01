#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = RTCInboundRTPStreamStats)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `RtcInboundRtpStreamStats` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcInboundRtpStreamStats`*"]
    pub type RtcInboundRtpStreamStats;
    #[wasm_bindgen(method, setter = "id")]
    fn id_shim(this: &RtcInboundRtpStreamStats, val: &str);
    #[wasm_bindgen(method, setter = "timestamp")]
    fn timestamp_shim(this: &RtcInboundRtpStreamStats, val: f64);
    #[cfg(feature = "RtcStatsType")]
    #[wasm_bindgen(method, setter = "type")]
    fn type__shim(this: &RtcInboundRtpStreamStats, val: RtcStatsType);
    #[wasm_bindgen(method, setter = "bitrateMean")]
    fn bitrate_mean_shim(this: &RtcInboundRtpStreamStats, val: f64);
    #[wasm_bindgen(method, setter = "bitrateStdDev")]
    fn bitrate_std_dev_shim(this: &RtcInboundRtpStreamStats, val: f64);
    #[wasm_bindgen(method, setter = "codecId")]
    fn codec_id_shim(this: &RtcInboundRtpStreamStats, val: &str);
    #[wasm_bindgen(method, setter = "firCount")]
    fn fir_count_shim(this: &RtcInboundRtpStreamStats, val: u32);
    #[wasm_bindgen(method, setter = "framerateMean")]
    fn framerate_mean_shim(this: &RtcInboundRtpStreamStats, val: f64);
    #[wasm_bindgen(method, setter = "framerateStdDev")]
    fn framerate_std_dev_shim(this: &RtcInboundRtpStreamStats, val: f64);
    #[wasm_bindgen(method, setter = "isRemote")]
    fn is_remote_shim(this: &RtcInboundRtpStreamStats, val: bool);
    #[wasm_bindgen(method, setter = "mediaTrackId")]
    fn media_track_id_shim(this: &RtcInboundRtpStreamStats, val: &str);
    #[wasm_bindgen(method, setter = "mediaType")]
    fn media_type_shim(this: &RtcInboundRtpStreamStats, val: &str);
    #[wasm_bindgen(method, setter = "nackCount")]
    fn nack_count_shim(this: &RtcInboundRtpStreamStats, val: u32);
    #[wasm_bindgen(method, setter = "pliCount")]
    fn pli_count_shim(this: &RtcInboundRtpStreamStats, val: u32);
    #[wasm_bindgen(method, setter = "remoteId")]
    fn remote_id_shim(this: &RtcInboundRtpStreamStats, val: &str);
    #[wasm_bindgen(method, setter = "ssrc")]
    fn ssrc_shim(this: &RtcInboundRtpStreamStats, val: &str);
    #[wasm_bindgen(method, setter = "transportId")]
    fn transport_id_shim(this: &RtcInboundRtpStreamStats, val: &str);
    #[wasm_bindgen(method, setter = "bytesReceived")]
    fn bytes_received_shim(this: &RtcInboundRtpStreamStats, val: f64);
    #[wasm_bindgen(method, setter = "discardedPackets")]
    fn discarded_packets_shim(this: &RtcInboundRtpStreamStats, val: u32);
    #[wasm_bindgen(method, setter = "framesDecoded")]
    fn frames_decoded_shim(this: &RtcInboundRtpStreamStats, val: u32);
    #[wasm_bindgen(method, setter = "jitter")]
    fn jitter_shim(this: &RtcInboundRtpStreamStats, val: f64);
    #[wasm_bindgen(method, setter = "packetsLost")]
    fn packets_lost_shim(this: &RtcInboundRtpStreamStats, val: u32);
    #[wasm_bindgen(method, setter = "packetsReceived")]
    fn packets_received_shim(this: &RtcInboundRtpStreamStats, val: u32);
    #[wasm_bindgen(method, setter = "roundTripTime")]
    fn round_trip_time_shim(this: &RtcInboundRtpStreamStats, val: i32);
}
impl RtcInboundRtpStreamStats {
    #[doc = "Construct a new `RtcInboundRtpStreamStats`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcInboundRtpStreamStats`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `id` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcInboundRtpStreamStats`*"]
    pub fn id(&mut self, val: &str) -> &mut Self {
        self.id_shim(val);
        self
    }
    #[doc = "Change the `timestamp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcInboundRtpStreamStats`*"]
    pub fn timestamp(&mut self, val: f64) -> &mut Self {
        self.timestamp_shim(val);
        self
    }
    #[cfg(feature = "RtcStatsType")]
    #[doc = "Change the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcInboundRtpStreamStats`, `RtcStatsType`*"]
    pub fn type_(&mut self, val: RtcStatsType) -> &mut Self {
        self.type__shim(val);
        self
    }
    #[doc = "Change the `bitrateMean` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcInboundRtpStreamStats`*"]
    pub fn bitrate_mean(&mut self, val: f64) -> &mut Self {
        self.bitrate_mean_shim(val);
        self
    }
    #[doc = "Change the `bitrateStdDev` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcInboundRtpStreamStats`*"]
    pub fn bitrate_std_dev(&mut self, val: f64) -> &mut Self {
        self.bitrate_std_dev_shim(val);
        self
    }
    #[doc = "Change the `codecId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcInboundRtpStreamStats`*"]
    pub fn codec_id(&mut self, val: &str) -> &mut Self {
        self.codec_id_shim(val);
        self
    }
    #[doc = "Change the `firCount` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcInboundRtpStreamStats`*"]
    pub fn fir_count(&mut self, val: u32) -> &mut Self {
        self.fir_count_shim(val);
        self
    }
    #[doc = "Change the `framerateMean` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcInboundRtpStreamStats`*"]
    pub fn framerate_mean(&mut self, val: f64) -> &mut Self {
        self.framerate_mean_shim(val);
        self
    }
    #[doc = "Change the `framerateStdDev` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcInboundRtpStreamStats`*"]
    pub fn framerate_std_dev(&mut self, val: f64) -> &mut Self {
        self.framerate_std_dev_shim(val);
        self
    }
    #[doc = "Change the `isRemote` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcInboundRtpStreamStats`*"]
    pub fn is_remote(&mut self, val: bool) -> &mut Self {
        self.is_remote_shim(val);
        self
    }
    #[doc = "Change the `mediaTrackId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcInboundRtpStreamStats`*"]
    pub fn media_track_id(&mut self, val: &str) -> &mut Self {
        self.media_track_id_shim(val);
        self
    }
    #[doc = "Change the `mediaType` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcInboundRtpStreamStats`*"]
    pub fn media_type(&mut self, val: &str) -> &mut Self {
        self.media_type_shim(val);
        self
    }
    #[doc = "Change the `nackCount` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcInboundRtpStreamStats`*"]
    pub fn nack_count(&mut self, val: u32) -> &mut Self {
        self.nack_count_shim(val);
        self
    }
    #[doc = "Change the `pliCount` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcInboundRtpStreamStats`*"]
    pub fn pli_count(&mut self, val: u32) -> &mut Self {
        self.pli_count_shim(val);
        self
    }
    #[doc = "Change the `remoteId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcInboundRtpStreamStats`*"]
    pub fn remote_id(&mut self, val: &str) -> &mut Self {
        self.remote_id_shim(val);
        self
    }
    #[doc = "Change the `ssrc` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcInboundRtpStreamStats`*"]
    pub fn ssrc(&mut self, val: &str) -> &mut Self {
        self.ssrc_shim(val);
        self
    }
    #[doc = "Change the `transportId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcInboundRtpStreamStats`*"]
    pub fn transport_id(&mut self, val: &str) -> &mut Self {
        self.transport_id_shim(val);
        self
    }
    #[doc = "Change the `bytesReceived` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcInboundRtpStreamStats`*"]
    pub fn bytes_received(&mut self, val: f64) -> &mut Self {
        self.bytes_received_shim(val);
        self
    }
    #[doc = "Change the `discardedPackets` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcInboundRtpStreamStats`*"]
    pub fn discarded_packets(&mut self, val: u32) -> &mut Self {
        self.discarded_packets_shim(val);
        self
    }
    #[doc = "Change the `framesDecoded` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcInboundRtpStreamStats`*"]
    pub fn frames_decoded(&mut self, val: u32) -> &mut Self {
        self.frames_decoded_shim(val);
        self
    }
    #[doc = "Change the `jitter` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcInboundRtpStreamStats`*"]
    pub fn jitter(&mut self, val: f64) -> &mut Self {
        self.jitter_shim(val);
        self
    }
    #[doc = "Change the `packetsLost` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcInboundRtpStreamStats`*"]
    pub fn packets_lost(&mut self, val: u32) -> &mut Self {
        self.packets_lost_shim(val);
        self
    }
    #[doc = "Change the `packetsReceived` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcInboundRtpStreamStats`*"]
    pub fn packets_received(&mut self, val: u32) -> &mut Self {
        self.packets_received_shim(val);
        self
    }
    #[doc = "Change the `roundTripTime` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcInboundRtpStreamStats`*"]
    pub fn round_trip_time(&mut self, val: i32) -> &mut Self {
        self.round_trip_time_shim(val);
        self
    }
}
impl Default for RtcInboundRtpStreamStats {
    fn default() -> Self {
        Self::new()
    }
}
