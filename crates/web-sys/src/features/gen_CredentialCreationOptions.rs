#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = CredentialCreationOptions)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `CredentialCreationOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CredentialCreationOptions`*"]
    pub type CredentialCreationOptions;
    #[cfg(feature = "PublicKeyCredentialCreationOptions")]
    #[wasm_bindgen(method, getter = "publicKey")]
    fn public_key_shim(this: &CredentialCreationOptions) -> PublicKeyCredentialCreationOptions;
    #[cfg(feature = "PublicKeyCredentialCreationOptions")]
    #[wasm_bindgen(method, setter = "publicKey")]
    fn set_public_key_shim(
        this: &CredentialCreationOptions,
        val: &PublicKeyCredentialCreationOptions,
    );
    #[cfg(feature = "AbortSignal")]
    #[wasm_bindgen(method, getter = "signal")]
    fn signal_shim(this: &CredentialCreationOptions) -> AbortSignal;
    #[cfg(feature = "AbortSignal")]
    #[wasm_bindgen(method, setter = "signal")]
    fn set_signal_shim(this: &CredentialCreationOptions, val: &AbortSignal);
}
#[doc = "The trait to access properties on the `CredentialCreationOptions` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `CredentialCreationOptions`*"]
pub trait CredentialCreationOptionsGetters {
    #[cfg(feature = "PublicKeyCredentialCreationOptions")]
    #[doc = "Get the `publicKey` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CredentialCreationOptions`, `PublicKeyCredentialCreationOptions`*"]
    fn public_key(&self) -> PublicKeyCredentialCreationOptions;
    #[cfg(feature = "AbortSignal")]
    #[doc = "Get the `signal` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AbortSignal`, `CredentialCreationOptions`*"]
    fn signal(&self) -> AbortSignal;
}
impl CredentialCreationOptionsGetters for CredentialCreationOptions {
    #[cfg(feature = "PublicKeyCredentialCreationOptions")]
    fn public_key(&self) -> PublicKeyCredentialCreationOptions {
        self.public_key_shim()
    }
    #[cfg(feature = "AbortSignal")]
    fn signal(&self) -> AbortSignal {
        self.signal_shim()
    }
}
impl CredentialCreationOptions {
    #[doc = "Construct a new `CredentialCreationOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CredentialCreationOptions`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[cfg(feature = "PublicKeyCredentialCreationOptions")]
    #[doc = "Change the `publicKey` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CredentialCreationOptions`, `PublicKeyCredentialCreationOptions`*"]
    pub fn public_key(&mut self, val: &PublicKeyCredentialCreationOptions) -> &mut Self {
        self.set_public_key_shim(val);
        self
    }
    #[cfg(feature = "AbortSignal")]
    #[doc = "Change the `signal` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AbortSignal`, `CredentialCreationOptions`*"]
    pub fn signal(&mut self, val: &AbortSignal) -> &mut Self {
        self.set_signal_shim(val);
        self
    }
}
impl Default for CredentialCreationOptions {
    fn default() -> Self {
        Self::new()
    }
}
