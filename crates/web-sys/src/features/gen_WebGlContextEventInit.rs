#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = WebGLContextEventInit)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `WebGlContextEventInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlContextEventInit`*"]
    pub type WebGlContextEventInit;
    #[doc = "Get the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlContextEventInit`*"]
    #[wasm_bindgen(method, getter = "bubbles")]
    pub fn get_bubbles(this: &WebGlContextEventInit) -> Option<bool>;
    #[wasm_bindgen(method, setter = "bubbles")]
    fn set_bubbles(this: &WebGlContextEventInit, val: bool);
    #[doc = "Get the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlContextEventInit`*"]
    #[wasm_bindgen(method, getter = "cancelable")]
    pub fn get_cancelable(this: &WebGlContextEventInit) -> Option<bool>;
    #[wasm_bindgen(method, setter = "cancelable")]
    fn set_cancelable(this: &WebGlContextEventInit, val: bool);
    #[doc = "Get the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlContextEventInit`*"]
    #[wasm_bindgen(method, getter = "composed")]
    pub fn get_composed(this: &WebGlContextEventInit) -> Option<bool>;
    #[wasm_bindgen(method, setter = "composed")]
    fn set_composed(this: &WebGlContextEventInit, val: bool);
    #[doc = "Get the `statusMessage` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlContextEventInit`*"]
    #[wasm_bindgen(method, getter = "statusMessage")]
    pub fn get_status_message(this: &WebGlContextEventInit) -> Option<String>;
    #[wasm_bindgen(method, setter = "statusMessage")]
    fn set_status_message(this: &WebGlContextEventInit, val: &str);
}
impl WebGlContextEventInit {
    #[doc = "Construct a new `WebGlContextEventInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlContextEventInit`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlContextEventInit`*"]
    pub fn bubbles(&mut self, val: bool) -> &mut Self {
        self.set_bubbles(val);
        self
    }
    #[doc = "Change the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlContextEventInit`*"]
    pub fn cancelable(&mut self, val: bool) -> &mut Self {
        self.set_cancelable(val);
        self
    }
    #[doc = "Change the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlContextEventInit`*"]
    pub fn composed(&mut self, val: bool) -> &mut Self {
        self.set_composed(val);
        self
    }
    #[doc = "Change the `statusMessage` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlContextEventInit`*"]
    pub fn status_message(&mut self, val: &str) -> &mut Self {
        self.set_status_message(val);
        self
    }
}
impl Default for WebGlContextEventInit {
    fn default() -> Self {
        Self::new()
    }
}
