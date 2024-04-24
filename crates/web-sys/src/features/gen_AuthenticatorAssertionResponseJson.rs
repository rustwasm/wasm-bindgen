#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = AuthenticatorAssertionResponseJSON)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `AuthenticatorAssertionResponseJson` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticatorAssertionResponseJson`*"]
    pub type AuthenticatorAssertionResponseJson;
    #[wasm_bindgen(method, setter = "authenticatorData")]
    fn authenticator_data_shim(this: &AuthenticatorAssertionResponseJson, val: &str);
    #[wasm_bindgen(method, setter = "clientDataJSON")]
    fn client_data_json_shim(this: &AuthenticatorAssertionResponseJson, val: &str);
    #[wasm_bindgen(method, setter = "signature")]
    fn signature_shim(this: &AuthenticatorAssertionResponseJson, val: &str);
    #[wasm_bindgen(method, setter = "userHandle")]
    fn user_handle_shim(this: &AuthenticatorAssertionResponseJson, val: &str);
}
impl AuthenticatorAssertionResponseJson {
    #[doc = "Construct a new `AuthenticatorAssertionResponseJson`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticatorAssertionResponseJson`*"]
    pub fn new(authenticator_data: &str, client_data_json: &str, signature: &str) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.authenticator_data(authenticator_data);
        ret.client_data_json(client_data_json);
        ret.signature(signature);
        ret
    }
    #[doc = "Change the `authenticatorData` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticatorAssertionResponseJson`*"]
    pub fn authenticator_data(&mut self, val: &str) -> &mut Self {
        self.authenticator_data_shim(val);
        self
    }
    #[doc = "Change the `clientDataJSON` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticatorAssertionResponseJson`*"]
    pub fn client_data_json(&mut self, val: &str) -> &mut Self {
        self.client_data_json_shim(val);
        self
    }
    #[doc = "Change the `signature` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticatorAssertionResponseJson`*"]
    pub fn signature(&mut self, val: &str) -> &mut Self {
        self.signature_shim(val);
        self
    }
    #[doc = "Change the `userHandle` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticatorAssertionResponseJson`*"]
    pub fn user_handle(&mut self, val: &str) -> &mut Self {
        self.user_handle_shim(val);
        self
    }
}
