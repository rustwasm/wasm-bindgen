#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = PromiseRejectionEventInit)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `PromiseRejectionEventInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PromiseRejectionEventInit`*"]
    pub type PromiseRejectionEventInit;
    #[wasm_bindgen(method, setter = "bubbles")]
    fn bubbles_shim(this: &PromiseRejectionEventInit, val: bool);
    #[wasm_bindgen(method, setter = "cancelable")]
    fn cancelable_shim(this: &PromiseRejectionEventInit, val: bool);
    #[wasm_bindgen(method, setter = "composed")]
    fn composed_shim(this: &PromiseRejectionEventInit, val: bool);
    #[wasm_bindgen(method, setter = "promise")]
    fn promise_shim(this: &PromiseRejectionEventInit, val: &::js_sys::Promise);
    #[wasm_bindgen(method, setter = "reason")]
    fn reason_shim(this: &PromiseRejectionEventInit, val: &::wasm_bindgen::JsValue);
}
impl PromiseRejectionEventInit {
    #[doc = "Construct a new `PromiseRejectionEventInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PromiseRejectionEventInit`*"]
    pub fn new(promise: &::js_sys::Promise) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.promise(promise);
        ret
    }
    #[doc = "Change the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PromiseRejectionEventInit`*"]
    pub fn bubbles(&mut self, val: bool) -> &mut Self {
        self.bubbles_shim(val);
        self
    }
    #[doc = "Change the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PromiseRejectionEventInit`*"]
    pub fn cancelable(&mut self, val: bool) -> &mut Self {
        self.cancelable_shim(val);
        self
    }
    #[doc = "Change the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PromiseRejectionEventInit`*"]
    pub fn composed(&mut self, val: bool) -> &mut Self {
        self.composed_shim(val);
        self
    }
    #[doc = "Change the `promise` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PromiseRejectionEventInit`*"]
    pub fn promise(&mut self, val: &::js_sys::Promise) -> &mut Self {
        self.promise_shim(val);
        self
    }
    #[doc = "Change the `reason` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PromiseRejectionEventInit`*"]
    pub fn reason(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.reason_shim(val);
        self
    }
}
