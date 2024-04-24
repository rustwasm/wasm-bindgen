#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = PopStateEventInit)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `PopStateEventInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PopStateEventInit`*"]
    pub type PopStateEventInit;
    #[wasm_bindgen(method, getter = "bubbles")]
    fn bubbles_shim(this: &PopStateEventInit) -> bool;
    #[wasm_bindgen(method, setter = "bubbles")]
    fn set_bubbles_shim(this: &PopStateEventInit, val: bool);
    #[wasm_bindgen(method, getter = "cancelable")]
    fn cancelable_shim(this: &PopStateEventInit) -> bool;
    #[wasm_bindgen(method, setter = "cancelable")]
    fn set_cancelable_shim(this: &PopStateEventInit, val: bool);
    #[wasm_bindgen(method, getter = "composed")]
    fn composed_shim(this: &PopStateEventInit) -> bool;
    #[wasm_bindgen(method, setter = "composed")]
    fn set_composed_shim(this: &PopStateEventInit, val: bool);
    #[wasm_bindgen(method, getter = "state")]
    fn state_shim(this: &PopStateEventInit) -> ::wasm_bindgen::JsValue;
    #[wasm_bindgen(method, setter = "state")]
    fn set_state_shim(this: &PopStateEventInit, val: &::wasm_bindgen::JsValue);
}
#[doc = "The trait to access properties on the `PopStateEventInit` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `PopStateEventInit`*"]
pub trait PopStateEventInitGetters {
    #[doc = "Get the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PopStateEventInit`*"]
    fn bubbles(&self) -> bool;
    #[doc = "Get the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PopStateEventInit`*"]
    fn cancelable(&self) -> bool;
    #[doc = "Get the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PopStateEventInit`*"]
    fn composed(&self) -> bool;
    #[doc = "Get the `state` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PopStateEventInit`*"]
    fn state(&self) -> ::wasm_bindgen::JsValue;
}
impl PopStateEventInitGetters for PopStateEventInit {
    fn bubbles(&self) -> bool {
        self.bubbles_shim()
    }
    fn cancelable(&self) -> bool {
        self.cancelable_shim()
    }
    fn composed(&self) -> bool {
        self.composed_shim()
    }
    fn state(&self) -> ::wasm_bindgen::JsValue {
        self.state_shim()
    }
}
impl PopStateEventInit {
    #[doc = "Construct a new `PopStateEventInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PopStateEventInit`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PopStateEventInit`*"]
    pub fn bubbles(&mut self, val: bool) -> &mut Self {
        self.set_bubbles_shim(val);
        self
    }
    #[doc = "Change the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PopStateEventInit`*"]
    pub fn cancelable(&mut self, val: bool) -> &mut Self {
        self.set_cancelable_shim(val);
        self
    }
    #[doc = "Change the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PopStateEventInit`*"]
    pub fn composed(&mut self, val: bool) -> &mut Self {
        self.set_composed_shim(val);
        self
    }
    #[doc = "Change the `state` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PopStateEventInit`*"]
    pub fn state(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_state_shim(val);
        self
    }
}
impl Default for PopStateEventInit {
    fn default() -> Self {
        Self::new()
    }
}
