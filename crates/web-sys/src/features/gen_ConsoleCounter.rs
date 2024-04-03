#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = ConsoleCounter)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ConsoleCounter` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleCounter`*"]
    pub type ConsoleCounter;
    #[wasm_bindgen(method, setter = "count")]
    fn count_shim(this: &ConsoleCounter, val: u32);
    #[wasm_bindgen(method, setter = "label")]
    fn label_shim(this: &ConsoleCounter, val: &str);
}
impl ConsoleCounter {
    #[doc = "Construct a new `ConsoleCounter`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleCounter`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `count` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleCounter`*"]
    pub fn count(&mut self, val: u32) -> &mut Self {
        self.count_shim(val);
        self
    }
    #[doc = "Change the `label` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleCounter`*"]
    pub fn label(&mut self, val: &str) -> &mut Self {
        self.label_shim(val);
        self
    }
}
impl Default for ConsoleCounter {
    fn default() -> Self {
        Self::new()
    }
}
