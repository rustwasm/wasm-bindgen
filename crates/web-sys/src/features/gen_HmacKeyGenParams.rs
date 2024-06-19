#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = HmacKeyGenParams)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `HmacKeyGenParams` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HmacKeyGenParams`*"]
    pub type HmacKeyGenParams;
    #[doc = "Get the `name` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HmacKeyGenParams`*"]
    #[wasm_bindgen(method, getter = "name")]
    pub fn get_name(this: &HmacKeyGenParams) -> String;
    #[wasm_bindgen(method, setter = "name")]
    fn set_name(this: &HmacKeyGenParams, val: &str);
    #[doc = "Get the `hash` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HmacKeyGenParams`*"]
    #[wasm_bindgen(method, getter = "hash")]
    pub fn get_hash(this: &HmacKeyGenParams) -> ::wasm_bindgen::JsValue;
    #[wasm_bindgen(method, setter = "hash")]
    fn set_hash(this: &HmacKeyGenParams, val: &::wasm_bindgen::JsValue);
    #[doc = "Get the `length` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HmacKeyGenParams`*"]
    #[wasm_bindgen(method, getter = "length")]
    pub fn get_length(this: &HmacKeyGenParams) -> Option<u32>;
    #[wasm_bindgen(method, setter = "length")]
    fn set_length(this: &HmacKeyGenParams, val: u32);
}
impl HmacKeyGenParams {
    #[doc = "Construct a new `HmacKeyGenParams`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HmacKeyGenParams`*"]
    pub fn new(name: &str, hash: &::wasm_bindgen::JsValue) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.name(name);
        ret.hash(hash);
        ret
    }
    #[doc = "Change the `name` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HmacKeyGenParams`*"]
    pub fn name(&mut self, val: &str) -> &mut Self {
        self.set_name(val);
        self
    }
    #[doc = "Change the `hash` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HmacKeyGenParams`*"]
    pub fn hash(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_hash(val);
        self
    }
    #[doc = "Change the `length` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HmacKeyGenParams`*"]
    pub fn length(&mut self, val: u32) -> &mut Self {
        self.set_length(val);
        self
    }
}
