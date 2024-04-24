#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = PublicKeyCredentialRequestOptions)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `PublicKeyCredentialRequestOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialRequestOptions`*"]
    pub type PublicKeyCredentialRequestOptions;
    #[wasm_bindgen(method, getter = "allowCredentials")]
    fn allow_credentials_shim(this: &PublicKeyCredentialRequestOptions)
        -> &::wasm_bindgen::JsValue;
    #[wasm_bindgen(method, setter = "allowCredentials")]
    fn set_allow_credentials_shim(
        this: &PublicKeyCredentialRequestOptions,
        val: &::wasm_bindgen::JsValue,
    );
    #[wasm_bindgen(method, getter = "challenge")]
    fn challenge_shim(this: &PublicKeyCredentialRequestOptions) -> &::js_sys::Object;
    #[wasm_bindgen(method, setter = "challenge")]
    fn set_challenge_shim(this: &PublicKeyCredentialRequestOptions, val: &::js_sys::Object);
    #[cfg(feature = "AuthenticationExtensionsClientInputs")]
    #[wasm_bindgen(method, getter = "extensions")]
    fn extensions_shim(
        this: &PublicKeyCredentialRequestOptions,
    ) -> &AuthenticationExtensionsClientInputs;
    #[cfg(feature = "AuthenticationExtensionsClientInputs")]
    #[wasm_bindgen(method, setter = "extensions")]
    fn set_extensions_shim(
        this: &PublicKeyCredentialRequestOptions,
        val: &AuthenticationExtensionsClientInputs,
    );
    #[wasm_bindgen(method, getter = "rpId")]
    fn rp_id_shim(this: &PublicKeyCredentialRequestOptions) -> &str;
    #[wasm_bindgen(method, setter = "rpId")]
    fn set_rp_id_shim(this: &PublicKeyCredentialRequestOptions, val: &str);
    #[wasm_bindgen(method, getter = "timeout")]
    fn timeout_shim(this: &PublicKeyCredentialRequestOptions) -> u32;
    #[wasm_bindgen(method, setter = "timeout")]
    fn set_timeout_shim(this: &PublicKeyCredentialRequestOptions, val: u32);
    #[cfg(feature = "UserVerificationRequirement")]
    #[wasm_bindgen(method, getter = "userVerification")]
    fn user_verification_shim(
        this: &PublicKeyCredentialRequestOptions,
    ) -> UserVerificationRequirement;
    #[cfg(feature = "UserVerificationRequirement")]
    #[wasm_bindgen(method, setter = "userVerification")]
    fn set_user_verification_shim(
        this: &PublicKeyCredentialRequestOptions,
        val: UserVerificationRequirement,
    );
}
#[doc = "The trait to access properties on the `PublicKeyCredentialRequestOptions` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialRequestOptions`*"]
pub trait PublicKeyCredentialRequestOptionsGetters {
    #[doc = "Get the `allowCredentials` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialRequestOptions`*"]
    fn allow_credentials(&self) -> &::wasm_bindgen::JsValue;
    #[doc = "Get the `challenge` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialRequestOptions`*"]
    fn challenge(&self) -> &::js_sys::Object;
    #[cfg(feature = "AuthenticationExtensionsClientInputs")]
    #[doc = "Get the `extensions` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticationExtensionsClientInputs`, `PublicKeyCredentialRequestOptions`*"]
    fn extensions(&self) -> &AuthenticationExtensionsClientInputs;
    #[doc = "Get the `rpId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialRequestOptions`*"]
    fn rp_id(&self) -> &str;
    #[doc = "Get the `timeout` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialRequestOptions`*"]
    fn timeout(&self) -> u32;
    #[cfg(feature = "UserVerificationRequirement")]
    #[doc = "Get the `userVerification` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialRequestOptions`, `UserVerificationRequirement`*"]
    fn user_verification(&self) -> UserVerificationRequirement;
}
impl PublicKeyCredentialRequestOptionsGetters for PublicKeyCredentialRequestOptions {
    fn allow_credentials(&self) -> &::wasm_bindgen::JsValue {
        self.allow_credentials_shim()
    }
    fn challenge(&self) -> &::js_sys::Object {
        self.challenge_shim()
    }
    #[cfg(feature = "AuthenticationExtensionsClientInputs")]
    fn extensions(&self) -> &AuthenticationExtensionsClientInputs {
        self.extensions_shim()
    }
    fn rp_id(&self) -> &str {
        self.rp_id_shim()
    }
    fn timeout(&self) -> u32 {
        self.timeout_shim()
    }
    #[cfg(feature = "UserVerificationRequirement")]
    fn user_verification(&self) -> UserVerificationRequirement {
        self.user_verification_shim()
    }
}
impl PublicKeyCredentialRequestOptions {
    #[doc = "Construct a new `PublicKeyCredentialRequestOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialRequestOptions`*"]
    pub fn new(challenge: &::js_sys::Object) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.challenge(challenge);
        ret
    }
    #[doc = "Change the `allowCredentials` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialRequestOptions`*"]
    pub fn allow_credentials(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_allow_credentials_shim(val);
        self
    }
    #[doc = "Change the `challenge` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialRequestOptions`*"]
    pub fn challenge(&mut self, val: &::js_sys::Object) -> &mut Self {
        self.set_challenge_shim(val);
        self
    }
    #[cfg(feature = "AuthenticationExtensionsClientInputs")]
    #[doc = "Change the `extensions` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticationExtensionsClientInputs`, `PublicKeyCredentialRequestOptions`*"]
    pub fn extensions(&mut self, val: &AuthenticationExtensionsClientInputs) -> &mut Self {
        self.set_extensions_shim(val);
        self
    }
    #[doc = "Change the `rpId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialRequestOptions`*"]
    pub fn rp_id(&mut self, val: &str) -> &mut Self {
        self.set_rp_id_shim(val);
        self
    }
    #[doc = "Change the `timeout` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialRequestOptions`*"]
    pub fn timeout(&mut self, val: u32) -> &mut Self {
        self.set_timeout_shim(val);
        self
    }
    #[cfg(feature = "UserVerificationRequirement")]
    #[doc = "Change the `userVerification` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialRequestOptions`, `UserVerificationRequirement`*"]
    pub fn user_verification(&mut self, val: UserVerificationRequirement) -> &mut Self {
        self.set_user_verification_shim(val);
        self
    }
}
