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
    #[doc = "Get the `autoGainControl` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackConstraintSet`*"]
    #[wasm_bindgen(method, getter = "autoGainControl")]
    pub fn get_auto_gain_control(this: &MediaTrackConstraintSet) -> ::wasm_bindgen::JsValue;
    #[wasm_bindgen(method, setter = "autoGainControl")]
    fn set_auto_gain_control(this: &MediaTrackConstraintSet, val: &::wasm_bindgen::JsValue);
    #[doc = "Get the `browserWindow` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackConstraintSet`*"]
    #[wasm_bindgen(method, getter = "browserWindow")]
    pub fn get_browser_window(this: &MediaTrackConstraintSet) -> Option<f64>;
    #[wasm_bindgen(method, setter = "browserWindow")]
    fn set_browser_window(this: &MediaTrackConstraintSet, val: f64);
    #[doc = "Get the `channelCount` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackConstraintSet`*"]
    #[wasm_bindgen(method, getter = "channelCount")]
    pub fn get_channel_count(this: &MediaTrackConstraintSet) -> ::wasm_bindgen::JsValue;
    #[wasm_bindgen(method, setter = "channelCount")]
    fn set_channel_count(this: &MediaTrackConstraintSet, val: &::wasm_bindgen::JsValue);
    #[doc = "Get the `deviceId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackConstraintSet`*"]
    #[wasm_bindgen(method, getter = "deviceId")]
    pub fn get_device_id(this: &MediaTrackConstraintSet) -> ::wasm_bindgen::JsValue;
    #[wasm_bindgen(method, setter = "deviceId")]
    fn set_device_id(this: &MediaTrackConstraintSet, val: &::wasm_bindgen::JsValue);
    #[doc = "Get the `echoCancellation` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackConstraintSet`*"]
    #[wasm_bindgen(method, getter = "echoCancellation")]
    pub fn get_echo_cancellation(this: &MediaTrackConstraintSet) -> ::wasm_bindgen::JsValue;
    #[wasm_bindgen(method, setter = "echoCancellation")]
    fn set_echo_cancellation(this: &MediaTrackConstraintSet, val: &::wasm_bindgen::JsValue);
    #[doc = "Get the `facingMode` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackConstraintSet`*"]
    #[wasm_bindgen(method, getter = "facingMode")]
    pub fn get_facing_mode(this: &MediaTrackConstraintSet) -> ::wasm_bindgen::JsValue;
    #[wasm_bindgen(method, setter = "facingMode")]
    fn set_facing_mode(this: &MediaTrackConstraintSet, val: &::wasm_bindgen::JsValue);
    #[doc = "Get the `frameRate` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackConstraintSet`*"]
    #[wasm_bindgen(method, getter = "frameRate")]
    pub fn get_frame_rate(this: &MediaTrackConstraintSet) -> ::wasm_bindgen::JsValue;
    #[wasm_bindgen(method, setter = "frameRate")]
    fn set_frame_rate(this: &MediaTrackConstraintSet, val: &::wasm_bindgen::JsValue);
    #[doc = "Get the `height` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackConstraintSet`*"]
    #[wasm_bindgen(method, getter = "height")]
    pub fn get_height(this: &MediaTrackConstraintSet) -> ::wasm_bindgen::JsValue;
    #[wasm_bindgen(method, setter = "height")]
    fn set_height(this: &MediaTrackConstraintSet, val: &::wasm_bindgen::JsValue);
    #[doc = "Get the `mediaSource` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackConstraintSet`*"]
    #[wasm_bindgen(method, getter = "mediaSource")]
    pub fn get_media_source(this: &MediaTrackConstraintSet) -> Option<String>;
    #[wasm_bindgen(method, setter = "mediaSource")]
    fn set_media_source(this: &MediaTrackConstraintSet, val: &str);
    #[doc = "Get the `noiseSuppression` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackConstraintSet`*"]
    #[wasm_bindgen(method, getter = "noiseSuppression")]
    pub fn get_noise_suppression(this: &MediaTrackConstraintSet) -> ::wasm_bindgen::JsValue;
    #[wasm_bindgen(method, setter = "noiseSuppression")]
    fn set_noise_suppression(this: &MediaTrackConstraintSet, val: &::wasm_bindgen::JsValue);
    #[doc = "Get the `scrollWithPage` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackConstraintSet`*"]
    #[wasm_bindgen(method, getter = "scrollWithPage")]
    pub fn get_scroll_with_page(this: &MediaTrackConstraintSet) -> Option<bool>;
    #[wasm_bindgen(method, setter = "scrollWithPage")]
    fn set_scroll_with_page(this: &MediaTrackConstraintSet, val: bool);
    #[doc = "Get the `viewportHeight` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackConstraintSet`*"]
    #[wasm_bindgen(method, getter = "viewportHeight")]
    pub fn get_viewport_height(this: &MediaTrackConstraintSet) -> ::wasm_bindgen::JsValue;
    #[wasm_bindgen(method, setter = "viewportHeight")]
    fn set_viewport_height(this: &MediaTrackConstraintSet, val: &::wasm_bindgen::JsValue);
    #[doc = "Get the `viewportOffsetX` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackConstraintSet`*"]
    #[wasm_bindgen(method, getter = "viewportOffsetX")]
    pub fn get_viewport_offset_x(this: &MediaTrackConstraintSet) -> ::wasm_bindgen::JsValue;
    #[wasm_bindgen(method, setter = "viewportOffsetX")]
    fn set_viewport_offset_x(this: &MediaTrackConstraintSet, val: &::wasm_bindgen::JsValue);
    #[doc = "Get the `viewportOffsetY` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackConstraintSet`*"]
    #[wasm_bindgen(method, getter = "viewportOffsetY")]
    pub fn get_viewport_offset_y(this: &MediaTrackConstraintSet) -> ::wasm_bindgen::JsValue;
    #[wasm_bindgen(method, setter = "viewportOffsetY")]
    fn set_viewport_offset_y(this: &MediaTrackConstraintSet, val: &::wasm_bindgen::JsValue);
    #[doc = "Get the `viewportWidth` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackConstraintSet`*"]
    #[wasm_bindgen(method, getter = "viewportWidth")]
    pub fn get_viewport_width(this: &MediaTrackConstraintSet) -> ::wasm_bindgen::JsValue;
    #[wasm_bindgen(method, setter = "viewportWidth")]
    fn set_viewport_width(this: &MediaTrackConstraintSet, val: &::wasm_bindgen::JsValue);
    #[doc = "Get the `width` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackConstraintSet`*"]
    #[wasm_bindgen(method, getter = "width")]
    pub fn get_width(this: &MediaTrackConstraintSet) -> ::wasm_bindgen::JsValue;
    #[wasm_bindgen(method, setter = "width")]
    fn set_width(this: &MediaTrackConstraintSet, val: &::wasm_bindgen::JsValue);
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
        self.set_auto_gain_control(val);
        self
    }
    #[doc = "Change the `browserWindow` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackConstraintSet`*"]
    pub fn browser_window(&mut self, val: f64) -> &mut Self {
        self.set_browser_window(val);
        self
    }
    #[doc = "Change the `channelCount` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackConstraintSet`*"]
    pub fn channel_count(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_channel_count(val);
        self
    }
    #[doc = "Change the `deviceId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackConstraintSet`*"]
    pub fn device_id(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_device_id(val);
        self
    }
    #[doc = "Change the `echoCancellation` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackConstraintSet`*"]
    pub fn echo_cancellation(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_echo_cancellation(val);
        self
    }
    #[doc = "Change the `facingMode` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackConstraintSet`*"]
    pub fn facing_mode(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_facing_mode(val);
        self
    }
    #[doc = "Change the `frameRate` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackConstraintSet`*"]
    pub fn frame_rate(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_frame_rate(val);
        self
    }
    #[doc = "Change the `height` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackConstraintSet`*"]
    pub fn height(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_height(val);
        self
    }
    #[doc = "Change the `mediaSource` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackConstraintSet`*"]
    pub fn media_source(&mut self, val: &str) -> &mut Self {
        self.set_media_source(val);
        self
    }
    #[doc = "Change the `noiseSuppression` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackConstraintSet`*"]
    pub fn noise_suppression(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_noise_suppression(val);
        self
    }
    #[doc = "Change the `scrollWithPage` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackConstraintSet`*"]
    pub fn scroll_with_page(&mut self, val: bool) -> &mut Self {
        self.set_scroll_with_page(val);
        self
    }
    #[doc = "Change the `viewportHeight` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackConstraintSet`*"]
    pub fn viewport_height(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_viewport_height(val);
        self
    }
    #[doc = "Change the `viewportOffsetX` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackConstraintSet`*"]
    pub fn viewport_offset_x(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_viewport_offset_x(val);
        self
    }
    #[doc = "Change the `viewportOffsetY` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackConstraintSet`*"]
    pub fn viewport_offset_y(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_viewport_offset_y(val);
        self
    }
    #[doc = "Change the `viewportWidth` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackConstraintSet`*"]
    pub fn viewport_width(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_viewport_width(val);
        self
    }
    #[doc = "Change the `width` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackConstraintSet`*"]
    pub fn width(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_width(val);
        self
    }
}
impl Default for MediaTrackConstraintSet {
    fn default() -> Self {
        Self::new()
    }
}
