#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = RsaHashedKeyAlgorithm)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `RsaHashedKeyAlgorithm` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RsaHashedKeyAlgorithm`*"]
    pub type RsaHashedKeyAlgorithm;
    #[doc = "Get the `name` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RsaHashedKeyAlgorithm`*"]
    #[wasm_bindgen(method, getter = "name")]
    pub fn get_name(this: &RsaHashedKeyAlgorithm) -> String;
    #[doc = "Change the `name` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RsaHashedKeyAlgorithm`*"]
    #[wasm_bindgen(method, setter = "name")]
    pub fn set_name(this: &RsaHashedKeyAlgorithm, val: &str);
    #[cfg(feature = "KeyAlgorithm")]
    #[doc = "Get the `hash` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `KeyAlgorithm`, `RsaHashedKeyAlgorithm`*"]
    #[wasm_bindgen(method, getter = "hash")]
    pub fn get_hash(this: &RsaHashedKeyAlgorithm) -> KeyAlgorithm;
    #[cfg(feature = "KeyAlgorithm")]
    #[doc = "Change the `hash` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `KeyAlgorithm`, `RsaHashedKeyAlgorithm`*"]
    #[wasm_bindgen(method, setter = "hash")]
    pub fn set_hash(this: &RsaHashedKeyAlgorithm, val: &KeyAlgorithm);
    #[doc = "Get the `modulusLength` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RsaHashedKeyAlgorithm`*"]
    #[wasm_bindgen(method, getter = "modulusLength")]
    pub fn get_modulus_length(this: &RsaHashedKeyAlgorithm) -> u16;
    #[doc = "Change the `modulusLength` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RsaHashedKeyAlgorithm`*"]
    #[wasm_bindgen(method, setter = "modulusLength")]
    pub fn set_modulus_length(this: &RsaHashedKeyAlgorithm, val: u16);
    #[cfg(feature = "Uint8Array")]
    #[doc = "Get the `publicExponent` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RsaHashedKeyAlgorithm`, `Uint8Array`*"]
    #[wasm_bindgen(method, getter = "publicExponent")]
    pub fn get_public_exponent(this: &RsaHashedKeyAlgorithm) -> Vec<u8>;
    #[cfg(feature = "Uint8Array")]
    #[doc = "Change the `publicExponent` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RsaHashedKeyAlgorithm`, `Uint8Array`*"]
    #[wasm_bindgen(method, setter = "publicExponent")]
    pub fn set_public_exponent(this: &RsaHashedKeyAlgorithm, val: &::js_sys::Uint8Array);
}
impl RsaHashedKeyAlgorithm {
    #[cfg(all(feature = "KeyAlgorithm", feature = "Uint8Array",))]
    #[doc = "Construct a new `RsaHashedKeyAlgorithm`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `KeyAlgorithm`, `RsaHashedKeyAlgorithm`, `Uint8Array`*"]
    pub fn new(
        name: &str,
        hash: &KeyAlgorithm,
        modulus_length: u16,
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
    #[cfg(feature = "KeyAlgorithm")]
    #[deprecated = "Use `set_hash()` instead."]
    pub fn hash(&mut self, val: &KeyAlgorithm) -> &mut Self {
        self.set_hash(val);
        self
    }
    #[deprecated = "Use `set_modulus_length()` instead."]
    pub fn modulus_length(&mut self, val: u16) -> &mut Self {
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
