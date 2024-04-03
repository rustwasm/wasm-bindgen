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
    #[wasm_bindgen(method, setter = "name")]
    fn name_shim(this: &AesCtrParams, val: &str);
    #[wasm_bindgen(method, setter = "counter")]
    fn counter_shim(this: &AesCtrParams, val: &::js_sys::Object);
    #[wasm_bindgen(method, setter = "length")]
    fn length_shim(this: &AesCtrParams, val: u8);
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
        self.name_shim(val);
        self
    }
    #[doc = "Change the `counter` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AesCtrParams`*"]
    pub fn counter(&mut self, val: &::js_sys::Object) -> &mut Self {
        self.counter_shim(val);
        self
    }
    #[doc = "Change the `length` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AesCtrParams`*"]
    pub fn length(&mut self, val: u8) -> &mut Self {
        self.length_shim(val);
        self
    }
}
