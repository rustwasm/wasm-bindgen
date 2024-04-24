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
    #[wasm_bindgen(method, getter = "name")]
    fn name_shim(this: &RsaOaepParams) -> String;
    #[wasm_bindgen(method, setter = "name")]
    fn set_name_shim(this: &RsaOaepParams, val: &str);
    #[wasm_bindgen(method, getter = "label")]
    fn label_shim(this: &RsaOaepParams) -> ::js_sys::Object;
    #[wasm_bindgen(method, setter = "label")]
    fn set_label_shim(this: &RsaOaepParams, val: &::js_sys::Object);
}
#[doc = "The trait to access properties on the `RsaOaepParams` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `RsaOaepParams`*"]
pub trait RsaOaepParamsGetters {
    #[doc = "Get the `name` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RsaOaepParams`*"]
    fn name(&self) -> String;
    #[doc = "Get the `label` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RsaOaepParams`*"]
    fn label(&self) -> ::js_sys::Object;
}
impl RsaOaepParamsGetters for RsaOaepParams {
    fn name(&self) -> String {
        self.name_shim()
    }
    fn label(&self) -> ::js_sys::Object {
        self.label_shim()
    }
}
impl RsaOaepParams {
    #[doc = "Construct a new `RsaOaepParams`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RsaOaepParams`*"]
    pub fn new(name: &str) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        Self::name(&mut ret, name);
        ret
    }
    #[doc = "Change the `name` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RsaOaepParams`*"]
    pub fn name(&mut self, val: &str) -> &mut Self {
        self.set_name_shim(val);
        self
    }
    #[doc = "Change the `label` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RsaOaepParams`*"]
    pub fn label(&mut self, val: &::js_sys::Object) -> &mut Self {
        self.set_label_shim(val);
        self
    }
}
