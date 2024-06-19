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
    #[doc = "Get the `name` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AesGcmParams`*"]
    #[wasm_bindgen(method, getter = "name")]
    pub fn get_name(this: &AesGcmParams) -> String;
    #[wasm_bindgen(method, setter = "name")]
    fn set_name(this: &AesGcmParams, val: &str);
    #[doc = "Get the `additionalData` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AesGcmParams`*"]
    #[wasm_bindgen(method, getter = "additionalData")]
    pub fn get_additional_data(this: &AesGcmParams) -> Option<::js_sys::Object>;
    #[wasm_bindgen(method, setter = "additionalData")]
    fn set_additional_data(this: &AesGcmParams, val: &::js_sys::Object);
    #[doc = "Get the `iv` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AesGcmParams`*"]
    #[wasm_bindgen(method, getter = "iv")]
    pub fn get_iv(this: &AesGcmParams) -> ::js_sys::Object;
    #[wasm_bindgen(method, setter = "iv")]
    fn set_iv(this: &AesGcmParams, val: &::js_sys::Object);
    #[doc = "Get the `tagLength` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AesGcmParams`*"]
    #[wasm_bindgen(method, getter = "tagLength")]
    pub fn get_tag_length(this: &AesGcmParams) -> Option<u8>;
    #[wasm_bindgen(method, setter = "tagLength")]
    fn set_tag_length(this: &AesGcmParams, val: u8);
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
        self.set_name(val);
        self
    }
    #[doc = "Change the `additionalData` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AesGcmParams`*"]
    pub fn additional_data(&mut self, val: &::js_sys::Object) -> &mut Self {
        self.set_additional_data(val);
        self
    }
    #[doc = "Change the `iv` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AesGcmParams`*"]
    pub fn iv(&mut self, val: &::js_sys::Object) -> &mut Self {
        self.set_iv(val);
        self
    }
    #[doc = "Change the `tagLength` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AesGcmParams`*"]
    pub fn tag_length(&mut self, val: u8) -> &mut Self {
        self.set_tag_length(val);
        self
    }
}
