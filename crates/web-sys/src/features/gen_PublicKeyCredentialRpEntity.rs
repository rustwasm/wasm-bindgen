#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = PublicKeyCredentialRpEntity)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `PublicKeyCredentialRpEntity` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialRpEntity`*"]
    pub type PublicKeyCredentialRpEntity;
    #[wasm_bindgen(method, getter = "icon")]
    fn icon_shim(this: &PublicKeyCredentialRpEntity) -> &str;
    #[wasm_bindgen(method, setter = "icon")]
    fn set_icon_shim(this: &PublicKeyCredentialRpEntity, val: &str);
    #[wasm_bindgen(method, getter = "name")]
    fn name_shim(this: &PublicKeyCredentialRpEntity) -> &str;
    #[wasm_bindgen(method, setter = "name")]
    fn set_name_shim(this: &PublicKeyCredentialRpEntity, val: &str);
    #[wasm_bindgen(method, getter = "id")]
    fn id_shim(this: &PublicKeyCredentialRpEntity) -> &str;
    #[wasm_bindgen(method, setter = "id")]
    fn set_id_shim(this: &PublicKeyCredentialRpEntity, val: &str);
}
#[doc = "The trait to access properties on the `PublicKeyCredentialRpEntity` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialRpEntity`*"]
pub trait PublicKeyCredentialRpEntityGetters {
    #[doc = "Get the `icon` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialRpEntity`*"]
    fn icon(&self) -> &str;
    #[doc = "Get the `name` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialRpEntity`*"]
    fn name(&self) -> &str;
    #[doc = "Get the `id` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialRpEntity`*"]
    fn id(&self) -> &str;
}
impl PublicKeyCredentialRpEntityGetters for PublicKeyCredentialRpEntity {
    fn icon(&self) -> &str {
        self.icon_shim()
    }
    fn name(&self) -> &str {
        self.name_shim()
    }
    fn id(&self) -> &str {
        self.id_shim()
    }
}
impl PublicKeyCredentialRpEntity {
    #[doc = "Construct a new `PublicKeyCredentialRpEntity`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialRpEntity`*"]
    pub fn new(name: &str) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.name(name);
        ret
    }
    #[doc = "Change the `icon` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialRpEntity`*"]
    pub fn icon(&mut self, val: &str) -> &mut Self {
        self.set_icon_shim(val);
        self
    }
    #[doc = "Change the `name` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialRpEntity`*"]
    pub fn name(&mut self, val: &str) -> &mut Self {
        self.set_name_shim(val);
        self
    }
    #[doc = "Change the `id` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialRpEntity`*"]
    pub fn id(&mut self, val: &str) -> &mut Self {
        self.set_id_shim(val);
        self
    }
}
