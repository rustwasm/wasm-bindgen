#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = PublicKeyCredentialCreationOptions)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `PublicKeyCredentialCreationOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialCreationOptions`*"]
    pub type PublicKeyCredentialCreationOptions;
    #[cfg(feature = "AttestationConveyancePreference")]
    #[wasm_bindgen(method, setter = "attestation")]
    fn attestation_shim(
        this: &PublicKeyCredentialCreationOptions,
        val: AttestationConveyancePreference,
    );
    #[cfg(feature = "AuthenticatorSelectionCriteria")]
    #[wasm_bindgen(method, setter = "authenticatorSelection")]
    fn authenticator_selection_shim(
        this: &PublicKeyCredentialCreationOptions,
        val: &AuthenticatorSelectionCriteria,
    );
    #[wasm_bindgen(method, setter = "challenge")]
    fn challenge_shim(this: &PublicKeyCredentialCreationOptions, val: &::js_sys::Object);
    #[wasm_bindgen(method, setter = "excludeCredentials")]
    fn exclude_credentials_shim(
        this: &PublicKeyCredentialCreationOptions,
        val: &::wasm_bindgen::JsValue,
    );
    #[cfg(feature = "AuthenticationExtensionsClientInputs")]
    #[wasm_bindgen(method, setter = "extensions")]
    fn extensions_shim(
        this: &PublicKeyCredentialCreationOptions,
        val: &AuthenticationExtensionsClientInputs,
    );
    #[wasm_bindgen(method, setter = "pubKeyCredParams")]
    fn pub_key_cred_params_shim(
        this: &PublicKeyCredentialCreationOptions,
        val: &::wasm_bindgen::JsValue,
    );
    #[cfg(feature = "PublicKeyCredentialRpEntity")]
    #[wasm_bindgen(method, setter = "rp")]
    fn rp_shim(this: &PublicKeyCredentialCreationOptions, val: &PublicKeyCredentialRpEntity);
    #[wasm_bindgen(method, setter = "timeout")]
    fn timeout_shim(this: &PublicKeyCredentialCreationOptions, val: u32);
    #[cfg(feature = "PublicKeyCredentialUserEntity")]
    #[wasm_bindgen(method, setter = "user")]
    fn user_shim(this: &PublicKeyCredentialCreationOptions, val: &PublicKeyCredentialUserEntity);
}
impl PublicKeyCredentialCreationOptions {
    #[cfg(all(
        feature = "PublicKeyCredentialRpEntity",
        feature = "PublicKeyCredentialUserEntity",
    ))]
    #[doc = "Construct a new `PublicKeyCredentialCreationOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialCreationOptions`, `PublicKeyCredentialRpEntity`, `PublicKeyCredentialUserEntity`*"]
    pub fn new(
        challenge: &::js_sys::Object,
        pub_key_cred_params: &::wasm_bindgen::JsValue,
        rp: &PublicKeyCredentialRpEntity,
        user: &PublicKeyCredentialUserEntity,
    ) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.challenge(challenge);
        ret.pub_key_cred_params(pub_key_cred_params);
        ret.rp(rp);
        ret.user(user);
        ret
    }
    #[cfg(feature = "AttestationConveyancePreference")]
    #[doc = "Change the `attestation` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AttestationConveyancePreference`, `PublicKeyCredentialCreationOptions`*"]
    pub fn attestation(&mut self, val: AttestationConveyancePreference) -> &mut Self {
        self.attestation_shim(val);
        self
    }
    #[cfg(feature = "AuthenticatorSelectionCriteria")]
    #[doc = "Change the `authenticatorSelection` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticatorSelectionCriteria`, `PublicKeyCredentialCreationOptions`*"]
    pub fn authenticator_selection(&mut self, val: &AuthenticatorSelectionCriteria) -> &mut Self {
        self.authenticator_selection_shim(val);
        self
    }
    #[doc = "Change the `challenge` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialCreationOptions`*"]
    pub fn challenge(&mut self, val: &::js_sys::Object) -> &mut Self {
        self.challenge_shim(val);
        self
    }
    #[doc = "Change the `excludeCredentials` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialCreationOptions`*"]
    pub fn exclude_credentials(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.exclude_credentials_shim(val);
        self
    }
    #[cfg(feature = "AuthenticationExtensionsClientInputs")]
    #[doc = "Change the `extensions` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticationExtensionsClientInputs`, `PublicKeyCredentialCreationOptions`*"]
    pub fn extensions(&mut self, val: &AuthenticationExtensionsClientInputs) -> &mut Self {
        self.extensions_shim(val);
        self
    }
    #[doc = "Change the `pubKeyCredParams` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialCreationOptions`*"]
    pub fn pub_key_cred_params(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.pub_key_cred_params_shim(val);
        self
    }
    #[cfg(feature = "PublicKeyCredentialRpEntity")]
    #[doc = "Change the `rp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialCreationOptions`, `PublicKeyCredentialRpEntity`*"]
    pub fn rp(&mut self, val: &PublicKeyCredentialRpEntity) -> &mut Self {
        self.rp_shim(val);
        self
    }
    #[doc = "Change the `timeout` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialCreationOptions`*"]
    pub fn timeout(&mut self, val: u32) -> &mut Self {
        self.timeout_shim(val);
        self
    }
    #[cfg(feature = "PublicKeyCredentialUserEntity")]
    #[doc = "Change the `user` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialCreationOptions`, `PublicKeyCredentialUserEntity`*"]
    pub fn user(&mut self, val: &PublicKeyCredentialUserEntity) -> &mut Self {
        self.user_shim(val);
        self
    }
}
