#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = AesKeyGenParams)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `AesKeyGenParams` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AesKeyGenParams`*"]
    pub type AesKeyGenParams;
    #[doc = "Get the `name` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AesKeyGenParams`*"]
    #[wasm_bindgen(method, getter = "name")]
    pub fn get_name(this: &AesKeyGenParams) -> String;
    #[wasm_bindgen(method, setter = "name")]
    fn set_name(this: &AesKeyGenParams, val: &str);
    #[doc = "Get the `length` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AesKeyGenParams`*"]
    #[wasm_bindgen(method, getter = "length")]
    pub fn get_length(this: &AesKeyGenParams) -> u16;
    #[wasm_bindgen(method, setter = "length")]
    fn set_length(this: &AesKeyGenParams, val: u16);
}
impl AesKeyGenParams {
    #[doc = "Construct a new `AesKeyGenParams`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AesKeyGenParams`*"]
    pub fn new(name: &str, length: u16) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.name(name);
        ret.length(length);
        ret
    }
    #[doc = "Change the `name` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AesKeyGenParams`*"]
    pub fn name(&mut self, val: &str) -> &mut Self {
        self.set_name(val);
        self
    }
    #[doc = "Change the `length` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AesKeyGenParams`*"]
    pub fn length(&mut self, val: u16) -> &mut Self {
        self.set_length(val);
        self
    }
}
