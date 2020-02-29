use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = GamepadButton , typescript_type = "GamepadButton" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `GamepadButton` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GamepadButton)
    ///
    ///*This API requires the following crate features to be activated: `GamepadButton`*
    pub type GamepadButton;

    # [ wasm_bindgen ( structural , method , getter , js_class = "GamepadButton" , js_name = pressed ) ]
    ///Getter for the `pressed` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GamepadButton/pressed)
    ///
    ///*This API requires the following crate features to be activated: `GamepadButton`*
    pub fn pressed(this: &GamepadButton) -> bool;

    # [ wasm_bindgen ( structural , method , getter , js_class = "GamepadButton" , js_name = touched ) ]
    ///Getter for the `touched` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GamepadButton/touched)
    ///
    ///*This API requires the following crate features to be activated: `GamepadButton`*
    pub fn touched(this: &GamepadButton) -> bool;

    # [ wasm_bindgen ( structural , method , getter , js_class = "GamepadButton" , js_name = value ) ]
    ///Getter for the `value` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GamepadButton/value)
    ///
    ///*This API requires the following crate features to be activated: `GamepadButton`*
    pub fn value(this: &GamepadButton) -> f64;

}
