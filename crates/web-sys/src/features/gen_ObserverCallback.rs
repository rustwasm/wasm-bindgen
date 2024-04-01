#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = ObserverCallback)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ObserverCallback` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ObserverCallback`*"]
    pub type ObserverCallback;
    #[wasm_bindgen(method, setter = "handleEvent")]
    fn handle_event_shim(this: &ObserverCallback, val: &::js_sys::Function);
}
impl ObserverCallback {
    #[doc = "Construct a new `ObserverCallback`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ObserverCallback`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `handleEvent` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ObserverCallback`*"]
    pub fn handle_event(&mut self, val: &::js_sys::Function) -> &mut Self {
        self.handle_event_shim(val);
        self
    }
}
impl Default for ObserverCallback {
    fn default() -> Self {
        Self::new()
    }
}
