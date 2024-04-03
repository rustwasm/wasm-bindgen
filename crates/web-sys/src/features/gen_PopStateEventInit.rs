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
    #[wasm_bindgen(method, setter = "bubbles")]
    fn bubbles_shim(this: &PopStateEventInit, val: bool);
    #[wasm_bindgen(method, setter = "cancelable")]
    fn cancelable_shim(this: &PopStateEventInit, val: bool);
    #[wasm_bindgen(method, setter = "composed")]
    fn composed_shim(this: &PopStateEventInit, val: bool);
    #[wasm_bindgen(method, setter = "state")]
    fn state_shim(this: &PopStateEventInit, val: &::wasm_bindgen::JsValue);
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
        self.bubbles_shim(val);
        self
    }
    #[doc = "Change the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PopStateEventInit`*"]
    pub fn cancelable(&mut self, val: bool) -> &mut Self {
        self.cancelable_shim(val);
        self
    }
    #[doc = "Change the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PopStateEventInit`*"]
    pub fn composed(&mut self, val: bool) -> &mut Self {
        self.composed_shim(val);
        self
    }
    #[doc = "Change the `state` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PopStateEventInit`*"]
    pub fn state(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.state_shim(val);
        self
    }
}
impl Default for PopStateEventInit {
    fn default() -> Self {
        Self::new()
    }
}
