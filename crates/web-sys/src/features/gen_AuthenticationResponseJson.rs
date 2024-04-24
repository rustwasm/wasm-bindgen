#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = AuthenticationResponseJSON)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `AuthenticationResponseJson` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticationResponseJson`*"]
    pub type AuthenticationResponseJson;
    #[wasm_bindgen(method, setter = "authenticatorAttachment")]
    fn authenticator_attachment_shim(this: &AuthenticationResponseJson, val: &str);
    #[cfg(feature = "AuthenticationExtensionsClientOutputsJson")]
    #[wasm_bindgen(method, setter = "clientExtensionResults")]
    fn client_extension_results_shim(
        this: &AuthenticationResponseJson,
        val: &AuthenticationExtensionsClientOutputsJson,
    );
    #[wasm_bindgen(method, setter = "id")]
    fn id_shim(this: &AuthenticationResponseJson, val: &str);
    #[wasm_bindgen(method, setter = "rawId")]
    fn raw_id_shim(this: &AuthenticationResponseJson, val: &str);
    #[cfg(feature = "AuthenticatorAssertionResponseJson")]
    #[wasm_bindgen(method, setter = "response")]
    fn response_shim(this: &AuthenticationResponseJson, val: &AuthenticatorAssertionResponseJson);
    #[wasm_bindgen(method, setter = "type")]
    fn type__shim(this: &AuthenticationResponseJson, val: &str);
}
impl AuthenticationResponseJson {
    #[cfg(all(
        feature = "AuthenticationExtensionsClientOutputsJson",
        feature = "AuthenticatorAssertionResponseJson",
    ))]
    #[doc = "Construct a new `AuthenticationResponseJson`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticationExtensionsClientOutputsJson`, `AuthenticationResponseJson`, `AuthenticatorAssertionResponseJson`*"]
    pub fn new(
        client_extension_results: &AuthenticationExtensionsClientOutputsJson,
        id: &str,
        raw_id: &str,
        response: &AuthenticatorAssertionResponseJson,
        type_: &str,
    ) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.client_extension_results(client_extension_results);
        ret.id(id);
        ret.raw_id(raw_id);
        ret.response(response);
        ret.type_(type_);
        ret
    }
    #[doc = "Change the `authenticatorAttachment` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticationResponseJson`*"]
    pub fn authenticator_attachment(&mut self, val: &str) -> &mut Self {
        self.authenticator_attachment_shim(val);
        self
    }
    #[cfg(feature = "AuthenticationExtensionsClientOutputsJson")]
    #[doc = "Change the `clientExtensionResults` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticationExtensionsClientOutputsJson`, `AuthenticationResponseJson`*"]
    pub fn client_extension_results(
        &mut self,
        val: &AuthenticationExtensionsClientOutputsJson,
    ) -> &mut Self {
        self.client_extension_results_shim(val);
        self
    }
    #[doc = "Change the `id` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticationResponseJson`*"]
    pub fn id(&mut self, val: &str) -> &mut Self {
        self.id_shim(val);
        self
    }
    #[doc = "Change the `rawId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticationResponseJson`*"]
    pub fn raw_id(&mut self, val: &str) -> &mut Self {
        self.raw_id_shim(val);
        self
    }
    #[cfg(feature = "AuthenticatorAssertionResponseJson")]
    #[doc = "Change the `response` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticationResponseJson`, `AuthenticatorAssertionResponseJson`*"]
    pub fn response(&mut self, val: &AuthenticatorAssertionResponseJson) -> &mut Self {
        self.response_shim(val);
        self
    }
    #[doc = "Change the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticationResponseJson`*"]
    pub fn type_(&mut self, val: &str) -> &mut Self {
        self.type__shim(val);
        self
    }
}
