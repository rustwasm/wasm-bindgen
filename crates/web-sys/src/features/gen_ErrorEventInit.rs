#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = ErrorEventInit)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ErrorEventInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ErrorEventInit`*"]
    pub type ErrorEventInit;
    #[wasm_bindgen(method, getter = "bubbles")]
    fn bubbles_shim(this: &ErrorEventInit) -> bool;
    #[wasm_bindgen(method, setter = "bubbles")]
    fn set_bubbles_shim(this: &ErrorEventInit, val: bool);
    #[wasm_bindgen(method, getter = "cancelable")]
    fn cancelable_shim(this: &ErrorEventInit) -> bool;
    #[wasm_bindgen(method, setter = "cancelable")]
    fn set_cancelable_shim(this: &ErrorEventInit, val: bool);
    #[wasm_bindgen(method, getter = "composed")]
    fn composed_shim(this: &ErrorEventInit) -> bool;
    #[wasm_bindgen(method, setter = "composed")]
    fn set_composed_shim(this: &ErrorEventInit, val: bool);
    #[wasm_bindgen(method, getter = "colno")]
    fn colno_shim(this: &ErrorEventInit) -> u32;
    #[wasm_bindgen(method, setter = "colno")]
    fn set_colno_shim(this: &ErrorEventInit, val: u32);
    #[wasm_bindgen(method, getter = "error")]
    fn error_shim(this: &ErrorEventInit) -> ::wasm_bindgen::JsValue;
    #[wasm_bindgen(method, setter = "error")]
    fn set_error_shim(this: &ErrorEventInit, val: &::wasm_bindgen::JsValue);
    #[wasm_bindgen(method, getter = "filename")]
    fn filename_shim(this: &ErrorEventInit) -> String;
    #[wasm_bindgen(method, setter = "filename")]
    fn set_filename_shim(this: &ErrorEventInit, val: &str);
    #[wasm_bindgen(method, getter = "lineno")]
    fn lineno_shim(this: &ErrorEventInit) -> u32;
    #[wasm_bindgen(method, setter = "lineno")]
    fn set_lineno_shim(this: &ErrorEventInit, val: u32);
    #[wasm_bindgen(method, getter = "message")]
    fn message_shim(this: &ErrorEventInit) -> String;
    #[wasm_bindgen(method, setter = "message")]
    fn set_message_shim(this: &ErrorEventInit, val: &str);
}
#[doc = "The trait to access properties on the `ErrorEventInit` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `ErrorEventInit`*"]
pub trait ErrorEventInitGetters {
    #[doc = "Get the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ErrorEventInit`*"]
    fn bubbles(&self) -> bool;
    #[doc = "Get the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ErrorEventInit`*"]
    fn cancelable(&self) -> bool;
    #[doc = "Get the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ErrorEventInit`*"]
    fn composed(&self) -> bool;
    #[doc = "Get the `colno` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ErrorEventInit`*"]
    fn colno(&self) -> u32;
    #[doc = "Get the `error` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ErrorEventInit`*"]
    fn error(&self) -> ::wasm_bindgen::JsValue;
    #[doc = "Get the `filename` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ErrorEventInit`*"]
    fn filename(&self) -> String;
    #[doc = "Get the `lineno` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ErrorEventInit`*"]
    fn lineno(&self) -> u32;
    #[doc = "Get the `message` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ErrorEventInit`*"]
    fn message(&self) -> String;
}
impl ErrorEventInitGetters for ErrorEventInit {
    fn bubbles(&self) -> bool {
        self.bubbles_shim()
    }
    fn cancelable(&self) -> bool {
        self.cancelable_shim()
    }
    fn composed(&self) -> bool {
        self.composed_shim()
    }
    fn colno(&self) -> u32 {
        self.colno_shim()
    }
    fn error(&self) -> ::wasm_bindgen::JsValue {
        self.error_shim()
    }
    fn filename(&self) -> String {
        self.filename_shim()
    }
    fn lineno(&self) -> u32 {
        self.lineno_shim()
    }
    fn message(&self) -> String {
        self.message_shim()
    }
}
impl ErrorEventInit {
    #[doc = "Construct a new `ErrorEventInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ErrorEventInit`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ErrorEventInit`*"]
    pub fn bubbles(&mut self, val: bool) -> &mut Self {
        self.set_bubbles_shim(val);
        self
    }
    #[doc = "Change the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ErrorEventInit`*"]
    pub fn cancelable(&mut self, val: bool) -> &mut Self {
        self.set_cancelable_shim(val);
        self
    }
    #[doc = "Change the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ErrorEventInit`*"]
    pub fn composed(&mut self, val: bool) -> &mut Self {
        self.set_composed_shim(val);
        self
    }
    #[doc = "Change the `colno` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ErrorEventInit`*"]
    pub fn colno(&mut self, val: u32) -> &mut Self {
        self.set_colno_shim(val);
        self
    }
    #[doc = "Change the `error` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ErrorEventInit`*"]
    pub fn error(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_error_shim(val);
        self
    }
    #[doc = "Change the `filename` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ErrorEventInit`*"]
    pub fn filename(&mut self, val: &str) -> &mut Self {
        self.set_filename_shim(val);
        self
    }
    #[doc = "Change the `lineno` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ErrorEventInit`*"]
    pub fn lineno(&mut self, val: u32) -> &mut Self {
        self.set_lineno_shim(val);
        self
    }
    #[doc = "Change the `message` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ErrorEventInit`*"]
    pub fn message(&mut self, val: &str) -> &mut Self {
        self.set_message_shim(val);
        self
    }
}
impl Default for ErrorEventInit {
    fn default() -> Self {
        Self::new()
    }
}
