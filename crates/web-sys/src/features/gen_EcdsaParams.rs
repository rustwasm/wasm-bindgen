#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = EcdsaParams)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `EcdsaParams` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EcdsaParams`*"]
    pub type EcdsaParams;
    #[wasm_bindgen(method, getter = "name")]
    fn name_shim(this: &EcdsaParams) -> &str;
    #[wasm_bindgen(method, setter = "name")]
    fn set_name_shim(this: &EcdsaParams, val: &str);
    #[wasm_bindgen(method, getter = "hash")]
    fn hash_shim(this: &EcdsaParams) -> &::wasm_bindgen::JsValue;
    #[wasm_bindgen(method, setter = "hash")]
    fn set_hash_shim(this: &EcdsaParams, val: &::wasm_bindgen::JsValue);
}
#[doc = "The trait to access properties on the `EcdsaParams` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `EcdsaParams`*"]
pub trait EcdsaParamsGetters {
    #[doc = "Get the `name` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EcdsaParams`*"]
    fn name(&self) -> &str;
    #[doc = "Get the `hash` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EcdsaParams`*"]
    fn hash(&self) -> &::wasm_bindgen::JsValue;
}
impl EcdsaParamsGetters for EcdsaParams {
    fn name(&self) -> &str {
        self.name_shim()
    }
    fn hash(&self) -> &::wasm_bindgen::JsValue {
        self.hash_shim()
    }
}
impl EcdsaParams {
    #[doc = "Construct a new `EcdsaParams`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EcdsaParams`*"]
    pub fn new(name: &str, hash: &::wasm_bindgen::JsValue) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.name(name);
        ret.hash(hash);
        ret
    }
    #[doc = "Change the `name` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EcdsaParams`*"]
    pub fn name(&mut self, val: &str) -> &mut Self {
        self.set_name_shim(val);
        self
    }
    #[doc = "Change the `hash` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EcdsaParams`*"]
    pub fn hash(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_hash_shim(val);
        self
    }
}
