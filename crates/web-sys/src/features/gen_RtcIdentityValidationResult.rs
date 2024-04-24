#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = RTCIdentityValidationResult)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `RtcIdentityValidationResult` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIdentityValidationResult`*"]
    pub type RtcIdentityValidationResult;
    #[wasm_bindgen(method, getter = "contents")]
    fn contents_shim(this: &RtcIdentityValidationResult) -> String;
    #[wasm_bindgen(method, setter = "contents")]
    fn set_contents_shim(this: &RtcIdentityValidationResult, val: &str);
    #[wasm_bindgen(method, getter = "identity")]
    fn identity_shim(this: &RtcIdentityValidationResult) -> String;
    #[wasm_bindgen(method, setter = "identity")]
    fn set_identity_shim(this: &RtcIdentityValidationResult, val: &str);
}
#[doc = "The trait to access properties on the `RtcIdentityValidationResult` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `RtcIdentityValidationResult`*"]
pub trait RtcIdentityValidationResultGetters {
    #[doc = "Get the `contents` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIdentityValidationResult`*"]
    fn contents(&self) -> String;
    #[doc = "Get the `identity` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIdentityValidationResult`*"]
    fn identity(&self) -> String;
}
impl RtcIdentityValidationResultGetters for RtcIdentityValidationResult {
    fn contents(&self) -> String {
        self.contents_shim()
    }
    fn identity(&self) -> String {
        self.identity_shim()
    }
}
impl RtcIdentityValidationResult {
    #[doc = "Construct a new `RtcIdentityValidationResult`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIdentityValidationResult`*"]
    pub fn new(contents: &str, identity: &str) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        Self::contents(&mut ret, contents);
        Self::identity(&mut ret, identity);
        ret
    }
    #[doc = "Change the `contents` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIdentityValidationResult`*"]
    pub fn contents(&mut self, val: &str) -> &mut Self {
        self.set_contents_shim(val);
        self
    }
    #[doc = "Change the `identity` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIdentityValidationResult`*"]
    pub fn identity(&mut self, val: &str) -> &mut Self {
        self.set_identity_shim(val);
        self
    }
}
