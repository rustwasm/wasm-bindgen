#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = CredentialPropertiesOutput)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `CredentialPropertiesOutput` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CredentialPropertiesOutput`*"]
    pub type CredentialPropertiesOutput;
    #[wasm_bindgen(method, setter = "authenticatorDisplayName")]
    fn authenticator_display_name_shim(this: &CredentialPropertiesOutput, val: &str);
    #[wasm_bindgen(method, setter = "rk")]
    fn rk_shim(this: &CredentialPropertiesOutput, val: bool);
}
impl CredentialPropertiesOutput {
    #[doc = "Construct a new `CredentialPropertiesOutput`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CredentialPropertiesOutput`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `authenticatorDisplayName` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CredentialPropertiesOutput`*"]
    pub fn authenticator_display_name(&mut self, val: &str) -> &mut Self {
        self.authenticator_display_name_shim(val);
        self
    }
    #[doc = "Change the `rk` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CredentialPropertiesOutput`*"]
    pub fn rk(&mut self, val: bool) -> &mut Self {
        self.rk_shim(val);
        self
    }
}
impl Default for CredentialPropertiesOutput {
    fn default() -> Self {
        Self::new()
    }
}
