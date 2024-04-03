#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = VoidCallback)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `VoidCallback` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VoidCallback`*"]
    pub type VoidCallback;
    #[wasm_bindgen(method, setter = "handleEvent")]
    fn handle_event_shim(this: &VoidCallback, val: &::js_sys::Function);
}
impl VoidCallback {
    #[doc = "Construct a new `VoidCallback`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VoidCallback`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `handleEvent` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VoidCallback`*"]
    pub fn handle_event(&mut self, val: &::js_sys::Function) -> &mut Self {
        self.handle_event_shim(val);
        self
    }
}
impl Default for VoidCallback {
    fn default() -> Self {
        Self::new()
    }
}
