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
    #[wasm_bindgen(method, getter = "id")]
    fn id_shim(this: &RtcInboundRtpStreamStats) -> String;
    #[wasm_bindgen(method, setter = "id")]
    fn set_id_shim(this: &RtcInboundRtpStreamStats, val: &str);
    #[wasm_bindgen(method, getter = "timestamp")]
    fn timestamp_shim(this: &RtcInboundRtpStreamStats) -> f64;
    #[wasm_bindgen(method, setter = "timestamp")]
    fn set_timestamp_shim(this: &RtcInboundRtpStreamStats, val: f64);
    #[cfg(feature = "RtcStatsType")]
    #[wasm_bindgen(method, getter = "type")]
    fn type__shim(this: &RtcInboundRtpStreamStats) -> RtcStatsType;
    #[cfg(feature = "RtcStatsType")]
    #[wasm_bindgen(method, setter = "type")]
    fn set_type__shim(this: &RtcInboundRtpStreamStats, val: RtcStatsType);
    #[wasm_bindgen(method, getter = "bitrateMean")]
    fn bitrate_mean_shim(this: &RtcInboundRtpStreamStats) -> f64;
    #[wasm_bindgen(method, setter = "bitrateMean")]
    fn set_bitrate_mean_shim(this: &RtcInboundRtpStreamStats, val: f64);
    #[wasm_bindgen(method, getter = "bitrateStdDev")]
    fn bitrate_std_dev_shim(this: &RtcInboundRtpStreamStats) -> f64;
    #[wasm_bindgen(method, setter = "bitrateStdDev")]
    fn set_bitrate_std_dev_shim(this: &RtcInboundRtpStreamStats, val: f64);
    #[wasm_bindgen(method, getter = "codecId")]
    fn codec_id_shim(this: &RtcInboundRtpStreamStats) -> String;
    #[wasm_bindgen(method, setter = "codecId")]
    fn set_codec_id_shim(this: &RtcInboundRtpStreamStats, val: &str);
    #[wasm_bindgen(method, getter = "firCount")]
    fn fir_count_shim(this: &RtcInboundRtpStreamStats) -> u32;
    #[wasm_bindgen(method, setter = "firCount")]
    fn set_fir_count_shim(this: &RtcInboundRtpStreamStats, val: u32);
    #[wasm_bindgen(method, getter = "framerateMean")]
    fn framerate_mean_shim(this: &RtcInboundRtpStreamStats) -> f64;
    #[wasm_bindgen(method, setter = "framerateMean")]
    fn set_framerate_mean_shim(this: &RtcInboundRtpStreamStats, val: f64);
    #[wasm_bindgen(method, getter = "framerateStdDev")]
    fn framerate_std_dev_shim(this: &RtcInboundRtpStreamStats) -> f64;
    #[wasm_bindgen(method, setter = "framerateStdDev")]
    fn set_framerate_std_dev_shim(this: &RtcInboundRtpStreamStats, val: f64);
    #[wasm_bindgen(method, getter = "isRemote")]
    fn is_remote_shim(this: &RtcInboundRtpStreamStats) -> bool;
    #[wasm_bindgen(method, setter = "isRemote")]
    fn set_is_remote_shim(this: &RtcInboundRtpStreamStats, val: bool);
    #[wasm_bindgen(method, getter = "mediaTrackId")]
    fn media_track_id_shim(this: &RtcInboundRtpStreamStats) -> String;
    #[wasm_bindgen(method, setter = "mediaTrackId")]
    fn set_media_track_id_shim(this: &RtcInboundRtpStreamStats, val: &str);
    #[wasm_bindgen(method, getter = "mediaType")]
    fn media_type_shim(this: &RtcInboundRtpStreamStats) -> String;
    #[wasm_bindgen(method, setter = "mediaType")]
    fn set_media_type_shim(this: &RtcInboundRtpStreamStats, val: &str);
    #[wasm_bindgen(method, getter = "nackCount")]
    fn nack_count_shim(this: &RtcInboundRtpStreamStats) -> u32;
    #[wasm_bindgen(method, setter = "nackCount")]
    fn set_nack_count_shim(this: &RtcInboundRtpStreamStats, val: u32);
    #[wasm_bindgen(method, getter = "pliCount")]
    fn pli_count_shim(this: &RtcInboundRtpStreamStats) -> u32;
    #[wasm_bindgen(method, setter = "pliCount")]
    fn set_pli_count_shim(this: &RtcInboundRtpStreamStats, val: u32);
    #[wasm_bindgen(method, getter = "remoteId")]
    fn remote_id_shim(this: &RtcInboundRtpStreamStats) -> String;
    #[wasm_bindgen(method, setter = "remoteId")]
    fn set_remote_id_shim(this: &RtcInboundRtpStreamStats, val: &str);
    #[wasm_bindgen(method, getter = "ssrc")]
    fn ssrc_shim(this: &RtcInboundRtpStreamStats) -> String;
    #[wasm_bindgen(method, setter = "ssrc")]
    fn set_ssrc_shim(this: &RtcInboundRtpStreamStats, val: &str);
    #[wasm_bindgen(method, getter = "transportId")]
    fn transport_id_shim(this: &RtcInboundRtpStreamStats) -> String;
    #[wasm_bindgen(method, setter = "transportId")]
    fn set_transport_id_shim(this: &RtcInboundRtpStreamStats, val: &str);
    #[wasm_bindgen(method, getter = "bytesReceived")]
    fn bytes_received_shim(this: &RtcInboundRtpStreamStats) -> f64;
    #[wasm_bindgen(method, setter = "bytesReceived")]
    fn set_bytes_received_shim(this: &RtcInboundRtpStreamStats, val: f64);
    #[wasm_bindgen(method, getter = "discardedPackets")]
    fn discarded_packets_shim(this: &RtcInboundRtpStreamStats) -> u32;
    #[wasm_bindgen(method, setter = "discardedPackets")]
    fn set_discarded_packets_shim(this: &RtcInboundRtpStreamStats, val: u32);
    #[wasm_bindgen(method, getter = "framesDecoded")]
    fn frames_decoded_shim(this: &RtcInboundRtpStreamStats) -> u32;
    #[wasm_bindgen(method, setter = "framesDecoded")]
    fn set_frames_decoded_shim(this: &RtcInboundRtpStreamStats, val: u32);
    #[wasm_bindgen(method, getter = "jitter")]
    fn jitter_shim(this: &RtcInboundRtpStreamStats) -> f64;
    #[wasm_bindgen(method, setter = "jitter")]
    fn set_jitter_shim(this: &RtcInboundRtpStreamStats, val: f64);
    #[wasm_bindgen(method, getter = "packetsLost")]
    fn packets_lost_shim(this: &RtcInboundRtpStreamStats) -> u32;
    #[wasm_bindgen(method, setter = "packetsLost")]
    fn set_packets_lost_shim(this: &RtcInboundRtpStreamStats, val: u32);
    #[wasm_bindgen(method, getter = "packetsReceived")]
    fn packets_received_shim(this: &RtcInboundRtpStreamStats) -> u32;
    #[wasm_bindgen(method, setter = "packetsReceived")]
    fn set_packets_received_shim(this: &RtcInboundRtpStreamStats, val: u32);
    #[wasm_bindgen(method, getter = "roundTripTime")]
    fn round_trip_time_shim(this: &RtcInboundRtpStreamStats) -> i32;
    #[wasm_bindgen(method, setter = "roundTripTime")]
    fn set_round_trip_time_shim(this: &RtcInboundRtpStreamStats, val: i32);
}
#[doc = "The trait to access properties on the `RtcInboundRtpStreamStats` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `RtcInboundRtpStreamStats`*"]
pub trait RtcInboundRtpStreamStatsGetters {
    #[doc = "Get the `id` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcInboundRtpStreamStats`*"]
    fn id(&self) -> String;
    #[doc = "Get the `timestamp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcInboundRtpStreamStats`*"]
    fn timestamp(&self) -> f64;
    #[cfg(feature = "RtcStatsType")]
    #[doc = "Get the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcInboundRtpStreamStats`, `RtcStatsType`*"]
    fn type_(&self) -> RtcStatsType;
    #[doc = "Get the `bitrateMean` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcInboundRtpStreamStats`*"]
    fn bitrate_mean(&self) -> f64;
    #[doc = "Get the `bitrateStdDev` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcInboundRtpStreamStats`*"]
    fn bitrate_std_dev(&self) -> f64;
    #[doc = "Get the `codecId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcInboundRtpStreamStats`*"]
    fn codec_id(&self) -> String;
    #[doc = "Get the `firCount` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcInboundRtpStreamStats`*"]
    fn fir_count(&self) -> u32;
    #[doc = "Get the `framerateMean` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcInboundRtpStreamStats`*"]
    fn framerate_mean(&self) -> f64;
    #[doc = "Get the `framerateStdDev` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcInboundRtpStreamStats`*"]
    fn framerate_std_dev(&self) -> f64;
    #[doc = "Get the `isRemote` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcInboundRtpStreamStats`*"]
    fn is_remote(&self) -> bool;
    #[doc = "Get the `mediaTrackId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcInboundRtpStreamStats`*"]
    fn media_track_id(&self) -> String;
    #[doc = "Get the `mediaType` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcInboundRtpStreamStats`*"]
    fn media_type(&self) -> String;
    #[doc = "Get the `nackCount` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcInboundRtpStreamStats`*"]
    fn nack_count(&self) -> u32;
    #[doc = "Get the `pliCount` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcInboundRtpStreamStats`*"]
    fn pli_count(&self) -> u32;
    #[doc = "Get the `remoteId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcInboundRtpStreamStats`*"]
    fn remote_id(&self) -> String;
    #[doc = "Get the `ssrc` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcInboundRtpStreamStats`*"]
    fn ssrc(&self) -> String;
    #[doc = "Get the `transportId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcInboundRtpStreamStats`*"]
    fn transport_id(&self) -> String;
    #[doc = "Get the `bytesReceived` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcInboundRtpStreamStats`*"]
    fn bytes_received(&self) -> f64;
    #[doc = "Get the `discardedPackets` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcInboundRtpStreamStats`*"]
    fn discarded_packets(&self) -> u32;
    #[doc = "Get the `framesDecoded` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcInboundRtpStreamStats`*"]
    fn frames_decoded(&self) -> u32;
    #[doc = "Get the `jitter` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcInboundRtpStreamStats`*"]
    fn jitter(&self) -> f64;
    #[doc = "Get the `packetsLost` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcInboundRtpStreamStats`*"]
    fn packets_lost(&self) -> u32;
    #[doc = "Get the `packetsReceived` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcInboundRtpStreamStats`*"]
    fn packets_received(&self) -> u32;
    #[doc = "Get the `roundTripTime` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcInboundRtpStreamStats`*"]
    fn round_trip_time(&self) -> i32;
}
impl RtcInboundRtpStreamStatsGetters for RtcInboundRtpStreamStats {
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
    fn bitrate_mean(&self) -> f64 {
        self.bitrate_mean_shim()
    }
    fn bitrate_std_dev(&self) -> f64 {
        self.bitrate_std_dev_shim()
    }
    fn codec_id(&self) -> String {
        self.codec_id_shim()
    }
    fn fir_count(&self) -> u32 {
        self.fir_count_shim()
    }
    fn framerate_mean(&self) -> f64 {
        self.framerate_mean_shim()
    }
    fn framerate_std_dev(&self) -> f64 {
        self.framerate_std_dev_shim()
    }
    fn is_remote(&self) -> bool {
        self.is_remote_shim()
    }
    fn media_track_id(&self) -> String {
        self.media_track_id_shim()
    }
    fn media_type(&self) -> String {
        self.media_type_shim()
    }
    fn nack_count(&self) -> u32 {
        self.nack_count_shim()
    }
    fn pli_count(&self) -> u32 {
        self.pli_count_shim()
    }
    fn remote_id(&self) -> String {
        self.remote_id_shim()
    }
    fn ssrc(&self) -> String {
        self.ssrc_shim()
    }
    fn transport_id(&self) -> String {
        self.transport_id_shim()
    }
    fn bytes_received(&self) -> f64 {
        self.bytes_received_shim()
    }
    fn discarded_packets(&self) -> u32 {
        self.discarded_packets_shim()
    }
    fn frames_decoded(&self) -> u32 {
        self.frames_decoded_shim()
    }
    fn jitter(&self) -> f64 {
        self.jitter_shim()
    }
    fn packets_lost(&self) -> u32 {
        self.packets_lost_shim()
    }
    fn packets_received(&self) -> u32 {
        self.packets_received_shim()
    }
    fn round_trip_time(&self) -> i32 {
        self.round_trip_time_shim()
    }
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
        self.set_id_shim(val);
        self
    }
    #[doc = "Change the `timestamp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcInboundRtpStreamStats`*"]
    pub fn timestamp(&mut self, val: f64) -> &mut Self {
        self.set_timestamp_shim(val);
        self
    }
    #[cfg(feature = "RtcStatsType")]
    #[doc = "Change the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcInboundRtpStreamStats`, `RtcStatsType`*"]
    pub fn type_(&mut self, val: RtcStatsType) -> &mut Self {
        self.set_type__shim(val);
        self
    }
    #[doc = "Change the `bitrateMean` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcInboundRtpStreamStats`*"]
    pub fn bitrate_mean(&mut self, val: f64) -> &mut Self {
        self.set_bitrate_mean_shim(val);
        self
    }
    #[doc = "Change the `bitrateStdDev` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcInboundRtpStreamStats`*"]
    pub fn bitrate_std_dev(&mut self, val: f64) -> &mut Self {
        self.set_bitrate_std_dev_shim(val);
        self
    }
    #[doc = "Change the `codecId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcInboundRtpStreamStats`*"]
    pub fn codec_id(&mut self, val: &str) -> &mut Self {
        self.set_codec_id_shim(val);
        self
    }
    #[doc = "Change the `firCount` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcInboundRtpStreamStats`*"]
    pub fn fir_count(&mut self, val: u32) -> &mut Self {
        self.set_fir_count_shim(val);
        self
    }
    #[doc = "Change the `framerateMean` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcInboundRtpStreamStats`*"]
    pub fn framerate_mean(&mut self, val: f64) -> &mut Self {
        self.set_framerate_mean_shim(val);
        self
    }
    #[doc = "Change the `framerateStdDev` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcInboundRtpStreamStats`*"]
    pub fn framerate_std_dev(&mut self, val: f64) -> &mut Self {
        self.set_framerate_std_dev_shim(val);
        self
    }
    #[doc = "Change the `isRemote` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcInboundRtpStreamStats`*"]
    pub fn is_remote(&mut self, val: bool) -> &mut Self {
        self.set_is_remote_shim(val);
        self
    }
    #[doc = "Change the `mediaTrackId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcInboundRtpStreamStats`*"]
    pub fn media_track_id(&mut self, val: &str) -> &mut Self {
        self.set_media_track_id_shim(val);
        self
    }
    #[doc = "Change the `mediaType` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcInboundRtpStreamStats`*"]
    pub fn media_type(&mut self, val: &str) -> &mut Self {
        self.set_media_type_shim(val);
        self
    }
    #[doc = "Change the `nackCount` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcInboundRtpStreamStats`*"]
    pub fn nack_count(&mut self, val: u32) -> &mut Self {
        self.set_nack_count_shim(val);
        self
    }
    #[doc = "Change the `pliCount` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcInboundRtpStreamStats`*"]
    pub fn pli_count(&mut self, val: u32) -> &mut Self {
        self.set_pli_count_shim(val);
        self
    }
    #[doc = "Change the `remoteId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcInboundRtpStreamStats`*"]
    pub fn remote_id(&mut self, val: &str) -> &mut Self {
        self.set_remote_id_shim(val);
        self
    }
    #[doc = "Change the `ssrc` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcInboundRtpStreamStats`*"]
    pub fn ssrc(&mut self, val: &str) -> &mut Self {
        self.set_ssrc_shim(val);
        self
    }
    #[doc = "Change the `transportId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcInboundRtpStreamStats`*"]
    pub fn transport_id(&mut self, val: &str) -> &mut Self {
        self.set_transport_id_shim(val);
        self
    }
    #[doc = "Change the `bytesReceived` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcInboundRtpStreamStats`*"]
    pub fn bytes_received(&mut self, val: f64) -> &mut Self {
        self.set_bytes_received_shim(val);
        self
    }
    #[doc = "Change the `discardedPackets` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcInboundRtpStreamStats`*"]
    pub fn discarded_packets(&mut self, val: u32) -> &mut Self {
        self.set_discarded_packets_shim(val);
        self
    }
    #[doc = "Change the `framesDecoded` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcInboundRtpStreamStats`*"]
    pub fn frames_decoded(&mut self, val: u32) -> &mut Self {
        self.set_frames_decoded_shim(val);
        self
    }
    #[doc = "Change the `jitter` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcInboundRtpStreamStats`*"]
    pub fn jitter(&mut self, val: f64) -> &mut Self {
        self.set_jitter_shim(val);
        self
    }
    #[doc = "Change the `packetsLost` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcInboundRtpStreamStats`*"]
    pub fn packets_lost(&mut self, val: u32) -> &mut Self {
        self.set_packets_lost_shim(val);
        self
    }
    #[doc = "Change the `packetsReceived` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcInboundRtpStreamStats`*"]
    pub fn packets_received(&mut self, val: u32) -> &mut Self {
        self.set_packets_received_shim(val);
        self
    }
    #[doc = "Change the `roundTripTime` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcInboundRtpStreamStats`*"]
    pub fn round_trip_time(&mut self, val: i32) -> &mut Self {
        self.set_round_trip_time_shim(val);
        self
    }
}
impl Default for RtcInboundRtpStreamStats {
    fn default() -> Self {
        Self::new()
    }
}
