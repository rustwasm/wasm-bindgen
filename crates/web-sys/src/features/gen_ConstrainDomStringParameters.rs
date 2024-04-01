#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = ConstrainDOMStringParameters)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ConstrainDomStringParameters` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConstrainDomStringParameters`*"]
    pub type ConstrainDomStringParameters;
    #[wasm_bindgen(method, setter = "exact")]
    fn exact_shim(this: &ConstrainDomStringParameters, val: &::wasm_bindgen::JsValue);
    #[wasm_bindgen(method, setter = "ideal")]
    fn ideal_shim(this: &ConstrainDomStringParameters, val: &::wasm_bindgen::JsValue);
}
impl ConstrainDomStringParameters {
    #[doc = "Construct a new `ConstrainDomStringParameters`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConstrainDomStringParameters`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `exact` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConstrainDomStringParameters`*"]
    pub fn exact(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.exact_shim(val);
        self
    }
    #[doc = "Change the `ideal` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConstrainDomStringParameters`*"]
    pub fn ideal(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.ideal_shim(val);
        self
    }
}
impl Default for ConstrainDomStringParameters {
    fn default() -> Self {
        Self::new()
    }
}
