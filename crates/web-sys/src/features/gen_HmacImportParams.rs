#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = HmacImportParams)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `HmacImportParams` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HmacImportParams`*"]
    pub type HmacImportParams;
    #[wasm_bindgen(method, setter = "name")]
    fn name_shim(this: &HmacImportParams, val: &str);
    #[wasm_bindgen(method, setter = "hash")]
    fn hash_shim(this: &HmacImportParams, val: &::wasm_bindgen::JsValue);
}
impl HmacImportParams {
    #[doc = "Construct a new `HmacImportParams`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HmacImportParams`*"]
    pub fn new(name: &str, hash: &::wasm_bindgen::JsValue) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.name(name);
        ret.hash(hash);
        ret
    }
    #[doc = "Change the `name` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HmacImportParams`*"]
    pub fn name(&mut self, val: &str) -> &mut Self {
        self.name_shim(val);
        self
    }
    #[doc = "Change the `hash` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HmacImportParams`*"]
    pub fn hash(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.hash_shim(val);
        self
    }
}
