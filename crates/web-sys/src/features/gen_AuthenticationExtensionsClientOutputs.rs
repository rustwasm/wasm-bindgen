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
    #[wasm_bindgen(method, getter = "appid")]
    fn appid_shim(this: &AuthenticationExtensionsClientOutputs) -> bool;
    #[wasm_bindgen(method, setter = "appid")]
    fn set_appid_shim(this: &AuthenticationExtensionsClientOutputs, val: bool);
}
#[doc = "The trait to access properties on the `AuthenticationExtensionsClientOutputs` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `AuthenticationExtensionsClientOutputs`*"]
pub trait AuthenticationExtensionsClientOutputsGetters {
    #[doc = "Get the `appid` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticationExtensionsClientOutputs`*"]
    fn appid(&self) -> bool;
}
impl AuthenticationExtensionsClientOutputsGetters for AuthenticationExtensionsClientOutputs {
    fn appid(&self) -> bool {
        self.appid_shim()
    }
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
        self.set_appid_shim(val);
        self
    }
}
impl Default for AuthenticationExtensionsClientOutputs {
    fn default() -> Self {
        Self::new()
    }
}
