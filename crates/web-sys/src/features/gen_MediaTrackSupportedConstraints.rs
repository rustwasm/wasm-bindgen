#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = MediaTrackSupportedConstraints)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `MediaTrackSupportedConstraints` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackSupportedConstraints`*"]
    pub type MediaTrackSupportedConstraints;
    #[wasm_bindgen(method, setter = "aspectRatio")]
    fn aspect_ratio_shim(this: &MediaTrackSupportedConstraints, val: bool);
    #[wasm_bindgen(method, setter = "autoGainControl")]
    fn auto_gain_control_shim(this: &MediaTrackSupportedConstraints, val: bool);
    #[wasm_bindgen(method, setter = "channelCount")]
    fn channel_count_shim(this: &MediaTrackSupportedConstraints, val: bool);
    #[wasm_bindgen(method, setter = "deviceId")]
    fn device_id_shim(this: &MediaTrackSupportedConstraints, val: bool);
    #[wasm_bindgen(method, setter = "echoCancellation")]
    fn echo_cancellation_shim(this: &MediaTrackSupportedConstraints, val: bool);
    #[wasm_bindgen(method, setter = "facingMode")]
    fn facing_mode_shim(this: &MediaTrackSupportedConstraints, val: bool);
    #[wasm_bindgen(method, setter = "frameRate")]
    fn frame_rate_shim(this: &MediaTrackSupportedConstraints, val: bool);
    #[wasm_bindgen(method, setter = "groupId")]
    fn group_id_shim(this: &MediaTrackSupportedConstraints, val: bool);
    #[wasm_bindgen(method, setter = "height")]
    fn height_shim(this: &MediaTrackSupportedConstraints, val: bool);
    #[wasm_bindgen(method, setter = "latency")]
    fn latency_shim(this: &MediaTrackSupportedConstraints, val: bool);
    #[wasm_bindgen(method, setter = "noiseSuppression")]
    fn noise_suppression_shim(this: &MediaTrackSupportedConstraints, val: bool);
    #[wasm_bindgen(method, setter = "sampleRate")]
    fn sample_rate_shim(this: &MediaTrackSupportedConstraints, val: bool);
    #[wasm_bindgen(method, setter = "sampleSize")]
    fn sample_size_shim(this: &MediaTrackSupportedConstraints, val: bool);
    #[wasm_bindgen(method, setter = "volume")]
    fn volume_shim(this: &MediaTrackSupportedConstraints, val: bool);
    #[wasm_bindgen(method, setter = "width")]
    fn width_shim(this: &MediaTrackSupportedConstraints, val: bool);
}
impl MediaTrackSupportedConstraints {
    #[doc = "Construct a new `MediaTrackSupportedConstraints`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackSupportedConstraints`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `aspectRatio` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackSupportedConstraints`*"]
    pub fn aspect_ratio(&mut self, val: bool) -> &mut Self {
        self.aspect_ratio_shim(val);
        self
    }
    #[doc = "Change the `autoGainControl` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackSupportedConstraints`*"]
    pub fn auto_gain_control(&mut self, val: bool) -> &mut Self {
        self.auto_gain_control_shim(val);
        self
    }
    #[doc = "Change the `channelCount` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackSupportedConstraints`*"]
    pub fn channel_count(&mut self, val: bool) -> &mut Self {
        self.channel_count_shim(val);
        self
    }
    #[doc = "Change the `deviceId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackSupportedConstraints`*"]
    pub fn device_id(&mut self, val: bool) -> &mut Self {
        self.device_id_shim(val);
        self
    }
    #[doc = "Change the `echoCancellation` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackSupportedConstraints`*"]
    pub fn echo_cancellation(&mut self, val: bool) -> &mut Self {
        self.echo_cancellation_shim(val);
        self
    }
    #[doc = "Change the `facingMode` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackSupportedConstraints`*"]
    pub fn facing_mode(&mut self, val: bool) -> &mut Self {
        self.facing_mode_shim(val);
        self
    }
    #[doc = "Change the `frameRate` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackSupportedConstraints`*"]
    pub fn frame_rate(&mut self, val: bool) -> &mut Self {
        self.frame_rate_shim(val);
        self
    }
    #[doc = "Change the `groupId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackSupportedConstraints`*"]
    pub fn group_id(&mut self, val: bool) -> &mut Self {
        self.group_id_shim(val);
        self
    }
    #[doc = "Change the `height` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackSupportedConstraints`*"]
    pub fn height(&mut self, val: bool) -> &mut Self {
        self.height_shim(val);
        self
    }
    #[doc = "Change the `latency` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackSupportedConstraints`*"]
    pub fn latency(&mut self, val: bool) -> &mut Self {
        self.latency_shim(val);
        self
    }
    #[doc = "Change the `noiseSuppression` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackSupportedConstraints`*"]
    pub fn noise_suppression(&mut self, val: bool) -> &mut Self {
        self.noise_suppression_shim(val);
        self
    }
    #[doc = "Change the `sampleRate` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackSupportedConstraints`*"]
    pub fn sample_rate(&mut self, val: bool) -> &mut Self {
        self.sample_rate_shim(val);
        self
    }
    #[doc = "Change the `sampleSize` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackSupportedConstraints`*"]
    pub fn sample_size(&mut self, val: bool) -> &mut Self {
        self.sample_size_shim(val);
        self
    }
    #[doc = "Change the `volume` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackSupportedConstraints`*"]
    pub fn volume(&mut self, val: bool) -> &mut Self {
        self.volume_shim(val);
        self
    }
    #[doc = "Change the `width` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackSupportedConstraints`*"]
    pub fn width(&mut self, val: bool) -> &mut Self {
        self.width_shim(val);
        self
    }
}
impl Default for MediaTrackSupportedConstraints {
    fn default() -> Self {
        Self::new()
    }
}
