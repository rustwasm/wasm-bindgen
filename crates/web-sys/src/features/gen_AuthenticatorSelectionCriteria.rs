#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = AuthenticatorSelectionCriteria)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `AuthenticatorSelectionCriteria` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticatorSelectionCriteria`*"]
    pub type AuthenticatorSelectionCriteria;
    #[wasm_bindgen(method, setter = "authenticatorAttachment")]
    fn authenticator_attachment_shim(this: &AuthenticatorSelectionCriteria, val: &str);
    #[wasm_bindgen(method, setter = "requireResidentKey")]
    fn require_resident_key_shim(this: &AuthenticatorSelectionCriteria, val: bool);
    #[wasm_bindgen(method, setter = "residentKey")]
    fn resident_key_shim(this: &AuthenticatorSelectionCriteria, val: &str);
    #[wasm_bindgen(method, setter = "userVerification")]
    fn user_verification_shim(this: &AuthenticatorSelectionCriteria, val: &str);
}
impl AuthenticatorSelectionCriteria {
    #[doc = "Construct a new `AuthenticatorSelectionCriteria`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticatorSelectionCriteria`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `authenticatorAttachment` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticatorSelectionCriteria`*"]
    pub fn authenticator_attachment(&mut self, val: &str) -> &mut Self {
        self.authenticator_attachment_shim(val);
        self
    }
    #[doc = "Change the `requireResidentKey` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticatorSelectionCriteria`*"]
    pub fn require_resident_key(&mut self, val: bool) -> &mut Self {
        self.require_resident_key_shim(val);
        self
    }
    #[doc = "Change the `residentKey` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticatorSelectionCriteria`*"]
    pub fn resident_key(&mut self, val: &str) -> &mut Self {
        self.resident_key_shim(val);
        self
    }
    #[doc = "Change the `userVerification` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticatorSelectionCriteria`*"]
    pub fn user_verification(&mut self, val: &str) -> &mut Self {
        self.user_verification_shim(val);
        self
    }
}
impl Default for AuthenticatorSelectionCriteria {
    fn default() -> Self {
        Self::new()
    }
}
