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
    #[wasm_bindgen(method, getter = "challenge")]
    fn challenge_shim(this: &CollectedClientData) -> &str;
    #[wasm_bindgen(method, setter = "challenge")]
    fn set_challenge_shim(this: &CollectedClientData, val: &str);
    #[cfg(feature = "AuthenticationExtensionsClientInputs")]
    #[wasm_bindgen(method, getter = "clientExtensions")]
    fn client_extensions_shim(this: &CollectedClientData) -> &AuthenticationExtensionsClientInputs;
    #[cfg(feature = "AuthenticationExtensionsClientInputs")]
    #[wasm_bindgen(method, setter = "clientExtensions")]
    fn set_client_extensions_shim(
        this: &CollectedClientData,
        val: &AuthenticationExtensionsClientInputs,
    );
    #[wasm_bindgen(method, getter = "hashAlgorithm")]
    fn hash_algorithm_shim(this: &CollectedClientData) -> &str;
    #[wasm_bindgen(method, setter = "hashAlgorithm")]
    fn set_hash_algorithm_shim(this: &CollectedClientData, val: &str);
    #[wasm_bindgen(method, getter = "origin")]
    fn origin_shim(this: &CollectedClientData) -> &str;
    #[wasm_bindgen(method, setter = "origin")]
    fn set_origin_shim(this: &CollectedClientData, val: &str);
    #[wasm_bindgen(method, getter = "tokenBindingId")]
    fn token_binding_id_shim(this: &CollectedClientData) -> &str;
    #[wasm_bindgen(method, setter = "tokenBindingId")]
    fn set_token_binding_id_shim(this: &CollectedClientData, val: &str);
    #[wasm_bindgen(method, getter = "type")]
    fn type__shim(this: &CollectedClientData) -> &str;
    #[wasm_bindgen(method, setter = "type")]
    fn set_type__shim(this: &CollectedClientData, val: &str);
}
#[doc = "The trait to access properties on the `CollectedClientData` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `CollectedClientData`*"]
pub trait CollectedClientDataGetters {
    #[doc = "Get the `challenge` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CollectedClientData`*"]
    fn challenge(&self) -> &str;
    #[cfg(feature = "AuthenticationExtensionsClientInputs")]
    #[doc = "Get the `clientExtensions` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticationExtensionsClientInputs`, `CollectedClientData`*"]
    fn client_extensions(&self) -> &AuthenticationExtensionsClientInputs;
    #[doc = "Get the `hashAlgorithm` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CollectedClientData`*"]
    fn hash_algorithm(&self) -> &str;
    #[doc = "Get the `origin` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CollectedClientData`*"]
    fn origin(&self) -> &str;
    #[doc = "Get the `tokenBindingId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CollectedClientData`*"]
    fn token_binding_id(&self) -> &str;
    #[doc = "Get the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CollectedClientData`*"]
    fn type_(&self) -> &str;
}
impl CollectedClientDataGetters for CollectedClientData {
    fn challenge(&self) -> &str {
        self.challenge_shim()
    }
    #[cfg(feature = "AuthenticationExtensionsClientInputs")]
    fn client_extensions(&self) -> &AuthenticationExtensionsClientInputs {
        self.client_extensions_shim()
    }
    fn hash_algorithm(&self) -> &str {
        self.hash_algorithm_shim()
    }
    fn origin(&self) -> &str {
        self.origin_shim()
    }
    fn token_binding_id(&self) -> &str {
        self.token_binding_id_shim()
    }
    fn type_(&self) -> &str {
        self.type__shim()
    }
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
        self.set_challenge_shim(val);
        self
    }
    #[cfg(feature = "AuthenticationExtensionsClientInputs")]
    #[doc = "Change the `clientExtensions` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticationExtensionsClientInputs`, `CollectedClientData`*"]
    pub fn client_extensions(&mut self, val: &AuthenticationExtensionsClientInputs) -> &mut Self {
        self.set_client_extensions_shim(val);
        self
    }
    #[doc = "Change the `hashAlgorithm` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CollectedClientData`*"]
    pub fn hash_algorithm(&mut self, val: &str) -> &mut Self {
        self.set_hash_algorithm_shim(val);
        self
    }
    #[doc = "Change the `origin` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CollectedClientData`*"]
    pub fn origin(&mut self, val: &str) -> &mut Self {
        self.set_origin_shim(val);
        self
    }
    #[doc = "Change the `tokenBindingId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CollectedClientData`*"]
    pub fn token_binding_id(&mut self, val: &str) -> &mut Self {
        self.set_token_binding_id_shim(val);
        self
    }
    #[doc = "Change the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CollectedClientData`*"]
    pub fn type_(&mut self, val: &str) -> &mut Self {
        self.set_type__shim(val);
        self
    }
}
