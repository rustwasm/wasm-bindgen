#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = AesGcmParams)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `AesGcmParams` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AesGcmParams`*"]
    pub type AesGcmParams;
    #[wasm_bindgen(method, setter = "name")]
    fn name_shim(this: &AesGcmParams, val: &str);
    #[wasm_bindgen(method, setter = "additionalData")]
    fn additional_data_shim(this: &AesGcmParams, val: &::js_sys::Object);
    #[wasm_bindgen(method, setter = "iv")]
    fn iv_shim(this: &AesGcmParams, val: &::js_sys::Object);
    #[wasm_bindgen(method, setter = "tagLength")]
    fn tag_length_shim(this: &AesGcmParams, val: u8);
}
impl AesGcmParams {
    #[doc = "Construct a new `AesGcmParams`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AesGcmParams`*"]
    pub fn new(name: &str, iv: &::js_sys::Object) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.name(name);
        ret.iv(iv);
        ret
    }
    #[doc = "Change the `name` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AesGcmParams`*"]
    pub fn name(&mut self, val: &str) -> &mut Self {
        self.name_shim(val);
        self
    }
    #[doc = "Change the `additionalData` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AesGcmParams`*"]
    pub fn additional_data(&mut self, val: &::js_sys::Object) -> &mut Self {
        self.additional_data_shim(val);
        self
    }
    #[doc = "Change the `iv` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AesGcmParams`*"]
    pub fn iv(&mut self, val: &::js_sys::Object) -> &mut Self {
        self.iv_shim(val);
        self
    }
    #[doc = "Change the `tagLength` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AesGcmParams`*"]
    pub fn tag_length(&mut self, val: u8) -> &mut Self {
        self.tag_length_shim(val);
        self
    }
}
