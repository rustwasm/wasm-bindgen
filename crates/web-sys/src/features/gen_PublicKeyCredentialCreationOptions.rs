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
    #[wasm_bindgen(method, getter = "attestation")]
    fn attestation_shim(
        this: &PublicKeyCredentialCreationOptions,
    ) -> AttestationConveyancePreference;
    #[cfg(feature = "AttestationConveyancePreference")]
    #[wasm_bindgen(method, setter = "attestation")]
    fn set_attestation_shim(
        this: &PublicKeyCredentialCreationOptions,
        val: AttestationConveyancePreference,
    );
    #[cfg(feature = "AuthenticatorSelectionCriteria")]
    #[wasm_bindgen(method, getter = "authenticatorSelection")]
    fn authenticator_selection_shim(
        this: &PublicKeyCredentialCreationOptions,
    ) -> AuthenticatorSelectionCriteria;
    #[cfg(feature = "AuthenticatorSelectionCriteria")]
    #[wasm_bindgen(method, setter = "authenticatorSelection")]
    fn set_authenticator_selection_shim(
        this: &PublicKeyCredentialCreationOptions,
        val: &AuthenticatorSelectionCriteria,
    );
    #[wasm_bindgen(method, getter = "challenge")]
    fn challenge_shim(this: &PublicKeyCredentialCreationOptions) -> ::js_sys::Object;
    #[wasm_bindgen(method, setter = "challenge")]
    fn set_challenge_shim(this: &PublicKeyCredentialCreationOptions, val: &::js_sys::Object);
    #[wasm_bindgen(method, getter = "excludeCredentials")]
    fn exclude_credentials_shim(this: &PublicKeyCredentialCreationOptions) -> ::js_sys::Array;
    #[wasm_bindgen(method, setter = "excludeCredentials")]
    fn set_exclude_credentials_shim(
        this: &PublicKeyCredentialCreationOptions,
        val: &::wasm_bindgen::JsValue,
    );
    #[cfg(feature = "AuthenticationExtensionsClientInputs")]
    #[wasm_bindgen(method, getter = "extensions")]
    fn extensions_shim(
        this: &PublicKeyCredentialCreationOptions,
    ) -> AuthenticationExtensionsClientInputs;
    #[cfg(feature = "AuthenticationExtensionsClientInputs")]
    #[wasm_bindgen(method, setter = "extensions")]
    fn set_extensions_shim(
        this: &PublicKeyCredentialCreationOptions,
        val: &AuthenticationExtensionsClientInputs,
    );
    #[wasm_bindgen(method, getter = "pubKeyCredParams")]
    fn pub_key_cred_params_shim(this: &PublicKeyCredentialCreationOptions) -> ::js_sys::Array;
    #[wasm_bindgen(method, setter = "pubKeyCredParams")]
    fn set_pub_key_cred_params_shim(
        this: &PublicKeyCredentialCreationOptions,
        val: &::wasm_bindgen::JsValue,
    );
    #[cfg(feature = "PublicKeyCredentialRpEntity")]
    #[wasm_bindgen(method, getter = "rp")]
    fn rp_shim(this: &PublicKeyCredentialCreationOptions) -> PublicKeyCredentialRpEntity;
    #[cfg(feature = "PublicKeyCredentialRpEntity")]
    #[wasm_bindgen(method, setter = "rp")]
    fn set_rp_shim(this: &PublicKeyCredentialCreationOptions, val: &PublicKeyCredentialRpEntity);
    #[wasm_bindgen(method, getter = "timeout")]
    fn timeout_shim(this: &PublicKeyCredentialCreationOptions) -> u32;
    #[wasm_bindgen(method, setter = "timeout")]
    fn set_timeout_shim(this: &PublicKeyCredentialCreationOptions, val: u32);
    #[cfg(feature = "PublicKeyCredentialUserEntity")]
    #[wasm_bindgen(method, getter = "user")]
    fn user_shim(this: &PublicKeyCredentialCreationOptions) -> PublicKeyCredentialUserEntity;
    #[cfg(feature = "PublicKeyCredentialUserEntity")]
    #[wasm_bindgen(method, setter = "user")]
    fn set_user_shim(
        this: &PublicKeyCredentialCreationOptions,
        val: &PublicKeyCredentialUserEntity,
    );
}
#[doc = "The trait to access properties on the `PublicKeyCredentialCreationOptions` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialCreationOptions`*"]
pub trait PublicKeyCredentialCreationOptionsGetters {
    #[cfg(feature = "AttestationConveyancePreference")]
    #[doc = "Get the `attestation` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AttestationConveyancePreference`, `PublicKeyCredentialCreationOptions`*"]
    fn attestation(&self) -> AttestationConveyancePreference;
    #[cfg(feature = "AuthenticatorSelectionCriteria")]
    #[doc = "Get the `authenticatorSelection` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticatorSelectionCriteria`, `PublicKeyCredentialCreationOptions`*"]
    fn authenticator_selection(&self) -> AuthenticatorSelectionCriteria;
    #[doc = "Get the `challenge` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialCreationOptions`*"]
    fn challenge(&self) -> ::js_sys::Object;
    #[doc = "Get the `excludeCredentials` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialCreationOptions`*"]
    fn exclude_credentials(&self) -> ::js_sys::Array;
    #[cfg(feature = "AuthenticationExtensionsClientInputs")]
    #[doc = "Get the `extensions` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticationExtensionsClientInputs`, `PublicKeyCredentialCreationOptions`*"]
    fn extensions(&self) -> AuthenticationExtensionsClientInputs;
    #[doc = "Get the `pubKeyCredParams` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialCreationOptions`*"]
    fn pub_key_cred_params(&self) -> ::js_sys::Array;
    #[cfg(feature = "PublicKeyCredentialRpEntity")]
    #[doc = "Get the `rp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialCreationOptions`, `PublicKeyCredentialRpEntity`*"]
    fn rp(&self) -> PublicKeyCredentialRpEntity;
    #[doc = "Get the `timeout` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialCreationOptions`*"]
    fn timeout(&self) -> u32;
    #[cfg(feature = "PublicKeyCredentialUserEntity")]
    #[doc = "Get the `user` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialCreationOptions`, `PublicKeyCredentialUserEntity`*"]
    fn user(&self) -> PublicKeyCredentialUserEntity;
}
impl PublicKeyCredentialCreationOptionsGetters for PublicKeyCredentialCreationOptions {
    #[cfg(feature = "AttestationConveyancePreference")]
    fn attestation(&self) -> AttestationConveyancePreference {
        self.attestation_shim()
    }
    #[cfg(feature = "AuthenticatorSelectionCriteria")]
    fn authenticator_selection(&self) -> AuthenticatorSelectionCriteria {
        self.authenticator_selection_shim()
    }
    fn challenge(&self) -> ::js_sys::Object {
        self.challenge_shim()
    }
    fn exclude_credentials(&self) -> ::js_sys::Array {
        self.exclude_credentials_shim()
    }
    #[cfg(feature = "AuthenticationExtensionsClientInputs")]
    fn extensions(&self) -> AuthenticationExtensionsClientInputs {
        self.extensions_shim()
    }
    fn pub_key_cred_params(&self) -> ::js_sys::Array {
        self.pub_key_cred_params_shim()
    }
    #[cfg(feature = "PublicKeyCredentialRpEntity")]
    fn rp(&self) -> PublicKeyCredentialRpEntity {
        self.rp_shim()
    }
    fn timeout(&self) -> u32 {
        self.timeout_shim()
    }
    #[cfg(feature = "PublicKeyCredentialUserEntity")]
    fn user(&self) -> PublicKeyCredentialUserEntity {
        self.user_shim()
    }
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
        Self::challenge(&mut ret, challenge);
        Self::pub_key_cred_params(&mut ret, pub_key_cred_params);
        Self::rp(&mut ret, rp);
        Self::user(&mut ret, user);
        ret
    }
    #[cfg(feature = "AttestationConveyancePreference")]
    #[doc = "Change the `attestation` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AttestationConveyancePreference`, `PublicKeyCredentialCreationOptions`*"]
    pub fn attestation(&mut self, val: AttestationConveyancePreference) -> &mut Self {
        self.set_attestation_shim(val);
        self
    }
    #[cfg(feature = "AuthenticatorSelectionCriteria")]
    #[doc = "Change the `authenticatorSelection` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticatorSelectionCriteria`, `PublicKeyCredentialCreationOptions`*"]
    pub fn authenticator_selection(&mut self, val: &AuthenticatorSelectionCriteria) -> &mut Self {
        self.set_authenticator_selection_shim(val);
        self
    }
    #[doc = "Change the `challenge` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialCreationOptions`*"]
    pub fn challenge(&mut self, val: &::js_sys::Object) -> &mut Self {
        self.set_challenge_shim(val);
        self
    }
    #[doc = "Change the `excludeCredentials` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialCreationOptions`*"]
    pub fn exclude_credentials(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_exclude_credentials_shim(val);
        self
    }
    #[cfg(feature = "AuthenticationExtensionsClientInputs")]
    #[doc = "Change the `extensions` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticationExtensionsClientInputs`, `PublicKeyCredentialCreationOptions`*"]
    pub fn extensions(&mut self, val: &AuthenticationExtensionsClientInputs) -> &mut Self {
        self.set_extensions_shim(val);
        self
    }
    #[doc = "Change the `pubKeyCredParams` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialCreationOptions`*"]
    pub fn pub_key_cred_params(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_pub_key_cred_params_shim(val);
        self
    }
    #[cfg(feature = "PublicKeyCredentialRpEntity")]
    #[doc = "Change the `rp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialCreationOptions`, `PublicKeyCredentialRpEntity`*"]
    pub fn rp(&mut self, val: &PublicKeyCredentialRpEntity) -> &mut Self {
        self.set_rp_shim(val);
        self
    }
    #[doc = "Change the `timeout` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialCreationOptions`*"]
    pub fn timeout(&mut self, val: u32) -> &mut Self {
        self.set_timeout_shim(val);
        self
    }
    #[cfg(feature = "PublicKeyCredentialUserEntity")]
    #[doc = "Change the `user` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialCreationOptions`, `PublicKeyCredentialUserEntity`*"]
    pub fn user(&mut self, val: &PublicKeyCredentialUserEntity) -> &mut Self {
        self.set_user_shim(val);
        self
    }
}
