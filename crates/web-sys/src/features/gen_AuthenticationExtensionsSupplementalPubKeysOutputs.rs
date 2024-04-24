#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = AuthenticationExtensionsSupplementalPubKeysOutputs)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `AuthenticationExtensionsSupplementalPubKeysOutputs` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticationExtensionsSupplementalPubKeysOutputs`*"]
    pub type AuthenticationExtensionsSupplementalPubKeysOutputs;
    #[wasm_bindgen(method, setter = "signatures")]
    fn signatures_shim(
        this: &AuthenticationExtensionsSupplementalPubKeysOutputs,
        val: &::wasm_bindgen::JsValue,
    );
}
impl AuthenticationExtensionsSupplementalPubKeysOutputs {
    #[doc = "Construct a new `AuthenticationExtensionsSupplementalPubKeysOutputs`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticationExtensionsSupplementalPubKeysOutputs`*"]
    pub fn new(signatures: &::wasm_bindgen::JsValue) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.signatures(signatures);
        ret
    }
    #[doc = "Change the `signatures` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticationExtensionsSupplementalPubKeysOutputs`*"]
    pub fn signatures(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.signatures_shim(val);
        self
    }
}
