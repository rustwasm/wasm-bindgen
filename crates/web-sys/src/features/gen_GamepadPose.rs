use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = GamepadPose , typescript_type = "GamepadPose" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `GamepadPose` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GamepadPose)
    ///
    ///*This API requires the following crate features to be activated: `GamepadPose`*
    pub type GamepadPose;

    # [ wasm_bindgen ( structural , method , getter , js_class = "GamepadPose" , js_name = hasOrientation ) ]
    ///Getter for the `hasOrientation` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GamepadPose/hasOrientation)
    ///
    ///*This API requires the following crate features to be activated: `GamepadPose`*
    pub fn has_orientation(this: &GamepadPose) -> bool;

    # [ wasm_bindgen ( structural , method , getter , js_class = "GamepadPose" , js_name = hasPosition ) ]
    ///Getter for the `hasPosition` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GamepadPose/hasPosition)
    ///
    ///*This API requires the following crate features to be activated: `GamepadPose`*
    pub fn has_position(this: &GamepadPose) -> bool;

    # [ wasm_bindgen ( structural , catch , method , getter , js_class = "GamepadPose" , js_name = position ) ]
    ///Getter for the `position` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GamepadPose/position)
    ///
    ///*This API requires the following crate features to be activated: `GamepadPose`*
    pub fn position(this: &GamepadPose) -> Result<Option<Vec<f32>>, JsValue>;

    # [ wasm_bindgen ( structural , catch , method , getter , js_class = "GamepadPose" , js_name = linearVelocity ) ]
    ///Getter for the `linearVelocity` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GamepadPose/linearVelocity)
    ///
    ///*This API requires the following crate features to be activated: `GamepadPose`*
    pub fn linear_velocity(this: &GamepadPose) -> Result<Option<Vec<f32>>, JsValue>;

    # [ wasm_bindgen ( structural , catch , method , getter , js_class = "GamepadPose" , js_name = linearAcceleration ) ]
    ///Getter for the `linearAcceleration` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GamepadPose/linearAcceleration)
    ///
    ///*This API requires the following crate features to be activated: `GamepadPose`*
    pub fn linear_acceleration(this: &GamepadPose) -> Result<Option<Vec<f32>>, JsValue>;

    # [ wasm_bindgen ( structural , catch , method , getter , js_class = "GamepadPose" , js_name = orientation ) ]
    ///Getter for the `orientation` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GamepadPose/orientation)
    ///
    ///*This API requires the following crate features to be activated: `GamepadPose`*
    pub fn orientation(this: &GamepadPose) -> Result<Option<Vec<f32>>, JsValue>;

    # [ wasm_bindgen ( structural , catch , method , getter , js_class = "GamepadPose" , js_name = angularVelocity ) ]
    ///Getter for the `angularVelocity` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GamepadPose/angularVelocity)
    ///
    ///*This API requires the following crate features to be activated: `GamepadPose`*
    pub fn angular_velocity(this: &GamepadPose) -> Result<Option<Vec<f32>>, JsValue>;

    # [ wasm_bindgen ( structural , catch , method , getter , js_class = "GamepadPose" , js_name = angularAcceleration ) ]
    ///Getter for the `angularAcceleration` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GamepadPose/angularAcceleration)
    ///
    ///*This API requires the following crate features to be activated: `GamepadPose`*
    pub fn angular_acceleration(this: &GamepadPose) -> Result<Option<Vec<f32>>, JsValue>;

}
