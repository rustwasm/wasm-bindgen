#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = CredentialRequestOptions)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `CredentialRequestOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CredentialRequestOptions`*"]
    pub type CredentialRequestOptions;
    #[cfg(feature = "PublicKeyCredentialRequestOptions")]
    #[wasm_bindgen(method, setter = "publicKey")]
    fn public_key_shim(this: &CredentialRequestOptions, val: &PublicKeyCredentialRequestOptions);
    #[cfg(feature = "AbortSignal")]
    #[wasm_bindgen(method, setter = "signal")]
    fn signal_shim(this: &CredentialRequestOptions, val: &AbortSignal);
}
impl CredentialRequestOptions {
    #[doc = "Construct a new `CredentialRequestOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CredentialRequestOptions`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[cfg(feature = "PublicKeyCredentialRequestOptions")]
    #[doc = "Change the `publicKey` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CredentialRequestOptions`, `PublicKeyCredentialRequestOptions`*"]
    pub fn public_key(&mut self, val: &PublicKeyCredentialRequestOptions) -> &mut Self {
        self.public_key_shim(val);
        self
    }
    #[cfg(feature = "AbortSignal")]
    #[doc = "Change the `signal` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AbortSignal`, `CredentialRequestOptions`*"]
    pub fn signal(&mut self, val: &AbortSignal) -> &mut Self {
        self.signal_shim(val);
        self
    }
}
impl Default for CredentialRequestOptions {
    fn default() -> Self {
        Self::new()
    }
}
