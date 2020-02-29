use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = VRPose , typescript_type = "VRPose" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `VrPose` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRPose)
    ///
    ///*This API requires the following crate features to be activated: `VrPose`*
    pub type VrPose;

    # [ wasm_bindgen ( structural , catch , method , getter , js_class = "VRPose" , js_name = position ) ]
    ///Getter for the `position` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRPose/position)
    ///
    ///*This API requires the following crate features to be activated: `VrPose`*
    pub fn position(this: &VrPose) -> Result<Option<Vec<f32>>, JsValue>;

    # [ wasm_bindgen ( structural , catch , method , getter , js_class = "VRPose" , js_name = linearVelocity ) ]
    ///Getter for the `linearVelocity` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRPose/linearVelocity)
    ///
    ///*This API requires the following crate features to be activated: `VrPose`*
    pub fn linear_velocity(this: &VrPose) -> Result<Option<Vec<f32>>, JsValue>;

    # [ wasm_bindgen ( structural , catch , method , getter , js_class = "VRPose" , js_name = linearAcceleration ) ]
    ///Getter for the `linearAcceleration` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRPose/linearAcceleration)
    ///
    ///*This API requires the following crate features to be activated: `VrPose`*
    pub fn linear_acceleration(this: &VrPose) -> Result<Option<Vec<f32>>, JsValue>;

    # [ wasm_bindgen ( structural , catch , method , getter , js_class = "VRPose" , js_name = orientation ) ]
    ///Getter for the `orientation` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRPose/orientation)
    ///
    ///*This API requires the following crate features to be activated: `VrPose`*
    pub fn orientation(this: &VrPose) -> Result<Option<Vec<f32>>, JsValue>;

    # [ wasm_bindgen ( structural , catch , method , getter , js_class = "VRPose" , js_name = angularVelocity ) ]
    ///Getter for the `angularVelocity` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRPose/angularVelocity)
    ///
    ///*This API requires the following crate features to be activated: `VrPose`*
    pub fn angular_velocity(this: &VrPose) -> Result<Option<Vec<f32>>, JsValue>;

    # [ wasm_bindgen ( structural , catch , method , getter , js_class = "VRPose" , js_name = angularAcceleration ) ]
    ///Getter for the `angularAcceleration` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRPose/angularAcceleration)
    ///
    ///*This API requires the following crate features to be activated: `VrPose`*
    pub fn angular_acceleration(this: &VrPose) -> Result<Option<Vec<f32>>, JsValue>;

}
