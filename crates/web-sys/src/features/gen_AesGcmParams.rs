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
    #[wasm_bindgen(method, getter = "name")]
    fn name_shim(this: &AesGcmParams) -> &str;
    #[wasm_bindgen(method, setter = "name")]
    fn set_name_shim(this: &AesGcmParams, val: &str);
    #[wasm_bindgen(method, getter = "additionalData")]
    fn additional_data_shim(this: &AesGcmParams) -> &::js_sys::Object;
    #[wasm_bindgen(method, setter = "additionalData")]
    fn set_additional_data_shim(this: &AesGcmParams, val: &::js_sys::Object);
    #[wasm_bindgen(method, getter = "iv")]
    fn iv_shim(this: &AesGcmParams) -> &::js_sys::Object;
    #[wasm_bindgen(method, setter = "iv")]
    fn set_iv_shim(this: &AesGcmParams, val: &::js_sys::Object);
    #[wasm_bindgen(method, getter = "tagLength")]
    fn tag_length_shim(this: &AesGcmParams) -> u8;
    #[wasm_bindgen(method, setter = "tagLength")]
    fn set_tag_length_shim(this: &AesGcmParams, val: u8);
}
#[doc = "The trait to access properties on the `AesGcmParams` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `AesGcmParams`*"]
pub trait AesGcmParamsGetters {
    #[doc = "Get the `name` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AesGcmParams`*"]
    fn name(&self) -> &str;
    #[doc = "Get the `additionalData` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AesGcmParams`*"]
    fn additional_data(&self) -> &::js_sys::Object;
    #[doc = "Get the `iv` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AesGcmParams`*"]
    fn iv(&self) -> &::js_sys::Object;
    #[doc = "Get the `tagLength` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AesGcmParams`*"]
    fn tag_length(&self) -> u8;
}
impl AesGcmParamsGetters for AesGcmParams {
    fn name(&self) -> &str {
        self.name_shim()
    }
    fn additional_data(&self) -> &::js_sys::Object {
        self.additional_data_shim()
    }
    fn iv(&self) -> &::js_sys::Object {
        self.iv_shim()
    }
    fn tag_length(&self) -> u8 {
        self.tag_length_shim()
    }
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
        self.set_name_shim(val);
        self
    }
    #[doc = "Change the `additionalData` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AesGcmParams`*"]
    pub fn additional_data(&mut self, val: &::js_sys::Object) -> &mut Self {
        self.set_additional_data_shim(val);
        self
    }
    #[doc = "Change the `iv` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AesGcmParams`*"]
    pub fn iv(&mut self, val: &::js_sys::Object) -> &mut Self {
        self.set_iv_shim(val);
        self
    }
    #[doc = "Change the `tagLength` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AesGcmParams`*"]
    pub fn tag_length(&mut self, val: u8) -> &mut Self {
        self.set_tag_length_shim(val);
        self
    }
}
