#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = AesDerivedKeyParams)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `AesDerivedKeyParams` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AesDerivedKeyParams`*"]
    pub type AesDerivedKeyParams;
    #[wasm_bindgen(method, setter = "name")]
    fn name_shim(this: &AesDerivedKeyParams, val: &str);
    #[wasm_bindgen(method, setter = "length")]
    fn length_shim(this: &AesDerivedKeyParams, val: u32);
}
impl AesDerivedKeyParams {
    #[doc = "Construct a new `AesDerivedKeyParams`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AesDerivedKeyParams`*"]
    pub fn new(name: &str, length: u32) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.name(name);
        ret.length(length);
        ret
    }
    #[doc = "Change the `name` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AesDerivedKeyParams`*"]
    pub fn name(&mut self, val: &str) -> &mut Self {
        self.name_shim(val);
        self
    }
    #[doc = "Change the `length` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AesDerivedKeyParams`*"]
    pub fn length(&mut self, val: u32) -> &mut Self {
        self.length_shim(val);
        self
    }
}
