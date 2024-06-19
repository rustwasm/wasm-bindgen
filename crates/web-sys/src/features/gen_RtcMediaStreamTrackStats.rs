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
    #[doc = "Get the `id` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcMediaStreamTrackStats`*"]
    #[wasm_bindgen(method, getter = "id")]
    pub fn get_id(this: &RtcMediaStreamTrackStats) -> Option<String>;
    #[wasm_bindgen(method, setter = "id")]
    fn set_id(this: &RtcMediaStreamTrackStats, val: &str);
    #[doc = "Get the `timestamp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcMediaStreamTrackStats`*"]
    #[wasm_bindgen(method, getter = "timestamp")]
    pub fn get_timestamp(this: &RtcMediaStreamTrackStats) -> Option<f64>;
    #[wasm_bindgen(method, setter = "timestamp")]
    fn set_timestamp(this: &RtcMediaStreamTrackStats, val: f64);
    #[cfg(feature = "RtcStatsType")]
    #[doc = "Get the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcMediaStreamTrackStats`, `RtcStatsType`*"]
    #[wasm_bindgen(method, getter = "type")]
    pub fn get_type(this: &RtcMediaStreamTrackStats) -> Option<RtcStatsType>;
    #[cfg(feature = "RtcStatsType")]
    #[wasm_bindgen(method, setter = "type")]
    fn set_type(this: &RtcMediaStreamTrackStats, val: RtcStatsType);
    #[doc = "Get the `audioLevel` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcMediaStreamTrackStats`*"]
    #[wasm_bindgen(method, getter = "audioLevel")]
    pub fn get_audio_level(this: &RtcMediaStreamTrackStats) -> Option<f64>;
    #[wasm_bindgen(method, setter = "audioLevel")]
    fn set_audio_level(this: &RtcMediaStreamTrackStats, val: f64);
    #[doc = "Get the `echoReturnLoss` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcMediaStreamTrackStats`*"]
    #[wasm_bindgen(method, getter = "echoReturnLoss")]
    pub fn get_echo_return_loss(this: &RtcMediaStreamTrackStats) -> Option<f64>;
    #[wasm_bindgen(method, setter = "echoReturnLoss")]
    fn set_echo_return_loss(this: &RtcMediaStreamTrackStats, val: f64);
    #[doc = "Get the `echoReturnLossEnhancement` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcMediaStreamTrackStats`*"]
    #[wasm_bindgen(method, getter = "echoReturnLossEnhancement")]
    pub fn get_echo_return_loss_enhancement(this: &RtcMediaStreamTrackStats) -> Option<f64>;
    #[wasm_bindgen(method, setter = "echoReturnLossEnhancement")]
    fn set_echo_return_loss_enhancement(this: &RtcMediaStreamTrackStats, val: f64);
    #[doc = "Get the `frameHeight` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcMediaStreamTrackStats`*"]
    #[wasm_bindgen(method, getter = "frameHeight")]
    pub fn get_frame_height(this: &RtcMediaStreamTrackStats) -> Option<u32>;
    #[wasm_bindgen(method, setter = "frameHeight")]
    fn set_frame_height(this: &RtcMediaStreamTrackStats, val: u32);
    #[doc = "Get the `frameWidth` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcMediaStreamTrackStats`*"]
    #[wasm_bindgen(method, getter = "frameWidth")]
    pub fn get_frame_width(this: &RtcMediaStreamTrackStats) -> Option<u32>;
    #[wasm_bindgen(method, setter = "frameWidth")]
    fn set_frame_width(this: &RtcMediaStreamTrackStats, val: u32);
    #[doc = "Get the `framesCorrupted` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcMediaStreamTrackStats`*"]
    #[wasm_bindgen(method, getter = "framesCorrupted")]
    pub fn get_frames_corrupted(this: &RtcMediaStreamTrackStats) -> Option<u32>;
    #[wasm_bindgen(method, setter = "framesCorrupted")]
    fn set_frames_corrupted(this: &RtcMediaStreamTrackStats, val: u32);
    #[doc = "Get the `framesDecoded` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcMediaStreamTrackStats`*"]
    #[wasm_bindgen(method, getter = "framesDecoded")]
    pub fn get_frames_decoded(this: &RtcMediaStreamTrackStats) -> Option<u32>;
    #[wasm_bindgen(method, setter = "framesDecoded")]
    fn set_frames_decoded(this: &RtcMediaStreamTrackStats, val: u32);
    #[doc = "Get the `framesDropped` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcMediaStreamTrackStats`*"]
    #[wasm_bindgen(method, getter = "framesDropped")]
    pub fn get_frames_dropped(this: &RtcMediaStreamTrackStats) -> Option<u32>;
    #[wasm_bindgen(method, setter = "framesDropped")]
    fn set_frames_dropped(this: &RtcMediaStreamTrackStats, val: u32);
    #[doc = "Get the `framesPerSecond` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcMediaStreamTrackStats`*"]
    #[wasm_bindgen(method, getter = "framesPerSecond")]
    pub fn get_frames_per_second(this: &RtcMediaStreamTrackStats) -> Option<f64>;
    #[wasm_bindgen(method, setter = "framesPerSecond")]
    fn set_frames_per_second(this: &RtcMediaStreamTrackStats, val: f64);
    #[doc = "Get the `framesReceived` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcMediaStreamTrackStats`*"]
    #[wasm_bindgen(method, getter = "framesReceived")]
    pub fn get_frames_received(this: &RtcMediaStreamTrackStats) -> Option<u32>;
    #[wasm_bindgen(method, setter = "framesReceived")]
    fn set_frames_received(this: &RtcMediaStreamTrackStats, val: u32);
    #[doc = "Get the `framesSent` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcMediaStreamTrackStats`*"]
    #[wasm_bindgen(method, getter = "framesSent")]
    pub fn get_frames_sent(this: &RtcMediaStreamTrackStats) -> Option<u32>;
    #[wasm_bindgen(method, setter = "framesSent")]
    fn set_frames_sent(this: &RtcMediaStreamTrackStats, val: u32);
    #[doc = "Get the `remoteSource` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcMediaStreamTrackStats`*"]
    #[wasm_bindgen(method, getter = "remoteSource")]
    pub fn get_remote_source(this: &RtcMediaStreamTrackStats) -> Option<bool>;
    #[wasm_bindgen(method, setter = "remoteSource")]
    fn set_remote_source(this: &RtcMediaStreamTrackStats, val: bool);
    #[doc = "Get the `ssrcIds` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcMediaStreamTrackStats`*"]
    #[wasm_bindgen(method, getter = "ssrcIds")]
    pub fn get_ssrc_ids(this: &RtcMediaStreamTrackStats) -> Option<::js_sys::Array>;
    #[wasm_bindgen(method, setter = "ssrcIds")]
    fn set_ssrc_ids(this: &RtcMediaStreamTrackStats, val: &::wasm_bindgen::JsValue);
    #[doc = "Get the `trackIdentifier` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcMediaStreamTrackStats`*"]
    #[wasm_bindgen(method, getter = "trackIdentifier")]
    pub fn get_track_identifier(this: &RtcMediaStreamTrackStats) -> Option<String>;
    #[wasm_bindgen(method, setter = "trackIdentifier")]
    fn set_track_identifier(this: &RtcMediaStreamTrackStats, val: &str);
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
        self.set_id(val);
        self
    }
    #[doc = "Change the `timestamp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcMediaStreamTrackStats`*"]
    pub fn timestamp(&mut self, val: f64) -> &mut Self {
        self.set_timestamp(val);
        self
    }
    #[cfg(feature = "RtcStatsType")]
    #[doc = "Change the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcMediaStreamTrackStats`, `RtcStatsType`*"]
    pub fn type_(&mut self, val: RtcStatsType) -> &mut Self {
        self.set_type(val);
        self
    }
    #[doc = "Change the `audioLevel` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcMediaStreamTrackStats`*"]
    pub fn audio_level(&mut self, val: f64) -> &mut Self {
        self.set_audio_level(val);
        self
    }
    #[doc = "Change the `echoReturnLoss` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcMediaStreamTrackStats`*"]
    pub fn echo_return_loss(&mut self, val: f64) -> &mut Self {
        self.set_echo_return_loss(val);
        self
    }
    #[doc = "Change the `echoReturnLossEnhancement` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcMediaStreamTrackStats`*"]
    pub fn echo_return_loss_enhancement(&mut self, val: f64) -> &mut Self {
        self.set_echo_return_loss_enhancement(val);
        self
    }
    #[doc = "Change the `frameHeight` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcMediaStreamTrackStats`*"]
    pub fn frame_height(&mut self, val: u32) -> &mut Self {
        self.set_frame_height(val);
        self
    }
    #[doc = "Change the `frameWidth` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcMediaStreamTrackStats`*"]
    pub fn frame_width(&mut self, val: u32) -> &mut Self {
        self.set_frame_width(val);
        self
    }
    #[doc = "Change the `framesCorrupted` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcMediaStreamTrackStats`*"]
    pub fn frames_corrupted(&mut self, val: u32) -> &mut Self {
        self.set_frames_corrupted(val);
        self
    }
    #[doc = "Change the `framesDecoded` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcMediaStreamTrackStats`*"]
    pub fn frames_decoded(&mut self, val: u32) -> &mut Self {
        self.set_frames_decoded(val);
        self
    }
    #[doc = "Change the `framesDropped` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcMediaStreamTrackStats`*"]
    pub fn frames_dropped(&mut self, val: u32) -> &mut Self {
        self.set_frames_dropped(val);
        self
    }
    #[doc = "Change the `framesPerSecond` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcMediaStreamTrackStats`*"]
    pub fn frames_per_second(&mut self, val: f64) -> &mut Self {
        self.set_frames_per_second(val);
        self
    }
    #[doc = "Change the `framesReceived` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcMediaStreamTrackStats`*"]
    pub fn frames_received(&mut self, val: u32) -> &mut Self {
        self.set_frames_received(val);
        self
    }
    #[doc = "Change the `framesSent` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcMediaStreamTrackStats`*"]
    pub fn frames_sent(&mut self, val: u32) -> &mut Self {
        self.set_frames_sent(val);
        self
    }
    #[doc = "Change the `remoteSource` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcMediaStreamTrackStats`*"]
    pub fn remote_source(&mut self, val: bool) -> &mut Self {
        self.set_remote_source(val);
        self
    }
    #[doc = "Change the `ssrcIds` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcMediaStreamTrackStats`*"]
    pub fn ssrc_ids(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_ssrc_ids(val);
        self
    }
    #[doc = "Change the `trackIdentifier` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcMediaStreamTrackStats`*"]
    pub fn track_identifier(&mut self, val: &str) -> &mut Self {
        self.set_track_identifier(val);
        self
    }
}
impl Default for RtcMediaStreamTrackStats {
    fn default() -> Self {
        Self::new()
    }
}
