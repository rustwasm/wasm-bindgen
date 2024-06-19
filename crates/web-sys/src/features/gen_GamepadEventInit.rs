#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = GamepadEventInit)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `GamepadEventInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GamepadEventInit`*"]
    pub type GamepadEventInit;
    #[doc = "Get the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GamepadEventInit`*"]
    #[wasm_bindgen(method, getter = "bubbles")]
    pub fn get_bubbles(this: &GamepadEventInit) -> Option<bool>;
    #[wasm_bindgen(method, setter = "bubbles")]
    fn set_bubbles(this: &GamepadEventInit, val: bool);
    #[doc = "Get the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GamepadEventInit`*"]
    #[wasm_bindgen(method, getter = "cancelable")]
    pub fn get_cancelable(this: &GamepadEventInit) -> Option<bool>;
    #[wasm_bindgen(method, setter = "cancelable")]
    fn set_cancelable(this: &GamepadEventInit, val: bool);
    #[doc = "Get the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GamepadEventInit`*"]
    #[wasm_bindgen(method, getter = "composed")]
    pub fn get_composed(this: &GamepadEventInit) -> Option<bool>;
    #[wasm_bindgen(method, setter = "composed")]
    fn set_composed(this: &GamepadEventInit, val: bool);
    #[cfg(feature = "Gamepad")]
    #[doc = "Get the `gamepad` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Gamepad`, `GamepadEventInit`*"]
    #[wasm_bindgen(method, getter = "gamepad")]
    pub fn get_gamepad(this: &GamepadEventInit) -> Option<Gamepad>;
    #[cfg(feature = "Gamepad")]
    #[wasm_bindgen(method, setter = "gamepad")]
    fn set_gamepad(this: &GamepadEventInit, val: Option<&Gamepad>);
}
impl GamepadEventInit {
    #[doc = "Construct a new `GamepadEventInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GamepadEventInit`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GamepadEventInit`*"]
    pub fn bubbles(&mut self, val: bool) -> &mut Self {
        self.set_bubbles(val);
        self
    }
    #[doc = "Change the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GamepadEventInit`*"]
    pub fn cancelable(&mut self, val: bool) -> &mut Self {
        self.set_cancelable(val);
        self
    }
    #[doc = "Change the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GamepadEventInit`*"]
    pub fn composed(&mut self, val: bool) -> &mut Self {
        self.set_composed(val);
        self
    }
    #[cfg(feature = "Gamepad")]
    #[doc = "Change the `gamepad` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Gamepad`, `GamepadEventInit`*"]
    pub fn gamepad(&mut self, val: Option<&Gamepad>) -> &mut Self {
        self.set_gamepad(val);
        self
    }
}
impl Default for GamepadEventInit {
    fn default() -> Self {
        Self::new()
    }
}
