#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = AuthenticationExtensionsSupplementalPubKeysInputs)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `AuthenticationExtensionsSupplementalPubKeysInputs` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticationExtensionsSupplementalPubKeysInputs`*"]
    pub type AuthenticationExtensionsSupplementalPubKeysInputs;
    #[wasm_bindgen(method, setter = "attestation")]
    fn attestation_shim(this: &AuthenticationExtensionsSupplementalPubKeysInputs, val: &str);
    #[wasm_bindgen(method, setter = "attestationFormats")]
    fn attestation_formats_shim(
        this: &AuthenticationExtensionsSupplementalPubKeysInputs,
        val: &::wasm_bindgen::JsValue,
    );
    #[wasm_bindgen(method, setter = "scopes")]
    fn scopes_shim(
        this: &AuthenticationExtensionsSupplementalPubKeysInputs,
        val: &::wasm_bindgen::JsValue,
    );
}
impl AuthenticationExtensionsSupplementalPubKeysInputs {
    #[doc = "Construct a new `AuthenticationExtensionsSupplementalPubKeysInputs`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticationExtensionsSupplementalPubKeysInputs`*"]
    pub fn new(scopes: &::wasm_bindgen::JsValue) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.scopes(scopes);
        ret
    }
    #[doc = "Change the `attestation` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticationExtensionsSupplementalPubKeysInputs`*"]
    pub fn attestation(&mut self, val: &str) -> &mut Self {
        self.attestation_shim(val);
        self
    }
    #[doc = "Change the `attestationFormats` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticationExtensionsSupplementalPubKeysInputs`*"]
    pub fn attestation_formats(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.attestation_formats_shim(val);
        self
    }
    #[doc = "Change the `scopes` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticationExtensionsSupplementalPubKeysInputs`*"]
    pub fn scopes(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.scopes_shim(val);
        self
    }
}
