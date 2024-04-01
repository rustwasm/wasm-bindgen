#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = AttributeNameValue)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `AttributeNameValue` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AttributeNameValue`*"]
    pub type AttributeNameValue;
    #[wasm_bindgen(method, setter = "name")]
    fn name_shim(this: &AttributeNameValue, val: &str);
    #[wasm_bindgen(method, setter = "value")]
    fn value_shim(this: &AttributeNameValue, val: &str);
}
impl AttributeNameValue {
    #[doc = "Construct a new `AttributeNameValue`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AttributeNameValue`*"]
    pub fn new(name: &str, value: &str) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.name(name);
        ret.value(value);
        ret
    }
    #[doc = "Change the `name` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AttributeNameValue`*"]
    pub fn name(&mut self, val: &str) -> &mut Self {
        self.name_shim(val);
        self
    }
    #[doc = "Change the `value` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AttributeNameValue`*"]
    pub fn value(&mut self, val: &str) -> &mut Self {
        self.value_shim(val);
        self
    }
}
