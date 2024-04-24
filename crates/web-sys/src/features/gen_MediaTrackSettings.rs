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
    #[wasm_bindgen(method, getter = "autoGainControl")]
    fn auto_gain_control_shim(this: &MediaTrackSettings) -> bool;
    #[wasm_bindgen(method, setter = "autoGainControl")]
    fn set_auto_gain_control_shim(this: &MediaTrackSettings, val: bool);
    #[wasm_bindgen(method, getter = "channelCount")]
    fn channel_count_shim(this: &MediaTrackSettings) -> i32;
    #[wasm_bindgen(method, setter = "channelCount")]
    fn set_channel_count_shim(this: &MediaTrackSettings, val: i32);
    #[wasm_bindgen(method, getter = "deviceId")]
    fn device_id_shim(this: &MediaTrackSettings) -> String;
    #[wasm_bindgen(method, setter = "deviceId")]
    fn set_device_id_shim(this: &MediaTrackSettings, val: &str);
    #[wasm_bindgen(method, getter = "echoCancellation")]
    fn echo_cancellation_shim(this: &MediaTrackSettings) -> bool;
    #[wasm_bindgen(method, setter = "echoCancellation")]
    fn set_echo_cancellation_shim(this: &MediaTrackSettings, val: bool);
    #[wasm_bindgen(method, getter = "facingMode")]
    fn facing_mode_shim(this: &MediaTrackSettings) -> String;
    #[wasm_bindgen(method, setter = "facingMode")]
    fn set_facing_mode_shim(this: &MediaTrackSettings, val: &str);
    #[wasm_bindgen(method, getter = "frameRate")]
    fn frame_rate_shim(this: &MediaTrackSettings) -> f64;
    #[wasm_bindgen(method, setter = "frameRate")]
    fn set_frame_rate_shim(this: &MediaTrackSettings, val: f64);
    #[wasm_bindgen(method, getter = "height")]
    fn height_shim(this: &MediaTrackSettings) -> i32;
    #[wasm_bindgen(method, setter = "height")]
    fn set_height_shim(this: &MediaTrackSettings, val: i32);
    #[wasm_bindgen(method, getter = "noiseSuppression")]
    fn noise_suppression_shim(this: &MediaTrackSettings) -> bool;
    #[wasm_bindgen(method, setter = "noiseSuppression")]
    fn set_noise_suppression_shim(this: &MediaTrackSettings, val: bool);
    #[wasm_bindgen(method, getter = "width")]
    fn width_shim(this: &MediaTrackSettings) -> i32;
    #[wasm_bindgen(method, setter = "width")]
    fn set_width_shim(this: &MediaTrackSettings, val: i32);
}
#[doc = "The trait to access properties on the `MediaTrackSettings` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `MediaTrackSettings`*"]
pub trait MediaTrackSettingsGetters {
    #[doc = "Get the `autoGainControl` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackSettings`*"]
    fn auto_gain_control(&self) -> bool;
    #[doc = "Get the `channelCount` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackSettings`*"]
    fn channel_count(&self) -> i32;
    #[doc = "Get the `deviceId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackSettings`*"]
    fn device_id(&self) -> String;
    #[doc = "Get the `echoCancellation` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackSettings`*"]
    fn echo_cancellation(&self) -> bool;
    #[doc = "Get the `facingMode` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackSettings`*"]
    fn facing_mode(&self) -> String;
    #[doc = "Get the `frameRate` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackSettings`*"]
    fn frame_rate(&self) -> f64;
    #[doc = "Get the `height` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackSettings`*"]
    fn height(&self) -> i32;
    #[doc = "Get the `noiseSuppression` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackSettings`*"]
    fn noise_suppression(&self) -> bool;
    #[doc = "Get the `width` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackSettings`*"]
    fn width(&self) -> i32;
}
impl MediaTrackSettingsGetters for MediaTrackSettings {
    fn auto_gain_control(&self) -> bool {
        self.auto_gain_control_shim()
    }
    fn channel_count(&self) -> i32 {
        self.channel_count_shim()
    }
    fn device_id(&self) -> String {
        self.device_id_shim()
    }
    fn echo_cancellation(&self) -> bool {
        self.echo_cancellation_shim()
    }
    fn facing_mode(&self) -> String {
        self.facing_mode_shim()
    }
    fn frame_rate(&self) -> f64 {
        self.frame_rate_shim()
    }
    fn height(&self) -> i32 {
        self.height_shim()
    }
    fn noise_suppression(&self) -> bool {
        self.noise_suppression_shim()
    }
    fn width(&self) -> i32 {
        self.width_shim()
    }
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
        self.set_auto_gain_control_shim(val);
        self
    }
    #[doc = "Change the `channelCount` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackSettings`*"]
    pub fn channel_count(&mut self, val: i32) -> &mut Self {
        self.set_channel_count_shim(val);
        self
    }
    #[doc = "Change the `deviceId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackSettings`*"]
    pub fn device_id(&mut self, val: &str) -> &mut Self {
        self.set_device_id_shim(val);
        self
    }
    #[doc = "Change the `echoCancellation` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackSettings`*"]
    pub fn echo_cancellation(&mut self, val: bool) -> &mut Self {
        self.set_echo_cancellation_shim(val);
        self
    }
    #[doc = "Change the `facingMode` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackSettings`*"]
    pub fn facing_mode(&mut self, val: &str) -> &mut Self {
        self.set_facing_mode_shim(val);
        self
    }
    #[doc = "Change the `frameRate` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackSettings`*"]
    pub fn frame_rate(&mut self, val: f64) -> &mut Self {
        self.set_frame_rate_shim(val);
        self
    }
    #[doc = "Change the `height` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackSettings`*"]
    pub fn height(&mut self, val: i32) -> &mut Self {
        self.set_height_shim(val);
        self
    }
    #[doc = "Change the `noiseSuppression` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackSettings`*"]
    pub fn noise_suppression(&mut self, val: bool) -> &mut Self {
        self.set_noise_suppression_shim(val);
        self
    }
    #[doc = "Change the `width` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackSettings`*"]
    pub fn width(&mut self, val: i32) -> &mut Self {
        self.set_width_shim(val);
        self
    }
}
impl Default for MediaTrackSettings {
    fn default() -> Self {
        Self::new()
    }
}
