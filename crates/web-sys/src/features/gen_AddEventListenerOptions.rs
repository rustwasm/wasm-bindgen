#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = AddEventListenerOptions)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `AddEventListenerOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AddEventListenerOptions`*"]
    pub type AddEventListenerOptions;
    #[wasm_bindgen(method, getter = "capture")]
    fn capture_shim(this: &AddEventListenerOptions) -> bool;
    #[wasm_bindgen(method, setter = "capture")]
    fn set_capture_shim(this: &AddEventListenerOptions, val: bool);
    #[wasm_bindgen(method, getter = "once")]
    fn once_shim(this: &AddEventListenerOptions) -> bool;
    #[wasm_bindgen(method, setter = "once")]
    fn set_once_shim(this: &AddEventListenerOptions, val: bool);
    #[wasm_bindgen(method, getter = "passive")]
    fn passive_shim(this: &AddEventListenerOptions) -> bool;
    #[wasm_bindgen(method, setter = "passive")]
    fn set_passive_shim(this: &AddEventListenerOptions, val: bool);
}
#[doc = "The trait to access properties on the `AddEventListenerOptions` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `AddEventListenerOptions`*"]
pub trait AddEventListenerOptionsGetters {
    #[doc = "Get the `capture` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AddEventListenerOptions`*"]
    fn capture(&self) -> bool;
    #[doc = "Get the `once` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AddEventListenerOptions`*"]
    fn once(&self) -> bool;
    #[doc = "Get the `passive` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AddEventListenerOptions`*"]
    fn passive(&self) -> bool;
}
impl AddEventListenerOptionsGetters for AddEventListenerOptions {
    fn capture(&self) -> bool {
        self.capture_shim()
    }
    fn once(&self) -> bool {
        self.once_shim()
    }
    fn passive(&self) -> bool {
        self.passive_shim()
    }
}
impl AddEventListenerOptions {
    #[doc = "Construct a new `AddEventListenerOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AddEventListenerOptions`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `capture` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AddEventListenerOptions`*"]
    pub fn capture(&mut self, val: bool) -> &mut Self {
        self.set_capture_shim(val);
        self
    }
    #[doc = "Change the `once` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AddEventListenerOptions`*"]
    pub fn once(&mut self, val: bool) -> &mut Self {
        self.set_once_shim(val);
        self
    }
    #[doc = "Change the `passive` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AddEventListenerOptions`*"]
    pub fn passive(&mut self, val: bool) -> &mut Self {
        self.set_passive_shim(val);
        self
    }
}
impl Default for AddEventListenerOptions {
    fn default() -> Self {
        Self::new()
    }
}
