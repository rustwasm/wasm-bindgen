#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = RsaHashedKeyGenParams)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `RsaHashedKeyGenParams` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RsaHashedKeyGenParams`*"]
    pub type RsaHashedKeyGenParams;
    #[doc = "Get the `name` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RsaHashedKeyGenParams`*"]
    #[wasm_bindgen(method, getter = "name")]
    pub fn get_name(this: &RsaHashedKeyGenParams) -> String;
    #[doc = "Change the `name` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RsaHashedKeyGenParams`*"]
    #[wasm_bindgen(method, setter = "name")]
    pub fn set_name(this: &RsaHashedKeyGenParams, val: &str);
    #[doc = "Get the `hash` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RsaHashedKeyGenParams`*"]
    #[wasm_bindgen(method, getter = "hash")]
    pub fn get_hash(this: &RsaHashedKeyGenParams) -> ::wasm_bindgen::JsValue;
    #[doc = "Change the `hash` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RsaHashedKeyGenParams`*"]
    #[wasm_bindgen(method, setter = "hash")]
    pub fn set_hash(this: &RsaHashedKeyGenParams, val: &::wasm_bindgen::JsValue);
    #[doc = "Get the `modulusLength` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RsaHashedKeyGenParams`*"]
    #[wasm_bindgen(method, getter = "modulusLength")]
    pub fn get_modulus_length(this: &RsaHashedKeyGenParams) -> u32;
    #[doc = "Change the `modulusLength` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RsaHashedKeyGenParams`*"]
    #[wasm_bindgen(method, setter = "modulusLength")]
    pub fn set_modulus_length(this: &RsaHashedKeyGenParams, val: u32);
    #[cfg(feature = "Uint8Array")]
    #[doc = "Get the `publicExponent` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RsaHashedKeyGenParams`, `Uint8Array`*"]
    #[wasm_bindgen(method, getter = "publicExponent")]
    pub fn get_public_exponent(this: &RsaHashedKeyGenParams) -> Vec<u8>;
    #[cfg(feature = "Uint8Array")]
    #[doc = "Change the `publicExponent` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RsaHashedKeyGenParams`, `Uint8Array`*"]
    #[wasm_bindgen(method, setter = "publicExponent")]
    pub fn set_public_exponent(this: &RsaHashedKeyGenParams, val: &::js_sys::Uint8Array);
}
impl RsaHashedKeyGenParams {
    #[cfg(feature = "Uint8Array")]
    #[doc = "Construct a new `RsaHashedKeyGenParams`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RsaHashedKeyGenParams`, `Uint8Array`*"]
    pub fn new(
        name: &str,
        hash: &::wasm_bindgen::JsValue,
        modulus_length: u32,
        public_exponent: &::js_sys::Uint8Array,
    ) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.name(name);
        ret.hash(hash);
        ret.modulus_length(modulus_length);
        ret.public_exponent(public_exponent);
        ret
    }
    #[deprecated = "Use `set_name()` instead."]
    pub fn name(&mut self, val: &str) -> &mut Self {
        self.set_name(val);
        self
    }
    #[deprecated = "Use `set_hash()` instead."]
    pub fn hash(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_hash(val);
        self
    }
    #[deprecated = "Use `set_modulus_length()` instead."]
    pub fn modulus_length(&mut self, val: u32) -> &mut Self {
        self.set_modulus_length(val);
        self
    }
    #[cfg(feature = "Uint8Array")]
    #[deprecated = "Use `set_public_exponent()` instead."]
    pub fn public_exponent(&mut self, val: &::js_sys::Uint8Array) -> &mut Self {
        self.set_public_exponent(val);
        self
    }
}
