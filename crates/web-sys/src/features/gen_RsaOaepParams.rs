#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = RsaOaepParams)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `RsaOaepParams` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RsaOaepParams`*"]
    pub type RsaOaepParams;
    #[doc = "Get the `name` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RsaOaepParams`*"]
    #[wasm_bindgen(method, getter = "name")]
    pub fn get_name(this: &RsaOaepParams) -> String;
    #[wasm_bindgen(method, setter = "name")]
    fn set_name(this: &RsaOaepParams, val: &str);
    #[doc = "Get the `label` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RsaOaepParams`*"]
    #[wasm_bindgen(method, getter = "label")]
    pub fn get_label(this: &RsaOaepParams) -> Option<::js_sys::Object>;
    #[wasm_bindgen(method, setter = "label")]
    fn set_label(this: &RsaOaepParams, val: &::js_sys::Object);
}
impl RsaOaepParams {
    #[doc = "Construct a new `RsaOaepParams`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RsaOaepParams`*"]
    pub fn new(name: &str) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.name(name);
        ret
    }
    #[doc = "Change the `name` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RsaOaepParams`*"]
    pub fn name(&mut self, val: &str) -> &mut Self {
        self.set_name(val);
        self
    }
    #[doc = "Change the `label` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RsaOaepParams`*"]
    pub fn label(&mut self, val: &::js_sys::Object) -> &mut Self {
        self.set_label(val);
        self
    }
}
