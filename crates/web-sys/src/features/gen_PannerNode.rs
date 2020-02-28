use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = AudioNode , extends = EventTarget , extends = :: js_sys :: Object , js_name = PannerNode , typescript_name = PannerNode ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `PannerNode` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PannerNode)\n\n*This API requires the following crate features to be activated: `PannerNode`*"]
    pub type PannerNode;
    # [ wasm_bindgen ( structural , method , getter , js_name = panningModel ) ]
    #[cfg(feature = "PanningModelType")]
    #[doc = "Getter for the `panningModel` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PannerNode/panningModel)\n\n*This API requires the following crate features to be activated: `PannerNode`, `PanningModelType`*"]
    pub fn panning_model(this: &PannerNode) -> PanningModelType;
    # [ wasm_bindgen ( structural , method , setter , js_name = panningModel ) ]
    #[cfg(feature = "PanningModelType")]
    #[doc = "Setter for the `panningModel` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PannerNode/panningModel)\n\n*This API requires the following crate features to be activated: `PannerNode`, `PanningModelType`*"]
    pub fn set_panning_model(this: &PannerNode, value: PanningModelType);
    # [ wasm_bindgen ( structural , method , getter , js_name = positionX ) ]
    #[cfg(feature = "AudioParam")]
    #[doc = "Getter for the `positionX` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PannerNode/positionX)\n\n*This API requires the following crate features to be activated: `AudioParam`, `PannerNode`*"]
    pub fn position_x(this: &PannerNode) -> AudioParam;
    # [ wasm_bindgen ( structural , method , getter , js_name = positionY ) ]
    #[cfg(feature = "AudioParam")]
    #[doc = "Getter for the `positionY` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PannerNode/positionY)\n\n*This API requires the following crate features to be activated: `AudioParam`, `PannerNode`*"]
    pub fn position_y(this: &PannerNode) -> AudioParam;
    # [ wasm_bindgen ( structural , method , getter , js_name = positionZ ) ]
    #[cfg(feature = "AudioParam")]
    #[doc = "Getter for the `positionZ` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PannerNode/positionZ)\n\n*This API requires the following crate features to be activated: `AudioParam`, `PannerNode`*"]
    pub fn position_z(this: &PannerNode) -> AudioParam;
    # [ wasm_bindgen ( structural , method , getter , js_name = orientationX ) ]
    #[cfg(feature = "AudioParam")]
    #[doc = "Getter for the `orientationX` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PannerNode/orientationX)\n\n*This API requires the following crate features to be activated: `AudioParam`, `PannerNode`*"]
    pub fn orientation_x(this: &PannerNode) -> AudioParam;
    # [ wasm_bindgen ( structural , method , getter , js_name = orientationY ) ]
    #[cfg(feature = "AudioParam")]
    #[doc = "Getter for the `orientationY` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PannerNode/orientationY)\n\n*This API requires the following crate features to be activated: `AudioParam`, `PannerNode`*"]
    pub fn orientation_y(this: &PannerNode) -> AudioParam;
    # [ wasm_bindgen ( structural , method , getter , js_name = orientationZ ) ]
    #[cfg(feature = "AudioParam")]
    #[doc = "Getter for the `orientationZ` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PannerNode/orientationZ)\n\n*This API requires the following crate features to be activated: `AudioParam`, `PannerNode`*"]
    pub fn orientation_z(this: &PannerNode) -> AudioParam;
    # [ wasm_bindgen ( structural , method , getter , js_name = distanceModel ) ]
    #[cfg(feature = "DistanceModelType")]
    #[doc = "Getter for the `distanceModel` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PannerNode/distanceModel)\n\n*This API requires the following crate features to be activated: `DistanceModelType`, `PannerNode`*"]
    pub fn distance_model(this: &PannerNode) -> DistanceModelType;
    # [ wasm_bindgen ( structural , method , setter , js_name = distanceModel ) ]
    #[cfg(feature = "DistanceModelType")]
    #[doc = "Setter for the `distanceModel` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PannerNode/distanceModel)\n\n*This API requires the following crate features to be activated: `DistanceModelType`, `PannerNode`*"]
    pub fn set_distance_model(this: &PannerNode, value: DistanceModelType);
    # [ wasm_bindgen ( structural , method , getter , js_name = refDistance ) ]
    #[doc = "Getter for the `refDistance` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PannerNode/refDistance)\n\n*This API requires the following crate features to be activated: `PannerNode`*"]
    pub fn ref_distance(this: &PannerNode) -> f64;
    # [ wasm_bindgen ( structural , method , setter , js_name = refDistance ) ]
    #[doc = "Setter for the `refDistance` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PannerNode/refDistance)\n\n*This API requires the following crate features to be activated: `PannerNode`*"]
    pub fn set_ref_distance(this: &PannerNode, value: f64);
    # [ wasm_bindgen ( structural , method , getter , js_name = maxDistance ) ]
    #[doc = "Getter for the `maxDistance` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PannerNode/maxDistance)\n\n*This API requires the following crate features to be activated: `PannerNode`*"]
    pub fn max_distance(this: &PannerNode) -> f64;
    # [ wasm_bindgen ( structural , method , setter , js_name = maxDistance ) ]
    #[doc = "Setter for the `maxDistance` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PannerNode/maxDistance)\n\n*This API requires the following crate features to be activated: `PannerNode`*"]
    pub fn set_max_distance(this: &PannerNode, value: f64);
    # [ wasm_bindgen ( structural , method , getter , js_name = rolloffFactor ) ]
    #[doc = "Getter for the `rolloffFactor` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PannerNode/rolloffFactor)\n\n*This API requires the following crate features to be activated: `PannerNode`*"]
    pub fn rolloff_factor(this: &PannerNode) -> f64;
    # [ wasm_bindgen ( structural , method , setter , js_name = rolloffFactor ) ]
    #[doc = "Setter for the `rolloffFactor` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PannerNode/rolloffFactor)\n\n*This API requires the following crate features to be activated: `PannerNode`*"]
    pub fn set_rolloff_factor(this: &PannerNode, value: f64);
    # [ wasm_bindgen ( structural , method , getter , js_name = coneInnerAngle ) ]
    #[doc = "Getter for the `coneInnerAngle` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PannerNode/coneInnerAngle)\n\n*This API requires the following crate features to be activated: `PannerNode`*"]
    pub fn cone_inner_angle(this: &PannerNode) -> f64;
    # [ wasm_bindgen ( structural , method , setter , js_name = coneInnerAngle ) ]
    #[doc = "Setter for the `coneInnerAngle` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PannerNode/coneInnerAngle)\n\n*This API requires the following crate features to be activated: `PannerNode`*"]
    pub fn set_cone_inner_angle(this: &PannerNode, value: f64);
    # [ wasm_bindgen ( structural , method , getter , js_name = coneOuterAngle ) ]
    #[doc = "Getter for the `coneOuterAngle` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PannerNode/coneOuterAngle)\n\n*This API requires the following crate features to be activated: `PannerNode`*"]
    pub fn cone_outer_angle(this: &PannerNode) -> f64;
    # [ wasm_bindgen ( structural , method , setter , js_name = coneOuterAngle ) ]
    #[doc = "Setter for the `coneOuterAngle` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PannerNode/coneOuterAngle)\n\n*This API requires the following crate features to be activated: `PannerNode`*"]
    pub fn set_cone_outer_angle(this: &PannerNode, value: f64);
    # [ wasm_bindgen ( structural , method , getter , js_name = coneOuterGain ) ]
    #[doc = "Getter for the `coneOuterGain` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PannerNode/coneOuterGain)\n\n*This API requires the following crate features to be activated: `PannerNode`*"]
    pub fn cone_outer_gain(this: &PannerNode) -> f64;
    # [ wasm_bindgen ( structural , method , setter , js_name = coneOuterGain ) ]
    #[doc = "Setter for the `coneOuterGain` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PannerNode/coneOuterGain)\n\n*This API requires the following crate features to be activated: `PannerNode`*"]
    pub fn set_cone_outer_gain(this: &PannerNode, value: f64);
    #[cfg(feature = "BaseAudioContext")]
    #[wasm_bindgen(catch, constructor)]
    #[doc = "The `new PannerNode(..)` constructor, creating a new instance of `PannerNode`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PannerNode/PannerNode)\n\n*This API requires the following crate features to be activated: `BaseAudioContext`, `PannerNode`*"]
    pub fn new(this: &PannerNode, context: &BaseAudioContext) -> Result<PannerNode, JsValue>;
    #[cfg(all(feature = "BaseAudioContext", feature = "PannerOptions",))]
    #[wasm_bindgen(catch, constructor)]
    #[doc = "The `new PannerNode(..)` constructor, creating a new instance of `PannerNode`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PannerNode/PannerNode)\n\n*This API requires the following crate features to be activated: `BaseAudioContext`, `PannerNode`, `PannerOptions`*"]
    pub fn new_with_options(
        this: &PannerNode,
        context: &BaseAudioContext,
        options: &PannerOptions,
    ) -> Result<PannerNode, JsValue>;
    # [ wasm_bindgen ( method , structural , js_name = setOrientation ) ]
    #[doc = "The `setOrientation()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PannerNode/setOrientation)\n\n*This API requires the following crate features to be activated: `PannerNode`*"]
    pub fn set_orientation(this: &PannerNode, x: f64, y: f64, z: f64);
    # [ wasm_bindgen ( method , structural , js_name = setPosition ) ]
    #[doc = "The `setPosition()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PannerNode/setPosition)\n\n*This API requires the following crate features to be activated: `PannerNode`*"]
    pub fn set_position(this: &PannerNode, x: f64, y: f64, z: f64);
    # [ wasm_bindgen ( method , structural , js_name = setVelocity ) ]
    #[doc = "The `setVelocity()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PannerNode/setVelocity)\n\n*This API requires the following crate features to be activated: `PannerNode`*"]
    pub fn set_velocity(this: &PannerNode, x: f64, y: f64, z: f64);
}
