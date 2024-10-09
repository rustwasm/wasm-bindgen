#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = DhKeyGenParams)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `DhKeyGenParams` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DhKeyGenParams`*"]
    pub type DhKeyGenParams;
    #[doc = "Get the `name` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DhKeyGenParams`*"]
    #[wasm_bindgen(method, getter = "name")]
    pub fn get_name(this: &DhKeyGenParams) -> String;
    #[doc = "Change the `name` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DhKeyGenParams`*"]
    #[wasm_bindgen(method, setter = "name")]
    pub fn set_name(this: &DhKeyGenParams, val: &str);
    #[doc = "Get the `generator` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DhKeyGenParams`*"]
    #[wasm_bindgen(method, getter = "generator")]
    pub fn get_generator(this: &DhKeyGenParams) -> Vec<u8>;
    #[doc = "Change the `generator` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DhKeyGenParams`*"]
    #[wasm_bindgen(method, setter = "generator")]
    pub fn set_generator(this: &DhKeyGenParams, val: &::js_sys::Uint8Array);
    #[doc = "Get the `prime` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DhKeyGenParams`*"]
    #[wasm_bindgen(method, getter = "prime")]
    pub fn get_prime(this: &DhKeyGenParams) -> Vec<u8>;
    #[doc = "Change the `prime` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DhKeyGenParams`*"]
    #[wasm_bindgen(method, setter = "prime")]
    pub fn set_prime(this: &DhKeyGenParams, val: &::js_sys::Uint8Array);
}
impl DhKeyGenParams {
    #[doc = "Construct a new `DhKeyGenParams`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DhKeyGenParams`*"]
    pub fn new(name: &str, generator: &::js_sys::Uint8Array, prime: &::js_sys::Uint8Array) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.name(name);
        ret.generator(generator);
        ret.prime(prime);
        ret
    }
    #[deprecated = "Use `set_name()` instead."]
    pub fn name(&mut self, val: &str) -> &mut Self {
        self.set_name(val);
        self
    }
    #[deprecated = "Use `set_generator()` instead."]
    pub fn generator(&mut self, val: &::js_sys::Uint8Array) -> &mut Self {
        self.set_generator(val);
        self
    }
    #[deprecated = "Use `set_prime()` instead."]
    pub fn prime(&mut self, val: &::js_sys::Uint8Array) -> &mut Self {
        self.set_prime(val);
        self
    }
}
