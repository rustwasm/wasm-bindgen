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
    #[wasm_bindgen(method, getter = "count")]
    fn count_shim(this: &ConsoleCounter) -> u32;
    #[wasm_bindgen(method, setter = "count")]
    fn set_count_shim(this: &ConsoleCounter, val: u32);
    #[wasm_bindgen(method, getter = "label")]
    fn label_shim(this: &ConsoleCounter) -> String;
    #[wasm_bindgen(method, setter = "label")]
    fn set_label_shim(this: &ConsoleCounter, val: &str);
}
#[doc = "The trait to access properties on the `ConsoleCounter` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `ConsoleCounter`*"]
pub trait ConsoleCounterGetters {
    #[doc = "Get the `count` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleCounter`*"]
    fn count(&self) -> u32;
    #[doc = "Get the `label` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleCounter`*"]
    fn label(&self) -> String;
}
impl ConsoleCounterGetters for ConsoleCounter {
    fn count(&self) -> u32 {
        self.count_shim()
    }
    fn label(&self) -> String {
        self.label_shim()
    }
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
        self.set_count_shim(val);
        self
    }
    #[doc = "Change the `label` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleCounter`*"]
    pub fn label(&mut self, val: &str) -> &mut Self {
        self.set_label_shim(val);
        self
    }
}
impl Default for ConsoleCounter {
    fn default() -> Self {
        Self::new()
    }
}
