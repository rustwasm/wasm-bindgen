#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = CloseEventInit)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `CloseEventInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CloseEventInit`*"]
    pub type CloseEventInit;
    #[wasm_bindgen(method, setter = "bubbles")]
    fn bubbles_shim(this: &CloseEventInit, val: bool);
    #[wasm_bindgen(method, setter = "cancelable")]
    fn cancelable_shim(this: &CloseEventInit, val: bool);
    #[wasm_bindgen(method, setter = "composed")]
    fn composed_shim(this: &CloseEventInit, val: bool);
    #[wasm_bindgen(method, setter = "code")]
    fn code_shim(this: &CloseEventInit, val: u16);
    #[wasm_bindgen(method, setter = "reason")]
    fn reason_shim(this: &CloseEventInit, val: &str);
    #[wasm_bindgen(method, setter = "wasClean")]
    fn was_clean_shim(this: &CloseEventInit, val: bool);
}
impl CloseEventInit {
    #[doc = "Construct a new `CloseEventInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CloseEventInit`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CloseEventInit`*"]
    pub fn bubbles(&mut self, val: bool) -> &mut Self {
        self.bubbles_shim(val);
        self
    }
    #[doc = "Change the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CloseEventInit`*"]
    pub fn cancelable(&mut self, val: bool) -> &mut Self {
        self.cancelable_shim(val);
        self
    }
    #[doc = "Change the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CloseEventInit`*"]
    pub fn composed(&mut self, val: bool) -> &mut Self {
        self.composed_shim(val);
        self
    }
    #[doc = "Change the `code` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CloseEventInit`*"]
    pub fn code(&mut self, val: u16) -> &mut Self {
        self.code_shim(val);
        self
    }
    #[doc = "Change the `reason` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CloseEventInit`*"]
    pub fn reason(&mut self, val: &str) -> &mut Self {
        self.reason_shim(val);
        self
    }
    #[doc = "Change the `wasClean` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CloseEventInit`*"]
    pub fn was_clean(&mut self, val: bool) -> &mut Self {
        self.was_clean_shim(val);
        self
    }
}
impl Default for CloseEventInit {
    fn default() -> Self {
        Self::new()
    }
}
