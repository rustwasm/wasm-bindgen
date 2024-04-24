#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = AuthenticationExtensionsPRFValues)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `AuthenticationExtensionsPrfValues` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticationExtensionsPrfValues`*"]
    pub type AuthenticationExtensionsPrfValues;
    #[wasm_bindgen(method, setter = "first")]
    fn first_shim(this: &AuthenticationExtensionsPrfValues, val: &::js_sys::Object);
    #[wasm_bindgen(method, setter = "second")]
    fn second_shim(this: &AuthenticationExtensionsPrfValues, val: &::js_sys::Object);
}
impl AuthenticationExtensionsPrfValues {
    #[doc = "Construct a new `AuthenticationExtensionsPrfValues`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticationExtensionsPrfValues`*"]
    pub fn new(first: &::js_sys::Object) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.first(first);
        ret
    }
    #[doc = "Change the `first` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticationExtensionsPrfValues`*"]
    pub fn first(&mut self, val: &::js_sys::Object) -> &mut Self {
        self.first_shim(val);
        self
    }
    #[doc = "Change the `second` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticationExtensionsPrfValues`*"]
    pub fn second(&mut self, val: &::js_sys::Object) -> &mut Self {
        self.second_shim(val);
        self
    }
}
