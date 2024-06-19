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
    #[doc = "Get the `icon` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialEntity`*"]
    #[wasm_bindgen(method, getter = "icon")]
    pub fn get_icon(this: &PublicKeyCredentialEntity) -> Option<String>;
    #[wasm_bindgen(method, setter = "icon")]
    fn set_icon(this: &PublicKeyCredentialEntity, val: &str);
    #[doc = "Get the `name` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialEntity`*"]
    #[wasm_bindgen(method, getter = "name")]
    pub fn get_name(this: &PublicKeyCredentialEntity) -> String;
    #[wasm_bindgen(method, setter = "name")]
    fn set_name(this: &PublicKeyCredentialEntity, val: &str);
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
        self.set_icon(val);
        self
    }
    #[doc = "Change the `name` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialEntity`*"]
    pub fn name(&mut self, val: &str) -> &mut Self {
        self.set_name(val);
        self
    }
}
