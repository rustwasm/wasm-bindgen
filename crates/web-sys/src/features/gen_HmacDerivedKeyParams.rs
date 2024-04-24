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
    #[wasm_bindgen(method, getter = "name")]
    fn name_shim(this: &HmacDerivedKeyParams) -> String;
    #[wasm_bindgen(method, setter = "name")]
    fn set_name_shim(this: &HmacDerivedKeyParams, val: &str);
    #[wasm_bindgen(method, getter = "hash")]
    fn hash_shim(this: &HmacDerivedKeyParams) -> ::wasm_bindgen::JsValue;
    #[wasm_bindgen(method, setter = "hash")]
    fn set_hash_shim(this: &HmacDerivedKeyParams, val: &::wasm_bindgen::JsValue);
    #[wasm_bindgen(method, getter = "length")]
    fn length_shim(this: &HmacDerivedKeyParams) -> u32;
    #[wasm_bindgen(method, setter = "length")]
    fn set_length_shim(this: &HmacDerivedKeyParams, val: u32);
}
#[doc = "The trait to access properties on the `HmacDerivedKeyParams` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `HmacDerivedKeyParams`*"]
pub trait HmacDerivedKeyParamsGetters {
    #[doc = "Get the `name` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HmacDerivedKeyParams`*"]
    fn name(&self) -> String;
    #[doc = "Get the `hash` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HmacDerivedKeyParams`*"]
    fn hash(&self) -> ::wasm_bindgen::JsValue;
    #[doc = "Get the `length` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HmacDerivedKeyParams`*"]
    fn length(&self) -> u32;
}
impl HmacDerivedKeyParamsGetters for HmacDerivedKeyParams {
    fn name(&self) -> String {
        self.name_shim()
    }
    fn hash(&self) -> ::wasm_bindgen::JsValue {
        self.hash_shim()
    }
    fn length(&self) -> u32 {
        self.length_shim()
    }
}
impl HmacDerivedKeyParams {
    #[doc = "Construct a new `HmacDerivedKeyParams`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HmacDerivedKeyParams`*"]
    pub fn new(name: &str, hash: &::wasm_bindgen::JsValue) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        Self::name(&mut ret, name);
        Self::hash(&mut ret, hash);
        ret
    }
    #[doc = "Change the `name` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HmacDerivedKeyParams`*"]
    pub fn name(&mut self, val: &str) -> &mut Self {
        self.set_name_shim(val);
        self
    }
    #[doc = "Change the `hash` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HmacDerivedKeyParams`*"]
    pub fn hash(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_hash_shim(val);
        self
    }
    #[doc = "Change the `length` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HmacDerivedKeyParams`*"]
    pub fn length(&mut self, val: u32) -> &mut Self {
        self.set_length_shim(val);
        self
    }
}
