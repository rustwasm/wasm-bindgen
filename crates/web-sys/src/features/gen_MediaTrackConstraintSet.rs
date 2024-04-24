#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = MediaTrackConstraintSet)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `MediaTrackConstraintSet` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackConstraintSet`*"]
    pub type MediaTrackConstraintSet;
    #[wasm_bindgen(method, getter = "autoGainControl")]
    fn auto_gain_control_shim(this: &MediaTrackConstraintSet) -> ::wasm_bindgen::JsValue;
    #[wasm_bindgen(method, setter = "autoGainControl")]
    fn set_auto_gain_control_shim(this: &MediaTrackConstraintSet, val: &::wasm_bindgen::JsValue);
    #[wasm_bindgen(method, getter = "browserWindow")]
    fn browser_window_shim(this: &MediaTrackConstraintSet) -> f64;
    #[wasm_bindgen(method, setter = "browserWindow")]
    fn set_browser_window_shim(this: &MediaTrackConstraintSet, val: f64);
    #[wasm_bindgen(method, getter = "channelCount")]
    fn channel_count_shim(this: &MediaTrackConstraintSet) -> ::wasm_bindgen::JsValue;
    #[wasm_bindgen(method, setter = "channelCount")]
    fn set_channel_count_shim(this: &MediaTrackConstraintSet, val: &::wasm_bindgen::JsValue);
    #[wasm_bindgen(method, getter = "deviceId")]
    fn device_id_shim(this: &MediaTrackConstraintSet) -> ::wasm_bindgen::JsValue;
    #[wasm_bindgen(method, setter = "deviceId")]
    fn set_device_id_shim(this: &MediaTrackConstraintSet, val: &::wasm_bindgen::JsValue);
    #[wasm_bindgen(method, getter = "echoCancellation")]
    fn echo_cancellation_shim(this: &MediaTrackConstraintSet) -> ::wasm_bindgen::JsValue;
    #[wasm_bindgen(method, setter = "echoCancellation")]
    fn set_echo_cancellation_shim(this: &MediaTrackConstraintSet, val: &::wasm_bindgen::JsValue);
    #[wasm_bindgen(method, getter = "facingMode")]
    fn facing_mode_shim(this: &MediaTrackConstraintSet) -> ::wasm_bindgen::JsValue;
    #[wasm_bindgen(method, setter = "facingMode")]
    fn set_facing_mode_shim(this: &MediaTrackConstraintSet, val: &::wasm_bindgen::JsValue);
    #[wasm_bindgen(method, getter = "frameRate")]
    fn frame_rate_shim(this: &MediaTrackConstraintSet) -> ::wasm_bindgen::JsValue;
    #[wasm_bindgen(method, setter = "frameRate")]
    fn set_frame_rate_shim(this: &MediaTrackConstraintSet, val: &::wasm_bindgen::JsValue);
    #[wasm_bindgen(method, getter = "height")]
    fn height_shim(this: &MediaTrackConstraintSet) -> ::wasm_bindgen::JsValue;
    #[wasm_bindgen(method, setter = "height")]
    fn set_height_shim(this: &MediaTrackConstraintSet, val: &::wasm_bindgen::JsValue);
    #[wasm_bindgen(method, getter = "mediaSource")]
    fn media_source_shim(this: &MediaTrackConstraintSet) -> String;
    #[wasm_bindgen(method, setter = "mediaSource")]
    fn set_media_source_shim(this: &MediaTrackConstraintSet, val: &str);
    #[wasm_bindgen(method, getter = "noiseSuppression")]
    fn noise_suppression_shim(this: &MediaTrackConstraintSet) -> ::wasm_bindgen::JsValue;
    #[wasm_bindgen(method, setter = "noiseSuppression")]
    fn set_noise_suppression_shim(this: &MediaTrackConstraintSet, val: &::wasm_bindgen::JsValue);
    #[wasm_bindgen(method, getter = "scrollWithPage")]
    fn scroll_with_page_shim(this: &MediaTrackConstraintSet) -> bool;
    #[wasm_bindgen(method, setter = "scrollWithPage")]
    fn set_scroll_with_page_shim(this: &MediaTrackConstraintSet, val: bool);
    #[wasm_bindgen(method, getter = "viewportHeight")]
    fn viewport_height_shim(this: &MediaTrackConstraintSet) -> ::wasm_bindgen::JsValue;
    #[wasm_bindgen(method, setter = "viewportHeight")]
    fn set_viewport_height_shim(this: &MediaTrackConstraintSet, val: &::wasm_bindgen::JsValue);
    #[wasm_bindgen(method, getter = "viewportOffsetX")]
    fn viewport_offset_x_shim(this: &MediaTrackConstraintSet) -> ::wasm_bindgen::JsValue;
    #[wasm_bindgen(method, setter = "viewportOffsetX")]
    fn set_viewport_offset_x_shim(this: &MediaTrackConstraintSet, val: &::wasm_bindgen::JsValue);
    #[wasm_bindgen(method, getter = "viewportOffsetY")]
    fn viewport_offset_y_shim(this: &MediaTrackConstraintSet) -> ::wasm_bindgen::JsValue;
    #[wasm_bindgen(method, setter = "viewportOffsetY")]
    fn set_viewport_offset_y_shim(this: &MediaTrackConstraintSet, val: &::wasm_bindgen::JsValue);
    #[wasm_bindgen(method, getter = "viewportWidth")]
    fn viewport_width_shim(this: &MediaTrackConstraintSet) -> ::wasm_bindgen::JsValue;
    #[wasm_bindgen(method, setter = "viewportWidth")]
    fn set_viewport_width_shim(this: &MediaTrackConstraintSet, val: &::wasm_bindgen::JsValue);
    #[wasm_bindgen(method, getter = "width")]
    fn width_shim(this: &MediaTrackConstraintSet) -> ::wasm_bindgen::JsValue;
    #[wasm_bindgen(method, setter = "width")]
    fn set_width_shim(this: &MediaTrackConstraintSet, val: &::wasm_bindgen::JsValue);
}
#[doc = "The trait to access properties on the `MediaTrackConstraintSet` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `MediaTrackConstraintSet`*"]
pub trait MediaTrackConstraintSetGetters {
    #[doc = "Get the `autoGainControl` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackConstraintSet`*"]
    fn auto_gain_control(&self) -> ::wasm_bindgen::JsValue;
    #[doc = "Get the `browserWindow` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackConstraintSet`*"]
    fn browser_window(&self) -> f64;
    #[doc = "Get the `channelCount` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackConstraintSet`*"]
    fn channel_count(&self) -> ::wasm_bindgen::JsValue;
    #[doc = "Get the `deviceId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackConstraintSet`*"]
    fn device_id(&self) -> ::wasm_bindgen::JsValue;
    #[doc = "Get the `echoCancellation` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackConstraintSet`*"]
    fn echo_cancellation(&self) -> ::wasm_bindgen::JsValue;
    #[doc = "Get the `facingMode` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackConstraintSet`*"]
    fn facing_mode(&self) -> ::wasm_bindgen::JsValue;
    #[doc = "Get the `frameRate` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackConstraintSet`*"]
    fn frame_rate(&self) -> ::wasm_bindgen::JsValue;
    #[doc = "Get the `height` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackConstraintSet`*"]
    fn height(&self) -> ::wasm_bindgen::JsValue;
    #[doc = "Get the `mediaSource` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackConstraintSet`*"]
    fn media_source(&self) -> String;
    #[doc = "Get the `noiseSuppression` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackConstraintSet`*"]
    fn noise_suppression(&self) -> ::wasm_bindgen::JsValue;
    #[doc = "Get the `scrollWithPage` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackConstraintSet`*"]
    fn scroll_with_page(&self) -> bool;
    #[doc = "Get the `viewportHeight` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackConstraintSet`*"]
    fn viewport_height(&self) -> ::wasm_bindgen::JsValue;
    #[doc = "Get the `viewportOffsetX` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackConstraintSet`*"]
    fn viewport_offset_x(&self) -> ::wasm_bindgen::JsValue;
    #[doc = "Get the `viewportOffsetY` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackConstraintSet`*"]
    fn viewport_offset_y(&self) -> ::wasm_bindgen::JsValue;
    #[doc = "Get the `viewportWidth` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackConstraintSet`*"]
    fn viewport_width(&self) -> ::wasm_bindgen::JsValue;
    #[doc = "Get the `width` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackConstraintSet`*"]
    fn width(&self) -> ::wasm_bindgen::JsValue;
}
impl MediaTrackConstraintSetGetters for MediaTrackConstraintSet {
    fn auto_gain_control(&self) -> ::wasm_bindgen::JsValue {
        self.auto_gain_control_shim()
    }
    fn browser_window(&self) -> f64 {
        self.browser_window_shim()
    }
    fn channel_count(&self) -> ::wasm_bindgen::JsValue {
        self.channel_count_shim()
    }
    fn device_id(&self) -> ::wasm_bindgen::JsValue {
        self.device_id_shim()
    }
    fn echo_cancellation(&self) -> ::wasm_bindgen::JsValue {
        self.echo_cancellation_shim()
    }
    fn facing_mode(&self) -> ::wasm_bindgen::JsValue {
        self.facing_mode_shim()
    }
    fn frame_rate(&self) -> ::wasm_bindgen::JsValue {
        self.frame_rate_shim()
    }
    fn height(&self) -> ::wasm_bindgen::JsValue {
        self.height_shim()
    }
    fn media_source(&self) -> String {
        self.media_source_shim()
    }
    fn noise_suppression(&self) -> ::wasm_bindgen::JsValue {
        self.noise_suppression_shim()
    }
    fn scroll_with_page(&self) -> bool {
        self.scroll_with_page_shim()
    }
    fn viewport_height(&self) -> ::wasm_bindgen::JsValue {
        self.viewport_height_shim()
    }
    fn viewport_offset_x(&self) -> ::wasm_bindgen::JsValue {
        self.viewport_offset_x_shim()
    }
    fn viewport_offset_y(&self) -> ::wasm_bindgen::JsValue {
        self.viewport_offset_y_shim()
    }
    fn viewport_width(&self) -> ::wasm_bindgen::JsValue {
        self.viewport_width_shim()
    }
    fn width(&self) -> ::wasm_bindgen::JsValue {
        self.width_shim()
    }
}
impl MediaTrackConstraintSet {
    #[doc = "Construct a new `MediaTrackConstraintSet`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackConstraintSet`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `autoGainControl` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackConstraintSet`*"]
    pub fn auto_gain_control(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_auto_gain_control_shim(val);
        self
    }
    #[doc = "Change the `browserWindow` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackConstraintSet`*"]
    pub fn browser_window(&mut self, val: f64) -> &mut Self {
        self.set_browser_window_shim(val);
        self
    }
    #[doc = "Change the `channelCount` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackConstraintSet`*"]
    pub fn channel_count(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_channel_count_shim(val);
        self
    }
    #[doc = "Change the `deviceId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackConstraintSet`*"]
    pub fn device_id(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_device_id_shim(val);
        self
    }
    #[doc = "Change the `echoCancellation` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackConstraintSet`*"]
    pub fn echo_cancellation(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_echo_cancellation_shim(val);
        self
    }
    #[doc = "Change the `facingMode` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackConstraintSet`*"]
    pub fn facing_mode(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_facing_mode_shim(val);
        self
    }
    #[doc = "Change the `frameRate` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackConstraintSet`*"]
    pub fn frame_rate(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_frame_rate_shim(val);
        self
    }
    #[doc = "Change the `height` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackConstraintSet`*"]
    pub fn height(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_height_shim(val);
        self
    }
    #[doc = "Change the `mediaSource` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackConstraintSet`*"]
    pub fn media_source(&mut self, val: &str) -> &mut Self {
        self.set_media_source_shim(val);
        self
    }
    #[doc = "Change the `noiseSuppression` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackConstraintSet`*"]
    pub fn noise_suppression(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_noise_suppression_shim(val);
        self
    }
    #[doc = "Change the `scrollWithPage` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackConstraintSet`*"]
    pub fn scroll_with_page(&mut self, val: bool) -> &mut Self {
        self.set_scroll_with_page_shim(val);
        self
    }
    #[doc = "Change the `viewportHeight` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackConstraintSet`*"]
    pub fn viewport_height(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_viewport_height_shim(val);
        self
    }
    #[doc = "Change the `viewportOffsetX` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackConstraintSet`*"]
    pub fn viewport_offset_x(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_viewport_offset_x_shim(val);
        self
    }
    #[doc = "Change the `viewportOffsetY` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackConstraintSet`*"]
    pub fn viewport_offset_y(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_viewport_offset_y_shim(val);
        self
    }
    #[doc = "Change the `viewportWidth` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackConstraintSet`*"]
    pub fn viewport_width(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_viewport_width_shim(val);
        self
    }
    #[doc = "Change the `width` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackConstraintSet`*"]
    pub fn width(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_width_shim(val);
        self
    }
}
impl Default for MediaTrackConstraintSet {
    fn default() -> Self {
        Self::new()
    }
}
