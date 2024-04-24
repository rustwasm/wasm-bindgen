#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = FontFaceSetIteratorResult)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `FontFaceSetIteratorResult` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FontFaceSetIteratorResult`*"]
    pub type FontFaceSetIteratorResult;
    #[wasm_bindgen(method, getter = "done")]
    fn done_shim(this: &FontFaceSetIteratorResult) -> bool;
    #[wasm_bindgen(method, setter = "done")]
    fn set_done_shim(this: &FontFaceSetIteratorResult, val: bool);
    #[wasm_bindgen(method, getter = "value")]
    fn value_shim(this: &FontFaceSetIteratorResult) -> ::wasm_bindgen::JsValue;
    #[wasm_bindgen(method, setter = "value")]
    fn set_value_shim(this: &FontFaceSetIteratorResult, val: &::wasm_bindgen::JsValue);
}
#[doc = "The trait to access properties on the `FontFaceSetIteratorResult` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `FontFaceSetIteratorResult`*"]
pub trait FontFaceSetIteratorResultGetters {
    #[doc = "Get the `done` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FontFaceSetIteratorResult`*"]
    fn done(&self) -> bool;
    #[doc = "Get the `value` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FontFaceSetIteratorResult`*"]
    fn value(&self) -> ::wasm_bindgen::JsValue;
}
impl FontFaceSetIteratorResultGetters for FontFaceSetIteratorResult {
    fn done(&self) -> bool {
        self.done_shim()
    }
    fn value(&self) -> ::wasm_bindgen::JsValue {
        self.value_shim()
    }
}
impl FontFaceSetIteratorResult {
    #[doc = "Construct a new `FontFaceSetIteratorResult`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FontFaceSetIteratorResult`*"]
    pub fn new(done: bool, value: &::wasm_bindgen::JsValue) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        Self::done(&mut ret, done);
        Self::value(&mut ret, value);
        ret
    }
    #[doc = "Change the `done` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FontFaceSetIteratorResult`*"]
    pub fn done(&mut self, val: bool) -> &mut Self {
        self.set_done_shim(val);
        self
    }
    #[doc = "Change the `value` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FontFaceSetIteratorResult`*"]
    pub fn value(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_value_shim(val);
        self
    }
}
