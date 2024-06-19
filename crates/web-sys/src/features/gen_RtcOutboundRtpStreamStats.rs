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
    #[doc = "Get the `id` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcOutboundRtpStreamStats`*"]
    #[wasm_bindgen(method, getter = "id")]
    pub fn get_id(this: &RtcOutboundRtpStreamStats) -> Option<String>;
    #[wasm_bindgen(method, setter = "id")]
    fn set_id(this: &RtcOutboundRtpStreamStats, val: &str);
    #[doc = "Get the `timestamp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcOutboundRtpStreamStats`*"]
    #[wasm_bindgen(method, getter = "timestamp")]
    pub fn get_timestamp(this: &RtcOutboundRtpStreamStats) -> Option<f64>;
    #[wasm_bindgen(method, setter = "timestamp")]
    fn set_timestamp(this: &RtcOutboundRtpStreamStats, val: f64);
    #[cfg(feature = "RtcStatsType")]
    #[doc = "Get the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcOutboundRtpStreamStats`, `RtcStatsType`*"]
    #[wasm_bindgen(method, getter = "type")]
    pub fn get_type(this: &RtcOutboundRtpStreamStats) -> Option<RtcStatsType>;
    #[cfg(feature = "RtcStatsType")]
    #[wasm_bindgen(method, setter = "type")]
    fn set_type(this: &RtcOutboundRtpStreamStats, val: RtcStatsType);
    #[doc = "Get the `bitrateMean` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcOutboundRtpStreamStats`*"]
    #[wasm_bindgen(method, getter = "bitrateMean")]
    pub fn get_bitrate_mean(this: &RtcOutboundRtpStreamStats) -> Option<f64>;
    #[wasm_bindgen(method, setter = "bitrateMean")]
    fn set_bitrate_mean(this: &RtcOutboundRtpStreamStats, val: f64);
    #[doc = "Get the `bitrateStdDev` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcOutboundRtpStreamStats`*"]
    #[wasm_bindgen(method, getter = "bitrateStdDev")]
    pub fn get_bitrate_std_dev(this: &RtcOutboundRtpStreamStats) -> Option<f64>;
    #[wasm_bindgen(method, setter = "bitrateStdDev")]
    fn set_bitrate_std_dev(this: &RtcOutboundRtpStreamStats, val: f64);
    #[doc = "Get the `codecId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcOutboundRtpStreamStats`*"]
    #[wasm_bindgen(method, getter = "codecId")]
    pub fn get_codec_id(this: &RtcOutboundRtpStreamStats) -> Option<String>;
    #[wasm_bindgen(method, setter = "codecId")]
    fn set_codec_id(this: &RtcOutboundRtpStreamStats, val: &str);
    #[doc = "Get the `firCount` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcOutboundRtpStreamStats`*"]
    #[wasm_bindgen(method, getter = "firCount")]
    pub fn get_fir_count(this: &RtcOutboundRtpStreamStats) -> Option<u32>;
    #[wasm_bindgen(method, setter = "firCount")]
    fn set_fir_count(this: &RtcOutboundRtpStreamStats, val: u32);
    #[doc = "Get the `framerateMean` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcOutboundRtpStreamStats`*"]
    #[wasm_bindgen(method, getter = "framerateMean")]
    pub fn get_framerate_mean(this: &RtcOutboundRtpStreamStats) -> Option<f64>;
    #[wasm_bindgen(method, setter = "framerateMean")]
    fn set_framerate_mean(this: &RtcOutboundRtpStreamStats, val: f64);
    #[doc = "Get the `framerateStdDev` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcOutboundRtpStreamStats`*"]
    #[wasm_bindgen(method, getter = "framerateStdDev")]
    pub fn get_framerate_std_dev(this: &RtcOutboundRtpStreamStats) -> Option<f64>;
    #[wasm_bindgen(method, setter = "framerateStdDev")]
    fn set_framerate_std_dev(this: &RtcOutboundRtpStreamStats, val: f64);
    #[doc = "Get the `isRemote` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcOutboundRtpStreamStats`*"]
    #[wasm_bindgen(method, getter = "isRemote")]
    pub fn get_is_remote(this: &RtcOutboundRtpStreamStats) -> Option<bool>;
    #[wasm_bindgen(method, setter = "isRemote")]
    fn set_is_remote(this: &RtcOutboundRtpStreamStats, val: bool);
    #[doc = "Get the `mediaTrackId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcOutboundRtpStreamStats`*"]
    #[wasm_bindgen(method, getter = "mediaTrackId")]
    pub fn get_media_track_id(this: &RtcOutboundRtpStreamStats) -> Option<String>;
    #[wasm_bindgen(method, setter = "mediaTrackId")]
    fn set_media_track_id(this: &RtcOutboundRtpStreamStats, val: &str);
    #[doc = "Get the `mediaType` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcOutboundRtpStreamStats`*"]
    #[wasm_bindgen(method, getter = "mediaType")]
    pub fn get_media_type(this: &RtcOutboundRtpStreamStats) -> Option<String>;
    #[wasm_bindgen(method, setter = "mediaType")]
    fn set_media_type(this: &RtcOutboundRtpStreamStats, val: &str);
    #[doc = "Get the `nackCount` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcOutboundRtpStreamStats`*"]
    #[wasm_bindgen(method, getter = "nackCount")]
    pub fn get_nack_count(this: &RtcOutboundRtpStreamStats) -> Option<u32>;
    #[wasm_bindgen(method, setter = "nackCount")]
    fn set_nack_count(this: &RtcOutboundRtpStreamStats, val: u32);
    #[doc = "Get the `pliCount` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcOutboundRtpStreamStats`*"]
    #[wasm_bindgen(method, getter = "pliCount")]
    pub fn get_pli_count(this: &RtcOutboundRtpStreamStats) -> Option<u32>;
    #[wasm_bindgen(method, setter = "pliCount")]
    fn set_pli_count(this: &RtcOutboundRtpStreamStats, val: u32);
    #[doc = "Get the `remoteId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcOutboundRtpStreamStats`*"]
    #[wasm_bindgen(method, getter = "remoteId")]
    pub fn get_remote_id(this: &RtcOutboundRtpStreamStats) -> Option<String>;
    #[wasm_bindgen(method, setter = "remoteId")]
    fn set_remote_id(this: &RtcOutboundRtpStreamStats, val: &str);
    #[doc = "Get the `ssrc` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcOutboundRtpStreamStats`*"]
    #[wasm_bindgen(method, getter = "ssrc")]
    pub fn get_ssrc(this: &RtcOutboundRtpStreamStats) -> Option<String>;
    #[wasm_bindgen(method, setter = "ssrc")]
    fn set_ssrc(this: &RtcOutboundRtpStreamStats, val: &str);
    #[doc = "Get the `transportId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcOutboundRtpStreamStats`*"]
    #[wasm_bindgen(method, getter = "transportId")]
    pub fn get_transport_id(this: &RtcOutboundRtpStreamStats) -> Option<String>;
    #[wasm_bindgen(method, setter = "transportId")]
    fn set_transport_id(this: &RtcOutboundRtpStreamStats, val: &str);
    #[doc = "Get the `bytesSent` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcOutboundRtpStreamStats`*"]
    #[wasm_bindgen(method, getter = "bytesSent")]
    pub fn get_bytes_sent(this: &RtcOutboundRtpStreamStats) -> Option<f64>;
    #[wasm_bindgen(method, setter = "bytesSent")]
    fn set_bytes_sent(this: &RtcOutboundRtpStreamStats, val: f64);
    #[doc = "Get the `droppedFrames` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcOutboundRtpStreamStats`*"]
    #[wasm_bindgen(method, getter = "droppedFrames")]
    pub fn get_dropped_frames(this: &RtcOutboundRtpStreamStats) -> Option<u32>;
    #[wasm_bindgen(method, setter = "droppedFrames")]
    fn set_dropped_frames(this: &RtcOutboundRtpStreamStats, val: u32);
    #[doc = "Get the `framesEncoded` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcOutboundRtpStreamStats`*"]
    #[wasm_bindgen(method, getter = "framesEncoded")]
    pub fn get_frames_encoded(this: &RtcOutboundRtpStreamStats) -> Option<u32>;
    #[wasm_bindgen(method, setter = "framesEncoded")]
    fn set_frames_encoded(this: &RtcOutboundRtpStreamStats, val: u32);
    #[doc = "Get the `packetsSent` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcOutboundRtpStreamStats`*"]
    #[wasm_bindgen(method, getter = "packetsSent")]
    pub fn get_packets_sent(this: &RtcOutboundRtpStreamStats) -> Option<u32>;
    #[wasm_bindgen(method, setter = "packetsSent")]
    fn set_packets_sent(this: &RtcOutboundRtpStreamStats, val: u32);
    #[doc = "Get the `targetBitrate` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcOutboundRtpStreamStats`*"]
    #[wasm_bindgen(method, getter = "targetBitrate")]
    pub fn get_target_bitrate(this: &RtcOutboundRtpStreamStats) -> Option<f64>;
    #[wasm_bindgen(method, setter = "targetBitrate")]
    fn set_target_bitrate(this: &RtcOutboundRtpStreamStats, val: f64);
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
        self.set_id(val);
        self
    }
    #[doc = "Change the `timestamp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcOutboundRtpStreamStats`*"]
    pub fn timestamp(&mut self, val: f64) -> &mut Self {
        self.set_timestamp(val);
        self
    }
    #[cfg(feature = "RtcStatsType")]
    #[doc = "Change the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcOutboundRtpStreamStats`, `RtcStatsType`*"]
    pub fn type_(&mut self, val: RtcStatsType) -> &mut Self {
        self.set_type(val);
        self
    }
    #[doc = "Change the `bitrateMean` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcOutboundRtpStreamStats`*"]
    pub fn bitrate_mean(&mut self, val: f64) -> &mut Self {
        self.set_bitrate_mean(val);
        self
    }
    #[doc = "Change the `bitrateStdDev` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcOutboundRtpStreamStats`*"]
    pub fn bitrate_std_dev(&mut self, val: f64) -> &mut Self {
        self.set_bitrate_std_dev(val);
        self
    }
    #[doc = "Change the `codecId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcOutboundRtpStreamStats`*"]
    pub fn codec_id(&mut self, val: &str) -> &mut Self {
        self.set_codec_id(val);
        self
    }
    #[doc = "Change the `firCount` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcOutboundRtpStreamStats`*"]
    pub fn fir_count(&mut self, val: u32) -> &mut Self {
        self.set_fir_count(val);
        self
    }
    #[doc = "Change the `framerateMean` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcOutboundRtpStreamStats`*"]
    pub fn framerate_mean(&mut self, val: f64) -> &mut Self {
        self.set_framerate_mean(val);
        self
    }
    #[doc = "Change the `framerateStdDev` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcOutboundRtpStreamStats`*"]
    pub fn framerate_std_dev(&mut self, val: f64) -> &mut Self {
        self.set_framerate_std_dev(val);
        self
    }
    #[doc = "Change the `isRemote` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcOutboundRtpStreamStats`*"]
    pub fn is_remote(&mut self, val: bool) -> &mut Self {
        self.set_is_remote(val);
        self
    }
    #[doc = "Change the `mediaTrackId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcOutboundRtpStreamStats`*"]
    pub fn media_track_id(&mut self, val: &str) -> &mut Self {
        self.set_media_track_id(val);
        self
    }
    #[doc = "Change the `mediaType` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcOutboundRtpStreamStats`*"]
    pub fn media_type(&mut self, val: &str) -> &mut Self {
        self.set_media_type(val);
        self
    }
    #[doc = "Change the `nackCount` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcOutboundRtpStreamStats`*"]
    pub fn nack_count(&mut self, val: u32) -> &mut Self {
        self.set_nack_count(val);
        self
    }
    #[doc = "Change the `pliCount` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcOutboundRtpStreamStats`*"]
    pub fn pli_count(&mut self, val: u32) -> &mut Self {
        self.set_pli_count(val);
        self
    }
    #[doc = "Change the `remoteId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcOutboundRtpStreamStats`*"]
    pub fn remote_id(&mut self, val: &str) -> &mut Self {
        self.set_remote_id(val);
        self
    }
    #[doc = "Change the `ssrc` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcOutboundRtpStreamStats`*"]
    pub fn ssrc(&mut self, val: &str) -> &mut Self {
        self.set_ssrc(val);
        self
    }
    #[doc = "Change the `transportId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcOutboundRtpStreamStats`*"]
    pub fn transport_id(&mut self, val: &str) -> &mut Self {
        self.set_transport_id(val);
        self
    }
    #[doc = "Change the `bytesSent` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcOutboundRtpStreamStats`*"]
    pub fn bytes_sent(&mut self, val: f64) -> &mut Self {
        self.set_bytes_sent(val);
        self
    }
    #[doc = "Change the `droppedFrames` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcOutboundRtpStreamStats`*"]
    pub fn dropped_frames(&mut self, val: u32) -> &mut Self {
        self.set_dropped_frames(val);
        self
    }
    #[doc = "Change the `framesEncoded` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcOutboundRtpStreamStats`*"]
    pub fn frames_encoded(&mut self, val: u32) -> &mut Self {
        self.set_frames_encoded(val);
        self
    }
    #[doc = "Change the `packetsSent` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcOutboundRtpStreamStats`*"]
    pub fn packets_sent(&mut self, val: u32) -> &mut Self {
        self.set_packets_sent(val);
        self
    }
    #[doc = "Change the `targetBitrate` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcOutboundRtpStreamStats`*"]
    pub fn target_bitrate(&mut self, val: f64) -> &mut Self {
        self.set_target_bitrate(val);
        self
    }
}
impl Default for RtcOutboundRtpStreamStats {
    fn default() -> Self {
        Self::new()
    }
}
