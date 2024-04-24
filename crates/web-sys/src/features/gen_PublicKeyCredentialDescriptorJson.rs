#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = PublicKeyCredentialDescriptorJSON)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `PublicKeyCredentialDescriptorJson` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialDescriptorJson`*"]
    pub type PublicKeyCredentialDescriptorJson;
    #[wasm_bindgen(method, setter = "id")]
    fn id_shim(this: &PublicKeyCredentialDescriptorJson, val: &str);
    #[wasm_bindgen(method, setter = "transports")]
    fn transports_shim(this: &PublicKeyCredentialDescriptorJson, val: &::wasm_bindgen::JsValue);
    #[wasm_bindgen(method, setter = "type")]
    fn type__shim(this: &PublicKeyCredentialDescriptorJson, val: &str);
}
impl PublicKeyCredentialDescriptorJson {
    #[doc = "Construct a new `PublicKeyCredentialDescriptorJson`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialDescriptorJson`*"]
    pub fn new(id: &str, type_: &str) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.id(id);
        ret.type_(type_);
        ret
    }
    #[doc = "Change the `id` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialDescriptorJson`*"]
    pub fn id(&mut self, val: &str) -> &mut Self {
        self.id_shim(val);
        self
    }
    #[doc = "Change the `transports` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialDescriptorJson`*"]
    pub fn transports(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.transports_shim(val);
        self
    }
    #[doc = "Change the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialDescriptorJson`*"]
    pub fn type_(&mut self, val: &str) -> &mut Self {
        self.type__shim(val);
        self
    }
}
