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
    #[wasm_bindgen(method, getter = "bubbles")]
    fn bubbles_shim(this: &WebGlContextEventInit) -> bool;
    #[wasm_bindgen(method, setter = "bubbles")]
    fn set_bubbles_shim(this: &WebGlContextEventInit, val: bool);
    #[wasm_bindgen(method, getter = "cancelable")]
    fn cancelable_shim(this: &WebGlContextEventInit) -> bool;
    #[wasm_bindgen(method, setter = "cancelable")]
    fn set_cancelable_shim(this: &WebGlContextEventInit, val: bool);
    #[wasm_bindgen(method, getter = "composed")]
    fn composed_shim(this: &WebGlContextEventInit) -> bool;
    #[wasm_bindgen(method, setter = "composed")]
    fn set_composed_shim(this: &WebGlContextEventInit, val: bool);
    #[wasm_bindgen(method, getter = "statusMessage")]
    fn status_message_shim(this: &WebGlContextEventInit) -> String;
    #[wasm_bindgen(method, setter = "statusMessage")]
    fn set_status_message_shim(this: &WebGlContextEventInit, val: &str);
}
#[doc = "The trait to access properties on the `WebGlContextEventInit` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `WebGlContextEventInit`*"]
pub trait WebGlContextEventInitGetters {
    #[doc = "Get the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlContextEventInit`*"]
    fn bubbles(&self) -> bool;
    #[doc = "Get the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlContextEventInit`*"]
    fn cancelable(&self) -> bool;
    #[doc = "Get the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlContextEventInit`*"]
    fn composed(&self) -> bool;
    #[doc = "Get the `statusMessage` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlContextEventInit`*"]
    fn status_message(&self) -> String;
}
impl WebGlContextEventInitGetters for WebGlContextEventInit {
    fn bubbles(&self) -> bool {
        self.bubbles_shim()
    }
    fn cancelable(&self) -> bool {
        self.cancelable_shim()
    }
    fn composed(&self) -> bool {
        self.composed_shim()
    }
    fn status_message(&self) -> String {
        self.status_message_shim()
    }
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
        self.set_bubbles_shim(val);
        self
    }
    #[doc = "Change the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlContextEventInit`*"]
    pub fn cancelable(&mut self, val: bool) -> &mut Self {
        self.set_cancelable_shim(val);
        self
    }
    #[doc = "Change the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlContextEventInit`*"]
    pub fn composed(&mut self, val: bool) -> &mut Self {
        self.set_composed_shim(val);
        self
    }
    #[doc = "Change the `statusMessage` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlContextEventInit`*"]
    pub fn status_message(&mut self, val: &str) -> &mut Self {
        self.set_status_message_shim(val);
        self
    }
}
impl Default for WebGlContextEventInit {
    fn default() -> Self {
        Self::new()
    }
}
