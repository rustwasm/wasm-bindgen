#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = AuthenticationExtensionsPRFOutputs)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `AuthenticationExtensionsPrfOutputs` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticationExtensionsPrfOutputs`*"]
    pub type AuthenticationExtensionsPrfOutputs;
    #[wasm_bindgen(method, setter = "enabled")]
    fn enabled_shim(this: &AuthenticationExtensionsPrfOutputs, val: bool);
    #[cfg(feature = "AuthenticationExtensionsPrfValues")]
    #[wasm_bindgen(method, setter = "results")]
    fn results_shim(
        this: &AuthenticationExtensionsPrfOutputs,
        val: &AuthenticationExtensionsPrfValues,
    );
}
impl AuthenticationExtensionsPrfOutputs {
    #[doc = "Construct a new `AuthenticationExtensionsPrfOutputs`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticationExtensionsPrfOutputs`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `enabled` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticationExtensionsPrfOutputs`*"]
    pub fn enabled(&mut self, val: bool) -> &mut Self {
        self.enabled_shim(val);
        self
    }
    #[cfg(feature = "AuthenticationExtensionsPrfValues")]
    #[doc = "Change the `results` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticationExtensionsPrfOutputs`, `AuthenticationExtensionsPrfValues`*"]
    pub fn results(&mut self, val: &AuthenticationExtensionsPrfValues) -> &mut Self {
        self.results_shim(val);
        self
    }
}
impl Default for AuthenticationExtensionsPrfOutputs {
    fn default() -> Self {
        Self::new()
    }
}
