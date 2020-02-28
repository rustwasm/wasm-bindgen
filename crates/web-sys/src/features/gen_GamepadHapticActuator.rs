use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = GamepadHapticActuator , typescript_name = GamepadHapticActuator ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `GamepadHapticActuator` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GamepadHapticActuator)\n\n*This API requires the following crate features to be activated: `GamepadHapticActuator`*"]
    pub type GamepadHapticActuator;
    # [ wasm_bindgen ( structural , method , getter , js_name = type ) ]
    #[cfg(feature = "GamepadHapticActuatorType")]
    #[doc = "Getter for the `type` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GamepadHapticActuator/type)\n\n*This API requires the following crate features to be activated: `GamepadHapticActuator`, `GamepadHapticActuatorType`*"]
    pub fn type_(this: &GamepadHapticActuator) -> GamepadHapticActuatorType;
    # [ wasm_bindgen ( catch , method , structural , js_name = pulse ) ]
    #[doc = "The `pulse()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GamepadHapticActuator/pulse)\n\n*This API requires the following crate features to be activated: `GamepadHapticActuator`*"]
    pub fn pulse(
        this: &GamepadHapticActuator,
        value: f64,
        duration: f64,
    ) -> Result<::js_sys::Promise, JsValue>;
}
