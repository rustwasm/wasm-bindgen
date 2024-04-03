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
    #[wasm_bindgen(method, setter = "channelCount")]
    fn channel_count_shim(this: &PannerOptions, val: u32);
    #[cfg(feature = "ChannelCountMode")]
    #[wasm_bindgen(method, setter = "channelCountMode")]
    fn channel_count_mode_shim(this: &PannerOptions, val: ChannelCountMode);
    #[cfg(feature = "ChannelInterpretation")]
    #[wasm_bindgen(method, setter = "channelInterpretation")]
    fn channel_interpretation_shim(this: &PannerOptions, val: ChannelInterpretation);
    #[wasm_bindgen(method, setter = "coneInnerAngle")]
    fn cone_inner_angle_shim(this: &PannerOptions, val: f64);
    #[wasm_bindgen(method, setter = "coneOuterAngle")]
    fn cone_outer_angle_shim(this: &PannerOptions, val: f64);
    #[wasm_bindgen(method, setter = "coneOuterGain")]
    fn cone_outer_gain_shim(this: &PannerOptions, val: f64);
    #[cfg(feature = "DistanceModelType")]
    #[wasm_bindgen(method, setter = "distanceModel")]
    fn distance_model_shim(this: &PannerOptions, val: DistanceModelType);
    #[wasm_bindgen(method, setter = "maxDistance")]
    fn max_distance_shim(this: &PannerOptions, val: f64);
    #[wasm_bindgen(method, setter = "orientationX")]
    fn orientation_x_shim(this: &PannerOptions, val: f32);
    #[wasm_bindgen(method, setter = "orientationY")]
    fn orientation_y_shim(this: &PannerOptions, val: f32);
    #[wasm_bindgen(method, setter = "orientationZ")]
    fn orientation_z_shim(this: &PannerOptions, val: f32);
    #[cfg(feature = "PanningModelType")]
    #[wasm_bindgen(method, setter = "panningModel")]
    fn panning_model_shim(this: &PannerOptions, val: PanningModelType);
    #[wasm_bindgen(method, setter = "positionX")]
    fn position_x_shim(this: &PannerOptions, val: f32);
    #[wasm_bindgen(method, setter = "positionY")]
    fn position_y_shim(this: &PannerOptions, val: f32);
    #[wasm_bindgen(method, setter = "positionZ")]
    fn position_z_shim(this: &PannerOptions, val: f32);
    #[wasm_bindgen(method, setter = "refDistance")]
    fn ref_distance_shim(this: &PannerOptions, val: f64);
    #[wasm_bindgen(method, setter = "rolloffFactor")]
    fn rolloff_factor_shim(this: &PannerOptions, val: f64);
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
        self.channel_count_shim(val);
        self
    }
    #[cfg(feature = "ChannelCountMode")]
    #[doc = "Change the `channelCountMode` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ChannelCountMode`, `PannerOptions`*"]
    pub fn channel_count_mode(&mut self, val: ChannelCountMode) -> &mut Self {
        self.channel_count_mode_shim(val);
        self
    }
    #[cfg(feature = "ChannelInterpretation")]
    #[doc = "Change the `channelInterpretation` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ChannelInterpretation`, `PannerOptions`*"]
    pub fn channel_interpretation(&mut self, val: ChannelInterpretation) -> &mut Self {
        self.channel_interpretation_shim(val);
        self
    }
    #[doc = "Change the `coneInnerAngle` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PannerOptions`*"]
    pub fn cone_inner_angle(&mut self, val: f64) -> &mut Self {
        self.cone_inner_angle_shim(val);
        self
    }
    #[doc = "Change the `coneOuterAngle` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PannerOptions`*"]
    pub fn cone_outer_angle(&mut self, val: f64) -> &mut Self {
        self.cone_outer_angle_shim(val);
        self
    }
    #[doc = "Change the `coneOuterGain` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PannerOptions`*"]
    pub fn cone_outer_gain(&mut self, val: f64) -> &mut Self {
        self.cone_outer_gain_shim(val);
        self
    }
    #[cfg(feature = "DistanceModelType")]
    #[doc = "Change the `distanceModel` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DistanceModelType`, `PannerOptions`*"]
    pub fn distance_model(&mut self, val: DistanceModelType) -> &mut Self {
        self.distance_model_shim(val);
        self
    }
    #[doc = "Change the `maxDistance` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PannerOptions`*"]
    pub fn max_distance(&mut self, val: f64) -> &mut Self {
        self.max_distance_shim(val);
        self
    }
    #[doc = "Change the `orientationX` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PannerOptions`*"]
    pub fn orientation_x(&mut self, val: f32) -> &mut Self {
        self.orientation_x_shim(val);
        self
    }
    #[doc = "Change the `orientationY` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PannerOptions`*"]
    pub fn orientation_y(&mut self, val: f32) -> &mut Self {
        self.orientation_y_shim(val);
        self
    }
    #[doc = "Change the `orientationZ` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PannerOptions`*"]
    pub fn orientation_z(&mut self, val: f32) -> &mut Self {
        self.orientation_z_shim(val);
        self
    }
    #[cfg(feature = "PanningModelType")]
    #[doc = "Change the `panningModel` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PannerOptions`, `PanningModelType`*"]
    pub fn panning_model(&mut self, val: PanningModelType) -> &mut Self {
        self.panning_model_shim(val);
        self
    }
    #[doc = "Change the `positionX` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PannerOptions`*"]
    pub fn position_x(&mut self, val: f32) -> &mut Self {
        self.position_x_shim(val);
        self
    }
    #[doc = "Change the `positionY` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PannerOptions`*"]
    pub fn position_y(&mut self, val: f32) -> &mut Self {
        self.position_y_shim(val);
        self
    }
    #[doc = "Change the `positionZ` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PannerOptions`*"]
    pub fn position_z(&mut self, val: f32) -> &mut Self {
        self.position_z_shim(val);
        self
    }
    #[doc = "Change the `refDistance` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PannerOptions`*"]
    pub fn ref_distance(&mut self, val: f64) -> &mut Self {
        self.ref_distance_shim(val);
        self
    }
    #[doc = "Change the `rolloffFactor` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PannerOptions`*"]
    pub fn rolloff_factor(&mut self, val: f64) -> &mut Self {
        self.rolloff_factor_shim(val);
        self
    }
}
impl Default for PannerOptions {
    fn default() -> Self {
        Self::new()
    }
}
