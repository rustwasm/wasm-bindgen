use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = Gamepad , typescript_name = Gamepad ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `Gamepad` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Gamepad)
    ///
    ///*This API requires the following crate features to be activated: `Gamepad`*
    pub type Gamepad;

    # [ wasm_bindgen ( structural , method , getter , js_class = "Gamepad" , js_name = id ) ]
    ///Getter for the `id` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Gamepad/id)
    ///
    ///*This API requires the following crate features to be activated: `Gamepad`*
    pub fn id(this: &Gamepad) -> String;

    # [ wasm_bindgen ( structural , method , getter , js_class = "Gamepad" , js_name = index ) ]
    ///Getter for the `index` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Gamepad/index)
    ///
    ///*This API requires the following crate features to be activated: `Gamepad`*
    pub fn index(this: &Gamepad) -> u32;

    #[cfg(feature = "GamepadMappingType")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "Gamepad" , js_name = mapping ) ]
    ///Getter for the `mapping` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Gamepad/mapping)
    ///
    ///*This API requires the following crate features to be activated: `Gamepad`, `GamepadMappingType`*
    pub fn mapping(this: &Gamepad) -> GamepadMappingType;

    #[cfg(feature = "GamepadHand")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "Gamepad" , js_name = hand ) ]
    ///Getter for the `hand` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Gamepad/hand)
    ///
    ///*This API requires the following crate features to be activated: `Gamepad`, `GamepadHand`*
    pub fn hand(this: &Gamepad) -> GamepadHand;

    # [ wasm_bindgen ( structural , method , getter , js_class = "Gamepad" , js_name = displayId ) ]
    ///Getter for the `displayId` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Gamepad/displayId)
    ///
    ///*This API requires the following crate features to be activated: `Gamepad`*
    pub fn display_id(this: &Gamepad) -> u32;

    # [ wasm_bindgen ( structural , method , getter , js_class = "Gamepad" , js_name = connected ) ]
    ///Getter for the `connected` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Gamepad/connected)
    ///
    ///*This API requires the following crate features to be activated: `Gamepad`*
    pub fn connected(this: &Gamepad) -> bool;

    # [ wasm_bindgen ( structural , method , getter , js_class = "Gamepad" , js_name = buttons ) ]
    ///Getter for the `buttons` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Gamepad/buttons)
    ///
    ///*This API requires the following crate features to be activated: `Gamepad`*
    pub fn buttons(this: &Gamepad) -> ::js_sys::Array;

    # [ wasm_bindgen ( structural , method , getter , js_class = "Gamepad" , js_name = axes ) ]
    ///Getter for the `axes` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Gamepad/axes)
    ///
    ///*This API requires the following crate features to be activated: `Gamepad`*
    pub fn axes(this: &Gamepad) -> ::js_sys::Array;

    # [ wasm_bindgen ( structural , method , getter , js_class = "Gamepad" , js_name = timestamp ) ]
    ///Getter for the `timestamp` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Gamepad/timestamp)
    ///
    ///*This API requires the following crate features to be activated: `Gamepad`*
    pub fn timestamp(this: &Gamepad) -> f64;

    #[cfg(feature = "GamepadPose")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "Gamepad" , js_name = pose ) ]
    ///Getter for the `pose` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Gamepad/pose)
    ///
    ///*This API requires the following crate features to be activated: `Gamepad`, `GamepadPose`*
    pub fn pose(this: &Gamepad) -> Option<GamepadPose>;

    # [ wasm_bindgen ( structural , method , getter , js_class = "Gamepad" , js_name = hapticActuators ) ]
    ///Getter for the `hapticActuators` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Gamepad/hapticActuators)
    ///
    ///*This API requires the following crate features to be activated: `Gamepad`*
    pub fn haptic_actuators(this: &Gamepad) -> ::js_sys::Array;

}
