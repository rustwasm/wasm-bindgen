use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = Gamepad , typescript_name = Gamepad ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `Gamepad` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Gamepad)\n\n*This API requires the following crate features to be activated: `Gamepad`*"]
    pub type Gamepad;
    # [ wasm_bindgen ( structural , method , getter , js_name = id ) ]
    #[doc = "Getter for the `id` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Gamepad/id)\n\n*This API requires the following crate features to be activated: `Gamepad`*"]
    pub fn id(this: &Gamepad) -> String;
    # [ wasm_bindgen ( structural , method , getter , js_name = index ) ]
    #[doc = "Getter for the `index` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Gamepad/index)\n\n*This API requires the following crate features to be activated: `Gamepad`*"]
    pub fn index(this: &Gamepad) -> u32;
    # [ wasm_bindgen ( structural , method , getter , js_name = mapping ) ]
    #[cfg(feature = "GamepadMappingType")]
    #[doc = "Getter for the `mapping` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Gamepad/mapping)\n\n*This API requires the following crate features to be activated: `Gamepad`, `GamepadMappingType`*"]
    pub fn mapping(this: &Gamepad) -> GamepadMappingType;
    # [ wasm_bindgen ( structural , method , getter , js_name = hand ) ]
    #[cfg(feature = "GamepadHand")]
    #[doc = "Getter for the `hand` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Gamepad/hand)\n\n*This API requires the following crate features to be activated: `Gamepad`, `GamepadHand`*"]
    pub fn hand(this: &Gamepad) -> GamepadHand;
    # [ wasm_bindgen ( structural , method , getter , js_name = displayId ) ]
    #[doc = "Getter for the `displayId` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Gamepad/displayId)\n\n*This API requires the following crate features to be activated: `Gamepad`*"]
    pub fn display_id(this: &Gamepad) -> u32;
    # [ wasm_bindgen ( structural , method , getter , js_name = connected ) ]
    #[doc = "Getter for the `connected` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Gamepad/connected)\n\n*This API requires the following crate features to be activated: `Gamepad`*"]
    pub fn connected(this: &Gamepad) -> bool;
    # [ wasm_bindgen ( structural , method , getter , js_name = buttons ) ]
    #[doc = "Getter for the `buttons` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Gamepad/buttons)\n\n*This API requires the following crate features to be activated: `Gamepad`*"]
    pub fn buttons(this: &Gamepad) -> ::js_sys::Array;
    # [ wasm_bindgen ( structural , method , getter , js_name = axes ) ]
    #[doc = "Getter for the `axes` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Gamepad/axes)\n\n*This API requires the following crate features to be activated: `Gamepad`*"]
    pub fn axes(this: &Gamepad) -> ::js_sys::Array;
    # [ wasm_bindgen ( structural , method , getter , js_name = timestamp ) ]
    #[doc = "Getter for the `timestamp` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Gamepad/timestamp)\n\n*This API requires the following crate features to be activated: `Gamepad`*"]
    pub fn timestamp(this: &Gamepad) -> f64;
    # [ wasm_bindgen ( structural , method , getter , js_name = pose ) ]
    #[cfg(feature = "GamepadPose")]
    #[doc = "Getter for the `pose` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Gamepad/pose)\n\n*This API requires the following crate features to be activated: `Gamepad`, `GamepadPose`*"]
    pub fn pose(this: &Gamepad) -> Option<GamepadPose>;
    # [ wasm_bindgen ( structural , method , getter , js_name = hapticActuators ) ]
    #[doc = "Getter for the `hapticActuators` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Gamepad/hapticActuators)\n\n*This API requires the following crate features to be activated: `Gamepad`*"]
    pub fn haptic_actuators(this: &Gamepad) -> ::js_sys::Array;
}
