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
    #[cfg(feature = "AuthenticatorAttachment")]
    #[doc = "Get the `authenticatorAttachment` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticatorAttachment`, `AuthenticatorSelectionCriteria`*"]
    #[wasm_bindgen(method, getter = "authenticatorAttachment")]
    pub fn get_authenticator_attachment(
        this: &AuthenticatorSelectionCriteria,
    ) -> Option<AuthenticatorAttachment>;
    #[cfg(feature = "AuthenticatorAttachment")]
    #[wasm_bindgen(method, setter = "authenticatorAttachment")]
    fn set_authenticator_attachment(
        this: &AuthenticatorSelectionCriteria,
        val: AuthenticatorAttachment,
    );
    #[doc = "Get the `requireResidentKey` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticatorSelectionCriteria`*"]
    #[wasm_bindgen(method, getter = "requireResidentKey")]
    pub fn get_require_resident_key(this: &AuthenticatorSelectionCriteria) -> Option<bool>;
    #[wasm_bindgen(method, setter = "requireResidentKey")]
    fn set_require_resident_key(this: &AuthenticatorSelectionCriteria, val: bool);
    #[cfg(feature = "UserVerificationRequirement")]
    #[doc = "Get the `userVerification` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticatorSelectionCriteria`, `UserVerificationRequirement`*"]
    #[wasm_bindgen(method, getter = "userVerification")]
    pub fn get_user_verification(
        this: &AuthenticatorSelectionCriteria,
    ) -> Option<UserVerificationRequirement>;
    #[cfg(feature = "UserVerificationRequirement")]
    #[wasm_bindgen(method, setter = "userVerification")]
    fn set_user_verification(
        this: &AuthenticatorSelectionCriteria,
        val: UserVerificationRequirement,
    );
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
    #[cfg(feature = "AuthenticatorAttachment")]
    #[doc = "Change the `authenticatorAttachment` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticatorAttachment`, `AuthenticatorSelectionCriteria`*"]
    pub fn authenticator_attachment(&mut self, val: AuthenticatorAttachment) -> &mut Self {
        self.set_authenticator_attachment(val);
        self
    }
    #[doc = "Change the `requireResidentKey` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticatorSelectionCriteria`*"]
    pub fn require_resident_key(&mut self, val: bool) -> &mut Self {
        self.set_require_resident_key(val);
        self
    }
    #[cfg(feature = "UserVerificationRequirement")]
    #[doc = "Change the `userVerification` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticatorSelectionCriteria`, `UserVerificationRequirement`*"]
    pub fn user_verification(&mut self, val: UserVerificationRequirement) -> &mut Self {
        self.set_user_verification(val);
        self
    }
}
impl Default for AuthenticatorSelectionCriteria {
    fn default() -> Self {
        Self::new()
    }
}
