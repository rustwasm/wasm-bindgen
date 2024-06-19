#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = HkdfParams)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `HkdfParams` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HkdfParams`*"]
    pub type HkdfParams;
    #[doc = "Get the `name` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HkdfParams`*"]
    #[wasm_bindgen(method, getter = "name")]
    pub fn get_name(this: &HkdfParams) -> String;
    #[wasm_bindgen(method, setter = "name")]
    fn set_name(this: &HkdfParams, val: &str);
    #[doc = "Get the `hash` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HkdfParams`*"]
    #[wasm_bindgen(method, getter = "hash")]
    pub fn get_hash(this: &HkdfParams) -> ::wasm_bindgen::JsValue;
    #[wasm_bindgen(method, setter = "hash")]
    fn set_hash(this: &HkdfParams, val: &::wasm_bindgen::JsValue);
    #[doc = "Get the `info` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HkdfParams`*"]
    #[wasm_bindgen(method, getter = "info")]
    pub fn get_info(this: &HkdfParams) -> ::js_sys::Object;
    #[wasm_bindgen(method, setter = "info")]
    fn set_info(this: &HkdfParams, val: &::js_sys::Object);
    #[doc = "Get the `salt` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HkdfParams`*"]
    #[wasm_bindgen(method, getter = "salt")]
    pub fn get_salt(this: &HkdfParams) -> ::js_sys::Object;
    #[wasm_bindgen(method, setter = "salt")]
    fn set_salt(this: &HkdfParams, val: &::js_sys::Object);
}
impl HkdfParams {
    #[doc = "Construct a new `HkdfParams`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HkdfParams`*"]
    pub fn new(
        name: &str,
        hash: &::wasm_bindgen::JsValue,
        info: &::js_sys::Object,
        salt: &::js_sys::Object,
    ) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.name(name);
        ret.hash(hash);
        ret.info(info);
        ret.salt(salt);
        ret
    }
    #[doc = "Change the `name` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HkdfParams`*"]
    pub fn name(&mut self, val: &str) -> &mut Self {
        self.set_name(val);
        self
    }
    #[doc = "Change the `hash` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HkdfParams`*"]
    pub fn hash(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_hash(val);
        self
    }
    #[doc = "Change the `info` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HkdfParams`*"]
    pub fn info(&mut self, val: &::js_sys::Object) -> &mut Self {
        self.set_info(val);
        self
    }
    #[doc = "Change the `salt` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HkdfParams`*"]
    pub fn salt(&mut self, val: &::js_sys::Object) -> &mut Self {
        self.set_salt(val);
        self
    }
}
