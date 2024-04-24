#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = ConsoleProfileEvent)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ConsoleProfileEvent` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleProfileEvent`*"]
    pub type ConsoleProfileEvent;
    #[wasm_bindgen(method, getter = "action")]
    fn action_shim(this: &ConsoleProfileEvent) -> String;
    #[wasm_bindgen(method, setter = "action")]
    fn set_action_shim(this: &ConsoleProfileEvent, val: &str);
    #[wasm_bindgen(method, getter = "arguments")]
    fn arguments_shim(this: &ConsoleProfileEvent) -> ::js_sys::Array;
    #[wasm_bindgen(method, setter = "arguments")]
    fn set_arguments_shim(this: &ConsoleProfileEvent, val: &::wasm_bindgen::JsValue);
}
#[doc = "The trait to access properties on the `ConsoleProfileEvent` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `ConsoleProfileEvent`*"]
pub trait ConsoleProfileEventGetters {
    #[doc = "Get the `action` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleProfileEvent`*"]
    fn action(&self) -> String;
    #[doc = "Get the `arguments` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleProfileEvent`*"]
    fn arguments(&self) -> ::js_sys::Array;
}
impl ConsoleProfileEventGetters for ConsoleProfileEvent {
    fn action(&self) -> String {
        self.action_shim()
    }
    fn arguments(&self) -> ::js_sys::Array {
        self.arguments_shim()
    }
}
impl ConsoleProfileEvent {
    #[doc = "Construct a new `ConsoleProfileEvent`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleProfileEvent`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `action` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleProfileEvent`*"]
    pub fn action(&mut self, val: &str) -> &mut Self {
        self.set_action_shim(val);
        self
    }
    #[doc = "Change the `arguments` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleProfileEvent`*"]
    pub fn arguments(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_arguments_shim(val);
        self
    }
}
impl Default for ConsoleProfileEvent {
    fn default() -> Self {
        Self::new()
    }
}
