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
    #[wasm_bindgen(method, setter = "id")]
    fn id_shim(this: &RtcMediaStreamTrackStats, val: &str);
    #[wasm_bindgen(method, setter = "timestamp")]
    fn timestamp_shim(this: &RtcMediaStreamTrackStats, val: f64);
    #[cfg(feature = "RtcStatsType")]
    #[wasm_bindgen(method, setter = "type")]
    fn type__shim(this: &RtcMediaStreamTrackStats, val: RtcStatsType);
    #[wasm_bindgen(method, setter = "audioLevel")]
    fn audio_level_shim(this: &RtcMediaStreamTrackStats, val: f64);
    #[wasm_bindgen(method, setter = "echoReturnLoss")]
    fn echo_return_loss_shim(this: &RtcMediaStreamTrackStats, val: f64);
    #[wasm_bindgen(method, setter = "echoReturnLossEnhancement")]
    fn echo_return_loss_enhancement_shim(this: &RtcMediaStreamTrackStats, val: f64);
    #[wasm_bindgen(method, setter = "frameHeight")]
    fn frame_height_shim(this: &RtcMediaStreamTrackStats, val: u32);
    #[wasm_bindgen(method, setter = "frameWidth")]
    fn frame_width_shim(this: &RtcMediaStreamTrackStats, val: u32);
    #[wasm_bindgen(method, setter = "framesCorrupted")]
    fn frames_corrupted_shim(this: &RtcMediaStreamTrackStats, val: u32);
    #[wasm_bindgen(method, setter = "framesDecoded")]
    fn frames_decoded_shim(this: &RtcMediaStreamTrackStats, val: u32);
    #[wasm_bindgen(method, setter = "framesDropped")]
    fn frames_dropped_shim(this: &RtcMediaStreamTrackStats, val: u32);
    #[wasm_bindgen(method, setter = "framesPerSecond")]
    fn frames_per_second_shim(this: &RtcMediaStreamTrackStats, val: f64);
    #[wasm_bindgen(method, setter = "framesReceived")]
    fn frames_received_shim(this: &RtcMediaStreamTrackStats, val: u32);
    #[wasm_bindgen(method, setter = "framesSent")]
    fn frames_sent_shim(this: &RtcMediaStreamTrackStats, val: u32);
    #[wasm_bindgen(method, setter = "remoteSource")]
    fn remote_source_shim(this: &RtcMediaStreamTrackStats, val: bool);
    #[wasm_bindgen(method, setter = "ssrcIds")]
    fn ssrc_ids_shim(this: &RtcMediaStreamTrackStats, val: &::wasm_bindgen::JsValue);
    #[wasm_bindgen(method, setter = "trackIdentifier")]
    fn track_identifier_shim(this: &RtcMediaStreamTrackStats, val: &str);
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
        self.id_shim(val);
        self
    }
    #[doc = "Change the `timestamp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcMediaStreamTrackStats`*"]
    pub fn timestamp(&mut self, val: f64) -> &mut Self {
        self.timestamp_shim(val);
        self
    }
    #[cfg(feature = "RtcStatsType")]
    #[doc = "Change the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcMediaStreamTrackStats`, `RtcStatsType`*"]
    pub fn type_(&mut self, val: RtcStatsType) -> &mut Self {
        self.type__shim(val);
        self
    }
    #[doc = "Change the `audioLevel` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcMediaStreamTrackStats`*"]
    pub fn audio_level(&mut self, val: f64) -> &mut Self {
        self.audio_level_shim(val);
        self
    }
    #[doc = "Change the `echoReturnLoss` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcMediaStreamTrackStats`*"]
    pub fn echo_return_loss(&mut self, val: f64) -> &mut Self {
        self.echo_return_loss_shim(val);
        self
    }
    #[doc = "Change the `echoReturnLossEnhancement` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcMediaStreamTrackStats`*"]
    pub fn echo_return_loss_enhancement(&mut self, val: f64) -> &mut Self {
        self.echo_return_loss_enhancement_shim(val);
        self
    }
    #[doc = "Change the `frameHeight` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcMediaStreamTrackStats`*"]
    pub fn frame_height(&mut self, val: u32) -> &mut Self {
        self.frame_height_shim(val);
        self
    }
    #[doc = "Change the `frameWidth` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcMediaStreamTrackStats`*"]
    pub fn frame_width(&mut self, val: u32) -> &mut Self {
        self.frame_width_shim(val);
        self
    }
    #[doc = "Change the `framesCorrupted` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcMediaStreamTrackStats`*"]
    pub fn frames_corrupted(&mut self, val: u32) -> &mut Self {
        self.frames_corrupted_shim(val);
        self
    }
    #[doc = "Change the `framesDecoded` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcMediaStreamTrackStats`*"]
    pub fn frames_decoded(&mut self, val: u32) -> &mut Self {
        self.frames_decoded_shim(val);
        self
    }
    #[doc = "Change the `framesDropped` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcMediaStreamTrackStats`*"]
    pub fn frames_dropped(&mut self, val: u32) -> &mut Self {
        self.frames_dropped_shim(val);
        self
    }
    #[doc = "Change the `framesPerSecond` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcMediaStreamTrackStats`*"]
    pub fn frames_per_second(&mut self, val: f64) -> &mut Self {
        self.frames_per_second_shim(val);
        self
    }
    #[doc = "Change the `framesReceived` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcMediaStreamTrackStats`*"]
    pub fn frames_received(&mut self, val: u32) -> &mut Self {
        self.frames_received_shim(val);
        self
    }
    #[doc = "Change the `framesSent` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcMediaStreamTrackStats`*"]
    pub fn frames_sent(&mut self, val: u32) -> &mut Self {
        self.frames_sent_shim(val);
        self
    }
    #[doc = "Change the `remoteSource` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcMediaStreamTrackStats`*"]
    pub fn remote_source(&mut self, val: bool) -> &mut Self {
        self.remote_source_shim(val);
        self
    }
    #[doc = "Change the `ssrcIds` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcMediaStreamTrackStats`*"]
    pub fn ssrc_ids(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.ssrc_ids_shim(val);
        self
    }
    #[doc = "Change the `trackIdentifier` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcMediaStreamTrackStats`*"]
    pub fn track_identifier(&mut self, val: &str) -> &mut Self {
        self.track_identifier_shim(val);
        self
    }
}
impl Default for RtcMediaStreamTrackStats {
    fn default() -> Self {
        Self::new()
    }
}
