#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = PannerOptions)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `PannerOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PannerOptions`*"]
    pub type PannerOptions;
    #[wasm_bindgen(method, getter = "channelCount")]
    fn channel_count_shim(this: &PannerOptions) -> u32;
    #[wasm_bindgen(method, setter = "channelCount")]
    fn set_channel_count_shim(this: &PannerOptions, val: u32);
    #[cfg(feature = "ChannelCountMode")]
    #[wasm_bindgen(method, getter = "channelCountMode")]
    fn channel_count_mode_shim(this: &PannerOptions) -> ChannelCountMode;
    #[cfg(feature = "ChannelCountMode")]
    #[wasm_bindgen(method, setter = "channelCountMode")]
    fn set_channel_count_mode_shim(this: &PannerOptions, val: ChannelCountMode);
    #[cfg(feature = "ChannelInterpretation")]
    #[wasm_bindgen(method, getter = "channelInterpretation")]
    fn channel_interpretation_shim(this: &PannerOptions) -> ChannelInterpretation;
    #[cfg(feature = "ChannelInterpretation")]
    #[wasm_bindgen(method, setter = "channelInterpretation")]
    fn set_channel_interpretation_shim(this: &PannerOptions, val: ChannelInterpretation);
    #[wasm_bindgen(method, getter = "coneInnerAngle")]
    fn cone_inner_angle_shim(this: &PannerOptions) -> f64;
    #[wasm_bindgen(method, setter = "coneInnerAngle")]
    fn set_cone_inner_angle_shim(this: &PannerOptions, val: f64);
    #[wasm_bindgen(method, getter = "coneOuterAngle")]
    fn cone_outer_angle_shim(this: &PannerOptions) -> f64;
    #[wasm_bindgen(method, setter = "coneOuterAngle")]
    fn set_cone_outer_angle_shim(this: &PannerOptions, val: f64);
    #[wasm_bindgen(method, getter = "coneOuterGain")]
    fn cone_outer_gain_shim(this: &PannerOptions) -> f64;
    #[wasm_bindgen(method, setter = "coneOuterGain")]
    fn set_cone_outer_gain_shim(this: &PannerOptions, val: f64);
    #[cfg(feature = "DistanceModelType")]
    #[wasm_bindgen(method, getter = "distanceModel")]
    fn distance_model_shim(this: &PannerOptions) -> DistanceModelType;
    #[cfg(feature = "DistanceModelType")]
    #[wasm_bindgen(method, setter = "distanceModel")]
    fn set_distance_model_shim(this: &PannerOptions, val: DistanceModelType);
    #[wasm_bindgen(method, getter = "maxDistance")]
    fn max_distance_shim(this: &PannerOptions) -> f64;
    #[wasm_bindgen(method, setter = "maxDistance")]
    fn set_max_distance_shim(this: &PannerOptions, val: f64);
    #[wasm_bindgen(method, getter = "orientationX")]
    fn orientation_x_shim(this: &PannerOptions) -> f32;
    #[wasm_bindgen(method, setter = "orientationX")]
    fn set_orientation_x_shim(this: &PannerOptions, val: f32);
    #[wasm_bindgen(method, getter = "orientationY")]
    fn orientation_y_shim(this: &PannerOptions) -> f32;
    #[wasm_bindgen(method, setter = "orientationY")]
    fn set_orientation_y_shim(this: &PannerOptions, val: f32);
    #[wasm_bindgen(method, getter = "orientationZ")]
    fn orientation_z_shim(this: &PannerOptions) -> f32;
    #[wasm_bindgen(method, setter = "orientationZ")]
    fn set_orientation_z_shim(this: &PannerOptions, val: f32);
    #[cfg(feature = "PanningModelType")]
    #[wasm_bindgen(method, getter = "panningModel")]
    fn panning_model_shim(this: &PannerOptions) -> PanningModelType;
    #[cfg(feature = "PanningModelType")]
    #[wasm_bindgen(method, setter = "panningModel")]
    fn set_panning_model_shim(this: &PannerOptions, val: PanningModelType);
    #[wasm_bindgen(method, getter = "positionX")]
    fn position_x_shim(this: &PannerOptions) -> f32;
    #[wasm_bindgen(method, setter = "positionX")]
    fn set_position_x_shim(this: &PannerOptions, val: f32);
    #[wasm_bindgen(method, getter = "positionY")]
    fn position_y_shim(this: &PannerOptions) -> f32;
    #[wasm_bindgen(method, setter = "positionY")]
    fn set_position_y_shim(this: &PannerOptions, val: f32);
    #[wasm_bindgen(method, getter = "positionZ")]
    fn position_z_shim(this: &PannerOptions) -> f32;
    #[wasm_bindgen(method, setter = "positionZ")]
    fn set_position_z_shim(this: &PannerOptions, val: f32);
    #[wasm_bindgen(method, getter = "refDistance")]
    fn ref_distance_shim(this: &PannerOptions) -> f64;
    #[wasm_bindgen(method, setter = "refDistance")]
    fn set_ref_distance_shim(this: &PannerOptions, val: f64);
    #[wasm_bindgen(method, getter = "rolloffFactor")]
    fn rolloff_factor_shim(this: &PannerOptions) -> f64;
    #[wasm_bindgen(method, setter = "rolloffFactor")]
    fn set_rolloff_factor_shim(this: &PannerOptions, val: f64);
}
#[doc = "The trait to access properties on the `PannerOptions` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `PannerOptions`*"]
pub trait PannerOptionsGetters {
    #[doc = "Get the `channelCount` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PannerOptions`*"]
    fn channel_count(&self) -> u32;
    #[cfg(feature = "ChannelCountMode")]
    #[doc = "Get the `channelCountMode` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ChannelCountMode`, `PannerOptions`*"]
    fn channel_count_mode(&self) -> ChannelCountMode;
    #[cfg(feature = "ChannelInterpretation")]
    #[doc = "Get the `channelInterpretation` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ChannelInterpretation`, `PannerOptions`*"]
    fn channel_interpretation(&self) -> ChannelInterpretation;
    #[doc = "Get the `coneInnerAngle` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PannerOptions`*"]
    fn cone_inner_angle(&self) -> f64;
    #[doc = "Get the `coneOuterAngle` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PannerOptions`*"]
    fn cone_outer_angle(&self) -> f64;
    #[doc = "Get the `coneOuterGain` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PannerOptions`*"]
    fn cone_outer_gain(&self) -> f64;
    #[cfg(feature = "DistanceModelType")]
    #[doc = "Get the `distanceModel` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DistanceModelType`, `PannerOptions`*"]
    fn distance_model(&self) -> DistanceModelType;
    #[doc = "Get the `maxDistance` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PannerOptions`*"]
    fn max_distance(&self) -> f64;
    #[doc = "Get the `orientationX` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PannerOptions`*"]
    fn orientation_x(&self) -> f32;
    #[doc = "Get the `orientationY` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PannerOptions`*"]
    fn orientation_y(&self) -> f32;
    #[doc = "Get the `orientationZ` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PannerOptions`*"]
    fn orientation_z(&self) -> f32;
    #[cfg(feature = "PanningModelType")]
    #[doc = "Get the `panningModel` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PannerOptions`, `PanningModelType`*"]
    fn panning_model(&self) -> PanningModelType;
    #[doc = "Get the `positionX` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PannerOptions`*"]
    fn position_x(&self) -> f32;
    #[doc = "Get the `positionY` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PannerOptions`*"]
    fn position_y(&self) -> f32;
    #[doc = "Get the `positionZ` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PannerOptions`*"]
    fn position_z(&self) -> f32;
    #[doc = "Get the `refDistance` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PannerOptions`*"]
    fn ref_distance(&self) -> f64;
    #[doc = "Get the `rolloffFactor` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PannerOptions`*"]
    fn rolloff_factor(&self) -> f64;
}
impl PannerOptionsGetters for PannerOptions {
    fn channel_count(&self) -> u32 {
        self.channel_count_shim()
    }
    #[cfg(feature = "ChannelCountMode")]
    fn channel_count_mode(&self) -> ChannelCountMode {
        self.channel_count_mode_shim()
    }
    #[cfg(feature = "ChannelInterpretation")]
    fn channel_interpretation(&self) -> ChannelInterpretation {
        self.channel_interpretation_shim()
    }
    fn cone_inner_angle(&self) -> f64 {
        self.cone_inner_angle_shim()
    }
    fn cone_outer_angle(&self) -> f64 {
        self.cone_outer_angle_shim()
    }
    fn cone_outer_gain(&self) -> f64 {
        self.cone_outer_gain_shim()
    }
    #[cfg(feature = "DistanceModelType")]
    fn distance_model(&self) -> DistanceModelType {
        self.distance_model_shim()
    }
    fn max_distance(&self) -> f64 {
        self.max_distance_shim()
    }
    fn orientation_x(&self) -> f32 {
        self.orientation_x_shim()
    }
    fn orientation_y(&self) -> f32 {
        self.orientation_y_shim()
    }
    fn orientation_z(&self) -> f32 {
        self.orientation_z_shim()
    }
    #[cfg(feature = "PanningModelType")]
    fn panning_model(&self) -> PanningModelType {
        self.panning_model_shim()
    }
    fn position_x(&self) -> f32 {
        self.position_x_shim()
    }
    fn position_y(&self) -> f32 {
        self.position_y_shim()
    }
    fn position_z(&self) -> f32 {
        self.position_z_shim()
    }
    fn ref_distance(&self) -> f64 {
        self.ref_distance_shim()
    }
    fn rolloff_factor(&self) -> f64 {
        self.rolloff_factor_shim()
    }
}
impl PannerOptions {
    #[doc = "Construct a new `PannerOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PannerOptions`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `channelCount` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PannerOptions`*"]
    pub fn channel_count(&mut self, val: u32) -> &mut Self {
        self.set_channel_count_shim(val);
        self
    }
    #[cfg(feature = "ChannelCountMode")]
    #[doc = "Change the `channelCountMode` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ChannelCountMode`, `PannerOptions`*"]
    pub fn channel_count_mode(&mut self, val: ChannelCountMode) -> &mut Self {
        self.set_channel_count_mode_shim(val);
        self
    }
    #[cfg(feature = "ChannelInterpretation")]
    #[doc = "Change the `channelInterpretation` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ChannelInterpretation`, `PannerOptions`*"]
    pub fn channel_interpretation(&mut self, val: ChannelInterpretation) -> &mut Self {
        self.set_channel_interpretation_shim(val);
        self
    }
    #[doc = "Change the `coneInnerAngle` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PannerOptions`*"]
    pub fn cone_inner_angle(&mut self, val: f64) -> &mut Self {
        self.set_cone_inner_angle_shim(val);
        self
    }
    #[doc = "Change the `coneOuterAngle` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PannerOptions`*"]
    pub fn cone_outer_angle(&mut self, val: f64) -> &mut Self {
        self.set_cone_outer_angle_shim(val);
        self
    }
    #[doc = "Change the `coneOuterGain` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PannerOptions`*"]
    pub fn cone_outer_gain(&mut self, val: f64) -> &mut Self {
        self.set_cone_outer_gain_shim(val);
        self
    }
    #[cfg(feature = "DistanceModelType")]
    #[doc = "Change the `distanceModel` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DistanceModelType`, `PannerOptions`*"]
    pub fn distance_model(&mut self, val: DistanceModelType) -> &mut Self {
        self.set_distance_model_shim(val);
        self
    }
    #[doc = "Change the `maxDistance` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PannerOptions`*"]
    pub fn max_distance(&mut self, val: f64) -> &mut Self {
        self.set_max_distance_shim(val);
        self
    }
    #[doc = "Change the `orientationX` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PannerOptions`*"]
    pub fn orientation_x(&mut self, val: f32) -> &mut Self {
        self.set_orientation_x_shim(val);
        self
    }
    #[doc = "Change the `orientationY` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PannerOptions`*"]
    pub fn orientation_y(&mut self, val: f32) -> &mut Self {
        self.set_orientation_y_shim(val);
        self
    }
    #[doc = "Change the `orientationZ` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PannerOptions`*"]
    pub fn orientation_z(&mut self, val: f32) -> &mut Self {
        self.set_orientation_z_shim(val);
        self
    }
    #[cfg(feature = "PanningModelType")]
    #[doc = "Change the `panningModel` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PannerOptions`, `PanningModelType`*"]
    pub fn panning_model(&mut self, val: PanningModelType) -> &mut Self {
        self.set_panning_model_shim(val);
        self
    }
    #[doc = "Change the `positionX` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PannerOptions`*"]
    pub fn position_x(&mut self, val: f32) -> &mut Self {
        self.set_position_x_shim(val);
        self
    }
    #[doc = "Change the `positionY` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PannerOptions`*"]
    pub fn position_y(&mut self, val: f32) -> &mut Self {
        self.set_position_y_shim(val);
        self
    }
    #[doc = "Change the `positionZ` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PannerOptions`*"]
    pub fn position_z(&mut self, val: f32) -> &mut Self {
        self.set_position_z_shim(val);
        self
    }
    #[doc = "Change the `refDistance` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PannerOptions`*"]
    pub fn ref_distance(&mut self, val: f64) -> &mut Self {
        self.set_ref_distance_shim(val);
        self
    }
    #[doc = "Change the `rolloffFactor` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PannerOptions`*"]
    pub fn rolloff_factor(&mut self, val: f64) -> &mut Self {
        self.set_rolloff_factor_shim(val);
        self
    }
}
impl Default for PannerOptions {
    fn default() -> Self {
        Self::new()
    }
}
