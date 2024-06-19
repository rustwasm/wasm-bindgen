#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = PublicKeyCredentialDescriptor)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `PublicKeyCredentialDescriptor` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialDescriptor`*"]
    pub type PublicKeyCredentialDescriptor;
    #[doc = "Get the `id` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialDescriptor`*"]
    #[wasm_bindgen(method, getter = "id")]
    pub fn get_id(this: &PublicKeyCredentialDescriptor) -> ::js_sys::Object;
    #[wasm_bindgen(method, setter = "id")]
    fn set_id(this: &PublicKeyCredentialDescriptor, val: &::js_sys::Object);
    #[doc = "Get the `transports` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialDescriptor`*"]
    #[wasm_bindgen(method, getter = "transports")]
    pub fn get_transports(this: &PublicKeyCredentialDescriptor) -> Option<::js_sys::Array>;
    #[wasm_bindgen(method, setter = "transports")]
    fn set_transports(this: &PublicKeyCredentialDescriptor, val: &::wasm_bindgen::JsValue);
    #[cfg(feature = "PublicKeyCredentialType")]
    #[doc = "Get the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialDescriptor`, `PublicKeyCredentialType`*"]
    #[wasm_bindgen(method, getter = "type")]
    pub fn get_type(this: &PublicKeyCredentialDescriptor) -> PublicKeyCredentialType;
    #[cfg(feature = "PublicKeyCredentialType")]
    #[wasm_bindgen(method, setter = "type")]
    fn set_type(this: &PublicKeyCredentialDescriptor, val: PublicKeyCredentialType);
}
impl PublicKeyCredentialDescriptor {
    #[cfg(feature = "PublicKeyCredentialType")]
    #[doc = "Construct a new `PublicKeyCredentialDescriptor`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialDescriptor`, `PublicKeyCredentialType`*"]
    pub fn new(id: &::js_sys::Object, type_: PublicKeyCredentialType) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.id(id);
        ret.type_(type_);
        ret
    }
    #[doc = "Change the `id` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialDescriptor`*"]
    pub fn id(&mut self, val: &::js_sys::Object) -> &mut Self {
        self.set_id(val);
        self
    }
    #[doc = "Change the `transports` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialDescriptor`*"]
    pub fn transports(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_transports(val);
        self
    }
    #[cfg(feature = "PublicKeyCredentialType")]
    #[doc = "Change the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialDescriptor`, `PublicKeyCredentialType`*"]
    pub fn type_(&mut self, val: PublicKeyCredentialType) -> &mut Self {
        self.set_type(val);
        self
    }
}
