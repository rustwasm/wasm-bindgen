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
    #[wasm_bindgen(method, getter = "bubbles")]
    fn bubbles_shim(this: &GamepadAxisMoveEventInit) -> bool;
    #[wasm_bindgen(method, setter = "bubbles")]
    fn set_bubbles_shim(this: &GamepadAxisMoveEventInit, val: bool);
    #[wasm_bindgen(method, getter = "cancelable")]
    fn cancelable_shim(this: &GamepadAxisMoveEventInit) -> bool;
    #[wasm_bindgen(method, setter = "cancelable")]
    fn set_cancelable_shim(this: &GamepadAxisMoveEventInit, val: bool);
    #[wasm_bindgen(method, getter = "composed")]
    fn composed_shim(this: &GamepadAxisMoveEventInit) -> bool;
    #[wasm_bindgen(method, setter = "composed")]
    fn set_composed_shim(this: &GamepadAxisMoveEventInit, val: bool);
    #[cfg(feature = "Gamepad")]
    #[wasm_bindgen(method, getter = "gamepad")]
    fn gamepad_shim(this: &GamepadAxisMoveEventInit) -> Option<Gamepad>;
    #[cfg(feature = "Gamepad")]
    #[wasm_bindgen(method, setter = "gamepad")]
    fn set_gamepad_shim(this: &GamepadAxisMoveEventInit, val: Option<&Gamepad>);
    #[wasm_bindgen(method, getter = "axis")]
    fn axis_shim(this: &GamepadAxisMoveEventInit) -> u32;
    #[wasm_bindgen(method, setter = "axis")]
    fn set_axis_shim(this: &GamepadAxisMoveEventInit, val: u32);
    #[wasm_bindgen(method, getter = "value")]
    fn value_shim(this: &GamepadAxisMoveEventInit) -> f64;
    #[wasm_bindgen(method, setter = "value")]
    fn set_value_shim(this: &GamepadAxisMoveEventInit, val: f64);
}
#[doc = "The trait to access properties on the `GamepadAxisMoveEventInit` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `GamepadAxisMoveEventInit`*"]
pub trait GamepadAxisMoveEventInitGetters {
    #[doc = "Get the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GamepadAxisMoveEventInit`*"]
    fn bubbles(&self) -> bool;
    #[doc = "Get the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GamepadAxisMoveEventInit`*"]
    fn cancelable(&self) -> bool;
    #[doc = "Get the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GamepadAxisMoveEventInit`*"]
    fn composed(&self) -> bool;
    #[cfg(feature = "Gamepad")]
    #[doc = "Get the `gamepad` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Gamepad`, `GamepadAxisMoveEventInit`*"]
    fn gamepad(&self) -> Option<Gamepad>;
    #[doc = "Get the `axis` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GamepadAxisMoveEventInit`*"]
    fn axis(&self) -> u32;
    #[doc = "Get the `value` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GamepadAxisMoveEventInit`*"]
    fn value(&self) -> f64;
}
impl GamepadAxisMoveEventInitGetters for GamepadAxisMoveEventInit {
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
    fn gamepad(&self) -> Option<Gamepad> {
        self.gamepad_shim()
    }
    fn axis(&self) -> u32 {
        self.axis_shim()
    }
    fn value(&self) -> f64 {
        self.value_shim()
    }
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
    #[doc = "Change the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GamepadAxisMoveEventInit`*"]
    pub fn bubbles(&mut self, val: bool) -> &mut Self {
        self.set_bubbles_shim(val);
        self
    }
    #[doc = "Change the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GamepadAxisMoveEventInit`*"]
    pub fn cancelable(&mut self, val: bool) -> &mut Self {
        self.set_cancelable_shim(val);
        self
    }
    #[doc = "Change the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GamepadAxisMoveEventInit`*"]
    pub fn composed(&mut self, val: bool) -> &mut Self {
        self.set_composed_shim(val);
        self
    }
    #[cfg(feature = "Gamepad")]
    #[doc = "Change the `gamepad` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Gamepad`, `GamepadAxisMoveEventInit`*"]
    pub fn gamepad(&mut self, val: Option<&Gamepad>) -> &mut Self {
        self.set_gamepad_shim(val);
        self
    }
    #[doc = "Change the `axis` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GamepadAxisMoveEventInit`*"]
    pub fn axis(&mut self, val: u32) -> &mut Self {
        self.set_axis_shim(val);
        self
    }
    #[doc = "Change the `value` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GamepadAxisMoveEventInit`*"]
    pub fn value(&mut self, val: f64) -> &mut Self {
        self.set_value_shim(val);
        self
    }
}
impl Default for GamepadAxisMoveEventInit {
    fn default() -> Self {
        Self::new()
    }
}
