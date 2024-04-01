#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = TransitionEventInit)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `TransitionEventInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TransitionEventInit`*"]
    pub type TransitionEventInit;
    #[wasm_bindgen(method, setter = "bubbles")]
    fn bubbles_shim(this: &TransitionEventInit, val: bool);
    #[wasm_bindgen(method, setter = "cancelable")]
    fn cancelable_shim(this: &TransitionEventInit, val: bool);
    #[wasm_bindgen(method, setter = "composed")]
    fn composed_shim(this: &TransitionEventInit, val: bool);
    #[wasm_bindgen(method, setter = "elapsedTime")]
    fn elapsed_time_shim(this: &TransitionEventInit, val: f32);
    #[wasm_bindgen(method, setter = "propertyName")]
    fn property_name_shim(this: &TransitionEventInit, val: &str);
    #[wasm_bindgen(method, setter = "pseudoElement")]
    fn pseudo_element_shim(this: &TransitionEventInit, val: &str);
}
impl TransitionEventInit {
    #[doc = "Construct a new `TransitionEventInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TransitionEventInit`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TransitionEventInit`*"]
    pub fn bubbles(&mut self, val: bool) -> &mut Self {
        self.bubbles_shim(val);
        self
    }
    #[doc = "Change the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TransitionEventInit`*"]
    pub fn cancelable(&mut self, val: bool) -> &mut Self {
        self.cancelable_shim(val);
        self
    }
    #[doc = "Change the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TransitionEventInit`*"]
    pub fn composed(&mut self, val: bool) -> &mut Self {
        self.composed_shim(val);
        self
    }
    #[doc = "Change the `elapsedTime` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TransitionEventInit`*"]
    pub fn elapsed_time(&mut self, val: f32) -> &mut Self {
        self.elapsed_time_shim(val);
        self
    }
    #[doc = "Change the `propertyName` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TransitionEventInit`*"]
    pub fn property_name(&mut self, val: &str) -> &mut Self {
        self.property_name_shim(val);
        self
    }
    #[doc = "Change the `pseudoElement` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TransitionEventInit`*"]
    pub fn pseudo_element(&mut self, val: &str) -> &mut Self {
        self.pseudo_element_shim(val);
        self
    }
}
impl Default for TransitionEventInit {
    fn default() -> Self {
        Self::new()
    }
}
