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
    #[doc = "Get the `attestation` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AttestationConveyancePreference`, `PublicKeyCredentialCreationOptions`*"]
    #[wasm_bindgen(method, getter = "attestation")]
    pub fn get_attestation(
        this: &PublicKeyCredentialCreationOptions,
    ) -> Option<AttestationConveyancePreference>;
    #[cfg(feature = "AttestationConveyancePreference")]
    #[wasm_bindgen(method, setter = "attestation")]
    fn set_attestation(
        this: &PublicKeyCredentialCreationOptions,
        val: AttestationConveyancePreference,
    );
    #[cfg(feature = "AuthenticatorSelectionCriteria")]
    #[doc = "Get the `authenticatorSelection` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticatorSelectionCriteria`, `PublicKeyCredentialCreationOptions`*"]
    #[wasm_bindgen(method, getter = "authenticatorSelection")]
    pub fn get_authenticator_selection(
        this: &PublicKeyCredentialCreationOptions,
    ) -> Option<AuthenticatorSelectionCriteria>;
    #[cfg(feature = "AuthenticatorSelectionCriteria")]
    #[wasm_bindgen(method, setter = "authenticatorSelection")]
    fn set_authenticator_selection(
        this: &PublicKeyCredentialCreationOptions,
        val: &AuthenticatorSelectionCriteria,
    );
    #[doc = "Get the `challenge` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialCreationOptions`*"]
    #[wasm_bindgen(method, getter = "challenge")]
    pub fn get_challenge(this: &PublicKeyCredentialCreationOptions) -> ::js_sys::Object;
    #[wasm_bindgen(method, setter = "challenge")]
    fn set_challenge(this: &PublicKeyCredentialCreationOptions, val: &::js_sys::Object);
    #[doc = "Get the `excludeCredentials` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialCreationOptions`*"]
    #[wasm_bindgen(method, getter = "excludeCredentials")]
    pub fn get_exclude_credentials(
        this: &PublicKeyCredentialCreationOptions,
    ) -> Option<::js_sys::Array>;
    #[wasm_bindgen(method, setter = "excludeCredentials")]
    fn set_exclude_credentials(
        this: &PublicKeyCredentialCreationOptions,
        val: &::wasm_bindgen::JsValue,
    );
    #[cfg(feature = "AuthenticationExtensionsClientInputs")]
    #[doc = "Get the `extensions` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticationExtensionsClientInputs`, `PublicKeyCredentialCreationOptions`*"]
    #[wasm_bindgen(method, getter = "extensions")]
    pub fn get_extensions(
        this: &PublicKeyCredentialCreationOptions,
    ) -> Option<AuthenticationExtensionsClientInputs>;
    #[cfg(feature = "AuthenticationExtensionsClientInputs")]
    #[wasm_bindgen(method, setter = "extensions")]
    fn set_extensions(
        this: &PublicKeyCredentialCreationOptions,
        val: &AuthenticationExtensionsClientInputs,
    );
    #[doc = "Get the `pubKeyCredParams` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialCreationOptions`*"]
    #[wasm_bindgen(method, getter = "pubKeyCredParams")]
    pub fn get_pub_key_cred_params(this: &PublicKeyCredentialCreationOptions) -> ::js_sys::Array;
    #[wasm_bindgen(method, setter = "pubKeyCredParams")]
    fn set_pub_key_cred_params(
        this: &PublicKeyCredentialCreationOptions,
        val: &::wasm_bindgen::JsValue,
    );
    #[cfg(feature = "PublicKeyCredentialRpEntity")]
    #[doc = "Get the `rp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialCreationOptions`, `PublicKeyCredentialRpEntity`*"]
    #[wasm_bindgen(method, getter = "rp")]
    pub fn get_rp(this: &PublicKeyCredentialCreationOptions) -> PublicKeyCredentialRpEntity;
    #[cfg(feature = "PublicKeyCredentialRpEntity")]
    #[wasm_bindgen(method, setter = "rp")]
    fn set_rp(this: &PublicKeyCredentialCreationOptions, val: &PublicKeyCredentialRpEntity);
    #[doc = "Get the `timeout` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialCreationOptions`*"]
    #[wasm_bindgen(method, getter = "timeout")]
    pub fn get_timeout(this: &PublicKeyCredentialCreationOptions) -> Option<u32>;
    #[wasm_bindgen(method, setter = "timeout")]
    fn set_timeout(this: &PublicKeyCredentialCreationOptions, val: u32);
    #[cfg(feature = "PublicKeyCredentialUserEntity")]
    #[doc = "Get the `user` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialCreationOptions`, `PublicKeyCredentialUserEntity`*"]
    #[wasm_bindgen(method, getter = "user")]
    pub fn get_user(this: &PublicKeyCredentialCreationOptions) -> PublicKeyCredentialUserEntity;
    #[cfg(feature = "PublicKeyCredentialUserEntity")]
    #[wasm_bindgen(method, setter = "user")]
    fn set_user(this: &PublicKeyCredentialCreationOptions, val: &PublicKeyCredentialUserEntity);
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
        self.set_attestation(val);
        self
    }
    #[cfg(feature = "AuthenticatorSelectionCriteria")]
    #[doc = "Change the `authenticatorSelection` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticatorSelectionCriteria`, `PublicKeyCredentialCreationOptions`*"]
    pub fn authenticator_selection(&mut self, val: &AuthenticatorSelectionCriteria) -> &mut Self {
        self.set_authenticator_selection(val);
        self
    }
    #[doc = "Change the `challenge` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialCreationOptions`*"]
    pub fn challenge(&mut self, val: &::js_sys::Object) -> &mut Self {
        self.set_challenge(val);
        self
    }
    #[doc = "Change the `excludeCredentials` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialCreationOptions`*"]
    pub fn exclude_credentials(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_exclude_credentials(val);
        self
    }
    #[cfg(feature = "AuthenticationExtensionsClientInputs")]
    #[doc = "Change the `extensions` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticationExtensionsClientInputs`, `PublicKeyCredentialCreationOptions`*"]
    pub fn extensions(&mut self, val: &AuthenticationExtensionsClientInputs) -> &mut Self {
        self.set_extensions(val);
        self
    }
    #[doc = "Change the `pubKeyCredParams` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialCreationOptions`*"]
    pub fn pub_key_cred_params(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_pub_key_cred_params(val);
        self
    }
    #[cfg(feature = "PublicKeyCredentialRpEntity")]
    #[doc = "Change the `rp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialCreationOptions`, `PublicKeyCredentialRpEntity`*"]
    pub fn rp(&mut self, val: &PublicKeyCredentialRpEntity) -> &mut Self {
        self.set_rp(val);
        self
    }
    #[doc = "Change the `timeout` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialCreationOptions`*"]
    pub fn timeout(&mut self, val: u32) -> &mut Self {
        self.set_timeout(val);
        self
    }
    #[cfg(feature = "PublicKeyCredentialUserEntity")]
    #[doc = "Change the `user` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialCreationOptions`, `PublicKeyCredentialUserEntity`*"]
    pub fn user(&mut self, val: &PublicKeyCredentialUserEntity) -> &mut Self {
        self.set_user(val);
        self
    }
}
