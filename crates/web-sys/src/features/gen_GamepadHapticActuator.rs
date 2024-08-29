#![allow(unused_imports)]
#![allow(clippy::all)]
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
    #[deprecated]
    pub fn type_(this: &GamepadHapticActuator) -> GamepadHapticActuatorType;
    # [wasm_bindgen (structural , method , getter , js_class = "GamepadHapticActuator" , js_name = effects)]
    #[doc = "Getter for the `effects` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GamepadHapticActuator/effects)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GamepadHapticActuator`*"]
    pub fn effects(this: &GamepadHapticActuator) -> ::js_sys::Array;
    #[cfg(feature = "GamepadHapticEffectType")]
    # [wasm_bindgen (method , structural , js_class = "GamepadHapticActuator" , js_name = playEffect)]
    #[doc = "The `playEffect()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GamepadHapticActuator/playEffect)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GamepadHapticActuator`, `GamepadHapticEffectType`*"]
    pub fn play_effect(
        this: &GamepadHapticActuator,
        type_: GamepadHapticEffectType,
    ) -> ::js_sys::Promise;
    #[cfg(all(
        feature = "GamepadEffectParameters",
        feature = "GamepadHapticEffectType",
    ))]
    # [wasm_bindgen (method , structural , js_class = "GamepadHapticActuator" , js_name = playEffect)]
    #[doc = "The `playEffect()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GamepadHapticActuator/playEffect)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GamepadEffectParameters`, `GamepadHapticActuator`, `GamepadHapticEffectType`*"]
    pub fn play_effect_with_params(
        this: &GamepadHapticActuator,
        type_: GamepadHapticEffectType,
        params: &GamepadEffectParameters,
    ) -> ::js_sys::Promise;
    # [wasm_bindgen (catch , method , structural , js_class = "GamepadHapticActuator" , js_name = pulse)]
    #[doc = "The `pulse()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GamepadHapticActuator/pulse)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GamepadHapticActuator`*"]
    #[deprecated]
    pub fn pulse(
        this: &GamepadHapticActuator,
        value: f64,
        duration: f64,
    ) -> Result<::js_sys::Promise, JsValue>;
    # [wasm_bindgen (method , structural , js_class = "GamepadHapticActuator" , js_name = reset)]
    #[doc = "The `reset()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GamepadHapticActuator/reset)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GamepadHapticActuator`*"]
    pub fn reset(this: &GamepadHapticActuator) -> ::js_sys::Promise;
}
