#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = AuthenticatorAttestationResponseJSON)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `AuthenticatorAttestationResponseJson` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticatorAttestationResponseJson`*"]
    pub type AuthenticatorAttestationResponseJson;
    #[wasm_bindgen(method, setter = "attestationObject")]
    fn attestation_object_shim(this: &AuthenticatorAttestationResponseJson, val: &str);
    #[wasm_bindgen(method, setter = "authenticatorData")]
    fn authenticator_data_shim(this: &AuthenticatorAttestationResponseJson, val: &str);
    #[wasm_bindgen(method, setter = "clientDataJSON")]
    fn client_data_json_shim(this: &AuthenticatorAttestationResponseJson, val: &str);
    #[wasm_bindgen(method, setter = "publicKey")]
    fn public_key_shim(this: &AuthenticatorAttestationResponseJson, val: &str);
    #[wasm_bindgen(method, setter = "publicKeyAlgorithm")]
    fn public_key_algorithm_shim(this: &AuthenticatorAttestationResponseJson, val: f64);
    #[wasm_bindgen(method, setter = "transports")]
    fn transports_shim(this: &AuthenticatorAttestationResponseJson, val: &::wasm_bindgen::JsValue);
}
impl AuthenticatorAttestationResponseJson {
    #[doc = "Construct a new `AuthenticatorAttestationResponseJson`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticatorAttestationResponseJson`*"]
    pub fn new(
        attestation_object: &str,
        authenticator_data: &str,
        client_data_json: &str,
        public_key_algorithm: f64,
        transports: &::wasm_bindgen::JsValue,
    ) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.attestation_object(attestation_object);
        ret.authenticator_data(authenticator_data);
        ret.client_data_json(client_data_json);
        ret.public_key_algorithm(public_key_algorithm);
        ret.transports(transports);
        ret
    }
    #[doc = "Change the `attestationObject` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticatorAttestationResponseJson`*"]
    pub fn attestation_object(&mut self, val: &str) -> &mut Self {
        self.attestation_object_shim(val);
        self
    }
    #[doc = "Change the `authenticatorData` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticatorAttestationResponseJson`*"]
    pub fn authenticator_data(&mut self, val: &str) -> &mut Self {
        self.authenticator_data_shim(val);
        self
    }
    #[doc = "Change the `clientDataJSON` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticatorAttestationResponseJson`*"]
    pub fn client_data_json(&mut self, val: &str) -> &mut Self {
        self.client_data_json_shim(val);
        self
    }
    #[doc = "Change the `publicKey` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticatorAttestationResponseJson`*"]
    pub fn public_key(&mut self, val: &str) -> &mut Self {
        self.public_key_shim(val);
        self
    }
    #[doc = "Change the `publicKeyAlgorithm` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticatorAttestationResponseJson`*"]
    pub fn public_key_algorithm(&mut self, val: f64) -> &mut Self {
        self.public_key_algorithm_shim(val);
        self
    }
    #[doc = "Change the `transports` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticatorAttestationResponseJson`*"]
    pub fn transports(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.transports_shim(val);
        self
    }
}
