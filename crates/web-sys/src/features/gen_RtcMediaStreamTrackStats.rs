#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = RTCMediaStreamTrackStats)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `RtcMediaStreamTrackStats` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcMediaStreamTrackStats`*"]
    pub type RtcMediaStreamTrackStats;
    #[wasm_bindgen(method, getter = "id")]
    fn id_shim(this: &RtcMediaStreamTrackStats) -> String;
    #[wasm_bindgen(method, setter = "id")]
    fn set_id_shim(this: &RtcMediaStreamTrackStats, val: &str);
    #[wasm_bindgen(method, getter = "timestamp")]
    fn timestamp_shim(this: &RtcMediaStreamTrackStats) -> f64;
    #[wasm_bindgen(method, setter = "timestamp")]
    fn set_timestamp_shim(this: &RtcMediaStreamTrackStats, val: f64);
    #[cfg(feature = "RtcStatsType")]
    #[wasm_bindgen(method, getter = "type")]
    fn type__shim(this: &RtcMediaStreamTrackStats) -> RtcStatsType;
    #[cfg(feature = "RtcStatsType")]
    #[wasm_bindgen(method, setter = "type")]
    fn set_type__shim(this: &RtcMediaStreamTrackStats, val: RtcStatsType);
    #[wasm_bindgen(method, getter = "audioLevel")]
    fn audio_level_shim(this: &RtcMediaStreamTrackStats) -> f64;
    #[wasm_bindgen(method, setter = "audioLevel")]
    fn set_audio_level_shim(this: &RtcMediaStreamTrackStats, val: f64);
    #[wasm_bindgen(method, getter = "echoReturnLoss")]
    fn echo_return_loss_shim(this: &RtcMediaStreamTrackStats) -> f64;
    #[wasm_bindgen(method, setter = "echoReturnLoss")]
    fn set_echo_return_loss_shim(this: &RtcMediaStreamTrackStats, val: f64);
    #[wasm_bindgen(method, getter = "echoReturnLossEnhancement")]
    fn echo_return_loss_enhancement_shim(this: &RtcMediaStreamTrackStats) -> f64;
    #[wasm_bindgen(method, setter = "echoReturnLossEnhancement")]
    fn set_echo_return_loss_enhancement_shim(this: &RtcMediaStreamTrackStats, val: f64);
    #[wasm_bindgen(method, getter = "frameHeight")]
    fn frame_height_shim(this: &RtcMediaStreamTrackStats) -> u32;
    #[wasm_bindgen(method, setter = "frameHeight")]
    fn set_frame_height_shim(this: &RtcMediaStreamTrackStats, val: u32);
    #[wasm_bindgen(method, getter = "frameWidth")]
    fn frame_width_shim(this: &RtcMediaStreamTrackStats) -> u32;
    #[wasm_bindgen(method, setter = "frameWidth")]
    fn set_frame_width_shim(this: &RtcMediaStreamTrackStats, val: u32);
    #[wasm_bindgen(method, getter = "framesCorrupted")]
    fn frames_corrupted_shim(this: &RtcMediaStreamTrackStats) -> u32;
    #[wasm_bindgen(method, setter = "framesCorrupted")]
    fn set_frames_corrupted_shim(this: &RtcMediaStreamTrackStats, val: u32);
    #[wasm_bindgen(method, getter = "framesDecoded")]
    fn frames_decoded_shim(this: &RtcMediaStreamTrackStats) -> u32;
    #[wasm_bindgen(method, setter = "framesDecoded")]
    fn set_frames_decoded_shim(this: &RtcMediaStreamTrackStats, val: u32);
    #[wasm_bindgen(method, getter = "framesDropped")]
    fn frames_dropped_shim(this: &RtcMediaStreamTrackStats) -> u32;
    #[wasm_bindgen(method, setter = "framesDropped")]
    fn set_frames_dropped_shim(this: &RtcMediaStreamTrackStats, val: u32);
    #[wasm_bindgen(method, getter = "framesPerSecond")]
    fn frames_per_second_shim(this: &RtcMediaStreamTrackStats) -> f64;
    #[wasm_bindgen(method, setter = "framesPerSecond")]
    fn set_frames_per_second_shim(this: &RtcMediaStreamTrackStats, val: f64);
    #[wasm_bindgen(method, getter = "framesReceived")]
    fn frames_received_shim(this: &RtcMediaStreamTrackStats) -> u32;
    #[wasm_bindgen(method, setter = "framesReceived")]
    fn set_frames_received_shim(this: &RtcMediaStreamTrackStats, val: u32);
    #[wasm_bindgen(method, getter = "framesSent")]
    fn frames_sent_shim(this: &RtcMediaStreamTrackStats) -> u32;
    #[wasm_bindgen(method, setter = "framesSent")]
    fn set_frames_sent_shim(this: &RtcMediaStreamTrackStats, val: u32);
    #[wasm_bindgen(method, getter = "remoteSource")]
    fn remote_source_shim(this: &RtcMediaStreamTrackStats) -> bool;
    #[wasm_bindgen(method, setter = "remoteSource")]
    fn set_remote_source_shim(this: &RtcMediaStreamTrackStats, val: bool);
    #[wasm_bindgen(method, getter = "ssrcIds")]
    fn ssrc_ids_shim(this: &RtcMediaStreamTrackStats) -> ::js_sys::Array;
    #[wasm_bindgen(method, setter = "ssrcIds")]
    fn set_ssrc_ids_shim(this: &RtcMediaStreamTrackStats, val: &::wasm_bindgen::JsValue);
    #[wasm_bindgen(method, getter = "trackIdentifier")]
    fn track_identifier_shim(this: &RtcMediaStreamTrackStats) -> String;
    #[wasm_bindgen(method, setter = "trackIdentifier")]
    fn set_track_identifier_shim(this: &RtcMediaStreamTrackStats, val: &str);
}
#[doc = "The trait to access properties on the `RtcMediaStreamTrackStats` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `RtcMediaStreamTrackStats`*"]
pub trait RtcMediaStreamTrackStatsGetters {
    #[doc = "Get the `id` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcMediaStreamTrackStats`*"]
    fn id(&self) -> String;
    #[doc = "Get the `timestamp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcMediaStreamTrackStats`*"]
    fn timestamp(&self) -> f64;
    #[cfg(feature = "RtcStatsType")]
    #[doc = "Get the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcMediaStreamTrackStats`, `RtcStatsType`*"]
    fn type_(&self) -> RtcStatsType;
    #[doc = "Get the `audioLevel` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcMediaStreamTrackStats`*"]
    fn audio_level(&self) -> f64;
    #[doc = "Get the `echoReturnLoss` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcMediaStreamTrackStats`*"]
    fn echo_return_loss(&self) -> f64;
    #[doc = "Get the `echoReturnLossEnhancement` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcMediaStreamTrackStats`*"]
    fn echo_return_loss_enhancement(&self) -> f64;
    #[doc = "Get the `frameHeight` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcMediaStreamTrackStats`*"]
    fn frame_height(&self) -> u32;
    #[doc = "Get the `frameWidth` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcMediaStreamTrackStats`*"]
    fn frame_width(&self) -> u32;
    #[doc = "Get the `framesCorrupted` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcMediaStreamTrackStats`*"]
    fn frames_corrupted(&self) -> u32;
    #[doc = "Get the `framesDecoded` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcMediaStreamTrackStats`*"]
    fn frames_decoded(&self) -> u32;
    #[doc = "Get the `framesDropped` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcMediaStreamTrackStats`*"]
    fn frames_dropped(&self) -> u32;
    #[doc = "Get the `framesPerSecond` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcMediaStreamTrackStats`*"]
    fn frames_per_second(&self) -> f64;
    #[doc = "Get the `framesReceived` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcMediaStreamTrackStats`*"]
    fn frames_received(&self) -> u32;
    #[doc = "Get the `framesSent` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcMediaStreamTrackStats`*"]
    fn frames_sent(&self) -> u32;
    #[doc = "Get the `remoteSource` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcMediaStreamTrackStats`*"]
    fn remote_source(&self) -> bool;
    #[doc = "Get the `ssrcIds` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcMediaStreamTrackStats`*"]
    fn ssrc_ids(&self) -> ::js_sys::Array;
    #[doc = "Get the `trackIdentifier` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcMediaStreamTrackStats`*"]
    fn track_identifier(&self) -> String;
}
impl RtcMediaStreamTrackStatsGetters for RtcMediaStreamTrackStats {
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
    fn audio_level(&self) -> f64 {
        self.audio_level_shim()
    }
    fn echo_return_loss(&self) -> f64 {
        self.echo_return_loss_shim()
    }
    fn echo_return_loss_enhancement(&self) -> f64 {
        self.echo_return_loss_enhancement_shim()
    }
    fn frame_height(&self) -> u32 {
        self.frame_height_shim()
    }
    fn frame_width(&self) -> u32 {
        self.frame_width_shim()
    }
    fn frames_corrupted(&self) -> u32 {
        self.frames_corrupted_shim()
    }
    fn frames_decoded(&self) -> u32 {
        self.frames_decoded_shim()
    }
    fn frames_dropped(&self) -> u32 {
        self.frames_dropped_shim()
    }
    fn frames_per_second(&self) -> f64 {
        self.frames_per_second_shim()
    }
    fn frames_received(&self) -> u32 {
        self.frames_received_shim()
    }
    fn frames_sent(&self) -> u32 {
        self.frames_sent_shim()
    }
    fn remote_source(&self) -> bool {
        self.remote_source_shim()
    }
    fn ssrc_ids(&self) -> ::js_sys::Array {
        self.ssrc_ids_shim()
    }
    fn track_identifier(&self) -> String {
        self.track_identifier_shim()
    }
}
impl RtcMediaStreamTrackStats {
    #[doc = "Construct a new `RtcMediaStreamTrackStats`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcMediaStreamTrackStats`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `id` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcMediaStreamTrackStats`*"]
    pub fn id(&mut self, val: &str) -> &mut Self {
        self.set_id_shim(val);
        self
    }
    #[doc = "Change the `timestamp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcMediaStreamTrackStats`*"]
    pub fn timestamp(&mut self, val: f64) -> &mut Self {
        self.set_timestamp_shim(val);
        self
    }
    #[cfg(feature = "RtcStatsType")]
    #[doc = "Change the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcMediaStreamTrackStats`, `RtcStatsType`*"]
    pub fn type_(&mut self, val: RtcStatsType) -> &mut Self {
        self.set_type__shim(val);
        self
    }
    #[doc = "Change the `audioLevel` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcMediaStreamTrackStats`*"]
    pub fn audio_level(&mut self, val: f64) -> &mut Self {
        self.set_audio_level_shim(val);
        self
    }
    #[doc = "Change the `echoReturnLoss` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcMediaStreamTrackStats`*"]
    pub fn echo_return_loss(&mut self, val: f64) -> &mut Self {
        self.set_echo_return_loss_shim(val);
        self
    }
    #[doc = "Change the `echoReturnLossEnhancement` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcMediaStreamTrackStats`*"]
    pub fn echo_return_loss_enhancement(&mut self, val: f64) -> &mut Self {
        self.set_echo_return_loss_enhancement_shim(val);
        self
    }
    #[doc = "Change the `frameHeight` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcMediaStreamTrackStats`*"]
    pub fn frame_height(&mut self, val: u32) -> &mut Self {
        self.set_frame_height_shim(val);
        self
    }
    #[doc = "Change the `frameWidth` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcMediaStreamTrackStats`*"]
    pub fn frame_width(&mut self, val: u32) -> &mut Self {
        self.set_frame_width_shim(val);
        self
    }
    #[doc = "Change the `framesCorrupted` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcMediaStreamTrackStats`*"]
    pub fn frames_corrupted(&mut self, val: u32) -> &mut Self {
        self.set_frames_corrupted_shim(val);
        self
    }
    #[doc = "Change the `framesDecoded` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcMediaStreamTrackStats`*"]
    pub fn frames_decoded(&mut self, val: u32) -> &mut Self {
        self.set_frames_decoded_shim(val);
        self
    }
    #[doc = "Change the `framesDropped` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcMediaStreamTrackStats`*"]
    pub fn frames_dropped(&mut self, val: u32) -> &mut Self {
        self.set_frames_dropped_shim(val);
        self
    }
    #[doc = "Change the `framesPerSecond` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcMediaStreamTrackStats`*"]
    pub fn frames_per_second(&mut self, val: f64) -> &mut Self {
        self.set_frames_per_second_shim(val);
        self
    }
    #[doc = "Change the `framesReceived` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcMediaStreamTrackStats`*"]
    pub fn frames_received(&mut self, val: u32) -> &mut Self {
        self.set_frames_received_shim(val);
        self
    }
    #[doc = "Change the `framesSent` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcMediaStreamTrackStats`*"]
    pub fn frames_sent(&mut self, val: u32) -> &mut Self {
        self.set_frames_sent_shim(val);
        self
    }
    #[doc = "Change the `remoteSource` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcMediaStreamTrackStats`*"]
    pub fn remote_source(&mut self, val: bool) -> &mut Self {
        self.set_remote_source_shim(val);
        self
    }
    #[doc = "Change the `ssrcIds` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcMediaStreamTrackStats`*"]
    pub fn ssrc_ids(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_ssrc_ids_shim(val);
        self
    }
    #[doc = "Change the `trackIdentifier` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcMediaStreamTrackStats`*"]
    pub fn track_identifier(&mut self, val: &str) -> &mut Self {
        self.set_track_identifier_shim(val);
        self
    }
}
impl Default for RtcMediaStreamTrackStats {
    fn default() -> Self {
        Self::new()
    }
}
