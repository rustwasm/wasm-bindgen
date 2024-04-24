#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = AuthenticationExtensionsLargeBlobOutputs)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `AuthenticationExtensionsLargeBlobOutputs` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticationExtensionsLargeBlobOutputs`*"]
    pub type AuthenticationExtensionsLargeBlobOutputs;
    #[wasm_bindgen(method, setter = "blob")]
    fn blob_shim(this: &AuthenticationExtensionsLargeBlobOutputs, val: &::js_sys::ArrayBuffer);
    #[wasm_bindgen(method, setter = "supported")]
    fn supported_shim(this: &AuthenticationExtensionsLargeBlobOutputs, val: bool);
    #[wasm_bindgen(method, setter = "written")]
    fn written_shim(this: &AuthenticationExtensionsLargeBlobOutputs, val: bool);
}
impl AuthenticationExtensionsLargeBlobOutputs {
    #[doc = "Construct a new `AuthenticationExtensionsLargeBlobOutputs`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticationExtensionsLargeBlobOutputs`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `blob` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticationExtensionsLargeBlobOutputs`*"]
    pub fn blob(&mut self, val: &::js_sys::ArrayBuffer) -> &mut Self {
        self.blob_shim(val);
        self
    }
    #[doc = "Change the `supported` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticationExtensionsLargeBlobOutputs`*"]
    pub fn supported(&mut self, val: bool) -> &mut Self {
        self.supported_shim(val);
        self
    }
    #[doc = "Change the `written` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticationExtensionsLargeBlobOutputs`*"]
    pub fn written(&mut self, val: bool) -> &mut Self {
        self.written_shim(val);
        self
    }
}
impl Default for AuthenticationExtensionsLargeBlobOutputs {
    fn default() -> Self {
        Self::new()
    }
}
