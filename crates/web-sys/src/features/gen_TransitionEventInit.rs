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
    #[doc = "Get the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TransitionEventInit`*"]
    #[wasm_bindgen(method, getter = "bubbles")]
    pub fn get_bubbles(this: &TransitionEventInit) -> Option<bool>;
    #[wasm_bindgen(method, setter = "bubbles")]
    fn set_bubbles(this: &TransitionEventInit, val: bool);
    #[doc = "Get the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TransitionEventInit`*"]
    #[wasm_bindgen(method, getter = "cancelable")]
    pub fn get_cancelable(this: &TransitionEventInit) -> Option<bool>;
    #[wasm_bindgen(method, setter = "cancelable")]
    fn set_cancelable(this: &TransitionEventInit, val: bool);
    #[doc = "Get the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TransitionEventInit`*"]
    #[wasm_bindgen(method, getter = "composed")]
    pub fn get_composed(this: &TransitionEventInit) -> Option<bool>;
    #[wasm_bindgen(method, setter = "composed")]
    fn set_composed(this: &TransitionEventInit, val: bool);
    #[doc = "Get the `elapsedTime` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TransitionEventInit`*"]
    #[wasm_bindgen(method, getter = "elapsedTime")]
    pub fn get_elapsed_time(this: &TransitionEventInit) -> Option<f32>;
    #[wasm_bindgen(method, setter = "elapsedTime")]
    fn set_elapsed_time(this: &TransitionEventInit, val: f32);
    #[doc = "Get the `propertyName` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TransitionEventInit`*"]
    #[wasm_bindgen(method, getter = "propertyName")]
    pub fn get_property_name(this: &TransitionEventInit) -> Option<String>;
    #[wasm_bindgen(method, setter = "propertyName")]
    fn set_property_name(this: &TransitionEventInit, val: &str);
    #[doc = "Get the `pseudoElement` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TransitionEventInit`*"]
    #[wasm_bindgen(method, getter = "pseudoElement")]
    pub fn get_pseudo_element(this: &TransitionEventInit) -> Option<String>;
    #[wasm_bindgen(method, setter = "pseudoElement")]
    fn set_pseudo_element(this: &TransitionEventInit, val: &str);
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
        self.set_bubbles(val);
        self
    }
    #[doc = "Change the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TransitionEventInit`*"]
    pub fn cancelable(&mut self, val: bool) -> &mut Self {
        self.set_cancelable(val);
        self
    }
    #[doc = "Change the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TransitionEventInit`*"]
    pub fn composed(&mut self, val: bool) -> &mut Self {
        self.set_composed(val);
        self
    }
    #[doc = "Change the `elapsedTime` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TransitionEventInit`*"]
    pub fn elapsed_time(&mut self, val: f32) -> &mut Self {
        self.set_elapsed_time(val);
        self
    }
    #[doc = "Change the `propertyName` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TransitionEventInit`*"]
    pub fn property_name(&mut self, val: &str) -> &mut Self {
        self.set_property_name(val);
        self
    }
    #[doc = "Change the `pseudoElement` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TransitionEventInit`*"]
    pub fn pseudo_element(&mut self, val: &str) -> &mut Self {
        self.set_pseudo_element(val);
        self
    }
}
impl Default for TransitionEventInit {
    fn default() -> Self {
        Self::new()
    }
}
