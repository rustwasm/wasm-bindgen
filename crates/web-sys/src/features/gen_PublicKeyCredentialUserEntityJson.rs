#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = PublicKeyCredentialUserEntityJSON)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `PublicKeyCredentialUserEntityJson` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialUserEntityJson`*"]
    pub type PublicKeyCredentialUserEntityJson;
    #[wasm_bindgen(method, setter = "displayName")]
    fn display_name_shim(this: &PublicKeyCredentialUserEntityJson, val: &str);
    #[wasm_bindgen(method, setter = "id")]
    fn id_shim(this: &PublicKeyCredentialUserEntityJson, val: &str);
    #[wasm_bindgen(method, setter = "name")]
    fn name_shim(this: &PublicKeyCredentialUserEntityJson, val: &str);
}
impl PublicKeyCredentialUserEntityJson {
    #[doc = "Construct a new `PublicKeyCredentialUserEntityJson`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialUserEntityJson`*"]
    pub fn new(display_name: &str, id: &str, name: &str) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.display_name(display_name);
        ret.id(id);
        ret.name(name);
        ret
    }
    #[doc = "Change the `displayName` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialUserEntityJson`*"]
    pub fn display_name(&mut self, val: &str) -> &mut Self {
        self.display_name_shim(val);
        self
    }
    #[doc = "Change the `id` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialUserEntityJson`*"]
    pub fn id(&mut self, val: &str) -> &mut Self {
        self.id_shim(val);
        self
    }
    #[doc = "Change the `name` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialUserEntityJson`*"]
    pub fn name(&mut self, val: &str) -> &mut Self {
        self.name_shim(val);
        self
    }
}
