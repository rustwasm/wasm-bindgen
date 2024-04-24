#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = RTCOutboundRTPStreamStats)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `RtcOutboundRtpStreamStats` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcOutboundRtpStreamStats`*"]
    pub type RtcOutboundRtpStreamStats;
    #[wasm_bindgen(method, getter = "id")]
    fn id_shim(this: &RtcOutboundRtpStreamStats) -> &str;
    #[wasm_bindgen(method, setter = "id")]
    fn set_id_shim(this: &RtcOutboundRtpStreamStats, val: &str);
    #[wasm_bindgen(method, getter = "timestamp")]
    fn timestamp_shim(this: &RtcOutboundRtpStreamStats) -> f64;
    #[wasm_bindgen(method, setter = "timestamp")]
    fn set_timestamp_shim(this: &RtcOutboundRtpStreamStats, val: f64);
    #[cfg(feature = "RtcStatsType")]
    #[wasm_bindgen(method, getter = "type")]
    fn type__shim(this: &RtcOutboundRtpStreamStats) -> RtcStatsType;
    #[cfg(feature = "RtcStatsType")]
    #[wasm_bindgen(method, setter = "type")]
    fn set_type__shim(this: &RtcOutboundRtpStreamStats, val: RtcStatsType);
    #[wasm_bindgen(method, getter = "bitrateMean")]
    fn bitrate_mean_shim(this: &RtcOutboundRtpStreamStats) -> f64;
    #[wasm_bindgen(method, setter = "bitrateMean")]
    fn set_bitrate_mean_shim(this: &RtcOutboundRtpStreamStats, val: f64);
    #[wasm_bindgen(method, getter = "bitrateStdDev")]
    fn bitrate_std_dev_shim(this: &RtcOutboundRtpStreamStats) -> f64;
    #[wasm_bindgen(method, setter = "bitrateStdDev")]
    fn set_bitrate_std_dev_shim(this: &RtcOutboundRtpStreamStats, val: f64);
    #[wasm_bindgen(method, getter = "codecId")]
    fn codec_id_shim(this: &RtcOutboundRtpStreamStats) -> &str;
    #[wasm_bindgen(method, setter = "codecId")]
    fn set_codec_id_shim(this: &RtcOutboundRtpStreamStats, val: &str);
    #[wasm_bindgen(method, getter = "firCount")]
    fn fir_count_shim(this: &RtcOutboundRtpStreamStats) -> u32;
    #[wasm_bindgen(method, setter = "firCount")]
    fn set_fir_count_shim(this: &RtcOutboundRtpStreamStats, val: u32);
    #[wasm_bindgen(method, getter = "framerateMean")]
    fn framerate_mean_shim(this: &RtcOutboundRtpStreamStats) -> f64;
    #[wasm_bindgen(method, setter = "framerateMean")]
    fn set_framerate_mean_shim(this: &RtcOutboundRtpStreamStats, val: f64);
    #[wasm_bindgen(method, getter = "framerateStdDev")]
    fn framerate_std_dev_shim(this: &RtcOutboundRtpStreamStats) -> f64;
    #[wasm_bindgen(method, setter = "framerateStdDev")]
    fn set_framerate_std_dev_shim(this: &RtcOutboundRtpStreamStats, val: f64);
    #[wasm_bindgen(method, getter = "isRemote")]
    fn is_remote_shim(this: &RtcOutboundRtpStreamStats) -> bool;
    #[wasm_bindgen(method, setter = "isRemote")]
    fn set_is_remote_shim(this: &RtcOutboundRtpStreamStats, val: bool);
    #[wasm_bindgen(method, getter = "mediaTrackId")]
    fn media_track_id_shim(this: &RtcOutboundRtpStreamStats) -> &str;
    #[wasm_bindgen(method, setter = "mediaTrackId")]
    fn set_media_track_id_shim(this: &RtcOutboundRtpStreamStats, val: &str);
    #[wasm_bindgen(method, getter = "mediaType")]
    fn media_type_shim(this: &RtcOutboundRtpStreamStats) -> &str;
    #[wasm_bindgen(method, setter = "mediaType")]
    fn set_media_type_shim(this: &RtcOutboundRtpStreamStats, val: &str);
    #[wasm_bindgen(method, getter = "nackCount")]
    fn nack_count_shim(this: &RtcOutboundRtpStreamStats) -> u32;
    #[wasm_bindgen(method, setter = "nackCount")]
    fn set_nack_count_shim(this: &RtcOutboundRtpStreamStats, val: u32);
    #[wasm_bindgen(method, getter = "pliCount")]
    fn pli_count_shim(this: &RtcOutboundRtpStreamStats) -> u32;
    #[wasm_bindgen(method, setter = "pliCount")]
    fn set_pli_count_shim(this: &RtcOutboundRtpStreamStats, val: u32);
    #[wasm_bindgen(method, getter = "remoteId")]
    fn remote_id_shim(this: &RtcOutboundRtpStreamStats) -> &str;
    #[wasm_bindgen(method, setter = "remoteId")]
    fn set_remote_id_shim(this: &RtcOutboundRtpStreamStats, val: &str);
    #[wasm_bindgen(method, getter = "ssrc")]
    fn ssrc_shim(this: &RtcOutboundRtpStreamStats) -> &str;
    #[wasm_bindgen(method, setter = "ssrc")]
    fn set_ssrc_shim(this: &RtcOutboundRtpStreamStats, val: &str);
    #[wasm_bindgen(method, getter = "transportId")]
    fn transport_id_shim(this: &RtcOutboundRtpStreamStats) -> &str;
    #[wasm_bindgen(method, setter = "transportId")]
    fn set_transport_id_shim(this: &RtcOutboundRtpStreamStats, val: &str);
    #[wasm_bindgen(method, getter = "bytesSent")]
    fn bytes_sent_shim(this: &RtcOutboundRtpStreamStats) -> f64;
    #[wasm_bindgen(method, setter = "bytesSent")]
    fn set_bytes_sent_shim(this: &RtcOutboundRtpStreamStats, val: f64);
    #[wasm_bindgen(method, getter = "droppedFrames")]
    fn dropped_frames_shim(this: &RtcOutboundRtpStreamStats) -> u32;
    #[wasm_bindgen(method, setter = "droppedFrames")]
    fn set_dropped_frames_shim(this: &RtcOutboundRtpStreamStats, val: u32);
    #[wasm_bindgen(method, getter = "framesEncoded")]
    fn frames_encoded_shim(this: &RtcOutboundRtpStreamStats) -> u32;
    #[wasm_bindgen(method, setter = "framesEncoded")]
    fn set_frames_encoded_shim(this: &RtcOutboundRtpStreamStats, val: u32);
    #[wasm_bindgen(method, getter = "packetsSent")]
    fn packets_sent_shim(this: &RtcOutboundRtpStreamStats) -> u32;
    #[wasm_bindgen(method, setter = "packetsSent")]
    fn set_packets_sent_shim(this: &RtcOutboundRtpStreamStats, val: u32);
    #[wasm_bindgen(method, getter = "targetBitrate")]
    fn target_bitrate_shim(this: &RtcOutboundRtpStreamStats) -> f64;
    #[wasm_bindgen(method, setter = "targetBitrate")]
    fn set_target_bitrate_shim(this: &RtcOutboundRtpStreamStats, val: f64);
}
#[doc = "The trait to access properties on the `RtcOutboundRtpStreamStats` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `RtcOutboundRtpStreamStats`*"]
pub trait RtcOutboundRtpStreamStatsGetters {
    #[doc = "Get the `id` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcOutboundRtpStreamStats`*"]
    fn id(&self) -> &str;
    #[doc = "Get the `timestamp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcOutboundRtpStreamStats`*"]
    fn timestamp(&self) -> f64;
    #[cfg(feature = "RtcStatsType")]
    #[doc = "Get the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcOutboundRtpStreamStats`, `RtcStatsType`*"]
    fn type_(&self) -> RtcStatsType;
    #[doc = "Get the `bitrateMean` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcOutboundRtpStreamStats`*"]
    fn bitrate_mean(&self) -> f64;
    #[doc = "Get the `bitrateStdDev` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcOutboundRtpStreamStats`*"]
    fn bitrate_std_dev(&self) -> f64;
    #[doc = "Get the `codecId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcOutboundRtpStreamStats`*"]
    fn codec_id(&self) -> &str;
    #[doc = "Get the `firCount` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcOutboundRtpStreamStats`*"]
    fn fir_count(&self) -> u32;
    #[doc = "Get the `framerateMean` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcOutboundRtpStreamStats`*"]
    fn framerate_mean(&self) -> f64;
    #[doc = "Get the `framerateStdDev` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcOutboundRtpStreamStats`*"]
    fn framerate_std_dev(&self) -> f64;
    #[doc = "Get the `isRemote` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcOutboundRtpStreamStats`*"]
    fn is_remote(&self) -> bool;
    #[doc = "Get the `mediaTrackId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcOutboundRtpStreamStats`*"]
    fn media_track_id(&self) -> &str;
    #[doc = "Get the `mediaType` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcOutboundRtpStreamStats`*"]
    fn media_type(&self) -> &str;
    #[doc = "Get the `nackCount` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcOutboundRtpStreamStats`*"]
    fn nack_count(&self) -> u32;
    #[doc = "Get the `pliCount` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcOutboundRtpStreamStats`*"]
    fn pli_count(&self) -> u32;
    #[doc = "Get the `remoteId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcOutboundRtpStreamStats`*"]
    fn remote_id(&self) -> &str;
    #[doc = "Get the `ssrc` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcOutboundRtpStreamStats`*"]
    fn ssrc(&self) -> &str;
    #[doc = "Get the `transportId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcOutboundRtpStreamStats`*"]
    fn transport_id(&self) -> &str;
    #[doc = "Get the `bytesSent` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcOutboundRtpStreamStats`*"]
    fn bytes_sent(&self) -> f64;
    #[doc = "Get the `droppedFrames` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcOutboundRtpStreamStats`*"]
    fn dropped_frames(&self) -> u32;
    #[doc = "Get the `framesEncoded` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcOutboundRtpStreamStats`*"]
    fn frames_encoded(&self) -> u32;
    #[doc = "Get the `packetsSent` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcOutboundRtpStreamStats`*"]
    fn packets_sent(&self) -> u32;
    #[doc = "Get the `targetBitrate` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcOutboundRtpStreamStats`*"]
    fn target_bitrate(&self) -> f64;
}
impl RtcOutboundRtpStreamStatsGetters for RtcOutboundRtpStreamStats {
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
    fn bitrate_mean(&self) -> f64 {
        self.bitrate_mean_shim()
    }
    fn bitrate_std_dev(&self) -> f64 {
        self.bitrate_std_dev_shim()
    }
    fn codec_id(&self) -> &str {
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
    fn media_track_id(&self) -> &str {
        self.media_track_id_shim()
    }
    fn media_type(&self) -> &str {
        self.media_type_shim()
    }
    fn nack_count(&self) -> u32 {
        self.nack_count_shim()
    }
    fn pli_count(&self) -> u32 {
        self.pli_count_shim()
    }
    fn remote_id(&self) -> &str {
        self.remote_id_shim()
    }
    fn ssrc(&self) -> &str {
        self.ssrc_shim()
    }
    fn transport_id(&self) -> &str {
        self.transport_id_shim()
    }
    fn bytes_sent(&self) -> f64 {
        self.bytes_sent_shim()
    }
    fn dropped_frames(&self) -> u32 {
        self.dropped_frames_shim()
    }
    fn frames_encoded(&self) -> u32 {
        self.frames_encoded_shim()
    }
    fn packets_sent(&self) -> u32 {
        self.packets_sent_shim()
    }
    fn target_bitrate(&self) -> f64 {
        self.target_bitrate_shim()
    }
}
impl RtcOutboundRtpStreamStats {
    #[doc = "Construct a new `RtcOutboundRtpStreamStats`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcOutboundRtpStreamStats`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `id` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcOutboundRtpStreamStats`*"]
    pub fn id(&mut self, val: &str) -> &mut Self {
        self.set_id_shim(val);
        self
    }
    #[doc = "Change the `timestamp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcOutboundRtpStreamStats`*"]
    pub fn timestamp(&mut self, val: f64) -> &mut Self {
        self.set_timestamp_shim(val);
        self
    }
    #[cfg(feature = "RtcStatsType")]
    #[doc = "Change the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcOutboundRtpStreamStats`, `RtcStatsType`*"]
    pub fn type_(&mut self, val: RtcStatsType) -> &mut Self {
        self.set_type__shim(val);
        self
    }
    #[doc = "Change the `bitrateMean` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcOutboundRtpStreamStats`*"]
    pub fn bitrate_mean(&mut self, val: f64) -> &mut Self {
        self.set_bitrate_mean_shim(val);
        self
    }
    #[doc = "Change the `bitrateStdDev` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcOutboundRtpStreamStats`*"]
    pub fn bitrate_std_dev(&mut self, val: f64) -> &mut Self {
        self.set_bitrate_std_dev_shim(val);
        self
    }
    #[doc = "Change the `codecId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcOutboundRtpStreamStats`*"]
    pub fn codec_id(&mut self, val: &str) -> &mut Self {
        self.set_codec_id_shim(val);
        self
    }
    #[doc = "Change the `firCount` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcOutboundRtpStreamStats`*"]
    pub fn fir_count(&mut self, val: u32) -> &mut Self {
        self.set_fir_count_shim(val);
        self
    }
    #[doc = "Change the `framerateMean` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcOutboundRtpStreamStats`*"]
    pub fn framerate_mean(&mut self, val: f64) -> &mut Self {
        self.set_framerate_mean_shim(val);
        self
    }
    #[doc = "Change the `framerateStdDev` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcOutboundRtpStreamStats`*"]
    pub fn framerate_std_dev(&mut self, val: f64) -> &mut Self {
        self.set_framerate_std_dev_shim(val);
        self
    }
    #[doc = "Change the `isRemote` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcOutboundRtpStreamStats`*"]
    pub fn is_remote(&mut self, val: bool) -> &mut Self {
        self.set_is_remote_shim(val);
        self
    }
    #[doc = "Change the `mediaTrackId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcOutboundRtpStreamStats`*"]
    pub fn media_track_id(&mut self, val: &str) -> &mut Self {
        self.set_media_track_id_shim(val);
        self
    }
    #[doc = "Change the `mediaType` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcOutboundRtpStreamStats`*"]
    pub fn media_type(&mut self, val: &str) -> &mut Self {
        self.set_media_type_shim(val);
        self
    }
    #[doc = "Change the `nackCount` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcOutboundRtpStreamStats`*"]
    pub fn nack_count(&mut self, val: u32) -> &mut Self {
        self.set_nack_count_shim(val);
        self
    }
    #[doc = "Change the `pliCount` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcOutboundRtpStreamStats`*"]
    pub fn pli_count(&mut self, val: u32) -> &mut Self {
        self.set_pli_count_shim(val);
        self
    }
    #[doc = "Change the `remoteId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcOutboundRtpStreamStats`*"]
    pub fn remote_id(&mut self, val: &str) -> &mut Self {
        self.set_remote_id_shim(val);
        self
    }
    #[doc = "Change the `ssrc` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcOutboundRtpStreamStats`*"]
    pub fn ssrc(&mut self, val: &str) -> &mut Self {
        self.set_ssrc_shim(val);
        self
    }
    #[doc = "Change the `transportId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcOutboundRtpStreamStats`*"]
    pub fn transport_id(&mut self, val: &str) -> &mut Self {
        self.set_transport_id_shim(val);
        self
    }
    #[doc = "Change the `bytesSent` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcOutboundRtpStreamStats`*"]
    pub fn bytes_sent(&mut self, val: f64) -> &mut Self {
        self.set_bytes_sent_shim(val);
        self
    }
    #[doc = "Change the `droppedFrames` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcOutboundRtpStreamStats`*"]
    pub fn dropped_frames(&mut self, val: u32) -> &mut Self {
        self.set_dropped_frames_shim(val);
        self
    }
    #[doc = "Change the `framesEncoded` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcOutboundRtpStreamStats`*"]
    pub fn frames_encoded(&mut self, val: u32) -> &mut Self {
        self.set_frames_encoded_shim(val);
        self
    }
    #[doc = "Change the `packetsSent` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcOutboundRtpStreamStats`*"]
    pub fn packets_sent(&mut self, val: u32) -> &mut Self {
        self.set_packets_sent_shim(val);
        self
    }
    #[doc = "Change the `targetBitrate` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcOutboundRtpStreamStats`*"]
    pub fn target_bitrate(&mut self, val: f64) -> &mut Self {
        self.set_target_bitrate_shim(val);
        self
    }
}
impl Default for RtcOutboundRtpStreamStats {
    fn default() -> Self {
        Self::new()
    }
}
