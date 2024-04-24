#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = PublicKeyCredentialRequestOptionsJSON)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `PublicKeyCredentialRequestOptionsJson` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialRequestOptionsJson`*"]
    pub type PublicKeyCredentialRequestOptionsJson;
    #[wasm_bindgen(method, setter = "allowCredentials")]
    fn allow_credentials_shim(
        this: &PublicKeyCredentialRequestOptionsJson,
        val: &::wasm_bindgen::JsValue,
    );
    #[wasm_bindgen(method, setter = "challenge")]
    fn challenge_shim(this: &PublicKeyCredentialRequestOptionsJson, val: &str);
    #[cfg(feature = "AuthenticationExtensionsClientInputsJson")]
    #[wasm_bindgen(method, setter = "extensions")]
    fn extensions_shim(
        this: &PublicKeyCredentialRequestOptionsJson,
        val: &AuthenticationExtensionsClientInputsJson,
    );
    #[wasm_bindgen(method, setter = "hints")]
    fn hints_shim(this: &PublicKeyCredentialRequestOptionsJson, val: &::wasm_bindgen::JsValue);
    #[wasm_bindgen(method, setter = "rpId")]
    fn rp_id_shim(this: &PublicKeyCredentialRequestOptionsJson, val: &str);
    #[wasm_bindgen(method, setter = "timeout")]
    fn timeout_shim(this: &PublicKeyCredentialRequestOptionsJson, val: u32);
    #[wasm_bindgen(method, setter = "userVerification")]
    fn user_verification_shim(this: &PublicKeyCredentialRequestOptionsJson, val: &str);
}
impl PublicKeyCredentialRequestOptionsJson {
    #[doc = "Construct a new `PublicKeyCredentialRequestOptionsJson`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialRequestOptionsJson`*"]
    pub fn new(challenge: &str) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.challenge(challenge);
        ret
    }
    #[doc = "Change the `allowCredentials` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialRequestOptionsJson`*"]
    pub fn allow_credentials(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.allow_credentials_shim(val);
        self
    }
    #[doc = "Change the `challenge` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialRequestOptionsJson`*"]
    pub fn challenge(&mut self, val: &str) -> &mut Self {
        self.challenge_shim(val);
        self
    }
    #[cfg(feature = "AuthenticationExtensionsClientInputsJson")]
    #[doc = "Change the `extensions` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticationExtensionsClientInputsJson`, `PublicKeyCredentialRequestOptionsJson`*"]
    pub fn extensions(&mut self, val: &AuthenticationExtensionsClientInputsJson) -> &mut Self {
        self.extensions_shim(val);
        self
    }
    #[doc = "Change the `hints` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialRequestOptionsJson`*"]
    pub fn hints(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.hints_shim(val);
        self
    }
    #[doc = "Change the `rpId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialRequestOptionsJson`*"]
    pub fn rp_id(&mut self, val: &str) -> &mut Self {
        self.rp_id_shim(val);
        self
    }
    #[doc = "Change the `timeout` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialRequestOptionsJson`*"]
    pub fn timeout(&mut self, val: u32) -> &mut Self {
        self.timeout_shim(val);
        self
    }
    #[doc = "Change the `userVerification` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialRequestOptionsJson`*"]
    pub fn user_verification(&mut self, val: &str) -> &mut Self {
        self.user_verification_shim(val);
        self
    }
}
