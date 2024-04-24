#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = ConsoleCounterError)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ConsoleCounterError` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleCounterError`*"]
    pub type ConsoleCounterError;
    #[wasm_bindgen(method, getter = "error")]
    fn error_shim(this: &ConsoleCounterError) -> &str;
    #[wasm_bindgen(method, setter = "error")]
    fn set_error_shim(this: &ConsoleCounterError, val: &str);
    #[wasm_bindgen(method, getter = "label")]
    fn label_shim(this: &ConsoleCounterError) -> &str;
    #[wasm_bindgen(method, setter = "label")]
    fn set_label_shim(this: &ConsoleCounterError, val: &str);
}
#[doc = "The trait to access properties on the `ConsoleCounterError` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `ConsoleCounterError`*"]
pub trait ConsoleCounterErrorGetters {
    #[doc = "Get the `error` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleCounterError`*"]
    fn error(&self) -> &str;
    #[doc = "Get the `label` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleCounterError`*"]
    fn label(&self) -> &str;
}
impl ConsoleCounterErrorGetters for ConsoleCounterError {
    fn error(&self) -> &str {
        self.error_shim()
    }
    fn label(&self) -> &str {
        self.label_shim()
    }
}
impl ConsoleCounterError {
    #[doc = "Construct a new `ConsoleCounterError`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleCounterError`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `error` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleCounterError`*"]
    pub fn error(&mut self, val: &str) -> &mut Self {
        self.set_error_shim(val);
        self
    }
    #[doc = "Change the `label` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleCounterError`*"]
    pub fn label(&mut self, val: &str) -> &mut Self {
        self.set_label_shim(val);
        self
    }
}
impl Default for ConsoleCounterError {
    fn default() -> Self {
        Self::new()
    }
}
