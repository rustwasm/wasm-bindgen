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
    #[doc = "Get the `icon` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialRpEntity`*"]
    #[wasm_bindgen(method, getter = "icon")]
    pub fn get_icon(this: &PublicKeyCredentialRpEntity) -> Option<String>;
    #[wasm_bindgen(method, setter = "icon")]
    fn set_icon(this: &PublicKeyCredentialRpEntity, val: &str);
    #[doc = "Get the `name` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialRpEntity`*"]
    #[wasm_bindgen(method, getter = "name")]
    pub fn get_name(this: &PublicKeyCredentialRpEntity) -> String;
    #[wasm_bindgen(method, setter = "name")]
    fn set_name(this: &PublicKeyCredentialRpEntity, val: &str);
    #[doc = "Get the `id` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialRpEntity`*"]
    #[wasm_bindgen(method, getter = "id")]
    pub fn get_id(this: &PublicKeyCredentialRpEntity) -> Option<String>;
    #[wasm_bindgen(method, setter = "id")]
    fn set_id(this: &PublicKeyCredentialRpEntity, val: &str);
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
        self.set_icon(val);
        self
    }
    #[doc = "Change the `name` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialRpEntity`*"]
    pub fn name(&mut self, val: &str) -> &mut Self {
        self.set_name(val);
        self
    }
    #[doc = "Change the `id` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialRpEntity`*"]
    pub fn id(&mut self, val: &str) -> &mut Self {
        self.set_id(val);
        self
    }
}
