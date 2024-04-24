#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = PublicKeyCredentialEntity)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `PublicKeyCredentialEntity` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialEntity`*"]
    pub type PublicKeyCredentialEntity;
    #[wasm_bindgen(method, getter = "icon")]
    fn icon_shim(this: &PublicKeyCredentialEntity) -> String;
    #[wasm_bindgen(method, setter = "icon")]
    fn set_icon_shim(this: &PublicKeyCredentialEntity, val: &str);
    #[wasm_bindgen(method, getter = "name")]
    fn name_shim(this: &PublicKeyCredentialEntity) -> String;
    #[wasm_bindgen(method, setter = "name")]
    fn set_name_shim(this: &PublicKeyCredentialEntity, val: &str);
}
#[doc = "The trait to access properties on the `PublicKeyCredentialEntity` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialEntity`*"]
pub trait PublicKeyCredentialEntityGetters {
    #[doc = "Get the `icon` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialEntity`*"]
    fn icon(&self) -> String;
    #[doc = "Get the `name` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialEntity`*"]
    fn name(&self) -> String;
}
impl PublicKeyCredentialEntityGetters for PublicKeyCredentialEntity {
    fn icon(&self) -> String {
        self.icon_shim()
    }
    fn name(&self) -> String {
        self.name_shim()
    }
}
impl PublicKeyCredentialEntity {
    #[doc = "Construct a new `PublicKeyCredentialEntity`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialEntity`*"]
    pub fn new(name: &str) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.name(name);
        ret
    }
    #[doc = "Change the `icon` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialEntity`*"]
    pub fn icon(&mut self, val: &str) -> &mut Self {
        self.set_icon_shim(val);
        self
    }
    #[doc = "Change the `name` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialEntity`*"]
    pub fn name(&mut self, val: &str) -> &mut Self {
        self.set_name_shim(val);
        self
    }
}
