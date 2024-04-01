#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = ClipboardEventInit)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ClipboardEventInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ClipboardEventInit`*"]
    pub type ClipboardEventInit;
    #[wasm_bindgen(method, setter = "bubbles")]
    fn bubbles_shim(this: &ClipboardEventInit, val: bool);
    #[wasm_bindgen(method, setter = "cancelable")]
    fn cancelable_shim(this: &ClipboardEventInit, val: bool);
    #[wasm_bindgen(method, setter = "composed")]
    fn composed_shim(this: &ClipboardEventInit, val: bool);
    #[cfg(feature = "DataTransfer")]
    #[wasm_bindgen(method, setter = "clipboardData")]
    fn clipboard_data_shim(this: &ClipboardEventInit, val: Option<&DataTransfer>);
}
impl ClipboardEventInit {
    #[doc = "Construct a new `ClipboardEventInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ClipboardEventInit`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ClipboardEventInit`*"]
    pub fn bubbles(&mut self, val: bool) -> &mut Self {
        self.bubbles_shim(val);
        self
    }
    #[doc = "Change the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ClipboardEventInit`*"]
    pub fn cancelable(&mut self, val: bool) -> &mut Self {
        self.cancelable_shim(val);
        self
    }
    #[doc = "Change the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ClipboardEventInit`*"]
    pub fn composed(&mut self, val: bool) -> &mut Self {
        self.composed_shim(val);
        self
    }
    #[cfg(feature = "DataTransfer")]
    #[doc = "Change the `clipboardData` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ClipboardEventInit`, `DataTransfer`*"]
    pub fn clipboard_data(&mut self, val: Option<&DataTransfer>) -> &mut Self {
        self.clipboard_data_shim(val);
        self
    }
}
impl Default for ClipboardEventInit {
    fn default() -> Self {
        Self::new()
    }
}
