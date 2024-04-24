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
    #[wasm_bindgen(method, getter = "appid")]
    fn appid_shim(this: &AuthenticationExtensionsClientInputs) -> String;
    #[wasm_bindgen(method, setter = "appid")]
    fn set_appid_shim(this: &AuthenticationExtensionsClientInputs, val: &str);
}
#[doc = "The trait to access properties on the `AuthenticationExtensionsClientInputs` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `AuthenticationExtensionsClientInputs`*"]
pub trait AuthenticationExtensionsClientInputsGetters {
    #[doc = "Get the `appid` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticationExtensionsClientInputs`*"]
    fn appid(&self) -> String;
}
impl AuthenticationExtensionsClientInputsGetters for AuthenticationExtensionsClientInputs {
    fn appid(&self) -> String {
        self.appid_shim()
    }
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
        self.set_appid_shim(val);
        self
    }
}
impl Default for AuthenticationExtensionsClientInputs {
    fn default() -> Self {
        Self::new()
    }
}
