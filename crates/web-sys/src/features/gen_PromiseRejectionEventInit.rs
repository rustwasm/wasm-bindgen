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
    #[wasm_bindgen(method, getter = "bubbles")]
    fn bubbles_shim(this: &PromiseRejectionEventInit) -> bool;
    #[wasm_bindgen(method, setter = "bubbles")]
    fn set_bubbles_shim(this: &PromiseRejectionEventInit, val: bool);
    #[wasm_bindgen(method, getter = "cancelable")]
    fn cancelable_shim(this: &PromiseRejectionEventInit) -> bool;
    #[wasm_bindgen(method, setter = "cancelable")]
    fn set_cancelable_shim(this: &PromiseRejectionEventInit, val: bool);
    #[wasm_bindgen(method, getter = "composed")]
    fn composed_shim(this: &PromiseRejectionEventInit) -> bool;
    #[wasm_bindgen(method, setter = "composed")]
    fn set_composed_shim(this: &PromiseRejectionEventInit, val: bool);
    #[wasm_bindgen(method, getter = "promise")]
    fn promise_shim(this: &PromiseRejectionEventInit) -> ::js_sys::Promise;
    #[wasm_bindgen(method, setter = "promise")]
    fn set_promise_shim(this: &PromiseRejectionEventInit, val: &::js_sys::Promise);
    #[wasm_bindgen(method, getter = "reason")]
    fn reason_shim(this: &PromiseRejectionEventInit) -> ::wasm_bindgen::JsValue;
    #[wasm_bindgen(method, setter = "reason")]
    fn set_reason_shim(this: &PromiseRejectionEventInit, val: &::wasm_bindgen::JsValue);
}
#[doc = "The trait to access properties on the `PromiseRejectionEventInit` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `PromiseRejectionEventInit`*"]
pub trait PromiseRejectionEventInitGetters {
    #[doc = "Get the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PromiseRejectionEventInit`*"]
    fn bubbles(&self) -> bool;
    #[doc = "Get the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PromiseRejectionEventInit`*"]
    fn cancelable(&self) -> bool;
    #[doc = "Get the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PromiseRejectionEventInit`*"]
    fn composed(&self) -> bool;
    #[doc = "Get the `promise` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PromiseRejectionEventInit`*"]
    fn promise(&self) -> ::js_sys::Promise;
    #[doc = "Get the `reason` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PromiseRejectionEventInit`*"]
    fn reason(&self) -> ::wasm_bindgen::JsValue;
}
impl PromiseRejectionEventInitGetters for PromiseRejectionEventInit {
    fn bubbles(&self) -> bool {
        self.bubbles_shim()
    }
    fn cancelable(&self) -> bool {
        self.cancelable_shim()
    }
    fn composed(&self) -> bool {
        self.composed_shim()
    }
    fn promise(&self) -> ::js_sys::Promise {
        self.promise_shim()
    }
    fn reason(&self) -> ::wasm_bindgen::JsValue {
        self.reason_shim()
    }
}
impl PromiseRejectionEventInit {
    #[doc = "Construct a new `PromiseRejectionEventInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PromiseRejectionEventInit`*"]
    pub fn new(promise: &::js_sys::Promise) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        Self::promise(&mut ret, promise);
        ret
    }
    #[doc = "Change the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PromiseRejectionEventInit`*"]
    pub fn bubbles(&mut self, val: bool) -> &mut Self {
        self.set_bubbles_shim(val);
        self
    }
    #[doc = "Change the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PromiseRejectionEventInit`*"]
    pub fn cancelable(&mut self, val: bool) -> &mut Self {
        self.set_cancelable_shim(val);
        self
    }
    #[doc = "Change the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PromiseRejectionEventInit`*"]
    pub fn composed(&mut self, val: bool) -> &mut Self {
        self.set_composed_shim(val);
        self
    }
    #[doc = "Change the `promise` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PromiseRejectionEventInit`*"]
    pub fn promise(&mut self, val: &::js_sys::Promise) -> &mut Self {
        self.set_promise_shim(val);
        self
    }
    #[doc = "Change the `reason` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PromiseRejectionEventInit`*"]
    pub fn reason(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_reason_shim(val);
        self
    }
}
