#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = WebSocketDict)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `WebSocketDict` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebSocketDict`*"]
    pub type WebSocketDict;
    #[wasm_bindgen(method, getter = "websockets")]
    fn websockets_shim(this: &WebSocketDict) -> &::wasm_bindgen::JsValue;
    #[wasm_bindgen(method, setter = "websockets")]
    fn set_websockets_shim(this: &WebSocketDict, val: &::wasm_bindgen::JsValue);
}
#[doc = "The trait to access properties on the `WebSocketDict` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `WebSocketDict`*"]
pub trait WebSocketDictGetters {
    #[doc = "Get the `websockets` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebSocketDict`*"]
    fn websockets(&self) -> &::wasm_bindgen::JsValue;
}
impl WebSocketDictGetters for WebSocketDict {
    fn websockets(&self) -> &::wasm_bindgen::JsValue {
        self.websockets_shim()
    }
}
impl WebSocketDict {
    #[doc = "Construct a new `WebSocketDict`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebSocketDict`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `websockets` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebSocketDict`*"]
    pub fn websockets(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_websockets_shim(val);
        self
    }
}
impl Default for WebSocketDict {
    fn default() -> Self {
        Self::new()
    }
}
