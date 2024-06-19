#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = AesCbcParams)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `AesCbcParams` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AesCbcParams`*"]
    pub type AesCbcParams;
    #[doc = "Get the `name` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AesCbcParams`*"]
    #[wasm_bindgen(method, getter = "name")]
    pub fn get_name(this: &AesCbcParams) -> String;
    #[wasm_bindgen(method, setter = "name")]
    fn set_name(this: &AesCbcParams, val: &str);
    #[doc = "Get the `iv` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AesCbcParams`*"]
    #[wasm_bindgen(method, getter = "iv")]
    pub fn get_iv(this: &AesCbcParams) -> ::js_sys::Object;
    #[wasm_bindgen(method, setter = "iv")]
    fn set_iv(this: &AesCbcParams, val: &::js_sys::Object);
}
impl AesCbcParams {
    #[doc = "Construct a new `AesCbcParams`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AesCbcParams`*"]
    pub fn new(name: &str, iv: &::js_sys::Object) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.name(name);
        ret.iv(iv);
        ret
    }
    #[doc = "Change the `name` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AesCbcParams`*"]
    pub fn name(&mut self, val: &str) -> &mut Self {
        self.set_name(val);
        self
    }
    #[doc = "Change the `iv` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AesCbcParams`*"]
    pub fn iv(&mut self, val: &::js_sys::Object) -> &mut Self {
        self.set_iv(val);
        self
    }
}
