#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = HttpConnDict)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `HttpConnDict` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HttpConnDict`*"]
    pub type HttpConnDict;
    #[wasm_bindgen(method, getter = "connections")]
    fn connections_shim(this: &HttpConnDict) -> &::wasm_bindgen::JsValue;
    #[wasm_bindgen(method, setter = "connections")]
    fn set_connections_shim(this: &HttpConnDict, val: &::wasm_bindgen::JsValue);
}
#[doc = "The trait to access properties on the `HttpConnDict` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `HttpConnDict`*"]
pub trait HttpConnDictGetters {
    #[doc = "Get the `connections` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HttpConnDict`*"]
    fn connections(&self) -> &::wasm_bindgen::JsValue;
}
impl HttpConnDictGetters for HttpConnDict {
    fn connections(&self) -> &::wasm_bindgen::JsValue {
        self.connections_shim()
    }
}
impl HttpConnDict {
    #[doc = "Construct a new `HttpConnDict`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HttpConnDict`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `connections` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HttpConnDict`*"]
    pub fn connections(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_connections_shim(val);
        self
    }
}
impl Default for HttpConnDict {
    fn default() -> Self {
        Self::new()
    }
}
