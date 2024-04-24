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
    #[wasm_bindgen(method, getter = "bubbles")]
    fn bubbles_shim(this: &GamepadButtonEventInit) -> bool;
    #[wasm_bindgen(method, setter = "bubbles")]
    fn set_bubbles_shim(this: &GamepadButtonEventInit, val: bool);
    #[wasm_bindgen(method, getter = "cancelable")]
    fn cancelable_shim(this: &GamepadButtonEventInit) -> bool;
    #[wasm_bindgen(method, setter = "cancelable")]
    fn set_cancelable_shim(this: &GamepadButtonEventInit, val: bool);
    #[wasm_bindgen(method, getter = "composed")]
    fn composed_shim(this: &GamepadButtonEventInit) -> bool;
    #[wasm_bindgen(method, setter = "composed")]
    fn set_composed_shim(this: &GamepadButtonEventInit, val: bool);
    #[cfg(feature = "Gamepad")]
    #[wasm_bindgen(method, getter = "gamepad")]
    fn gamepad_shim(this: &GamepadButtonEventInit) -> Option<&Gamepad>;
    #[cfg(feature = "Gamepad")]
    #[wasm_bindgen(method, setter = "gamepad")]
    fn set_gamepad_shim(this: &GamepadButtonEventInit, val: Option<&Gamepad>);
    #[wasm_bindgen(method, getter = "button")]
    fn button_shim(this: &GamepadButtonEventInit) -> u32;
    #[wasm_bindgen(method, setter = "button")]
    fn set_button_shim(this: &GamepadButtonEventInit, val: u32);
}
#[doc = "The trait to access properties on the `GamepadButtonEventInit` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `GamepadButtonEventInit`*"]
pub trait GamepadButtonEventInitGetters {
    #[doc = "Get the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GamepadButtonEventInit`*"]
    fn bubbles(&self) -> bool;
    #[doc = "Get the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GamepadButtonEventInit`*"]
    fn cancelable(&self) -> bool;
    #[doc = "Get the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GamepadButtonEventInit`*"]
    fn composed(&self) -> bool;
    #[cfg(feature = "Gamepad")]
    #[doc = "Get the `gamepad` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Gamepad`, `GamepadButtonEventInit`*"]
    fn gamepad(&self) -> Option<&Gamepad>;
    #[doc = "Get the `button` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GamepadButtonEventInit`*"]
    fn button(&self) -> u32;
}
impl GamepadButtonEventInitGetters for GamepadButtonEventInit {
    fn bubbles(&self) -> bool {
        self.bubbles_shim()
    }
    fn cancelable(&self) -> bool {
        self.cancelable_shim()
    }
    fn composed(&self) -> bool {
        self.composed_shim()
    }
    #[cfg(feature = "Gamepad")]
    fn gamepad(&self) -> Option<&Gamepad> {
        self.gamepad_shim()
    }
    fn button(&self) -> u32 {
        self.button_shim()
    }
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
    #[doc = "Change the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GamepadButtonEventInit`*"]
    pub fn bubbles(&mut self, val: bool) -> &mut Self {
        self.set_bubbles_shim(val);
        self
    }
    #[doc = "Change the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GamepadButtonEventInit`*"]
    pub fn cancelable(&mut self, val: bool) -> &mut Self {
        self.set_cancelable_shim(val);
        self
    }
    #[doc = "Change the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GamepadButtonEventInit`*"]
    pub fn composed(&mut self, val: bool) -> &mut Self {
        self.set_composed_shim(val);
        self
    }
    #[cfg(feature = "Gamepad")]
    #[doc = "Change the `gamepad` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Gamepad`, `GamepadButtonEventInit`*"]
    pub fn gamepad(&mut self, val: Option<&Gamepad>) -> &mut Self {
        self.set_gamepad_shim(val);
        self
    }
    #[doc = "Change the `button` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GamepadButtonEventInit`*"]
    pub fn button(&mut self, val: u32) -> &mut Self {
        self.set_button_shim(val);
        self
    }
}
impl Default for GamepadButtonEventInit {
    fn default() -> Self {
        Self::new()
    }
}
