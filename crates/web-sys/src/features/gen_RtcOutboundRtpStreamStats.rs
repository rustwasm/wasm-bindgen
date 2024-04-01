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
    #[wasm_bindgen(method, setter = "id")]
    fn id_shim(this: &RtcOutboundRtpStreamStats, val: &str);
    #[wasm_bindgen(method, setter = "timestamp")]
    fn timestamp_shim(this: &RtcOutboundRtpStreamStats, val: f64);
    #[cfg(feature = "RtcStatsType")]
    #[wasm_bindgen(method, setter = "type")]
    fn type__shim(this: &RtcOutboundRtpStreamStats, val: RtcStatsType);
    #[wasm_bindgen(method, setter = "bitrateMean")]
    fn bitrate_mean_shim(this: &RtcOutboundRtpStreamStats, val: f64);
    #[wasm_bindgen(method, setter = "bitrateStdDev")]
    fn bitrate_std_dev_shim(this: &RtcOutboundRtpStreamStats, val: f64);
    #[wasm_bindgen(method, setter = "codecId")]
    fn codec_id_shim(this: &RtcOutboundRtpStreamStats, val: &str);
    #[wasm_bindgen(method, setter = "firCount")]
    fn fir_count_shim(this: &RtcOutboundRtpStreamStats, val: u32);
    #[wasm_bindgen(method, setter = "framerateMean")]
    fn framerate_mean_shim(this: &RtcOutboundRtpStreamStats, val: f64);
    #[wasm_bindgen(method, setter = "framerateStdDev")]
    fn framerate_std_dev_shim(this: &RtcOutboundRtpStreamStats, val: f64);
    #[wasm_bindgen(method, setter = "isRemote")]
    fn is_remote_shim(this: &RtcOutboundRtpStreamStats, val: bool);
    #[wasm_bindgen(method, setter = "mediaTrackId")]
    fn media_track_id_shim(this: &RtcOutboundRtpStreamStats, val: &str);
    #[wasm_bindgen(method, setter = "mediaType")]
    fn media_type_shim(this: &RtcOutboundRtpStreamStats, val: &str);
    #[wasm_bindgen(method, setter = "nackCount")]
    fn nack_count_shim(this: &RtcOutboundRtpStreamStats, val: u32);
    #[wasm_bindgen(method, setter = "pliCount")]
    fn pli_count_shim(this: &RtcOutboundRtpStreamStats, val: u32);
    #[wasm_bindgen(method, setter = "remoteId")]
    fn remote_id_shim(this: &RtcOutboundRtpStreamStats, val: &str);
    #[wasm_bindgen(method, setter = "ssrc")]
    fn ssrc_shim(this: &RtcOutboundRtpStreamStats, val: &str);
    #[wasm_bindgen(method, setter = "transportId")]
    fn transport_id_shim(this: &RtcOutboundRtpStreamStats, val: &str);
    #[wasm_bindgen(method, setter = "bytesSent")]
    fn bytes_sent_shim(this: &RtcOutboundRtpStreamStats, val: f64);
    #[wasm_bindgen(method, setter = "droppedFrames")]
    fn dropped_frames_shim(this: &RtcOutboundRtpStreamStats, val: u32);
    #[wasm_bindgen(method, setter = "framesEncoded")]
    fn frames_encoded_shim(this: &RtcOutboundRtpStreamStats, val: u32);
    #[wasm_bindgen(method, setter = "packetsSent")]
    fn packets_sent_shim(this: &RtcOutboundRtpStreamStats, val: u32);
    #[wasm_bindgen(method, setter = "targetBitrate")]
    fn target_bitrate_shim(this: &RtcOutboundRtpStreamStats, val: f64);
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
        self.id_shim(val);
        self
    }
    #[doc = "Change the `timestamp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcOutboundRtpStreamStats`*"]
    pub fn timestamp(&mut self, val: f64) -> &mut Self {
        self.timestamp_shim(val);
        self
    }
    #[cfg(feature = "RtcStatsType")]
    #[doc = "Change the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcOutboundRtpStreamStats`, `RtcStatsType`*"]
    pub fn type_(&mut self, val: RtcStatsType) -> &mut Self {
        self.type__shim(val);
        self
    }
    #[doc = "Change the `bitrateMean` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcOutboundRtpStreamStats`*"]
    pub fn bitrate_mean(&mut self, val: f64) -> &mut Self {
        self.bitrate_mean_shim(val);
        self
    }
    #[doc = "Change the `bitrateStdDev` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcOutboundRtpStreamStats`*"]
    pub fn bitrate_std_dev(&mut self, val: f64) -> &mut Self {
        self.bitrate_std_dev_shim(val);
        self
    }
    #[doc = "Change the `codecId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcOutboundRtpStreamStats`*"]
    pub fn codec_id(&mut self, val: &str) -> &mut Self {
        self.codec_id_shim(val);
        self
    }
    #[doc = "Change the `firCount` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcOutboundRtpStreamStats`*"]
    pub fn fir_count(&mut self, val: u32) -> &mut Self {
        self.fir_count_shim(val);
        self
    }
    #[doc = "Change the `framerateMean` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcOutboundRtpStreamStats`*"]
    pub fn framerate_mean(&mut self, val: f64) -> &mut Self {
        self.framerate_mean_shim(val);
        self
    }
    #[doc = "Change the `framerateStdDev` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcOutboundRtpStreamStats`*"]
    pub fn framerate_std_dev(&mut self, val: f64) -> &mut Self {
        self.framerate_std_dev_shim(val);
        self
    }
    #[doc = "Change the `isRemote` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcOutboundRtpStreamStats`*"]
    pub fn is_remote(&mut self, val: bool) -> &mut Self {
        self.is_remote_shim(val);
        self
    }
    #[doc = "Change the `mediaTrackId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcOutboundRtpStreamStats`*"]
    pub fn media_track_id(&mut self, val: &str) -> &mut Self {
        self.media_track_id_shim(val);
        self
    }
    #[doc = "Change the `mediaType` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcOutboundRtpStreamStats`*"]
    pub fn media_type(&mut self, val: &str) -> &mut Self {
        self.media_type_shim(val);
        self
    }
    #[doc = "Change the `nackCount` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcOutboundRtpStreamStats`*"]
    pub fn nack_count(&mut self, val: u32) -> &mut Self {
        self.nack_count_shim(val);
        self
    }
    #[doc = "Change the `pliCount` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcOutboundRtpStreamStats`*"]
    pub fn pli_count(&mut self, val: u32) -> &mut Self {
        self.pli_count_shim(val);
        self
    }
    #[doc = "Change the `remoteId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcOutboundRtpStreamStats`*"]
    pub fn remote_id(&mut self, val: &str) -> &mut Self {
        self.remote_id_shim(val);
        self
    }
    #[doc = "Change the `ssrc` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcOutboundRtpStreamStats`*"]
    pub fn ssrc(&mut self, val: &str) -> &mut Self {
        self.ssrc_shim(val);
        self
    }
    #[doc = "Change the `transportId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcOutboundRtpStreamStats`*"]
    pub fn transport_id(&mut self, val: &str) -> &mut Self {
        self.transport_id_shim(val);
        self
    }
    #[doc = "Change the `bytesSent` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcOutboundRtpStreamStats`*"]
    pub fn bytes_sent(&mut self, val: f64) -> &mut Self {
        self.bytes_sent_shim(val);
        self
    }
    #[doc = "Change the `droppedFrames` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcOutboundRtpStreamStats`*"]
    pub fn dropped_frames(&mut self, val: u32) -> &mut Self {
        self.dropped_frames_shim(val);
        self
    }
    #[doc = "Change the `framesEncoded` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcOutboundRtpStreamStats`*"]
    pub fn frames_encoded(&mut self, val: u32) -> &mut Self {
        self.frames_encoded_shim(val);
        self
    }
    #[doc = "Change the `packetsSent` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcOutboundRtpStreamStats`*"]
    pub fn packets_sent(&mut self, val: u32) -> &mut Self {
        self.packets_sent_shim(val);
        self
    }
    #[doc = "Change the `targetBitrate` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcOutboundRtpStreamStats`*"]
    pub fn target_bitrate(&mut self, val: f64) -> &mut Self {
        self.target_bitrate_shim(val);
        self
    }
}
impl Default for RtcOutboundRtpStreamStats {
    fn default() -> Self {
        Self::new()
    }
}
