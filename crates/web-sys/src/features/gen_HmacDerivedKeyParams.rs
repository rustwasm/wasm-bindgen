#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = HmacDerivedKeyParams)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `HmacDerivedKeyParams` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HmacDerivedKeyParams`*"]
    pub type HmacDerivedKeyParams;
    #[wasm_bindgen(method, setter = "name")]
    fn name_shim(this: &HmacDerivedKeyParams, val: &str);
    #[wasm_bindgen(method, setter = "hash")]
    fn hash_shim(this: &HmacDerivedKeyParams, val: &::wasm_bindgen::JsValue);
    #[wasm_bindgen(method, setter = "length")]
    fn length_shim(this: &HmacDerivedKeyParams, val: u32);
}
impl HmacDerivedKeyParams {
    #[doc = "Construct a new `HmacDerivedKeyParams`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HmacDerivedKeyParams`*"]
    pub fn new(name: &str, hash: &::wasm_bindgen::JsValue) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.name(name);
        ret.hash(hash);
        ret
    }
    #[doc = "Change the `name` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HmacDerivedKeyParams`*"]
    pub fn name(&mut self, val: &str) -> &mut Self {
        self.name_shim(val);
        self
    }
    #[doc = "Change the `hash` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HmacDerivedKeyParams`*"]
    pub fn hash(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.hash_shim(val);
        self
    }
    #[doc = "Change the `length` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HmacDerivedKeyParams`*"]
    pub fn length(&mut self, val: u32) -> &mut Self {
        self.length_shim(val);
        self
    }
}
