#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = PublicKeyCredentialParameters)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `PublicKeyCredentialParameters` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialParameters`*"]
    pub type PublicKeyCredentialParameters;
    #[doc = "Get the `alg` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialParameters`*"]
    #[wasm_bindgen(method, getter = "alg")]
    pub fn get_alg(this: &PublicKeyCredentialParameters) -> i32;
    #[wasm_bindgen(method, setter = "alg")]
    fn set_alg(this: &PublicKeyCredentialParameters, val: i32);
    #[cfg(feature = "PublicKeyCredentialType")]
    #[doc = "Get the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialParameters`, `PublicKeyCredentialType`*"]
    #[wasm_bindgen(method, getter = "type")]
    pub fn get_type(this: &PublicKeyCredentialParameters) -> PublicKeyCredentialType;
    #[cfg(feature = "PublicKeyCredentialType")]
    #[wasm_bindgen(method, setter = "type")]
    fn set_type(this: &PublicKeyCredentialParameters, val: PublicKeyCredentialType);
}
impl PublicKeyCredentialParameters {
    #[cfg(feature = "PublicKeyCredentialType")]
    #[doc = "Construct a new `PublicKeyCredentialParameters`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialParameters`, `PublicKeyCredentialType`*"]
    pub fn new(alg: i32, type_: PublicKeyCredentialType) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.alg(alg);
        ret.type_(type_);
        ret
    }
    #[doc = "Change the `alg` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialParameters`*"]
    pub fn alg(&mut self, val: i32) -> &mut Self {
        self.set_alg(val);
        self
    }
    #[cfg(feature = "PublicKeyCredentialType")]
    #[doc = "Change the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialParameters`, `PublicKeyCredentialType`*"]
    pub fn type_(&mut self, val: PublicKeyCredentialType) -> &mut Self {
        self.set_type(val);
        self
    }
}
