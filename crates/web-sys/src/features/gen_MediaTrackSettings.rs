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
    #[doc = "Get the `autoGainControl` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackSettings`*"]
    #[wasm_bindgen(method, getter = "autoGainControl")]
    pub fn get_auto_gain_control(this: &MediaTrackSettings) -> Option<bool>;
    #[wasm_bindgen(method, setter = "autoGainControl")]
    fn set_auto_gain_control(this: &MediaTrackSettings, val: bool);
    #[doc = "Get the `channelCount` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackSettings`*"]
    #[wasm_bindgen(method, getter = "channelCount")]
    pub fn get_channel_count(this: &MediaTrackSettings) -> Option<i32>;
    #[wasm_bindgen(method, setter = "channelCount")]
    fn set_channel_count(this: &MediaTrackSettings, val: i32);
    #[doc = "Get the `deviceId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackSettings`*"]
    #[wasm_bindgen(method, getter = "deviceId")]
    pub fn get_device_id(this: &MediaTrackSettings) -> Option<String>;
    #[wasm_bindgen(method, setter = "deviceId")]
    fn set_device_id(this: &MediaTrackSettings, val: &str);
    #[doc = "Get the `echoCancellation` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackSettings`*"]
    #[wasm_bindgen(method, getter = "echoCancellation")]
    pub fn get_echo_cancellation(this: &MediaTrackSettings) -> Option<bool>;
    #[wasm_bindgen(method, setter = "echoCancellation")]
    fn set_echo_cancellation(this: &MediaTrackSettings, val: bool);
    #[doc = "Get the `facingMode` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackSettings`*"]
    #[wasm_bindgen(method, getter = "facingMode")]
    pub fn get_facing_mode(this: &MediaTrackSettings) -> Option<String>;
    #[wasm_bindgen(method, setter = "facingMode")]
    fn set_facing_mode(this: &MediaTrackSettings, val: &str);
    #[doc = "Get the `frameRate` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackSettings`*"]
    #[wasm_bindgen(method, getter = "frameRate")]
    pub fn get_frame_rate(this: &MediaTrackSettings) -> Option<f64>;
    #[wasm_bindgen(method, setter = "frameRate")]
    fn set_frame_rate(this: &MediaTrackSettings, val: f64);
    #[doc = "Get the `height` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackSettings`*"]
    #[wasm_bindgen(method, getter = "height")]
    pub fn get_height(this: &MediaTrackSettings) -> Option<i32>;
    #[wasm_bindgen(method, setter = "height")]
    fn set_height(this: &MediaTrackSettings, val: i32);
    #[doc = "Get the `noiseSuppression` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackSettings`*"]
    #[wasm_bindgen(method, getter = "noiseSuppression")]
    pub fn get_noise_suppression(this: &MediaTrackSettings) -> Option<bool>;
    #[wasm_bindgen(method, setter = "noiseSuppression")]
    fn set_noise_suppression(this: &MediaTrackSettings, val: bool);
    #[doc = "Get the `width` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackSettings`*"]
    #[wasm_bindgen(method, getter = "width")]
    pub fn get_width(this: &MediaTrackSettings) -> Option<i32>;
    #[wasm_bindgen(method, setter = "width")]
    fn set_width(this: &MediaTrackSettings, val: i32);
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
        self.set_auto_gain_control(val);
        self
    }
    #[doc = "Change the `channelCount` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackSettings`*"]
    pub fn channel_count(&mut self, val: i32) -> &mut Self {
        self.set_channel_count(val);
        self
    }
    #[doc = "Change the `deviceId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackSettings`*"]
    pub fn device_id(&mut self, val: &str) -> &mut Self {
        self.set_device_id(val);
        self
    }
    #[doc = "Change the `echoCancellation` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackSettings`*"]
    pub fn echo_cancellation(&mut self, val: bool) -> &mut Self {
        self.set_echo_cancellation(val);
        self
    }
    #[doc = "Change the `facingMode` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackSettings`*"]
    pub fn facing_mode(&mut self, val: &str) -> &mut Self {
        self.set_facing_mode(val);
        self
    }
    #[doc = "Change the `frameRate` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackSettings`*"]
    pub fn frame_rate(&mut self, val: f64) -> &mut Self {
        self.set_frame_rate(val);
        self
    }
    #[doc = "Change the `height` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackSettings`*"]
    pub fn height(&mut self, val: i32) -> &mut Self {
        self.set_height(val);
        self
    }
    #[doc = "Change the `noiseSuppression` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackSettings`*"]
    pub fn noise_suppression(&mut self, val: bool) -> &mut Self {
        self.set_noise_suppression(val);
        self
    }
    #[doc = "Change the `width` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackSettings`*"]
    pub fn width(&mut self, val: i32) -> &mut Self {
        self.set_width(val);
        self
    }
}
impl Default for MediaTrackSettings {
    fn default() -> Self {
        Self::new()
    }
}
