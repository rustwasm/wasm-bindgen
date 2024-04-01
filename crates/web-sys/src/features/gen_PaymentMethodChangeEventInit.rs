#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = PaymentMethodChangeEventInit)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `PaymentMethodChangeEventInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PaymentMethodChangeEventInit`*"]
    pub type PaymentMethodChangeEventInit;
    #[wasm_bindgen(method, setter = "bubbles")]
    fn bubbles_shim(this: &PaymentMethodChangeEventInit, val: bool);
    #[wasm_bindgen(method, setter = "cancelable")]
    fn cancelable_shim(this: &PaymentMethodChangeEventInit, val: bool);
    #[wasm_bindgen(method, setter = "composed")]
    fn composed_shim(this: &PaymentMethodChangeEventInit, val: bool);
    #[wasm_bindgen(method, setter = "methodDetails")]
    fn method_details_shim(this: &PaymentMethodChangeEventInit, val: Option<&::js_sys::Object>);
    #[wasm_bindgen(method, setter = "methodName")]
    fn method_name_shim(this: &PaymentMethodChangeEventInit, val: &str);
}
impl PaymentMethodChangeEventInit {
    #[doc = "Construct a new `PaymentMethodChangeEventInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PaymentMethodChangeEventInit`*"]
    pub fn new(method_name: &str) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.method_name(method_name);
        ret
    }
    #[doc = "Change the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PaymentMethodChangeEventInit`*"]
    pub fn bubbles(&mut self, val: bool) -> &mut Self {
        self.bubbles_shim(val);
        self
    }
    #[doc = "Change the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PaymentMethodChangeEventInit`*"]
    pub fn cancelable(&mut self, val: bool) -> &mut Self {
        self.cancelable_shim(val);
        self
    }
    #[doc = "Change the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PaymentMethodChangeEventInit`*"]
    pub fn composed(&mut self, val: bool) -> &mut Self {
        self.composed_shim(val);
        self
    }
    #[doc = "Change the `methodDetails` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PaymentMethodChangeEventInit`*"]
    pub fn method_details(&mut self, val: Option<&::js_sys::Object>) -> &mut Self {
        self.method_details_shim(val);
        self
    }
    #[doc = "Change the `methodName` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PaymentMethodChangeEventInit`*"]
    pub fn method_name(&mut self, val: &str) -> &mut Self {
        self.method_name_shim(val);
        self
    }
}
