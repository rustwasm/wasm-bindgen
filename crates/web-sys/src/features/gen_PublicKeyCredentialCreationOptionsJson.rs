#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = PublicKeyCredentialCreationOptionsJSON)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `PublicKeyCredentialCreationOptionsJson` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialCreationOptionsJson`*"]
    pub type PublicKeyCredentialCreationOptionsJson;
    #[wasm_bindgen(method, setter = "attestation")]
    fn attestation_shim(this: &PublicKeyCredentialCreationOptionsJson, val: &str);
    #[wasm_bindgen(method, setter = "attestationFormats")]
    fn attestation_formats_shim(
        this: &PublicKeyCredentialCreationOptionsJson,
        val: &::wasm_bindgen::JsValue,
    );
    #[cfg(feature = "AuthenticatorSelectionCriteria")]
    #[wasm_bindgen(method, setter = "authenticatorSelection")]
    fn authenticator_selection_shim(
        this: &PublicKeyCredentialCreationOptionsJson,
        val: &AuthenticatorSelectionCriteria,
    );
    #[wasm_bindgen(method, setter = "challenge")]
    fn challenge_shim(this: &PublicKeyCredentialCreationOptionsJson, val: &str);
    #[wasm_bindgen(method, setter = "excludeCredentials")]
    fn exclude_credentials_shim(
        this: &PublicKeyCredentialCreationOptionsJson,
        val: &::wasm_bindgen::JsValue,
    );
    #[cfg(feature = "AuthenticationExtensionsClientInputsJson")]
    #[wasm_bindgen(method, setter = "extensions")]
    fn extensions_shim(
        this: &PublicKeyCredentialCreationOptionsJson,
        val: &AuthenticationExtensionsClientInputsJson,
    );
    #[wasm_bindgen(method, setter = "hints")]
    fn hints_shim(this: &PublicKeyCredentialCreationOptionsJson, val: &::wasm_bindgen::JsValue);
    #[wasm_bindgen(method, setter = "pubKeyCredParams")]
    fn pub_key_cred_params_shim(
        this: &PublicKeyCredentialCreationOptionsJson,
        val: &::wasm_bindgen::JsValue,
    );
    #[cfg(feature = "PublicKeyCredentialRpEntity")]
    #[wasm_bindgen(method, setter = "rp")]
    fn rp_shim(this: &PublicKeyCredentialCreationOptionsJson, val: &PublicKeyCredentialRpEntity);
    #[wasm_bindgen(method, setter = "timeout")]
    fn timeout_shim(this: &PublicKeyCredentialCreationOptionsJson, val: u32);
    #[cfg(feature = "PublicKeyCredentialUserEntityJson")]
    #[wasm_bindgen(method, setter = "user")]
    fn user_shim(
        this: &PublicKeyCredentialCreationOptionsJson,
        val: &PublicKeyCredentialUserEntityJson,
    );
}
impl PublicKeyCredentialCreationOptionsJson {
    #[cfg(all(
        feature = "PublicKeyCredentialRpEntity",
        feature = "PublicKeyCredentialUserEntityJson",
    ))]
    #[doc = "Construct a new `PublicKeyCredentialCreationOptionsJson`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialCreationOptionsJson`, `PublicKeyCredentialRpEntity`, `PublicKeyCredentialUserEntityJson`*"]
    pub fn new(
        challenge: &str,
        pub_key_cred_params: &::wasm_bindgen::JsValue,
        rp: &PublicKeyCredentialRpEntity,
        user: &PublicKeyCredentialUserEntityJson,
    ) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.challenge(challenge);
        ret.pub_key_cred_params(pub_key_cred_params);
        ret.rp(rp);
        ret.user(user);
        ret
    }
    #[doc = "Change the `attestation` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialCreationOptionsJson`*"]
    pub fn attestation(&mut self, val: &str) -> &mut Self {
        self.attestation_shim(val);
        self
    }
    #[doc = "Change the `attestationFormats` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialCreationOptionsJson`*"]
    pub fn attestation_formats(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.attestation_formats_shim(val);
        self
    }
    #[cfg(feature = "AuthenticatorSelectionCriteria")]
    #[doc = "Change the `authenticatorSelection` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticatorSelectionCriteria`, `PublicKeyCredentialCreationOptionsJson`*"]
    pub fn authenticator_selection(&mut self, val: &AuthenticatorSelectionCriteria) -> &mut Self {
        self.authenticator_selection_shim(val);
        self
    }
    #[doc = "Change the `challenge` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialCreationOptionsJson`*"]
    pub fn challenge(&mut self, val: &str) -> &mut Self {
        self.challenge_shim(val);
        self
    }
    #[doc = "Change the `excludeCredentials` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialCreationOptionsJson`*"]
    pub fn exclude_credentials(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.exclude_credentials_shim(val);
        self
    }
    #[cfg(feature = "AuthenticationExtensionsClientInputsJson")]
    #[doc = "Change the `extensions` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticationExtensionsClientInputsJson`, `PublicKeyCredentialCreationOptionsJson`*"]
    pub fn extensions(&mut self, val: &AuthenticationExtensionsClientInputsJson) -> &mut Self {
        self.extensions_shim(val);
        self
    }
    #[doc = "Change the `hints` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialCreationOptionsJson`*"]
    pub fn hints(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.hints_shim(val);
        self
    }
    #[doc = "Change the `pubKeyCredParams` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialCreationOptionsJson`*"]
    pub fn pub_key_cred_params(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.pub_key_cred_params_shim(val);
        self
    }
    #[cfg(feature = "PublicKeyCredentialRpEntity")]
    #[doc = "Change the `rp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialCreationOptionsJson`, `PublicKeyCredentialRpEntity`*"]
    pub fn rp(&mut self, val: &PublicKeyCredentialRpEntity) -> &mut Self {
        self.rp_shim(val);
        self
    }
    #[doc = "Change the `timeout` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialCreationOptionsJson`*"]
    pub fn timeout(&mut self, val: u32) -> &mut Self {
        self.timeout_shim(val);
        self
    }
    #[cfg(feature = "PublicKeyCredentialUserEntityJson")]
    #[doc = "Change the `user` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialCreationOptionsJson`, `PublicKeyCredentialUserEntityJson`*"]
    pub fn user(&mut self, val: &PublicKeyCredentialUserEntityJson) -> &mut Self {
        self.user_shim(val);
        self
    }
}
