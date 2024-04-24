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
    #[wasm_bindgen(method, getter = "challenge")]
    fn challenge_shim(this: &RegisterRequest) -> &str;
    #[wasm_bindgen(method, setter = "challenge")]
    fn set_challenge_shim(this: &RegisterRequest, val: &str);
    #[wasm_bindgen(method, getter = "version")]
    fn version_shim(this: &RegisterRequest) -> &str;
    #[wasm_bindgen(method, setter = "version")]
    fn set_version_shim(this: &RegisterRequest, val: &str);
}
#[doc = "The trait to access properties on the `RegisterRequest` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `RegisterRequest`*"]
pub trait RegisterRequestGetters {
    #[doc = "Get the `challenge` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RegisterRequest`*"]
    fn challenge(&self) -> &str;
    #[doc = "Get the `version` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RegisterRequest`*"]
    fn version(&self) -> &str;
}
impl RegisterRequestGetters for RegisterRequest {
    fn challenge(&self) -> &str {
        self.challenge_shim()
    }
    fn version(&self) -> &str {
        self.version_shim()
    }
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
        self.set_challenge_shim(val);
        self
    }
    #[doc = "Change the `version` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RegisterRequest`*"]
    pub fn version(&mut self, val: &str) -> &mut Self {
        self.set_version_shim(val);
        self
    }
}
impl Default for RegisterRequest {
    fn default() -> Self {
        Self::new()
    }
}
