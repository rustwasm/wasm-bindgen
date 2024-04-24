#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = AuthenticationExtensionsPRFInputs)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `AuthenticationExtensionsPrfInputs` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticationExtensionsPrfInputs`*"]
    pub type AuthenticationExtensionsPrfInputs;
    #[cfg(feature = "AuthenticationExtensionsPrfValues")]
    #[wasm_bindgen(method, setter = "eval")]
    fn eval_shim(this: &AuthenticationExtensionsPrfInputs, val: &AuthenticationExtensionsPrfValues);
}
impl AuthenticationExtensionsPrfInputs {
    #[doc = "Construct a new `AuthenticationExtensionsPrfInputs`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticationExtensionsPrfInputs`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[cfg(feature = "AuthenticationExtensionsPrfValues")]
    #[doc = "Change the `eval` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticationExtensionsPrfInputs`, `AuthenticationExtensionsPrfValues`*"]
    pub fn eval(&mut self, val: &AuthenticationExtensionsPrfValues) -> &mut Self {
        self.eval_shim(val);
        self
    }
}
impl Default for AuthenticationExtensionsPrfInputs {
    fn default() -> Self {
        Self::new()
    }
}
