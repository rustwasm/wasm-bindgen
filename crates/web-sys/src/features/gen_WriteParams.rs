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
    #[wasm_bindgen(method, getter = "data")]
    fn data_shim(this: &WriteParams) -> ::wasm_bindgen::JsValue;
    #[wasm_bindgen(method, setter = "data")]
    fn set_data_shim(this: &WriteParams, val: &::wasm_bindgen::JsValue);
    #[wasm_bindgen(method, getter = "position")]
    fn position_shim(this: &WriteParams) -> Option<f64>;
    #[wasm_bindgen(method, setter = "position")]
    fn set_position_shim(this: &WriteParams, val: Option<f64>);
    #[wasm_bindgen(method, getter = "size")]
    fn size_shim(this: &WriteParams) -> Option<f64>;
    #[wasm_bindgen(method, setter = "size")]
    fn set_size_shim(this: &WriteParams, val: Option<f64>);
    #[cfg(feature = "WriteCommandType")]
    #[wasm_bindgen(method, getter = "type")]
    fn type__shim(this: &WriteParams) -> WriteCommandType;
    #[cfg(feature = "WriteCommandType")]
    #[wasm_bindgen(method, setter = "type")]
    fn set_type__shim(this: &WriteParams, val: WriteCommandType);
}
#[doc = "The trait to access properties on the `WriteParams` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `WriteParams`*"]
pub trait WriteParamsGetters {
    #[doc = "Get the `data` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WriteParams`*"]
    fn data(&self) -> ::wasm_bindgen::JsValue;
    #[doc = "Get the `position` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WriteParams`*"]
    fn position(&self) -> Option<f64>;
    #[doc = "Get the `size` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WriteParams`*"]
    fn size(&self) -> Option<f64>;
    #[cfg(feature = "WriteCommandType")]
    #[doc = "Get the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WriteCommandType`, `WriteParams`*"]
    fn type_(&self) -> WriteCommandType;
}
impl WriteParamsGetters for WriteParams {
    fn data(&self) -> ::wasm_bindgen::JsValue {
        self.data_shim()
    }
    fn position(&self) -> Option<f64> {
        self.position_shim()
    }
    fn size(&self) -> Option<f64> {
        self.size_shim()
    }
    #[cfg(feature = "WriteCommandType")]
    fn type_(&self) -> WriteCommandType {
        self.type__shim()
    }
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
        self.set_data_shim(val.unwrap_or(&::wasm_bindgen::JsValue::NULL));
        self
    }
    #[doc = "Change the `position` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WriteParams`*"]
    pub fn position(&mut self, val: Option<f64>) -> &mut Self {
        self.set_position_shim(val);
        self
    }
    #[doc = "Change the `size` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WriteParams`*"]
    pub fn size(&mut self, val: Option<f64>) -> &mut Self {
        self.set_size_shim(val);
        self
    }
    #[cfg(feature = "WriteCommandType")]
    #[doc = "Change the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WriteCommandType`, `WriteParams`*"]
    pub fn type_(&mut self, val: WriteCommandType) -> &mut Self {
        self.set_type__shim(val);
        self
    }
}
