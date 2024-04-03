#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = MediaTrackSettings)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `MediaTrackSettings` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackSettings`*"]
    pub type MediaTrackSettings;
    #[wasm_bindgen(method, setter = "autoGainControl")]
    fn auto_gain_control_shim(this: &MediaTrackSettings, val: bool);
    #[wasm_bindgen(method, setter = "channelCount")]
    fn channel_count_shim(this: &MediaTrackSettings, val: i32);
    #[wasm_bindgen(method, setter = "deviceId")]
    fn device_id_shim(this: &MediaTrackSettings, val: &str);
    #[wasm_bindgen(method, setter = "echoCancellation")]
    fn echo_cancellation_shim(this: &MediaTrackSettings, val: bool);
    #[wasm_bindgen(method, setter = "facingMode")]
    fn facing_mode_shim(this: &MediaTrackSettings, val: &str);
    #[wasm_bindgen(method, setter = "frameRate")]
    fn frame_rate_shim(this: &MediaTrackSettings, val: f64);
    #[wasm_bindgen(method, setter = "height")]
    fn height_shim(this: &MediaTrackSettings, val: i32);
    #[wasm_bindgen(method, setter = "noiseSuppression")]
    fn noise_suppression_shim(this: &MediaTrackSettings, val: bool);
    #[wasm_bindgen(method, setter = "width")]
    fn width_shim(this: &MediaTrackSettings, val: i32);
}
impl MediaTrackSettings {
    #[doc = "Construct a new `MediaTrackSettings`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackSettings`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `autoGainControl` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackSettings`*"]
    pub fn auto_gain_control(&mut self, val: bool) -> &mut Self {
        self.auto_gain_control_shim(val);
        self
    }
    #[doc = "Change the `channelCount` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackSettings`*"]
    pub fn channel_count(&mut self, val: i32) -> &mut Self {
        self.channel_count_shim(val);
        self
    }
    #[doc = "Change the `deviceId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackSettings`*"]
    pub fn device_id(&mut self, val: &str) -> &mut Self {
        self.device_id_shim(val);
        self
    }
    #[doc = "Change the `echoCancellation` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackSettings`*"]
    pub fn echo_cancellation(&mut self, val: bool) -> &mut Self {
        self.echo_cancellation_shim(val);
        self
    }
    #[doc = "Change the `facingMode` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackSettings`*"]
    pub fn facing_mode(&mut self, val: &str) -> &mut Self {
        self.facing_mode_shim(val);
        self
    }
    #[doc = "Change the `frameRate` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackSettings`*"]
    pub fn frame_rate(&mut self, val: f64) -> &mut Self {
        self.frame_rate_shim(val);
        self
    }
    #[doc = "Change the `height` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackSettings`*"]
    pub fn height(&mut self, val: i32) -> &mut Self {
        self.height_shim(val);
        self
    }
    #[doc = "Change the `noiseSuppression` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackSettings`*"]
    pub fn noise_suppression(&mut self, val: bool) -> &mut Self {
        self.noise_suppression_shim(val);
        self
    }
    #[doc = "Change the `width` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackSettings`*"]
    pub fn width(&mut self, val: i32) -> &mut Self {
        self.width_shim(val);
        self
    }
}
impl Default for MediaTrackSettings {
    fn default() -> Self {
        Self::new()
    }
}
