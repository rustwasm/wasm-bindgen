#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = AuthenticationExtensionsClientOutputs)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `AuthenticationExtensionsClientOutputs` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticationExtensionsClientOutputs`*"]
    pub type AuthenticationExtensionsClientOutputs;
    #[wasm_bindgen(method, setter = "appid")]
    fn appid_shim(this: &AuthenticationExtensionsClientOutputs, val: bool);
    #[wasm_bindgen(method, setter = "appidExclude")]
    fn appid_exclude_shim(this: &AuthenticationExtensionsClientOutputs, val: bool);
    #[cfg(feature = "CredentialPropertiesOutput")]
    #[wasm_bindgen(method, setter = "credProps")]
    fn cred_props_shim(
        this: &AuthenticationExtensionsClientOutputs,
        val: &CredentialPropertiesOutput,
    );
    #[cfg(feature = "AuthenticationExtensionsLargeBlobOutputs")]
    #[wasm_bindgen(method, setter = "largeBlob")]
    fn large_blob_shim(
        this: &AuthenticationExtensionsClientOutputs,
        val: &AuthenticationExtensionsLargeBlobOutputs,
    );
    #[cfg(feature = "AuthenticationExtensionsPrfOutputs")]
    #[wasm_bindgen(method, setter = "prf")]
    fn prf_shim(
        this: &AuthenticationExtensionsClientOutputs,
        val: &AuthenticationExtensionsPrfOutputs,
    );
    #[cfg(feature = "AuthenticationExtensionsSupplementalPubKeysOutputs")]
    #[wasm_bindgen(method, setter = "supplementalPubKeys")]
    fn supplemental_pub_keys_shim(
        this: &AuthenticationExtensionsClientOutputs,
        val: &AuthenticationExtensionsSupplementalPubKeysOutputs,
    );
    #[wasm_bindgen(method, setter = "uvm")]
    fn uvm_shim(this: &AuthenticationExtensionsClientOutputs, val: &::wasm_bindgen::JsValue);
}
impl AuthenticationExtensionsClientOutputs {
    #[doc = "Construct a new `AuthenticationExtensionsClientOutputs`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticationExtensionsClientOutputs`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `appid` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticationExtensionsClientOutputs`*"]
    pub fn appid(&mut self, val: bool) -> &mut Self {
        self.appid_shim(val);
        self
    }
    #[doc = "Change the `appidExclude` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticationExtensionsClientOutputs`*"]
    pub fn appid_exclude(&mut self, val: bool) -> &mut Self {
        self.appid_exclude_shim(val);
        self
    }
    #[cfg(feature = "CredentialPropertiesOutput")]
    #[doc = "Change the `credProps` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticationExtensionsClientOutputs`, `CredentialPropertiesOutput`*"]
    pub fn cred_props(&mut self, val: &CredentialPropertiesOutput) -> &mut Self {
        self.cred_props_shim(val);
        self
    }
    #[cfg(feature = "AuthenticationExtensionsLargeBlobOutputs")]
    #[doc = "Change the `largeBlob` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticationExtensionsClientOutputs`, `AuthenticationExtensionsLargeBlobOutputs`*"]
    pub fn large_blob(&mut self, val: &AuthenticationExtensionsLargeBlobOutputs) -> &mut Self {
        self.large_blob_shim(val);
        self
    }
    #[cfg(feature = "AuthenticationExtensionsPrfOutputs")]
    #[doc = "Change the `prf` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticationExtensionsClientOutputs`, `AuthenticationExtensionsPrfOutputs`*"]
    pub fn prf(&mut self, val: &AuthenticationExtensionsPrfOutputs) -> &mut Self {
        self.prf_shim(val);
        self
    }
    #[cfg(feature = "AuthenticationExtensionsSupplementalPubKeysOutputs")]
    #[doc = "Change the `supplementalPubKeys` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticationExtensionsClientOutputs`, `AuthenticationExtensionsSupplementalPubKeysOutputs`*"]
    pub fn supplemental_pub_keys(
        &mut self,
        val: &AuthenticationExtensionsSupplementalPubKeysOutputs,
    ) -> &mut Self {
        self.supplemental_pub_keys_shim(val);
        self
    }
    #[doc = "Change the `uvm` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticationExtensionsClientOutputs`*"]
    pub fn uvm(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.uvm_shim(val);
        self
    }
}
impl Default for AuthenticationExtensionsClientOutputs {
    fn default() -> Self {
        Self::new()
    }
}
