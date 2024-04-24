#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = PublicKeyCredentialUserEntity)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `PublicKeyCredentialUserEntity` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialUserEntity`*"]
    pub type PublicKeyCredentialUserEntity;
    #[wasm_bindgen(method, getter = "icon")]
    fn icon_shim(this: &PublicKeyCredentialUserEntity) -> String;
    #[wasm_bindgen(method, setter = "icon")]
    fn set_icon_shim(this: &PublicKeyCredentialUserEntity, val: &str);
    #[wasm_bindgen(method, getter = "name")]
    fn name_shim(this: &PublicKeyCredentialUserEntity) -> String;
    #[wasm_bindgen(method, setter = "name")]
    fn set_name_shim(this: &PublicKeyCredentialUserEntity, val: &str);
    #[wasm_bindgen(method, getter = "displayName")]
    fn display_name_shim(this: &PublicKeyCredentialUserEntity) -> String;
    #[wasm_bindgen(method, setter = "displayName")]
    fn set_display_name_shim(this: &PublicKeyCredentialUserEntity, val: &str);
    #[wasm_bindgen(method, getter = "id")]
    fn id_shim(this: &PublicKeyCredentialUserEntity) -> ::js_sys::Object;
    #[wasm_bindgen(method, setter = "id")]
    fn set_id_shim(this: &PublicKeyCredentialUserEntity, val: &::js_sys::Object);
}
#[doc = "The trait to access properties on the `PublicKeyCredentialUserEntity` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialUserEntity`*"]
pub trait PublicKeyCredentialUserEntityGetters {
    #[doc = "Get the `icon` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialUserEntity`*"]
    fn icon(&self) -> String;
    #[doc = "Get the `name` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialUserEntity`*"]
    fn name(&self) -> String;
    #[doc = "Get the `displayName` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialUserEntity`*"]
    fn display_name(&self) -> String;
    #[doc = "Get the `id` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialUserEntity`*"]
    fn id(&self) -> ::js_sys::Object;
}
impl PublicKeyCredentialUserEntityGetters for PublicKeyCredentialUserEntity {
    fn icon(&self) -> String {
        self.icon_shim()
    }
    fn name(&self) -> String {
        self.name_shim()
    }
    fn display_name(&self) -> String {
        self.display_name_shim()
    }
    fn id(&self) -> ::js_sys::Object {
        self.id_shim()
    }
}
impl PublicKeyCredentialUserEntity {
    #[doc = "Construct a new `PublicKeyCredentialUserEntity`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialUserEntity`*"]
    pub fn new(name: &str, display_name: &str, id: &::js_sys::Object) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        Self::name(&mut ret, name);
        Self::display_name(&mut ret, display_name);
        Self::id(&mut ret, id);
        ret
    }
    #[doc = "Change the `icon` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialUserEntity`*"]
    pub fn icon(&mut self, val: &str) -> &mut Self {
        self.set_icon_shim(val);
        self
    }
    #[doc = "Change the `name` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialUserEntity`*"]
    pub fn name(&mut self, val: &str) -> &mut Self {
        self.set_name_shim(val);
        self
    }
    #[doc = "Change the `displayName` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialUserEntity`*"]
    pub fn display_name(&mut self, val: &str) -> &mut Self {
        self.set_display_name_shim(val);
        self
    }
    #[doc = "Change the `id` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialUserEntity`*"]
    pub fn id(&mut self, val: &::js_sys::Object) -> &mut Self {
        self.set_id_shim(val);
        self
    }
}
