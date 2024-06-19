#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = GamepadAxisMoveEventInit)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `GamepadAxisMoveEventInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GamepadAxisMoveEventInit`*"]
    pub type GamepadAxisMoveEventInit;
    #[doc = "Get the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GamepadAxisMoveEventInit`*"]
    #[wasm_bindgen(method, getter = "bubbles")]
    pub fn get_bubbles(this: &GamepadAxisMoveEventInit) -> Option<bool>;
    #[doc = "Change the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GamepadAxisMoveEventInit`*"]
    #[wasm_bindgen(method, setter = "bubbles")]
    pub fn set_bubbles(this: &GamepadAxisMoveEventInit, val: bool);
    #[doc = "Get the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GamepadAxisMoveEventInit`*"]
    #[wasm_bindgen(method, getter = "cancelable")]
    pub fn get_cancelable(this: &GamepadAxisMoveEventInit) -> Option<bool>;
    #[doc = "Change the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GamepadAxisMoveEventInit`*"]
    #[wasm_bindgen(method, setter = "cancelable")]
    pub fn set_cancelable(this: &GamepadAxisMoveEventInit, val: bool);
    #[doc = "Get the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GamepadAxisMoveEventInit`*"]
    #[wasm_bindgen(method, getter = "composed")]
    pub fn get_composed(this: &GamepadAxisMoveEventInit) -> Option<bool>;
    #[doc = "Change the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GamepadAxisMoveEventInit`*"]
    #[wasm_bindgen(method, setter = "composed")]
    pub fn set_composed(this: &GamepadAxisMoveEventInit, val: bool);
    #[cfg(feature = "Gamepad")]
    #[doc = "Get the `gamepad` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Gamepad`, `GamepadAxisMoveEventInit`*"]
    #[wasm_bindgen(method, getter = "gamepad")]
    pub fn get_gamepad(this: &GamepadAxisMoveEventInit) -> Option<Gamepad>;
    #[cfg(feature = "Gamepad")]
    #[doc = "Change the `gamepad` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Gamepad`, `GamepadAxisMoveEventInit`*"]
    #[wasm_bindgen(method, setter = "gamepad")]
    pub fn set_gamepad(this: &GamepadAxisMoveEventInit, val: Option<&Gamepad>);
    #[doc = "Get the `axis` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GamepadAxisMoveEventInit`*"]
    #[wasm_bindgen(method, getter = "axis")]
    pub fn get_axis(this: &GamepadAxisMoveEventInit) -> Option<u32>;
    #[doc = "Change the `axis` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GamepadAxisMoveEventInit`*"]
    #[wasm_bindgen(method, setter = "axis")]
    pub fn set_axis(this: &GamepadAxisMoveEventInit, val: u32);
    #[doc = "Get the `value` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GamepadAxisMoveEventInit`*"]
    #[wasm_bindgen(method, getter = "value")]
    pub fn get_value(this: &GamepadAxisMoveEventInit) -> Option<f64>;
    #[doc = "Change the `value` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GamepadAxisMoveEventInit`*"]
    #[wasm_bindgen(method, setter = "value")]
    pub fn set_value(this: &GamepadAxisMoveEventInit, val: f64);
}
impl GamepadAxisMoveEventInit {
    #[doc = "Construct a new `GamepadAxisMoveEventInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GamepadAxisMoveEventInit`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_bubbles()` instead."]
    pub fn bubbles(&mut self, val: bool) -> &mut Self {
        self.set_bubbles(val);
        self
    }
    #[deprecated = "Use `set_cancelable()` instead."]
    pub fn cancelable(&mut self, val: bool) -> &mut Self {
        self.set_cancelable(val);
        self
    }
    #[deprecated = "Use `set_composed()` instead."]
    pub fn composed(&mut self, val: bool) -> &mut Self {
        self.set_composed(val);
        self
    }
    #[cfg(feature = "Gamepad")]
    #[deprecated = "Use `set_gamepad()` instead."]
    pub fn gamepad(&mut self, val: Option<&Gamepad>) -> &mut Self {
        self.set_gamepad(val);
        self
    }
    #[deprecated = "Use `set_axis()` instead."]
    pub fn axis(&mut self, val: u32) -> &mut Self {
        self.set_axis(val);
        self
    }
    #[deprecated = "Use `set_value()` instead."]
    pub fn value(&mut self, val: f64) -> &mut Self {
        self.set_value(val);
        self
    }
}
impl Default for GamepadAxisMoveEventInit {
    fn default() -> Self {
        Self::new()
    }
}
