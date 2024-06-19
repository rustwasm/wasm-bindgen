#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = UserProximityEventInit)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `UserProximityEventInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UserProximityEventInit`*"]
    pub type UserProximityEventInit;
    #[doc = "Get the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UserProximityEventInit`*"]
    #[wasm_bindgen(method, getter = "bubbles")]
    pub fn get_bubbles(this: &UserProximityEventInit) -> Option<bool>;
    #[wasm_bindgen(method, setter = "bubbles")]
    fn set_bubbles(this: &UserProximityEventInit, val: bool);
    #[doc = "Get the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UserProximityEventInit`*"]
    #[wasm_bindgen(method, getter = "cancelable")]
    pub fn get_cancelable(this: &UserProximityEventInit) -> Option<bool>;
    #[wasm_bindgen(method, setter = "cancelable")]
    fn set_cancelable(this: &UserProximityEventInit, val: bool);
    #[doc = "Get the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UserProximityEventInit`*"]
    #[wasm_bindgen(method, getter = "composed")]
    pub fn get_composed(this: &UserProximityEventInit) -> Option<bool>;
    #[wasm_bindgen(method, setter = "composed")]
    fn set_composed(this: &UserProximityEventInit, val: bool);
    #[doc = "Get the `near` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UserProximityEventInit`*"]
    #[wasm_bindgen(method, getter = "near")]
    pub fn get_near(this: &UserProximityEventInit) -> Option<bool>;
    #[wasm_bindgen(method, setter = "near")]
    fn set_near(this: &UserProximityEventInit, val: bool);
}
impl UserProximityEventInit {
    #[doc = "Construct a new `UserProximityEventInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UserProximityEventInit`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UserProximityEventInit`*"]
    pub fn bubbles(&mut self, val: bool) -> &mut Self {
        self.set_bubbles(val);
        self
    }
    #[doc = "Change the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UserProximityEventInit`*"]
    pub fn cancelable(&mut self, val: bool) -> &mut Self {
        self.set_cancelable(val);
        self
    }
    #[doc = "Change the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UserProximityEventInit`*"]
    pub fn composed(&mut self, val: bool) -> &mut Self {
        self.set_composed(val);
        self
    }
    #[doc = "Change the `near` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UserProximityEventInit`*"]
    pub fn near(&mut self, val: bool) -> &mut Self {
        self.set_near(val);
        self
    }
}
impl Default for UserProximityEventInit {
    fn default() -> Self {
        Self::new()
    }
}
