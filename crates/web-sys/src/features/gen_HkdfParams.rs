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
    #[wasm_bindgen(method, getter = "name")]
    fn name_shim(this: &HkdfParams) -> &str;
    #[wasm_bindgen(method, setter = "name")]
    fn set_name_shim(this: &HkdfParams, val: &str);
    #[wasm_bindgen(method, getter = "hash")]
    fn hash_shim(this: &HkdfParams) -> &::wasm_bindgen::JsValue;
    #[wasm_bindgen(method, setter = "hash")]
    fn set_hash_shim(this: &HkdfParams, val: &::wasm_bindgen::JsValue);
    #[wasm_bindgen(method, getter = "info")]
    fn info_shim(this: &HkdfParams) -> &::js_sys::Object;
    #[wasm_bindgen(method, setter = "info")]
    fn set_info_shim(this: &HkdfParams, val: &::js_sys::Object);
    #[wasm_bindgen(method, getter = "salt")]
    fn salt_shim(this: &HkdfParams) -> &::js_sys::Object;
    #[wasm_bindgen(method, setter = "salt")]
    fn set_salt_shim(this: &HkdfParams, val: &::js_sys::Object);
}
#[doc = "The trait to access properties on the `HkdfParams` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `HkdfParams`*"]
pub trait HkdfParamsGetters {
    #[doc = "Get the `name` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HkdfParams`*"]
    fn name(&self) -> &str;
    #[doc = "Get the `hash` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HkdfParams`*"]
    fn hash(&self) -> &::wasm_bindgen::JsValue;
    #[doc = "Get the `info` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HkdfParams`*"]
    fn info(&self) -> &::js_sys::Object;
    #[doc = "Get the `salt` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HkdfParams`*"]
    fn salt(&self) -> &::js_sys::Object;
}
impl HkdfParamsGetters for HkdfParams {
    fn name(&self) -> &str {
        self.name_shim()
    }
    fn hash(&self) -> &::wasm_bindgen::JsValue {
        self.hash_shim()
    }
    fn info(&self) -> &::js_sys::Object {
        self.info_shim()
    }
    fn salt(&self) -> &::js_sys::Object {
        self.salt_shim()
    }
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
        self.set_name_shim(val);
        self
    }
    #[doc = "Change the `hash` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HkdfParams`*"]
    pub fn hash(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_hash_shim(val);
        self
    }
    #[doc = "Change the `info` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HkdfParams`*"]
    pub fn info(&mut self, val: &::js_sys::Object) -> &mut Self {
        self.set_info_shim(val);
        self
    }
    #[doc = "Change the `salt` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HkdfParams`*"]
    pub fn salt(&mut self, val: &::js_sys::Object) -> &mut Self {
        self.set_salt_shim(val);
        self
    }
}
