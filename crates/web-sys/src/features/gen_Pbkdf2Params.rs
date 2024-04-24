#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = Pbkdf2Params)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `Pbkdf2Params` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Pbkdf2Params`*"]
    pub type Pbkdf2Params;
    #[wasm_bindgen(method, getter = "name")]
    fn name_shim(this: &Pbkdf2Params) -> String;
    #[wasm_bindgen(method, setter = "name")]
    fn set_name_shim(this: &Pbkdf2Params, val: &str);
    #[wasm_bindgen(method, getter = "hash")]
    fn hash_shim(this: &Pbkdf2Params) -> ::wasm_bindgen::JsValue;
    #[wasm_bindgen(method, setter = "hash")]
    fn set_hash_shim(this: &Pbkdf2Params, val: &::wasm_bindgen::JsValue);
    #[wasm_bindgen(method, getter = "iterations")]
    fn iterations_shim(this: &Pbkdf2Params) -> u32;
    #[wasm_bindgen(method, setter = "iterations")]
    fn set_iterations_shim(this: &Pbkdf2Params, val: u32);
    #[wasm_bindgen(method, getter = "salt")]
    fn salt_shim(this: &Pbkdf2Params) -> ::js_sys::Object;
    #[wasm_bindgen(method, setter = "salt")]
    fn set_salt_shim(this: &Pbkdf2Params, val: &::js_sys::Object);
}
#[doc = "The trait to access properties on the `Pbkdf2Params` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `Pbkdf2Params`*"]
pub trait Pbkdf2ParamsGetters {
    #[doc = "Get the `name` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Pbkdf2Params`*"]
    fn name(&self) -> String;
    #[doc = "Get the `hash` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Pbkdf2Params`*"]
    fn hash(&self) -> ::wasm_bindgen::JsValue;
    #[doc = "Get the `iterations` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Pbkdf2Params`*"]
    fn iterations(&self) -> u32;
    #[doc = "Get the `salt` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Pbkdf2Params`*"]
    fn salt(&self) -> ::js_sys::Object;
}
impl Pbkdf2ParamsGetters for Pbkdf2Params {
    fn name(&self) -> String {
        self.name_shim()
    }
    fn hash(&self) -> ::wasm_bindgen::JsValue {
        self.hash_shim()
    }
    fn iterations(&self) -> u32 {
        self.iterations_shim()
    }
    fn salt(&self) -> ::js_sys::Object {
        self.salt_shim()
    }
}
impl Pbkdf2Params {
    #[doc = "Construct a new `Pbkdf2Params`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Pbkdf2Params`*"]
    pub fn new(
        name: &str,
        hash: &::wasm_bindgen::JsValue,
        iterations: u32,
        salt: &::js_sys::Object,
    ) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.name(name);
        ret.hash(hash);
        ret.iterations(iterations);
        ret.salt(salt);
        ret
    }
    #[doc = "Change the `name` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Pbkdf2Params`*"]
    pub fn name(&mut self, val: &str) -> &mut Self {
        self.set_name_shim(val);
        self
    }
    #[doc = "Change the `hash` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Pbkdf2Params`*"]
    pub fn hash(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_hash_shim(val);
        self
    }
    #[doc = "Change the `iterations` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Pbkdf2Params`*"]
    pub fn iterations(&mut self, val: u32) -> &mut Self {
        self.set_iterations_shim(val);
        self
    }
    #[doc = "Change the `salt` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Pbkdf2Params`*"]
    pub fn salt(&mut self, val: &::js_sys::Object) -> &mut Self {
        self.set_salt_shim(val);
        self
    }
}
