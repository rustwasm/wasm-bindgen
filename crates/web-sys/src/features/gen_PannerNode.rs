use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = AudioNode , extends = EventTarget , extends = :: js_sys :: Object , js_name = PannerNode , typescript_name = PannerNode ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `PannerNode` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PannerNode)
    ///
    ///*This API requires the following crate features to be activated: `PannerNode`*
    pub type PannerNode;

    #[cfg(feature = "PanningModelType")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "PannerNode" , js_name = panningModel ) ]
    ///Getter for the `panningModel` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PannerNode/panningModel)
    ///
    ///*This API requires the following crate features to be activated: `PannerNode`, `PanningModelType`*
    pub fn panning_model(this: &PannerNode) -> PanningModelType;

    #[cfg(feature = "PanningModelType")]
    # [ wasm_bindgen ( structural , method , setter , js_class = "PannerNode" , js_name = panningModel ) ]
    ///Setter for the `panningModel` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PannerNode/panningModel)
    ///
    ///*This API requires the following crate features to be activated: `PannerNode`, `PanningModelType`*
    pub fn set_panning_model(this: &PannerNode, value: PanningModelType);

    #[cfg(feature = "AudioParam")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "PannerNode" , js_name = positionX ) ]
    ///Getter for the `positionX` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PannerNode/positionX)
    ///
    ///*This API requires the following crate features to be activated: `AudioParam`, `PannerNode`*
    pub fn position_x(this: &PannerNode) -> AudioParam;

    #[cfg(feature = "AudioParam")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "PannerNode" , js_name = positionY ) ]
    ///Getter for the `positionY` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PannerNode/positionY)
    ///
    ///*This API requires the following crate features to be activated: `AudioParam`, `PannerNode`*
    pub fn position_y(this: &PannerNode) -> AudioParam;

    #[cfg(feature = "AudioParam")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "PannerNode" , js_name = positionZ ) ]
    ///Getter for the `positionZ` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PannerNode/positionZ)
    ///
    ///*This API requires the following crate features to be activated: `AudioParam`, `PannerNode`*
    pub fn position_z(this: &PannerNode) -> AudioParam;

    #[cfg(feature = "AudioParam")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "PannerNode" , js_name = orientationX ) ]
    ///Getter for the `orientationX` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PannerNode/orientationX)
    ///
    ///*This API requires the following crate features to be activated: `AudioParam`, `PannerNode`*
    pub fn orientation_x(this: &PannerNode) -> AudioParam;

    #[cfg(feature = "AudioParam")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "PannerNode" , js_name = orientationY ) ]
    ///Getter for the `orientationY` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PannerNode/orientationY)
    ///
    ///*This API requires the following crate features to be activated: `AudioParam`, `PannerNode`*
    pub fn orientation_y(this: &PannerNode) -> AudioParam;

    #[cfg(feature = "AudioParam")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "PannerNode" , js_name = orientationZ ) ]
    ///Getter for the `orientationZ` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PannerNode/orientationZ)
    ///
    ///*This API requires the following crate features to be activated: `AudioParam`, `PannerNode`*
    pub fn orientation_z(this: &PannerNode) -> AudioParam;

    #[cfg(feature = "DistanceModelType")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "PannerNode" , js_name = distanceModel ) ]
    ///Getter for the `distanceModel` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PannerNode/distanceModel)
    ///
    ///*This API requires the following crate features to be activated: `DistanceModelType`, `PannerNode`*
    pub fn distance_model(this: &PannerNode) -> DistanceModelType;

    #[cfg(feature = "DistanceModelType")]
    # [ wasm_bindgen ( structural , method , setter , js_class = "PannerNode" , js_name = distanceModel ) ]
    ///Setter for the `distanceModel` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PannerNode/distanceModel)
    ///
    ///*This API requires the following crate features to be activated: `DistanceModelType`, `PannerNode`*
    pub fn set_distance_model(this: &PannerNode, value: DistanceModelType);

    # [ wasm_bindgen ( structural , method , getter , js_class = "PannerNode" , js_name = refDistance ) ]
    ///Getter for the `refDistance` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PannerNode/refDistance)
    ///
    ///*This API requires the following crate features to be activated: `PannerNode`*
    pub fn ref_distance(this: &PannerNode) -> f64;

    # [ wasm_bindgen ( structural , method , setter , js_class = "PannerNode" , js_name = refDistance ) ]
    ///Setter for the `refDistance` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PannerNode/refDistance)
    ///
    ///*This API requires the following crate features to be activated: `PannerNode`*
    pub fn set_ref_distance(this: &PannerNode, value: f64);

    # [ wasm_bindgen ( structural , method , getter , js_class = "PannerNode" , js_name = maxDistance ) ]
    ///Getter for the `maxDistance` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PannerNode/maxDistance)
    ///
    ///*This API requires the following crate features to be activated: `PannerNode`*
    pub fn max_distance(this: &PannerNode) -> f64;

    # [ wasm_bindgen ( structural , method , setter , js_class = "PannerNode" , js_name = maxDistance ) ]
    ///Setter for the `maxDistance` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PannerNode/maxDistance)
    ///
    ///*This API requires the following crate features to be activated: `PannerNode`*
    pub fn set_max_distance(this: &PannerNode, value: f64);

    # [ wasm_bindgen ( structural , method , getter , js_class = "PannerNode" , js_name = rolloffFactor ) ]
    ///Getter for the `rolloffFactor` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PannerNode/rolloffFactor)
    ///
    ///*This API requires the following crate features to be activated: `PannerNode`*
    pub fn rolloff_factor(this: &PannerNode) -> f64;

    # [ wasm_bindgen ( structural , method , setter , js_class = "PannerNode" , js_name = rolloffFactor ) ]
    ///Setter for the `rolloffFactor` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PannerNode/rolloffFactor)
    ///
    ///*This API requires the following crate features to be activated: `PannerNode`*
    pub fn set_rolloff_factor(this: &PannerNode, value: f64);

    # [ wasm_bindgen ( structural , method , getter , js_class = "PannerNode" , js_name = coneInnerAngle ) ]
    ///Getter for the `coneInnerAngle` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PannerNode/coneInnerAngle)
    ///
    ///*This API requires the following crate features to be activated: `PannerNode`*
    pub fn cone_inner_angle(this: &PannerNode) -> f64;

    # [ wasm_bindgen ( structural , method , setter , js_class = "PannerNode" , js_name = coneInnerAngle ) ]
    ///Setter for the `coneInnerAngle` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PannerNode/coneInnerAngle)
    ///
    ///*This API requires the following crate features to be activated: `PannerNode`*
    pub fn set_cone_inner_angle(this: &PannerNode, value: f64);

    # [ wasm_bindgen ( structural , method , getter , js_class = "PannerNode" , js_name = coneOuterAngle ) ]
    ///Getter for the `coneOuterAngle` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PannerNode/coneOuterAngle)
    ///
    ///*This API requires the following crate features to be activated: `PannerNode`*
    pub fn cone_outer_angle(this: &PannerNode) -> f64;

    # [ wasm_bindgen ( structural , method , setter , js_class = "PannerNode" , js_name = coneOuterAngle ) ]
    ///Setter for the `coneOuterAngle` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PannerNode/coneOuterAngle)
    ///
    ///*This API requires the following crate features to be activated: `PannerNode`*
    pub fn set_cone_outer_angle(this: &PannerNode, value: f64);

    # [ wasm_bindgen ( structural , method , getter , js_class = "PannerNode" , js_name = coneOuterGain ) ]
    ///Getter for the `coneOuterGain` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PannerNode/coneOuterGain)
    ///
    ///*This API requires the following crate features to be activated: `PannerNode`*
    pub fn cone_outer_gain(this: &PannerNode) -> f64;

    # [ wasm_bindgen ( structural , method , setter , js_class = "PannerNode" , js_name = coneOuterGain ) ]
    ///Setter for the `coneOuterGain` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PannerNode/coneOuterGain)
    ///
    ///*This API requires the following crate features to be activated: `PannerNode`*
    pub fn set_cone_outer_gain(this: &PannerNode, value: f64);

    #[cfg(feature = "BaseAudioContext")]
    #[wasm_bindgen(catch, constructor, js_class = "PannerNode")]
    ///The `new PannerNode(..)` constructor, creating a new instance of `PannerNode`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PannerNode/PannerNode)
    ///
    ///*This API requires the following crate features to be activated: `BaseAudioContext`, `PannerNode`*
    pub fn new(context: &BaseAudioContext) -> Result<PannerNode, JsValue>;

    #[cfg(all(feature = "BaseAudioContext", feature = "PannerOptions",))]
    #[wasm_bindgen(catch, constructor, js_class = "PannerNode")]
    ///The `new PannerNode(..)` constructor, creating a new instance of `PannerNode`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PannerNode/PannerNode)
    ///
    ///*This API requires the following crate features to be activated: `BaseAudioContext`, `PannerNode`, `PannerOptions`*
    pub fn new_with_options(
        context: &BaseAudioContext,
        options: &PannerOptions,
    ) -> Result<PannerNode, JsValue>;

    # [ wasm_bindgen ( method , structural , js_class = "PannerNode" , js_name = setOrientation ) ]
    ///The `setOrientation()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PannerNode/setOrientation)
    ///
    ///*This API requires the following crate features to be activated: `PannerNode`*
    pub fn set_orientation(this: &PannerNode, x: f64, y: f64, z: f64);

    # [ wasm_bindgen ( method , structural , js_class = "PannerNode" , js_name = setPosition ) ]
    ///The `setPosition()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PannerNode/setPosition)
    ///
    ///*This API requires the following crate features to be activated: `PannerNode`*
    pub fn set_position(this: &PannerNode, x: f64, y: f64, z: f64);

    # [ wasm_bindgen ( method , structural , js_class = "PannerNode" , js_name = setVelocity ) ]
    ///The `setVelocity()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PannerNode/setVelocity)
    ///
    ///*This API requires the following crate features to be activated: `PannerNode`*
    pub fn set_velocity(this: &PannerNode, x: f64, y: f64, z: f64);

}
