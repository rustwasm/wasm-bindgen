#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = GamepadEffectParameters)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `GamepadEffectParameters` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GamepadEffectParameters`*"]
    pub type GamepadEffectParameters;
    #[doc = "Get the `duration` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GamepadEffectParameters`*"]
    #[wasm_bindgen(method, getter = "duration")]
    pub fn get_duration(this: &GamepadEffectParameters) -> Option<f64>;
    #[doc = "Change the `duration` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GamepadEffectParameters`*"]
    #[wasm_bindgen(method, setter = "duration")]
    pub fn set_duration(this: &GamepadEffectParameters, val: f64);
    #[doc = "Get the `leftTrigger` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GamepadEffectParameters`*"]
    #[wasm_bindgen(method, getter = "leftTrigger")]
    pub fn get_left_trigger(this: &GamepadEffectParameters) -> Option<f64>;
    #[doc = "Change the `leftTrigger` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GamepadEffectParameters`*"]
    #[wasm_bindgen(method, setter = "leftTrigger")]
    pub fn set_left_trigger(this: &GamepadEffectParameters, val: f64);
    #[doc = "Get the `rightTrigger` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GamepadEffectParameters`*"]
    #[wasm_bindgen(method, getter = "rightTrigger")]
    pub fn get_right_trigger(this: &GamepadEffectParameters) -> Option<f64>;
    #[doc = "Change the `rightTrigger` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GamepadEffectParameters`*"]
    #[wasm_bindgen(method, setter = "rightTrigger")]
    pub fn set_right_trigger(this: &GamepadEffectParameters, val: f64);
    #[doc = "Get the `startDelay` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GamepadEffectParameters`*"]
    #[wasm_bindgen(method, getter = "startDelay")]
    pub fn get_start_delay(this: &GamepadEffectParameters) -> Option<f64>;
    #[doc = "Change the `startDelay` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GamepadEffectParameters`*"]
    #[wasm_bindgen(method, setter = "startDelay")]
    pub fn set_start_delay(this: &GamepadEffectParameters, val: f64);
    #[doc = "Get the `strongMagnitude` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GamepadEffectParameters`*"]
    #[wasm_bindgen(method, getter = "strongMagnitude")]
    pub fn get_strong_magnitude(this: &GamepadEffectParameters) -> Option<f64>;
    #[doc = "Change the `strongMagnitude` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GamepadEffectParameters`*"]
    #[wasm_bindgen(method, setter = "strongMagnitude")]
    pub fn set_strong_magnitude(this: &GamepadEffectParameters, val: f64);
    #[doc = "Get the `weakMagnitude` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GamepadEffectParameters`*"]
    #[wasm_bindgen(method, getter = "weakMagnitude")]
    pub fn get_weak_magnitude(this: &GamepadEffectParameters) -> Option<f64>;
    #[doc = "Change the `weakMagnitude` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GamepadEffectParameters`*"]
    #[wasm_bindgen(method, setter = "weakMagnitude")]
    pub fn set_weak_magnitude(this: &GamepadEffectParameters, val: f64);
}
impl GamepadEffectParameters {
    #[doc = "Construct a new `GamepadEffectParameters`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GamepadEffectParameters`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_duration()` instead."]
    pub fn duration(&mut self, val: f64) -> &mut Self {
        self.set_duration(val);
        self
    }
    #[deprecated = "Use `set_left_trigger()` instead."]
    pub fn left_trigger(&mut self, val: f64) -> &mut Self {
        self.set_left_trigger(val);
        self
    }
    #[deprecated = "Use `set_right_trigger()` instead."]
    pub fn right_trigger(&mut self, val: f64) -> &mut Self {
        self.set_right_trigger(val);
        self
    }
    #[deprecated = "Use `set_start_delay()` instead."]
    pub fn start_delay(&mut self, val: f64) -> &mut Self {
        self.set_start_delay(val);
        self
    }
    #[deprecated = "Use `set_strong_magnitude()` instead."]
    pub fn strong_magnitude(&mut self, val: f64) -> &mut Self {
        self.set_strong_magnitude(val);
        self
    }
    #[deprecated = "Use `set_weak_magnitude()` instead."]
    pub fn weak_magnitude(&mut self, val: f64) -> &mut Self {
        self.set_weak_magnitude(val);
        self
    }
}
impl Default for GamepadEffectParameters {
    fn default() -> Self {
        Self::new()
    }
}
