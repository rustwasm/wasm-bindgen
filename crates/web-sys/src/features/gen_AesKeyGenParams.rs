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
    #[wasm_bindgen(method, getter = "name")]
    fn name_shim(this: &AesKeyGenParams) -> &str;
    #[wasm_bindgen(method, setter = "name")]
    fn set_name_shim(this: &AesKeyGenParams, val: &str);
    #[wasm_bindgen(method, getter = "length")]
    fn length_shim(this: &AesKeyGenParams) -> u16;
    #[wasm_bindgen(method, setter = "length")]
    fn set_length_shim(this: &AesKeyGenParams, val: u16);
}
#[doc = "The trait to access properties on the `AesKeyGenParams` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `AesKeyGenParams`*"]
pub trait AesKeyGenParamsGetters {
    #[doc = "Get the `name` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AesKeyGenParams`*"]
    fn name(&self) -> &str;
    #[doc = "Get the `length` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AesKeyGenParams`*"]
    fn length(&self) -> u16;
}
impl AesKeyGenParamsGetters for AesKeyGenParams {
    fn name(&self) -> &str {
        self.name_shim()
    }
    fn length(&self) -> u16 {
        self.length_shim()
    }
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
        self.set_name_shim(val);
        self
    }
    #[doc = "Change the `length` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AesKeyGenParams`*"]
    pub fn length(&mut self, val: u16) -> &mut Self {
        self.set_length_shim(val);
        self
    }
}
