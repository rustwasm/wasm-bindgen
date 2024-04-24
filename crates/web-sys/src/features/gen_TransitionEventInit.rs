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
    #[wasm_bindgen(method, getter = "bubbles")]
    fn bubbles_shim(this: &TransitionEventInit) -> bool;
    #[wasm_bindgen(method, setter = "bubbles")]
    fn set_bubbles_shim(this: &TransitionEventInit, val: bool);
    #[wasm_bindgen(method, getter = "cancelable")]
    fn cancelable_shim(this: &TransitionEventInit) -> bool;
    #[wasm_bindgen(method, setter = "cancelable")]
    fn set_cancelable_shim(this: &TransitionEventInit, val: bool);
    #[wasm_bindgen(method, getter = "composed")]
    fn composed_shim(this: &TransitionEventInit) -> bool;
    #[wasm_bindgen(method, setter = "composed")]
    fn set_composed_shim(this: &TransitionEventInit, val: bool);
    #[wasm_bindgen(method, getter = "elapsedTime")]
    fn elapsed_time_shim(this: &TransitionEventInit) -> f32;
    #[wasm_bindgen(method, setter = "elapsedTime")]
    fn set_elapsed_time_shim(this: &TransitionEventInit, val: f32);
    #[wasm_bindgen(method, getter = "propertyName")]
    fn property_name_shim(this: &TransitionEventInit) -> String;
    #[wasm_bindgen(method, setter = "propertyName")]
    fn set_property_name_shim(this: &TransitionEventInit, val: &str);
    #[wasm_bindgen(method, getter = "pseudoElement")]
    fn pseudo_element_shim(this: &TransitionEventInit) -> String;
    #[wasm_bindgen(method, setter = "pseudoElement")]
    fn set_pseudo_element_shim(this: &TransitionEventInit, val: &str);
}
#[doc = "The trait to access properties on the `TransitionEventInit` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `TransitionEventInit`*"]
pub trait TransitionEventInitGetters {
    #[doc = "Get the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TransitionEventInit`*"]
    fn bubbles(&self) -> bool;
    #[doc = "Get the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TransitionEventInit`*"]
    fn cancelable(&self) -> bool;
    #[doc = "Get the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TransitionEventInit`*"]
    fn composed(&self) -> bool;
    #[doc = "Get the `elapsedTime` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TransitionEventInit`*"]
    fn elapsed_time(&self) -> f32;
    #[doc = "Get the `propertyName` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TransitionEventInit`*"]
    fn property_name(&self) -> String;
    #[doc = "Get the `pseudoElement` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TransitionEventInit`*"]
    fn pseudo_element(&self) -> String;
}
impl TransitionEventInitGetters for TransitionEventInit {
    fn bubbles(&self) -> bool {
        self.bubbles_shim()
    }
    fn cancelable(&self) -> bool {
        self.cancelable_shim()
    }
    fn composed(&self) -> bool {
        self.composed_shim()
    }
    fn elapsed_time(&self) -> f32 {
        self.elapsed_time_shim()
    }
    fn property_name(&self) -> String {
        self.property_name_shim()
    }
    fn pseudo_element(&self) -> String {
        self.pseudo_element_shim()
    }
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
        self.set_bubbles_shim(val);
        self
    }
    #[doc = "Change the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TransitionEventInit`*"]
    pub fn cancelable(&mut self, val: bool) -> &mut Self {
        self.set_cancelable_shim(val);
        self
    }
    #[doc = "Change the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TransitionEventInit`*"]
    pub fn composed(&mut self, val: bool) -> &mut Self {
        self.set_composed_shim(val);
        self
    }
    #[doc = "Change the `elapsedTime` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TransitionEventInit`*"]
    pub fn elapsed_time(&mut self, val: f32) -> &mut Self {
        self.set_elapsed_time_shim(val);
        self
    }
    #[doc = "Change the `propertyName` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TransitionEventInit`*"]
    pub fn property_name(&mut self, val: &str) -> &mut Self {
        self.set_property_name_shim(val);
        self
    }
    #[doc = "Change the `pseudoElement` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TransitionEventInit`*"]
    pub fn pseudo_element(&mut self, val: &str) -> &mut Self {
        self.set_pseudo_element_shim(val);
        self
    }
}
impl Default for TransitionEventInit {
    fn default() -> Self {
        Self::new()
    }
}
