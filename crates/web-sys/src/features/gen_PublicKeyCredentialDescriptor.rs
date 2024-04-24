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
    #[wasm_bindgen(method, getter = "id")]
    fn id_shim(this: &PublicKeyCredentialDescriptor) -> &::js_sys::Object;
    #[wasm_bindgen(method, setter = "id")]
    fn set_id_shim(this: &PublicKeyCredentialDescriptor, val: &::js_sys::Object);
    #[wasm_bindgen(method, getter = "transports")]
    fn transports_shim(this: &PublicKeyCredentialDescriptor) -> &::wasm_bindgen::JsValue;
    #[wasm_bindgen(method, setter = "transports")]
    fn set_transports_shim(this: &PublicKeyCredentialDescriptor, val: &::wasm_bindgen::JsValue);
    #[cfg(feature = "PublicKeyCredentialType")]
    #[wasm_bindgen(method, getter = "type")]
    fn type__shim(this: &PublicKeyCredentialDescriptor) -> PublicKeyCredentialType;
    #[cfg(feature = "PublicKeyCredentialType")]
    #[wasm_bindgen(method, setter = "type")]
    fn set_type__shim(this: &PublicKeyCredentialDescriptor, val: PublicKeyCredentialType);
}
#[doc = "The trait to access properties on the `PublicKeyCredentialDescriptor` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialDescriptor`*"]
pub trait PublicKeyCredentialDescriptorGetters {
    #[doc = "Get the `id` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialDescriptor`*"]
    fn id(&self) -> &::js_sys::Object;
    #[doc = "Get the `transports` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialDescriptor`*"]
    fn transports(&self) -> &::wasm_bindgen::JsValue;
    #[cfg(feature = "PublicKeyCredentialType")]
    #[doc = "Get the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialDescriptor`, `PublicKeyCredentialType`*"]
    fn type_(&self) -> PublicKeyCredentialType;
}
impl PublicKeyCredentialDescriptorGetters for PublicKeyCredentialDescriptor {
    fn id(&self) -> &::js_sys::Object {
        self.id_shim()
    }
    fn transports(&self) -> &::wasm_bindgen::JsValue {
        self.transports_shim()
    }
    #[cfg(feature = "PublicKeyCredentialType")]
    fn type_(&self) -> PublicKeyCredentialType {
        self.type__shim()
    }
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
        self.set_id_shim(val);
        self
    }
    #[doc = "Change the `transports` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialDescriptor`*"]
    pub fn transports(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_transports_shim(val);
        self
    }
    #[cfg(feature = "PublicKeyCredentialType")]
    #[doc = "Change the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialDescriptor`, `PublicKeyCredentialType`*"]
    pub fn type_(&mut self, val: PublicKeyCredentialType) -> &mut Self {
        self.set_type__shim(val);
        self
    }
}
