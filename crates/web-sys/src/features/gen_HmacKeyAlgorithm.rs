#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = HmacKeyAlgorithm)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `HmacKeyAlgorithm` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HmacKeyAlgorithm`*"]
    pub type HmacKeyAlgorithm;
    #[wasm_bindgen(method, getter = "name")]
    fn name_shim(this: &HmacKeyAlgorithm) -> &str;
    #[wasm_bindgen(method, setter = "name")]
    fn set_name_shim(this: &HmacKeyAlgorithm, val: &str);
    #[cfg(feature = "KeyAlgorithm")]
    #[wasm_bindgen(method, getter = "hash")]
    fn hash_shim(this: &HmacKeyAlgorithm) -> &KeyAlgorithm;
    #[cfg(feature = "KeyAlgorithm")]
    #[wasm_bindgen(method, setter = "hash")]
    fn set_hash_shim(this: &HmacKeyAlgorithm, val: &KeyAlgorithm);
    #[wasm_bindgen(method, getter = "length")]
    fn length_shim(this: &HmacKeyAlgorithm) -> u32;
    #[wasm_bindgen(method, setter = "length")]
    fn set_length_shim(this: &HmacKeyAlgorithm, val: u32);
}
#[doc = "The trait to access properties on the `HmacKeyAlgorithm` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `HmacKeyAlgorithm`*"]
pub trait HmacKeyAlgorithmGetters {
    #[doc = "Get the `name` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HmacKeyAlgorithm`*"]
    fn name(&self) -> &str;
    #[cfg(feature = "KeyAlgorithm")]
    #[doc = "Get the `hash` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HmacKeyAlgorithm`, `KeyAlgorithm`*"]
    fn hash(&self) -> &KeyAlgorithm;
    #[doc = "Get the `length` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HmacKeyAlgorithm`*"]
    fn length(&self) -> u32;
}
impl HmacKeyAlgorithmGetters for HmacKeyAlgorithm {
    fn name(&self) -> &str {
        self.name_shim()
    }
    #[cfg(feature = "KeyAlgorithm")]
    fn hash(&self) -> &KeyAlgorithm {
        self.hash_shim()
    }
    fn length(&self) -> u32 {
        self.length_shim()
    }
}
impl HmacKeyAlgorithm {
    #[cfg(feature = "KeyAlgorithm")]
    #[doc = "Construct a new `HmacKeyAlgorithm`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HmacKeyAlgorithm`, `KeyAlgorithm`*"]
    pub fn new(name: &str, hash: &KeyAlgorithm, length: u32) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.name(name);
        ret.hash(hash);
        ret.length(length);
        ret
    }
    #[doc = "Change the `name` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HmacKeyAlgorithm`*"]
    pub fn name(&mut self, val: &str) -> &mut Self {
        self.set_name_shim(val);
        self
    }
    #[cfg(feature = "KeyAlgorithm")]
    #[doc = "Change the `hash` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HmacKeyAlgorithm`, `KeyAlgorithm`*"]
    pub fn hash(&mut self, val: &KeyAlgorithm) -> &mut Self {
        self.set_hash_shim(val);
        self
    }
    #[doc = "Change the `length` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HmacKeyAlgorithm`*"]
    pub fn length(&mut self, val: u32) -> &mut Self {
        self.set_length_shim(val);
        self
    }
}
