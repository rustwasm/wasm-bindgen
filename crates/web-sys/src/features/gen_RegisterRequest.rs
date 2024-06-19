#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = RegisterRequest)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `RegisterRequest` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RegisterRequest`*"]
    pub type RegisterRequest;
    #[doc = "Get the `challenge` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RegisterRequest`*"]
    #[wasm_bindgen(method, getter = "challenge")]
    pub fn get_challenge(this: &RegisterRequest) -> Option<String>;
    #[wasm_bindgen(method, setter = "challenge")]
    fn set_challenge(this: &RegisterRequest, val: &str);
    #[doc = "Get the `version` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RegisterRequest`*"]
    #[wasm_bindgen(method, getter = "version")]
    pub fn get_version(this: &RegisterRequest) -> Option<String>;
    #[wasm_bindgen(method, setter = "version")]
    fn set_version(this: &RegisterRequest, val: &str);
}
impl RegisterRequest {
    #[doc = "Construct a new `RegisterRequest`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RegisterRequest`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `challenge` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RegisterRequest`*"]
    pub fn challenge(&mut self, val: &str) -> &mut Self {
        self.set_challenge(val);
        self
    }
    #[doc = "Change the `version` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RegisterRequest`*"]
    pub fn version(&mut self, val: &str) -> &mut Self {
        self.set_version(val);
        self
    }
}
impl Default for RegisterRequest {
    fn default() -> Self {
        Self::new()
    }
}
