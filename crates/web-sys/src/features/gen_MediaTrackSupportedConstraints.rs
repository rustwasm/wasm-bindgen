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
    #[wasm_bindgen(method, getter = "aspectRatio")]
    fn aspect_ratio_shim(this: &MediaTrackSupportedConstraints) -> bool;
    #[wasm_bindgen(method, setter = "aspectRatio")]
    fn set_aspect_ratio_shim(this: &MediaTrackSupportedConstraints, val: bool);
    #[wasm_bindgen(method, getter = "autoGainControl")]
    fn auto_gain_control_shim(this: &MediaTrackSupportedConstraints) -> bool;
    #[wasm_bindgen(method, setter = "autoGainControl")]
    fn set_auto_gain_control_shim(this: &MediaTrackSupportedConstraints, val: bool);
    #[wasm_bindgen(method, getter = "channelCount")]
    fn channel_count_shim(this: &MediaTrackSupportedConstraints) -> bool;
    #[wasm_bindgen(method, setter = "channelCount")]
    fn set_channel_count_shim(this: &MediaTrackSupportedConstraints, val: bool);
    #[wasm_bindgen(method, getter = "deviceId")]
    fn device_id_shim(this: &MediaTrackSupportedConstraints) -> bool;
    #[wasm_bindgen(method, setter = "deviceId")]
    fn set_device_id_shim(this: &MediaTrackSupportedConstraints, val: bool);
    #[wasm_bindgen(method, getter = "echoCancellation")]
    fn echo_cancellation_shim(this: &MediaTrackSupportedConstraints) -> bool;
    #[wasm_bindgen(method, setter = "echoCancellation")]
    fn set_echo_cancellation_shim(this: &MediaTrackSupportedConstraints, val: bool);
    #[wasm_bindgen(method, getter = "facingMode")]
    fn facing_mode_shim(this: &MediaTrackSupportedConstraints) -> bool;
    #[wasm_bindgen(method, setter = "facingMode")]
    fn set_facing_mode_shim(this: &MediaTrackSupportedConstraints, val: bool);
    #[wasm_bindgen(method, getter = "frameRate")]
    fn frame_rate_shim(this: &MediaTrackSupportedConstraints) -> bool;
    #[wasm_bindgen(method, setter = "frameRate")]
    fn set_frame_rate_shim(this: &MediaTrackSupportedConstraints, val: bool);
    #[wasm_bindgen(method, getter = "groupId")]
    fn group_id_shim(this: &MediaTrackSupportedConstraints) -> bool;
    #[wasm_bindgen(method, setter = "groupId")]
    fn set_group_id_shim(this: &MediaTrackSupportedConstraints, val: bool);
    #[wasm_bindgen(method, getter = "height")]
    fn height_shim(this: &MediaTrackSupportedConstraints) -> bool;
    #[wasm_bindgen(method, setter = "height")]
    fn set_height_shim(this: &MediaTrackSupportedConstraints, val: bool);
    #[wasm_bindgen(method, getter = "latency")]
    fn latency_shim(this: &MediaTrackSupportedConstraints) -> bool;
    #[wasm_bindgen(method, setter = "latency")]
    fn set_latency_shim(this: &MediaTrackSupportedConstraints, val: bool);
    #[wasm_bindgen(method, getter = "noiseSuppression")]
    fn noise_suppression_shim(this: &MediaTrackSupportedConstraints) -> bool;
    #[wasm_bindgen(method, setter = "noiseSuppression")]
    fn set_noise_suppression_shim(this: &MediaTrackSupportedConstraints, val: bool);
    #[wasm_bindgen(method, getter = "sampleRate")]
    fn sample_rate_shim(this: &MediaTrackSupportedConstraints) -> bool;
    #[wasm_bindgen(method, setter = "sampleRate")]
    fn set_sample_rate_shim(this: &MediaTrackSupportedConstraints, val: bool);
    #[wasm_bindgen(method, getter = "sampleSize")]
    fn sample_size_shim(this: &MediaTrackSupportedConstraints) -> bool;
    #[wasm_bindgen(method, setter = "sampleSize")]
    fn set_sample_size_shim(this: &MediaTrackSupportedConstraints, val: bool);
    #[wasm_bindgen(method, getter = "volume")]
    fn volume_shim(this: &MediaTrackSupportedConstraints) -> bool;
    #[wasm_bindgen(method, setter = "volume")]
    fn set_volume_shim(this: &MediaTrackSupportedConstraints, val: bool);
    #[wasm_bindgen(method, getter = "width")]
    fn width_shim(this: &MediaTrackSupportedConstraints) -> bool;
    #[wasm_bindgen(method, setter = "width")]
    fn set_width_shim(this: &MediaTrackSupportedConstraints, val: bool);
}
#[doc = "The trait to access properties on the `MediaTrackSupportedConstraints` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `MediaTrackSupportedConstraints`*"]
pub trait MediaTrackSupportedConstraintsGetters {
    #[doc = "Get the `aspectRatio` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackSupportedConstraints`*"]
    fn aspect_ratio(&self) -> bool;
    #[doc = "Get the `autoGainControl` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackSupportedConstraints`*"]
    fn auto_gain_control(&self) -> bool;
    #[doc = "Get the `channelCount` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackSupportedConstraints`*"]
    fn channel_count(&self) -> bool;
    #[doc = "Get the `deviceId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackSupportedConstraints`*"]
    fn device_id(&self) -> bool;
    #[doc = "Get the `echoCancellation` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackSupportedConstraints`*"]
    fn echo_cancellation(&self) -> bool;
    #[doc = "Get the `facingMode` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackSupportedConstraints`*"]
    fn facing_mode(&self) -> bool;
    #[doc = "Get the `frameRate` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackSupportedConstraints`*"]
    fn frame_rate(&self) -> bool;
    #[doc = "Get the `groupId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackSupportedConstraints`*"]
    fn group_id(&self) -> bool;
    #[doc = "Get the `height` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackSupportedConstraints`*"]
    fn height(&self) -> bool;
    #[doc = "Get the `latency` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackSupportedConstraints`*"]
    fn latency(&self) -> bool;
    #[doc = "Get the `noiseSuppression` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackSupportedConstraints`*"]
    fn noise_suppression(&self) -> bool;
    #[doc = "Get the `sampleRate` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackSupportedConstraints`*"]
    fn sample_rate(&self) -> bool;
    #[doc = "Get the `sampleSize` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackSupportedConstraints`*"]
    fn sample_size(&self) -> bool;
    #[doc = "Get the `volume` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackSupportedConstraints`*"]
    fn volume(&self) -> bool;
    #[doc = "Get the `width` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackSupportedConstraints`*"]
    fn width(&self) -> bool;
}
impl MediaTrackSupportedConstraintsGetters for MediaTrackSupportedConstraints {
    fn aspect_ratio(&self) -> bool {
        self.aspect_ratio_shim()
    }
    fn auto_gain_control(&self) -> bool {
        self.auto_gain_control_shim()
    }
    fn channel_count(&self) -> bool {
        self.channel_count_shim()
    }
    fn device_id(&self) -> bool {
        self.device_id_shim()
    }
    fn echo_cancellation(&self) -> bool {
        self.echo_cancellation_shim()
    }
    fn facing_mode(&self) -> bool {
        self.facing_mode_shim()
    }
    fn frame_rate(&self) -> bool {
        self.frame_rate_shim()
    }
    fn group_id(&self) -> bool {
        self.group_id_shim()
    }
    fn height(&self) -> bool {
        self.height_shim()
    }
    fn latency(&self) -> bool {
        self.latency_shim()
    }
    fn noise_suppression(&self) -> bool {
        self.noise_suppression_shim()
    }
    fn sample_rate(&self) -> bool {
        self.sample_rate_shim()
    }
    fn sample_size(&self) -> bool {
        self.sample_size_shim()
    }
    fn volume(&self) -> bool {
        self.volume_shim()
    }
    fn width(&self) -> bool {
        self.width_shim()
    }
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
        self.set_aspect_ratio_shim(val);
        self
    }
    #[doc = "Change the `autoGainControl` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackSupportedConstraints`*"]
    pub fn auto_gain_control(&mut self, val: bool) -> &mut Self {
        self.set_auto_gain_control_shim(val);
        self
    }
    #[doc = "Change the `channelCount` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackSupportedConstraints`*"]
    pub fn channel_count(&mut self, val: bool) -> &mut Self {
        self.set_channel_count_shim(val);
        self
    }
    #[doc = "Change the `deviceId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackSupportedConstraints`*"]
    pub fn device_id(&mut self, val: bool) -> &mut Self {
        self.set_device_id_shim(val);
        self
    }
    #[doc = "Change the `echoCancellation` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackSupportedConstraints`*"]
    pub fn echo_cancellation(&mut self, val: bool) -> &mut Self {
        self.set_echo_cancellation_shim(val);
        self
    }
    #[doc = "Change the `facingMode` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackSupportedConstraints`*"]
    pub fn facing_mode(&mut self, val: bool) -> &mut Self {
        self.set_facing_mode_shim(val);
        self
    }
    #[doc = "Change the `frameRate` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackSupportedConstraints`*"]
    pub fn frame_rate(&mut self, val: bool) -> &mut Self {
        self.set_frame_rate_shim(val);
        self
    }
    #[doc = "Change the `groupId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackSupportedConstraints`*"]
    pub fn group_id(&mut self, val: bool) -> &mut Self {
        self.set_group_id_shim(val);
        self
    }
    #[doc = "Change the `height` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackSupportedConstraints`*"]
    pub fn height(&mut self, val: bool) -> &mut Self {
        self.set_height_shim(val);
        self
    }
    #[doc = "Change the `latency` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackSupportedConstraints`*"]
    pub fn latency(&mut self, val: bool) -> &mut Self {
        self.set_latency_shim(val);
        self
    }
    #[doc = "Change the `noiseSuppression` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackSupportedConstraints`*"]
    pub fn noise_suppression(&mut self, val: bool) -> &mut Self {
        self.set_noise_suppression_shim(val);
        self
    }
    #[doc = "Change the `sampleRate` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackSupportedConstraints`*"]
    pub fn sample_rate(&mut self, val: bool) -> &mut Self {
        self.set_sample_rate_shim(val);
        self
    }
    #[doc = "Change the `sampleSize` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackSupportedConstraints`*"]
    pub fn sample_size(&mut self, val: bool) -> &mut Self {
        self.set_sample_size_shim(val);
        self
    }
    #[doc = "Change the `volume` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackSupportedConstraints`*"]
    pub fn volume(&mut self, val: bool) -> &mut Self {
        self.set_volume_shim(val);
        self
    }
    #[doc = "Change the `width` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackSupportedConstraints`*"]
    pub fn width(&mut self, val: bool) -> &mut Self {
        self.set_width_shim(val);
        self
    }
}
impl Default for MediaTrackSupportedConstraints {
    fn default() -> Self {
        Self::new()
    }
}
