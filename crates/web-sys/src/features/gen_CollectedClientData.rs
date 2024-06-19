#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = CollectedClientData)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `CollectedClientData` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CollectedClientData`*"]
    pub type CollectedClientData;
    #[doc = "Get the `challenge` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CollectedClientData`*"]
    #[wasm_bindgen(method, getter = "challenge")]
    pub fn get_challenge(this: &CollectedClientData) -> String;
    #[wasm_bindgen(method, setter = "challenge")]
    fn set_challenge(this: &CollectedClientData, val: &str);
    #[cfg(feature = "AuthenticationExtensionsClientInputs")]
    #[doc = "Get the `clientExtensions` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticationExtensionsClientInputs`, `CollectedClientData`*"]
    #[wasm_bindgen(method, getter = "clientExtensions")]
    pub fn get_client_extensions(
        this: &CollectedClientData,
    ) -> Option<AuthenticationExtensionsClientInputs>;
    #[cfg(feature = "AuthenticationExtensionsClientInputs")]
    #[wasm_bindgen(method, setter = "clientExtensions")]
    fn set_client_extensions(
        this: &CollectedClientData,
        val: &AuthenticationExtensionsClientInputs,
    );
    #[doc = "Get the `hashAlgorithm` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CollectedClientData`*"]
    #[wasm_bindgen(method, getter = "hashAlgorithm")]
    pub fn get_hash_algorithm(this: &CollectedClientData) -> String;
    #[wasm_bindgen(method, setter = "hashAlgorithm")]
    fn set_hash_algorithm(this: &CollectedClientData, val: &str);
    #[doc = "Get the `origin` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CollectedClientData`*"]
    #[wasm_bindgen(method, getter = "origin")]
    pub fn get_origin(this: &CollectedClientData) -> String;
    #[wasm_bindgen(method, setter = "origin")]
    fn set_origin(this: &CollectedClientData, val: &str);
    #[doc = "Get the `tokenBindingId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CollectedClientData`*"]
    #[wasm_bindgen(method, getter = "tokenBindingId")]
    pub fn get_token_binding_id(this: &CollectedClientData) -> Option<String>;
    #[wasm_bindgen(method, setter = "tokenBindingId")]
    fn set_token_binding_id(this: &CollectedClientData, val: &str);
    #[doc = "Get the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CollectedClientData`*"]
    #[wasm_bindgen(method, getter = "type")]
    pub fn get_type(this: &CollectedClientData) -> String;
    #[wasm_bindgen(method, setter = "type")]
    fn set_type(this: &CollectedClientData, val: &str);
}
impl CollectedClientData {
    #[doc = "Construct a new `CollectedClientData`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CollectedClientData`*"]
    pub fn new(challenge: &str, hash_algorithm: &str, origin: &str, type_: &str) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.challenge(challenge);
        ret.hash_algorithm(hash_algorithm);
        ret.origin(origin);
        ret.type_(type_);
        ret
    }
    #[doc = "Change the `challenge` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CollectedClientData`*"]
    pub fn challenge(&mut self, val: &str) -> &mut Self {
        self.set_challenge(val);
        self
    }
    #[cfg(feature = "AuthenticationExtensionsClientInputs")]
    #[doc = "Change the `clientExtensions` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticationExtensionsClientInputs`, `CollectedClientData`*"]
    pub fn client_extensions(&mut self, val: &AuthenticationExtensionsClientInputs) -> &mut Self {
        self.set_client_extensions(val);
        self
    }
    #[doc = "Change the `hashAlgorithm` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CollectedClientData`*"]
    pub fn hash_algorithm(&mut self, val: &str) -> &mut Self {
        self.set_hash_algorithm(val);
        self
    }
    #[doc = "Change the `origin` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CollectedClientData`*"]
    pub fn origin(&mut self, val: &str) -> &mut Self {
        self.set_origin(val);
        self
    }
    #[doc = "Change the `tokenBindingId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CollectedClientData`*"]
    pub fn token_binding_id(&mut self, val: &str) -> &mut Self {
        self.set_token_binding_id(val);
        self
    }
    #[doc = "Change the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CollectedClientData`*"]
    pub fn type_(&mut self, val: &str) -> &mut Self {
        self.set_type(val);
        self
    }
}
