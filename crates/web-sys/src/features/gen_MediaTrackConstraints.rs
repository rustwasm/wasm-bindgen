#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = MediaTrackConstraints)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `MediaTrackConstraints` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackConstraints`*"]
    pub type MediaTrackConstraints;
    #[wasm_bindgen(method, setter = "autoGainControl")]
    fn auto_gain_control_shim(this: &MediaTrackConstraints, val: &::wasm_bindgen::JsValue);
    #[wasm_bindgen(method, setter = "browserWindow")]
    fn browser_window_shim(this: &MediaTrackConstraints, val: f64);
    #[wasm_bindgen(method, setter = "channelCount")]
    fn channel_count_shim(this: &MediaTrackConstraints, val: &::wasm_bindgen::JsValue);
    #[wasm_bindgen(method, setter = "deviceId")]
    fn device_id_shim(this: &MediaTrackConstraints, val: &::wasm_bindgen::JsValue);
    #[wasm_bindgen(method, setter = "echoCancellation")]
    fn echo_cancellation_shim(this: &MediaTrackConstraints, val: &::wasm_bindgen::JsValue);
    #[wasm_bindgen(method, setter = "facingMode")]
    fn facing_mode_shim(this: &MediaTrackConstraints, val: &::wasm_bindgen::JsValue);
    #[wasm_bindgen(method, setter = "frameRate")]
    fn frame_rate_shim(this: &MediaTrackConstraints, val: &::wasm_bindgen::JsValue);
    #[wasm_bindgen(method, setter = "height")]
    fn height_shim(this: &MediaTrackConstraints, val: &::wasm_bindgen::JsValue);
    #[wasm_bindgen(method, setter = "mediaSource")]
    fn media_source_shim(this: &MediaTrackConstraints, val: &str);
    #[wasm_bindgen(method, setter = "noiseSuppression")]
    fn noise_suppression_shim(this: &MediaTrackConstraints, val: &::wasm_bindgen::JsValue);
    #[wasm_bindgen(method, setter = "scrollWithPage")]
    fn scroll_with_page_shim(this: &MediaTrackConstraints, val: bool);
    #[wasm_bindgen(method, setter = "viewportHeight")]
    fn viewport_height_shim(this: &MediaTrackConstraints, val: &::wasm_bindgen::JsValue);
    #[wasm_bindgen(method, setter = "viewportOffsetX")]
    fn viewport_offset_x_shim(this: &MediaTrackConstraints, val: &::wasm_bindgen::JsValue);
    #[wasm_bindgen(method, setter = "viewportOffsetY")]
    fn viewport_offset_y_shim(this: &MediaTrackConstraints, val: &::wasm_bindgen::JsValue);
    #[wasm_bindgen(method, setter = "viewportWidth")]
    fn viewport_width_shim(this: &MediaTrackConstraints, val: &::wasm_bindgen::JsValue);
    #[wasm_bindgen(method, setter = "width")]
    fn width_shim(this: &MediaTrackConstraints, val: &::wasm_bindgen::JsValue);
    #[wasm_bindgen(method, setter = "advanced")]
    fn advanced_shim(this: &MediaTrackConstraints, val: &::wasm_bindgen::JsValue);
}
impl MediaTrackConstraints {
    #[doc = "Construct a new `MediaTrackConstraints`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackConstraints`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `autoGainControl` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackConstraints`*"]
    pub fn auto_gain_control(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.auto_gain_control_shim(val);
        self
    }
    #[doc = "Change the `browserWindow` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackConstraints`*"]
    pub fn browser_window(&mut self, val: f64) -> &mut Self {
        self.browser_window_shim(val);
        self
    }
    #[doc = "Change the `channelCount` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackConstraints`*"]
    pub fn channel_count(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.channel_count_shim(val);
        self
    }
    #[doc = "Change the `deviceId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackConstraints`*"]
    pub fn device_id(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.device_id_shim(val);
        self
    }
    #[doc = "Change the `echoCancellation` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackConstraints`*"]
    pub fn echo_cancellation(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.echo_cancellation_shim(val);
        self
    }
    #[doc = "Change the `facingMode` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackConstraints`*"]
    pub fn facing_mode(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.facing_mode_shim(val);
        self
    }
    #[doc = "Change the `frameRate` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackConstraints`*"]
    pub fn frame_rate(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.frame_rate_shim(val);
        self
    }
    #[doc = "Change the `height` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackConstraints`*"]
    pub fn height(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.height_shim(val);
        self
    }
    #[doc = "Change the `mediaSource` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackConstraints`*"]
    pub fn media_source(&mut self, val: &str) -> &mut Self {
        self.media_source_shim(val);
        self
    }
    #[doc = "Change the `noiseSuppression` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackConstraints`*"]
    pub fn noise_suppression(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.noise_suppression_shim(val);
        self
    }
    #[doc = "Change the `scrollWithPage` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackConstraints`*"]
    pub fn scroll_with_page(&mut self, val: bool) -> &mut Self {
        self.scroll_with_page_shim(val);
        self
    }
    #[doc = "Change the `viewportHeight` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackConstraints`*"]
    pub fn viewport_height(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.viewport_height_shim(val);
        self
    }
    #[doc = "Change the `viewportOffsetX` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackConstraints`*"]
    pub fn viewport_offset_x(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.viewport_offset_x_shim(val);
        self
    }
    #[doc = "Change the `viewportOffsetY` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackConstraints`*"]
    pub fn viewport_offset_y(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.viewport_offset_y_shim(val);
        self
    }
    #[doc = "Change the `viewportWidth` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackConstraints`*"]
    pub fn viewport_width(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.viewport_width_shim(val);
        self
    }
    #[doc = "Change the `width` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackConstraints`*"]
    pub fn width(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.width_shim(val);
        self
    }
    #[doc = "Change the `advanced` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackConstraints`*"]
    pub fn advanced(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.advanced_shim(val);
        self
    }
}
impl Default for MediaTrackConstraints {
    fn default() -> Self {
        Self::new()
    }
}
