use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = GamepadHapticActuator , typescript_type = "GamepadHapticActuator" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `GamepadHapticActuator` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GamepadHapticActuator)
    ///
    ///*This API requires the following crate features to be activated: `GamepadHapticActuator`*
    pub type GamepadHapticActuator;

    #[cfg(feature = "GamepadHapticActuatorType")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "GamepadHapticActuator" , js_name = type ) ]
    ///Getter for the `type` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GamepadHapticActuator/type)
    ///
    ///*This API requires the following crate features to be activated: `GamepadHapticActuator`, `GamepadHapticActuatorType`*
    pub fn type_(this: &GamepadHapticActuator) -> GamepadHapticActuatorType;

    # [ wasm_bindgen ( catch , method , structural , js_class = "GamepadHapticActuator" , js_name = pulse ) ]
    ///The `pulse()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GamepadHapticActuator/pulse)
    ///
    ///*This API requires the following crate features to be activated: `GamepadHapticActuator`*
    pub fn pulse(
        this: &GamepadHapticActuator,
        value: f64,
        duration: f64,
    ) -> Result<::js_sys::Promise, JsValue>;

}
