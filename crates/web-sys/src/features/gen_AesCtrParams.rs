#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = AesCtrParams)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `AesCtrParams` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AesCtrParams`*"]
    pub type AesCtrParams;
    #[wasm_bindgen(method, getter = "name")]
    fn name_shim(this: &AesCtrParams) -> &str;
    #[wasm_bindgen(method, setter = "name")]
    fn set_name_shim(this: &AesCtrParams, val: &str);
    #[wasm_bindgen(method, getter = "counter")]
    fn counter_shim(this: &AesCtrParams) -> &::js_sys::Object;
    #[wasm_bindgen(method, setter = "counter")]
    fn set_counter_shim(this: &AesCtrParams, val: &::js_sys::Object);
    #[wasm_bindgen(method, getter = "length")]
    fn length_shim(this: &AesCtrParams) -> u8;
    #[wasm_bindgen(method, setter = "length")]
    fn set_length_shim(this: &AesCtrParams, val: u8);
}
#[doc = "The trait to access properties on the `AesCtrParams` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `AesCtrParams`*"]
pub trait AesCtrParamsGetters {
    #[doc = "Get the `name` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AesCtrParams`*"]
    fn name(&self) -> &str;
    #[doc = "Get the `counter` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AesCtrParams`*"]
    fn counter(&self) -> &::js_sys::Object;
    #[doc = "Get the `length` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AesCtrParams`*"]
    fn length(&self) -> u8;
}
impl AesCtrParamsGetters for AesCtrParams {
    fn name(&self) -> &str {
        self.name_shim()
    }
    fn counter(&self) -> &::js_sys::Object {
        self.counter_shim()
    }
    fn length(&self) -> u8 {
        self.length_shim()
    }
}
impl AesCtrParams {
    #[doc = "Construct a new `AesCtrParams`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AesCtrParams`*"]
    pub fn new(name: &str, counter: &::js_sys::Object, length: u8) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.name(name);
        ret.counter(counter);
        ret.length(length);
        ret
    }
    #[doc = "Change the `name` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AesCtrParams`*"]
    pub fn name(&mut self, val: &str) -> &mut Self {
        self.set_name_shim(val);
        self
    }
    #[doc = "Change the `counter` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AesCtrParams`*"]
    pub fn counter(&mut self, val: &::js_sys::Object) -> &mut Self {
        self.set_counter_shim(val);
        self
    }
    #[doc = "Change the `length` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AesCtrParams`*"]
    pub fn length(&mut self, val: u8) -> &mut Self {
        self.set_length_shim(val);
        self
    }
}
