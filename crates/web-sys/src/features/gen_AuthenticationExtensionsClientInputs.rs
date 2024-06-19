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
    #[doc = "Get the `appid` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticationExtensionsClientInputs`*"]
    #[wasm_bindgen(method, getter = "appid")]
    pub fn get_appid(this: &AuthenticationExtensionsClientInputs) -> Option<String>;
    #[doc = "Change the `appid` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticationExtensionsClientInputs`*"]
    #[wasm_bindgen(method, setter = "appid")]
    pub fn set_appid(this: &AuthenticationExtensionsClientInputs, val: &str);
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
    #[deprecated = "Use `set_appid()` instead."]
    pub fn appid(&mut self, val: &str) -> &mut Self {
        self.set_appid(val);
        self
    }
}
impl Default for AuthenticationExtensionsClientInputs {
    fn default() -> Self {
        Self::new()
    }
}
