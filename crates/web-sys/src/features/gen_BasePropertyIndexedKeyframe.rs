#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = BasePropertyIndexedKeyframe)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `BasePropertyIndexedKeyframe` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BasePropertyIndexedKeyframe`*"]
    pub type BasePropertyIndexedKeyframe;
    #[doc = "Get the `composite` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BasePropertyIndexedKeyframe`*"]
    #[wasm_bindgen(method, getter = "composite")]
    pub fn get_composite(this: &BasePropertyIndexedKeyframe) -> ::wasm_bindgen::JsValue;
    #[wasm_bindgen(method, setter = "composite")]
    fn set_composite(this: &BasePropertyIndexedKeyframe, val: &::wasm_bindgen::JsValue);
    #[doc = "Get the `easing` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BasePropertyIndexedKeyframe`*"]
    #[wasm_bindgen(method, getter = "easing")]
    pub fn get_easing(this: &BasePropertyIndexedKeyframe) -> ::wasm_bindgen::JsValue;
    #[wasm_bindgen(method, setter = "easing")]
    fn set_easing(this: &BasePropertyIndexedKeyframe, val: &::wasm_bindgen::JsValue);
    #[doc = "Get the `offset` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BasePropertyIndexedKeyframe`*"]
    #[wasm_bindgen(method, getter = "offset")]
    pub fn get_offset(this: &BasePropertyIndexedKeyframe) -> ::wasm_bindgen::JsValue;
    #[wasm_bindgen(method, setter = "offset")]
    fn set_offset(this: &BasePropertyIndexedKeyframe, val: &::wasm_bindgen::JsValue);
}
impl BasePropertyIndexedKeyframe {
    #[doc = "Construct a new `BasePropertyIndexedKeyframe`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BasePropertyIndexedKeyframe`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `composite` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BasePropertyIndexedKeyframe`*"]
    pub fn composite(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_composite(val);
        self
    }
    #[doc = "Change the `easing` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BasePropertyIndexedKeyframe`*"]
    pub fn easing(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_easing(val);
        self
    }
    #[doc = "Change the `offset` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BasePropertyIndexedKeyframe`*"]
    pub fn offset(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_offset(val);
        self
    }
}
impl Default for BasePropertyIndexedKeyframe {
    fn default() -> Self {
        Self::new()
    }
}
