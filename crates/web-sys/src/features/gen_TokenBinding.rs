#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = TokenBinding)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `TokenBinding` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TokenBinding`*"]
    pub type TokenBinding;
    #[wasm_bindgen(method, setter = "id")]
    fn id_shim(this: &TokenBinding, val: &str);
    #[wasm_bindgen(method, setter = "status")]
    fn status_shim(this: &TokenBinding, val: &str);
}
impl TokenBinding {
    #[doc = "Construct a new `TokenBinding`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TokenBinding`*"]
    pub fn new(status: &str) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.status(status);
        ret
    }
    #[doc = "Change the `id` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TokenBinding`*"]
    pub fn id(&mut self, val: &str) -> &mut Self {
        self.id_shim(val);
        self
    }
    #[doc = "Change the `status` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TokenBinding`*"]
    pub fn status(&mut self, val: &str) -> &mut Self {
        self.status_shim(val);
        self
    }
}
