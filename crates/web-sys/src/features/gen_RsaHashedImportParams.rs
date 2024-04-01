#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = RsaHashedImportParams)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `RsaHashedImportParams` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RsaHashedImportParams`*"]
    pub type RsaHashedImportParams;
    #[wasm_bindgen(method, setter = "hash")]
    fn hash_shim(this: &RsaHashedImportParams, val: &::wasm_bindgen::JsValue);
}
impl RsaHashedImportParams {
    #[doc = "Construct a new `RsaHashedImportParams`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RsaHashedImportParams`*"]
    pub fn new(hash: &::wasm_bindgen::JsValue) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.hash(hash);
        ret
    }
    #[doc = "Change the `hash` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RsaHashedImportParams`*"]
    pub fn hash(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.hash_shim(val);
        self
    }
}
