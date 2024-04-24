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
    #[wasm_bindgen(method, getter = "flush")]
    fn flush_shim(this: &Transformer) -> ::js_sys::Function;
    #[wasm_bindgen(method, setter = "flush")]
    fn set_flush_shim(this: &Transformer, val: &::js_sys::Function);
    #[wasm_bindgen(method, getter = "readableType")]
    fn readable_type_shim(this: &Transformer) -> ::wasm_bindgen::JsValue;
    #[wasm_bindgen(method, setter = "readableType")]
    fn set_readable_type_shim(this: &Transformer, val: &::wasm_bindgen::JsValue);
    #[wasm_bindgen(method, getter = "start")]
    fn start_shim(this: &Transformer) -> ::js_sys::Function;
    #[wasm_bindgen(method, setter = "start")]
    fn set_start_shim(this: &Transformer, val: &::js_sys::Function);
    #[wasm_bindgen(method, getter = "transform")]
    fn transform_shim(this: &Transformer) -> ::js_sys::Function;
    #[wasm_bindgen(method, setter = "transform")]
    fn set_transform_shim(this: &Transformer, val: &::js_sys::Function);
    #[wasm_bindgen(method, getter = "writableType")]
    fn writable_type_shim(this: &Transformer) -> ::wasm_bindgen::JsValue;
    #[wasm_bindgen(method, setter = "writableType")]
    fn set_writable_type_shim(this: &Transformer, val: &::wasm_bindgen::JsValue);
}
#[doc = "The trait to access properties on the `Transformer` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `Transformer`*"]
pub trait TransformerGetters {
    #[doc = "Get the `flush` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Transformer`*"]
    fn flush(&self) -> ::js_sys::Function;
    #[doc = "Get the `readableType` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Transformer`*"]
    fn readable_type(&self) -> ::wasm_bindgen::JsValue;
    #[doc = "Get the `start` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Transformer`*"]
    fn start(&self) -> ::js_sys::Function;
    #[doc = "Get the `transform` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Transformer`*"]
    fn transform(&self) -> ::js_sys::Function;
    #[doc = "Get the `writableType` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Transformer`*"]
    fn writable_type(&self) -> ::wasm_bindgen::JsValue;
}
impl TransformerGetters for Transformer {
    fn flush(&self) -> ::js_sys::Function {
        self.flush_shim()
    }
    fn readable_type(&self) -> ::wasm_bindgen::JsValue {
        self.readable_type_shim()
    }
    fn start(&self) -> ::js_sys::Function {
        self.start_shim()
    }
    fn transform(&self) -> ::js_sys::Function {
        self.transform_shim()
    }
    fn writable_type(&self) -> ::wasm_bindgen::JsValue {
        self.writable_type_shim()
    }
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
        self.set_flush_shim(val);
        self
    }
    #[doc = "Change the `readableType` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Transformer`*"]
    pub fn readable_type(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_readable_type_shim(val);
        self
    }
    #[doc = "Change the `start` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Transformer`*"]
    pub fn start(&mut self, val: &::js_sys::Function) -> &mut Self {
        self.set_start_shim(val);
        self
    }
    #[doc = "Change the `transform` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Transformer`*"]
    pub fn transform(&mut self, val: &::js_sys::Function) -> &mut Self {
        self.set_transform_shim(val);
        self
    }
    #[doc = "Change the `writableType` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Transformer`*"]
    pub fn writable_type(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_writable_type_shim(val);
        self
    }
}
impl Default for Transformer {
    fn default() -> Self {
        Self::new()
    }
}
