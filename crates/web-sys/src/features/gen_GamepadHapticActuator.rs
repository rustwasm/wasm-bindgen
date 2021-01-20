#![allow(unused_imports)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = GamepadHapticActuator , typescript_type = "GamepadHapticActuator")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `GamepadHapticActuator` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GamepadHapticActuator)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GamepadHapticActuator`*"]
    pub type GamepadHapticActuator;
    #[cfg(feature = "GamepadHapticActuatorType")]
    # [wasm_bindgen (structural , method , getter , js_class = "GamepadHapticActuator" , js_name = type)]
    #[doc = "Getter for the `type` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GamepadHapticActuator/type)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GamepadHapticActuator`, `GamepadHapticActuatorType`*"]
    pub fn type_(this: &GamepadHapticActuator) -> GamepadHapticActuatorType;
    # [wasm_bindgen (catch , method , structural , js_class = "GamepadHapticActuator" , js_name = pulse)]
    #[doc = "The `pulse()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GamepadHapticActuator/pulse)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GamepadHapticActuator`*"]
    #[doc = ""]
    #[doc = "Return value: While the Promise of the successful result can produce any JsValue as far as the type system is concerned, practically it is expected to contain a <code>[bool]</code>. It can be converted like `<code>let result: [bool] = result?.await.into();</code>."]
    pub fn pulse(
        this: &GamepadHapticActuator,
        value: f64,
        duration: f64,
    ) -> Result<::js_sys::Promise, JsValue>;
}
