#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = RsaPssParams)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `RsaPssParams` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RsaPssParams`*"]
    pub type RsaPssParams;
    #[doc = "Get the `name` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RsaPssParams`*"]
    #[wasm_bindgen(method, getter = "name")]
    pub fn get_name(this: &RsaPssParams) -> String;
    #[wasm_bindgen(method, setter = "name")]
    fn set_name(this: &RsaPssParams, val: &str);
    #[doc = "Get the `saltLength` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RsaPssParams`*"]
    #[wasm_bindgen(method, getter = "saltLength")]
    pub fn get_salt_length(this: &RsaPssParams) -> u32;
    #[wasm_bindgen(method, setter = "saltLength")]
    fn set_salt_length(this: &RsaPssParams, val: u32);
}
impl RsaPssParams {
    #[doc = "Construct a new `RsaPssParams`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RsaPssParams`*"]
    pub fn new(name: &str, salt_length: u32) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.name(name);
        ret.salt_length(salt_length);
        ret
    }
    #[doc = "Change the `name` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RsaPssParams`*"]
    pub fn name(&mut self, val: &str) -> &mut Self {
        self.set_name(val);
        self
    }
    #[doc = "Change the `saltLength` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RsaPssParams`*"]
    pub fn salt_length(&mut self, val: u32) -> &mut Self {
        self.set_salt_length(val);
        self
    }
}
