#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = PaymentRequestUpdateEventInit)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `PaymentRequestUpdateEventInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PaymentRequestUpdateEventInit`*"]
    pub type PaymentRequestUpdateEventInit;
    #[wasm_bindgen(method, getter = "bubbles")]
    fn bubbles_shim(this: &PaymentRequestUpdateEventInit) -> bool;
    #[wasm_bindgen(method, setter = "bubbles")]
    fn set_bubbles_shim(this: &PaymentRequestUpdateEventInit, val: bool);
    #[wasm_bindgen(method, getter = "cancelable")]
    fn cancelable_shim(this: &PaymentRequestUpdateEventInit) -> bool;
    #[wasm_bindgen(method, setter = "cancelable")]
    fn set_cancelable_shim(this: &PaymentRequestUpdateEventInit, val: bool);
    #[wasm_bindgen(method, getter = "composed")]
    fn composed_shim(this: &PaymentRequestUpdateEventInit) -> bool;
    #[wasm_bindgen(method, setter = "composed")]
    fn set_composed_shim(this: &PaymentRequestUpdateEventInit, val: bool);
}
#[doc = "The trait to access properties on the `PaymentRequestUpdateEventInit` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `PaymentRequestUpdateEventInit`*"]
pub trait PaymentRequestUpdateEventInitGetters {
    #[doc = "Get the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PaymentRequestUpdateEventInit`*"]
    fn bubbles(&self) -> bool;
    #[doc = "Get the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PaymentRequestUpdateEventInit`*"]
    fn cancelable(&self) -> bool;
    #[doc = "Get the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PaymentRequestUpdateEventInit`*"]
    fn composed(&self) -> bool;
}
impl PaymentRequestUpdateEventInitGetters for PaymentRequestUpdateEventInit {
    fn bubbles(&self) -> bool {
        self.bubbles_shim()
    }
    fn cancelable(&self) -> bool {
        self.cancelable_shim()
    }
    fn composed(&self) -> bool {
        self.composed_shim()
    }
}
impl PaymentRequestUpdateEventInit {
    #[doc = "Construct a new `PaymentRequestUpdateEventInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PaymentRequestUpdateEventInit`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PaymentRequestUpdateEventInit`*"]
    pub fn bubbles(&mut self, val: bool) -> &mut Self {
        self.set_bubbles_shim(val);
        self
    }
    #[doc = "Change the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PaymentRequestUpdateEventInit`*"]
    pub fn cancelable(&mut self, val: bool) -> &mut Self {
        self.set_cancelable_shim(val);
        self
    }
    #[doc = "Change the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PaymentRequestUpdateEventInit`*"]
    pub fn composed(&mut self, val: bool) -> &mut Self {
        self.set_composed_shim(val);
        self
    }
}
impl Default for PaymentRequestUpdateEventInit {
    fn default() -> Self {
        Self::new()
    }
}
