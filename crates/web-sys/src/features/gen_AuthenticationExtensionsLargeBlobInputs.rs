#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = AuthenticationExtensionsLargeBlobInputs)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `AuthenticationExtensionsLargeBlobInputs` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticationExtensionsLargeBlobInputs`*"]
    pub type AuthenticationExtensionsLargeBlobInputs;
    #[wasm_bindgen(method, setter = "read")]
    fn read_shim(this: &AuthenticationExtensionsLargeBlobInputs, val: bool);
    #[wasm_bindgen(method, setter = "support")]
    fn support_shim(this: &AuthenticationExtensionsLargeBlobInputs, val: &str);
    #[wasm_bindgen(method, setter = "write")]
    fn write_shim(this: &AuthenticationExtensionsLargeBlobInputs, val: &::js_sys::Object);
}
impl AuthenticationExtensionsLargeBlobInputs {
    #[doc = "Construct a new `AuthenticationExtensionsLargeBlobInputs`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticationExtensionsLargeBlobInputs`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `read` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticationExtensionsLargeBlobInputs`*"]
    pub fn read(&mut self, val: bool) -> &mut Self {
        self.read_shim(val);
        self
    }
    #[doc = "Change the `support` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticationExtensionsLargeBlobInputs`*"]
    pub fn support(&mut self, val: &str) -> &mut Self {
        self.support_shim(val);
        self
    }
    #[doc = "Change the `write` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticationExtensionsLargeBlobInputs`*"]
    pub fn write(&mut self, val: &::js_sys::Object) -> &mut Self {
        self.write_shim(val);
        self
    }
}
impl Default for AuthenticationExtensionsLargeBlobInputs {
    fn default() -> Self {
        Self::new()
    }
}
