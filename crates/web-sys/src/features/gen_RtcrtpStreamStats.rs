#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = RTCRTPStreamStats)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `RtcrtpStreamStats` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcrtpStreamStats`*"]
    pub type RtcrtpStreamStats;
    #[doc = "Get the `id` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcrtpStreamStats`*"]
    #[wasm_bindgen(method, getter = "id")]
    pub fn get_id(this: &RtcrtpStreamStats) -> Option<String>;
    #[wasm_bindgen(method, setter = "id")]
    fn set_id(this: &RtcrtpStreamStats, val: &str);
    #[doc = "Get the `timestamp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcrtpStreamStats`*"]
    #[wasm_bindgen(method, getter = "timestamp")]
    pub fn get_timestamp(this: &RtcrtpStreamStats) -> Option<f64>;
    #[wasm_bindgen(method, setter = "timestamp")]
    fn set_timestamp(this: &RtcrtpStreamStats, val: f64);
    #[cfg(feature = "RtcStatsType")]
    #[doc = "Get the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcStatsType`, `RtcrtpStreamStats`*"]
    #[wasm_bindgen(method, getter = "type")]
    pub fn get_type(this: &RtcrtpStreamStats) -> Option<RtcStatsType>;
    #[cfg(feature = "RtcStatsType")]
    #[wasm_bindgen(method, setter = "type")]
    fn set_type(this: &RtcrtpStreamStats, val: RtcStatsType);
    #[doc = "Get the `bitrateMean` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcrtpStreamStats`*"]
    #[wasm_bindgen(method, getter = "bitrateMean")]
    pub fn get_bitrate_mean(this: &RtcrtpStreamStats) -> Option<f64>;
    #[wasm_bindgen(method, setter = "bitrateMean")]
    fn set_bitrate_mean(this: &RtcrtpStreamStats, val: f64);
    #[doc = "Get the `bitrateStdDev` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcrtpStreamStats`*"]
    #[wasm_bindgen(method, getter = "bitrateStdDev")]
    pub fn get_bitrate_std_dev(this: &RtcrtpStreamStats) -> Option<f64>;
    #[wasm_bindgen(method, setter = "bitrateStdDev")]
    fn set_bitrate_std_dev(this: &RtcrtpStreamStats, val: f64);
    #[doc = "Get the `codecId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcrtpStreamStats`*"]
    #[wasm_bindgen(method, getter = "codecId")]
    pub fn get_codec_id(this: &RtcrtpStreamStats) -> Option<String>;
    #[wasm_bindgen(method, setter = "codecId")]
    fn set_codec_id(this: &RtcrtpStreamStats, val: &str);
    #[doc = "Get the `firCount` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcrtpStreamStats`*"]
    #[wasm_bindgen(method, getter = "firCount")]
    pub fn get_fir_count(this: &RtcrtpStreamStats) -> Option<u32>;
    #[wasm_bindgen(method, setter = "firCount")]
    fn set_fir_count(this: &RtcrtpStreamStats, val: u32);
    #[doc = "Get the `framerateMean` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcrtpStreamStats`*"]
    #[wasm_bindgen(method, getter = "framerateMean")]
    pub fn get_framerate_mean(this: &RtcrtpStreamStats) -> Option<f64>;
    #[wasm_bindgen(method, setter = "framerateMean")]
    fn set_framerate_mean(this: &RtcrtpStreamStats, val: f64);
    #[doc = "Get the `framerateStdDev` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcrtpStreamStats`*"]
    #[wasm_bindgen(method, getter = "framerateStdDev")]
    pub fn get_framerate_std_dev(this: &RtcrtpStreamStats) -> Option<f64>;
    #[wasm_bindgen(method, setter = "framerateStdDev")]
    fn set_framerate_std_dev(this: &RtcrtpStreamStats, val: f64);
    #[doc = "Get the `isRemote` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcrtpStreamStats`*"]
    #[wasm_bindgen(method, getter = "isRemote")]
    pub fn get_is_remote(this: &RtcrtpStreamStats) -> Option<bool>;
    #[wasm_bindgen(method, setter = "isRemote")]
    fn set_is_remote(this: &RtcrtpStreamStats, val: bool);
    #[doc = "Get the `mediaTrackId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcrtpStreamStats`*"]
    #[wasm_bindgen(method, getter = "mediaTrackId")]
    pub fn get_media_track_id(this: &RtcrtpStreamStats) -> Option<String>;
    #[wasm_bindgen(method, setter = "mediaTrackId")]
    fn set_media_track_id(this: &RtcrtpStreamStats, val: &str);
    #[doc = "Get the `mediaType` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcrtpStreamStats`*"]
    #[wasm_bindgen(method, getter = "mediaType")]
    pub fn get_media_type(this: &RtcrtpStreamStats) -> Option<String>;
    #[wasm_bindgen(method, setter = "mediaType")]
    fn set_media_type(this: &RtcrtpStreamStats, val: &str);
    #[doc = "Get the `nackCount` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcrtpStreamStats`*"]
    #[wasm_bindgen(method, getter = "nackCount")]
    pub fn get_nack_count(this: &RtcrtpStreamStats) -> Option<u32>;
    #[wasm_bindgen(method, setter = "nackCount")]
    fn set_nack_count(this: &RtcrtpStreamStats, val: u32);
    #[doc = "Get the `pliCount` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcrtpStreamStats`*"]
    #[wasm_bindgen(method, getter = "pliCount")]
    pub fn get_pli_count(this: &RtcrtpStreamStats) -> Option<u32>;
    #[wasm_bindgen(method, setter = "pliCount")]
    fn set_pli_count(this: &RtcrtpStreamStats, val: u32);
    #[doc = "Get the `remoteId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcrtpStreamStats`*"]
    #[wasm_bindgen(method, getter = "remoteId")]
    pub fn get_remote_id(this: &RtcrtpStreamStats) -> Option<String>;
    #[wasm_bindgen(method, setter = "remoteId")]
    fn set_remote_id(this: &RtcrtpStreamStats, val: &str);
    #[doc = "Get the `ssrc` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcrtpStreamStats`*"]
    #[wasm_bindgen(method, getter = "ssrc")]
    pub fn get_ssrc(this: &RtcrtpStreamStats) -> Option<String>;
    #[wasm_bindgen(method, setter = "ssrc")]
    fn set_ssrc(this: &RtcrtpStreamStats, val: &str);
    #[doc = "Get the `transportId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcrtpStreamStats`*"]
    #[wasm_bindgen(method, getter = "transportId")]
    pub fn get_transport_id(this: &RtcrtpStreamStats) -> Option<String>;
    #[wasm_bindgen(method, setter = "transportId")]
    fn set_transport_id(this: &RtcrtpStreamStats, val: &str);
}
impl RtcrtpStreamStats {
    #[doc = "Construct a new `RtcrtpStreamStats`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcrtpStreamStats`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `id` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcrtpStreamStats`*"]
    pub fn id(&mut self, val: &str) -> &mut Self {
        self.set_id(val);
        self
    }
    #[doc = "Change the `timestamp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcrtpStreamStats`*"]
    pub fn timestamp(&mut self, val: f64) -> &mut Self {
        self.set_timestamp(val);
        self
    }
    #[cfg(feature = "RtcStatsType")]
    #[doc = "Change the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcStatsType`, `RtcrtpStreamStats`*"]
    pub fn type_(&mut self, val: RtcStatsType) -> &mut Self {
        self.set_type(val);
        self
    }
    #[doc = "Change the `bitrateMean` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcrtpStreamStats`*"]
    pub fn bitrate_mean(&mut self, val: f64) -> &mut Self {
        self.set_bitrate_mean(val);
        self
    }
    #[doc = "Change the `bitrateStdDev` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcrtpStreamStats`*"]
    pub fn bitrate_std_dev(&mut self, val: f64) -> &mut Self {
        self.set_bitrate_std_dev(val);
        self
    }
    #[doc = "Change the `codecId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcrtpStreamStats`*"]
    pub fn codec_id(&mut self, val: &str) -> &mut Self {
        self.set_codec_id(val);
        self
    }
    #[doc = "Change the `firCount` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcrtpStreamStats`*"]
    pub fn fir_count(&mut self, val: u32) -> &mut Self {
        self.set_fir_count(val);
        self
    }
    #[doc = "Change the `framerateMean` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcrtpStreamStats`*"]
    pub fn framerate_mean(&mut self, val: f64) -> &mut Self {
        self.set_framerate_mean(val);
        self
    }
    #[doc = "Change the `framerateStdDev` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcrtpStreamStats`*"]
    pub fn framerate_std_dev(&mut self, val: f64) -> &mut Self {
        self.set_framerate_std_dev(val);
        self
    }
    #[doc = "Change the `isRemote` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcrtpStreamStats`*"]
    pub fn is_remote(&mut self, val: bool) -> &mut Self {
        self.set_is_remote(val);
        self
    }
    #[doc = "Change the `mediaTrackId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcrtpStreamStats`*"]
    pub fn media_track_id(&mut self, val: &str) -> &mut Self {
        self.set_media_track_id(val);
        self
    }
    #[doc = "Change the `mediaType` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcrtpStreamStats`*"]
    pub fn media_type(&mut self, val: &str) -> &mut Self {
        self.set_media_type(val);
        self
    }
    #[doc = "Change the `nackCount` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcrtpStreamStats`*"]
    pub fn nack_count(&mut self, val: u32) -> &mut Self {
        self.set_nack_count(val);
        self
    }
    #[doc = "Change the `pliCount` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcrtpStreamStats`*"]
    pub fn pli_count(&mut self, val: u32) -> &mut Self {
        self.set_pli_count(val);
        self
    }
    #[doc = "Change the `remoteId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcrtpStreamStats`*"]
    pub fn remote_id(&mut self, val: &str) -> &mut Self {
        self.set_remote_id(val);
        self
    }
    #[doc = "Change the `ssrc` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcrtpStreamStats`*"]
    pub fn ssrc(&mut self, val: &str) -> &mut Self {
        self.set_ssrc(val);
        self
    }
    #[doc = "Change the `transportId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcrtpStreamStats`*"]
    pub fn transport_id(&mut self, val: &str) -> &mut Self {
        self.set_transport_id(val);
        self
    }
}
impl Default for RtcrtpStreamStats {
    fn default() -> Self {
        Self::new()
    }
}
