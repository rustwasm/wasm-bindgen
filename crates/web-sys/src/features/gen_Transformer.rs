#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = Transformer)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `Transformer` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Transformer`*"]
    pub type Transformer;
    #[doc = "Get the `flush` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Transformer`*"]
    #[wasm_bindgen(method, getter = "flush")]
    pub fn get_flush(this: &Transformer) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter = "flush")]
    fn set_flush(this: &Transformer, val: &::js_sys::Function);
    #[doc = "Get the `readableType` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Transformer`*"]
    #[wasm_bindgen(method, getter = "readableType")]
    pub fn get_readable_type(this: &Transformer) -> ::wasm_bindgen::JsValue;
    #[wasm_bindgen(method, setter = "readableType")]
    fn set_readable_type(this: &Transformer, val: &::wasm_bindgen::JsValue);
    #[doc = "Get the `start` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Transformer`*"]
    #[wasm_bindgen(method, getter = "start")]
    pub fn get_start(this: &Transformer) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter = "start")]
    fn set_start(this: &Transformer, val: &::js_sys::Function);
    #[doc = "Get the `transform` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Transformer`*"]
    #[wasm_bindgen(method, getter = "transform")]
    pub fn get_transform(this: &Transformer) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter = "transform")]
    fn set_transform(this: &Transformer, val: &::js_sys::Function);
    #[doc = "Get the `writableType` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Transformer`*"]
    #[wasm_bindgen(method, getter = "writableType")]
    pub fn get_writable_type(this: &Transformer) -> ::wasm_bindgen::JsValue;
    #[wasm_bindgen(method, setter = "writableType")]
    fn set_writable_type(this: &Transformer, val: &::wasm_bindgen::JsValue);
}
impl Transformer {
    #[doc = "Construct a new `Transformer`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Transformer`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `flush` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Transformer`*"]
    pub fn flush(&mut self, val: &::js_sys::Function) -> &mut Self {
        self.set_flush(val);
        self
    }
    #[doc = "Change the `readableType` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Transformer`*"]
    pub fn readable_type(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_readable_type(val);
        self
    }
    #[doc = "Change the `start` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Transformer`*"]
    pub fn start(&mut self, val: &::js_sys::Function) -> &mut Self {
        self.set_start(val);
        self
    }
    #[doc = "Change the `transform` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Transformer`*"]
    pub fn transform(&mut self, val: &::js_sys::Function) -> &mut Self {
        self.set_transform(val);
        self
    }
    #[doc = "Change the `writableType` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Transformer`*"]
    pub fn writable_type(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_writable_type(val);
        self
    }
}
impl Default for Transformer {
    fn default() -> Self {
        Self::new()
    }
}
