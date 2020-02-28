use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = GamepadButton , typescript_name = GamepadButton ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `GamepadButton` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GamepadButton)\n\n*This API requires the following crate features to be activated: `GamepadButton`*"]
    pub type GamepadButton;
    # [ wasm_bindgen ( structural , method , getter , js_class = "GamepadButton" , js_name = pressed ) ]
    #[doc = "Getter for the `pressed` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GamepadButton/pressed)\n\n*This API requires the following crate features to be activated: `GamepadButton`*"]
    pub fn pressed(this: &GamepadButton) -> bool;
    # [ wasm_bindgen ( structural , method , getter , js_class = "GamepadButton" , js_name = touched ) ]
    #[doc = "Getter for the `touched` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GamepadButton/touched)\n\n*This API requires the following crate features to be activated: `GamepadButton`*"]
    pub fn touched(this: &GamepadButton) -> bool;
    # [ wasm_bindgen ( structural , method , getter , js_class = "GamepadButton" , js_name = value ) ]
    #[doc = "Getter for the `value` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GamepadButton/value)\n\n*This API requires the following crate features to be activated: `GamepadButton`*"]
    pub fn value(this: &GamepadButton) -> f64;
}
