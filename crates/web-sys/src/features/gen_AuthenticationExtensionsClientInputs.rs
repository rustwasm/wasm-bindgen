#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = AuthenticationExtensionsClientInputs)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `AuthenticationExtensionsClientInputs` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticationExtensionsClientInputs`*"]
    pub type AuthenticationExtensionsClientInputs;
    #[wasm_bindgen(method, setter = "appid")]
    fn appid_shim(this: &AuthenticationExtensionsClientInputs, val: &str);
    #[wasm_bindgen(method, setter = "appidExclude")]
    fn appid_exclude_shim(this: &AuthenticationExtensionsClientInputs, val: &str);
    #[wasm_bindgen(method, setter = "credProps")]
    fn cred_props_shim(this: &AuthenticationExtensionsClientInputs, val: bool);
    #[cfg(feature = "AuthenticationExtensionsLargeBlobInputs")]
    #[wasm_bindgen(method, setter = "largeBlob")]
    fn large_blob_shim(
        this: &AuthenticationExtensionsClientInputs,
        val: &AuthenticationExtensionsLargeBlobInputs,
    );
    #[cfg(feature = "AuthenticationExtensionsPrfInputs")]
    #[wasm_bindgen(method, setter = "prf")]
    fn prf_shim(
        this: &AuthenticationExtensionsClientInputs,
        val: &AuthenticationExtensionsPrfInputs,
    );
    #[cfg(feature = "AuthenticationExtensionsSupplementalPubKeysInputs")]
    #[wasm_bindgen(method, setter = "supplementalPubKeys")]
    fn supplemental_pub_keys_shim(
        this: &AuthenticationExtensionsClientInputs,
        val: &AuthenticationExtensionsSupplementalPubKeysInputs,
    );
    #[wasm_bindgen(method, setter = "uvm")]
    fn uvm_shim(this: &AuthenticationExtensionsClientInputs, val: bool);
}
impl AuthenticationExtensionsClientInputs {
    #[doc = "Construct a new `AuthenticationExtensionsClientInputs`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticationExtensionsClientInputs`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `appid` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticationExtensionsClientInputs`*"]
    pub fn appid(&mut self, val: &str) -> &mut Self {
        self.appid_shim(val);
        self
    }
    #[doc = "Change the `appidExclude` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticationExtensionsClientInputs`*"]
    pub fn appid_exclude(&mut self, val: &str) -> &mut Self {
        self.appid_exclude_shim(val);
        self
    }
    #[doc = "Change the `credProps` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticationExtensionsClientInputs`*"]
    pub fn cred_props(&mut self, val: bool) -> &mut Self {
        self.cred_props_shim(val);
        self
    }
    #[cfg(feature = "AuthenticationExtensionsLargeBlobInputs")]
    #[doc = "Change the `largeBlob` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticationExtensionsClientInputs`, `AuthenticationExtensionsLargeBlobInputs`*"]
    pub fn large_blob(&mut self, val: &AuthenticationExtensionsLargeBlobInputs) -> &mut Self {
        self.large_blob_shim(val);
        self
    }
    #[cfg(feature = "AuthenticationExtensionsPrfInputs")]
    #[doc = "Change the `prf` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticationExtensionsClientInputs`, `AuthenticationExtensionsPrfInputs`*"]
    pub fn prf(&mut self, val: &AuthenticationExtensionsPrfInputs) -> &mut Self {
        self.prf_shim(val);
        self
    }
    #[cfg(feature = "AuthenticationExtensionsSupplementalPubKeysInputs")]
    #[doc = "Change the `supplementalPubKeys` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticationExtensionsClientInputs`, `AuthenticationExtensionsSupplementalPubKeysInputs`*"]
    pub fn supplemental_pub_keys(
        &mut self,
        val: &AuthenticationExtensionsSupplementalPubKeysInputs,
    ) -> &mut Self {
        self.supplemental_pub_keys_shim(val);
        self
    }
    #[doc = "Change the `uvm` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticationExtensionsClientInputs`*"]
    pub fn uvm(&mut self, val: bool) -> &mut Self {
        self.uvm_shim(val);
        self
    }
}
impl Default for AuthenticationExtensionsClientInputs {
    fn default() -> Self {
        Self::new()
    }
}
