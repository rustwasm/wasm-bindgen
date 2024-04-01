#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = WriteParams)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `WriteParams` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WriteParams`*"]
    pub type WriteParams;
    #[wasm_bindgen(method, setter = "data")]
    fn data_shim(this: &WriteParams, val: &::wasm_bindgen::JsValue);
    #[wasm_bindgen(method, setter = "position")]
    fn position_shim(this: &WriteParams, val: Option<f64>);
    #[wasm_bindgen(method, setter = "size")]
    fn size_shim(this: &WriteParams, val: Option<f64>);
    #[cfg(feature = "WriteCommandType")]
    #[wasm_bindgen(method, setter = "type")]
    fn type__shim(this: &WriteParams, val: WriteCommandType);
}
impl WriteParams {
    #[cfg(feature = "WriteCommandType")]
    #[doc = "Construct a new `WriteParams`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WriteCommandType`, `WriteParams`*"]
    pub fn new(type_: WriteCommandType) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.type_(type_);
        ret
    }
    #[doc = "Change the `data` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WriteParams`*"]
    pub fn data(&mut self, val: Option<&::wasm_bindgen::JsValue>) -> &mut Self {
        self.data_shim(val.unwrap_or(&::wasm_bindgen::JsValue::NULL));
        self
    }
    #[doc = "Change the `position` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WriteParams`*"]
    pub fn position(&mut self, val: Option<f64>) -> &mut Self {
        self.position_shim(val);
        self
    }
    #[doc = "Change the `size` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WriteParams`*"]
    pub fn size(&mut self, val: Option<f64>) -> &mut Self {
        self.size_shim(val);
        self
    }
    #[cfg(feature = "WriteCommandType")]
    #[doc = "Change the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WriteCommandType`, `WriteParams`*"]
    pub fn type_(&mut self, val: WriteCommandType) -> &mut Self {
        self.type__shim(val);
        self
    }
}
