#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = AesKeyAlgorithm)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `AesKeyAlgorithm` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AesKeyAlgorithm`*"]
    pub type AesKeyAlgorithm;
    #[wasm_bindgen(method, setter = "name")]
    fn name_shim(this: &AesKeyAlgorithm, val: &str);
    #[wasm_bindgen(method, setter = "length")]
    fn length_shim(this: &AesKeyAlgorithm, val: u16);
}
impl AesKeyAlgorithm {
    #[doc = "Construct a new `AesKeyAlgorithm`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AesKeyAlgorithm`*"]
    pub fn new(name: &str, length: u16) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.name(name);
        ret.length(length);
        ret
    }
    #[doc = "Change the `name` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AesKeyAlgorithm`*"]
    pub fn name(&mut self, val: &str) -> &mut Self {
        self.name_shim(val);
        self
    }
    #[doc = "Change the `length` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AesKeyAlgorithm`*"]
    pub fn length(&mut self, val: u16) -> &mut Self {
        self.length_shim(val);
        self
    }
}
