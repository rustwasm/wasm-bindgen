#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = ConsoleTimerError)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ConsoleTimerError` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleTimerError`*"]
    pub type ConsoleTimerError;
    #[wasm_bindgen(method, setter = "error")]
    fn error_shim(this: &ConsoleTimerError, val: &str);
    #[wasm_bindgen(method, setter = "name")]
    fn name_shim(this: &ConsoleTimerError, val: &str);
}
impl ConsoleTimerError {
    #[doc = "Construct a new `ConsoleTimerError`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleTimerError`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `error` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleTimerError`*"]
    pub fn error(&mut self, val: &str) -> &mut Self {
        self.error_shim(val);
        self
    }
    #[doc = "Change the `name` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleTimerError`*"]
    pub fn name(&mut self, val: &str) -> &mut Self {
        self.name_shim(val);
        self
    }
}
impl Default for ConsoleTimerError {
    fn default() -> Self {
        Self::new()
    }
}
