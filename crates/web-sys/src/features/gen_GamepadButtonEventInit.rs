#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = GamepadButtonEventInit)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `GamepadButtonEventInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GamepadButtonEventInit`*"]
    pub type GamepadButtonEventInit;
    #[doc = "Get the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GamepadButtonEventInit`*"]
    #[wasm_bindgen(method, getter = "bubbles")]
    pub fn get_bubbles(this: &GamepadButtonEventInit) -> Option<bool>;
    #[doc = "Change the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GamepadButtonEventInit`*"]
    #[wasm_bindgen(method, setter = "bubbles")]
    pub fn set_bubbles(this: &GamepadButtonEventInit, val: bool);
    #[doc = "Get the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GamepadButtonEventInit`*"]
    #[wasm_bindgen(method, getter = "cancelable")]
    pub fn get_cancelable(this: &GamepadButtonEventInit) -> Option<bool>;
    #[doc = "Change the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GamepadButtonEventInit`*"]
    #[wasm_bindgen(method, setter = "cancelable")]
    pub fn set_cancelable(this: &GamepadButtonEventInit, val: bool);
    #[doc = "Get the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GamepadButtonEventInit`*"]
    #[wasm_bindgen(method, getter = "composed")]
    pub fn get_composed(this: &GamepadButtonEventInit) -> Option<bool>;
    #[doc = "Change the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GamepadButtonEventInit`*"]
    #[wasm_bindgen(method, setter = "composed")]
    pub fn set_composed(this: &GamepadButtonEventInit, val: bool);
    #[cfg(feature = "Gamepad")]
    #[doc = "Get the `gamepad` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Gamepad`, `GamepadButtonEventInit`*"]
    #[wasm_bindgen(method, getter = "gamepad")]
    pub fn get_gamepad(this: &GamepadButtonEventInit) -> Option<Gamepad>;
    #[cfg(feature = "Gamepad")]
    #[doc = "Change the `gamepad` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Gamepad`, `GamepadButtonEventInit`*"]
    #[wasm_bindgen(method, setter = "gamepad")]
    pub fn set_gamepad(this: &GamepadButtonEventInit, val: Option<&Gamepad>);
    #[doc = "Get the `button` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GamepadButtonEventInit`*"]
    #[wasm_bindgen(method, getter = "button")]
    pub fn get_button(this: &GamepadButtonEventInit) -> Option<u32>;
    #[doc = "Change the `button` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GamepadButtonEventInit`*"]
    #[wasm_bindgen(method, setter = "button")]
    pub fn set_button(this: &GamepadButtonEventInit, val: u32);
}
impl GamepadButtonEventInit {
    #[doc = "Construct a new `GamepadButtonEventInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GamepadButtonEventInit`*"]
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
    #[deprecated = "Use `set_button()` instead."]
    pub fn button(&mut self, val: u32) -> &mut Self {
        self.set_button(val);
        self
    }
}
impl Default for GamepadButtonEventInit {
    fn default() -> Self {
        Self::new()
    }
}
