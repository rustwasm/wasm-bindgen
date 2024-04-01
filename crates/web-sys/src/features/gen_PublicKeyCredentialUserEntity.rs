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
    #[wasm_bindgen(method, setter = "icon")]
    fn icon_shim(this: &PublicKeyCredentialUserEntity, val: &str);
    #[wasm_bindgen(method, setter = "name")]
    fn name_shim(this: &PublicKeyCredentialUserEntity, val: &str);
    #[wasm_bindgen(method, setter = "displayName")]
    fn display_name_shim(this: &PublicKeyCredentialUserEntity, val: &str);
    #[wasm_bindgen(method, setter = "id")]
    fn id_shim(this: &PublicKeyCredentialUserEntity, val: &::js_sys::Object);
}
impl PublicKeyCredentialUserEntity {
    #[doc = "Construct a new `PublicKeyCredentialUserEntity`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialUserEntity`*"]
    pub fn new(name: &str, display_name: &str, id: &::js_sys::Object) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.name(name);
        ret.display_name(display_name);
        ret.id(id);
        ret
    }
    #[doc = "Change the `icon` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialUserEntity`*"]
    pub fn icon(&mut self, val: &str) -> &mut Self {
        self.icon_shim(val);
        self
    }
    #[doc = "Change the `name` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialUserEntity`*"]
    pub fn name(&mut self, val: &str) -> &mut Self {
        self.name_shim(val);
        self
    }
    #[doc = "Change the `displayName` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialUserEntity`*"]
    pub fn display_name(&mut self, val: &str) -> &mut Self {
        self.display_name_shim(val);
        self
    }
    #[doc = "Change the `id` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialUserEntity`*"]
    pub fn id(&mut self, val: &::js_sys::Object) -> &mut Self {
        self.id_shim(val);
        self
    }
}
