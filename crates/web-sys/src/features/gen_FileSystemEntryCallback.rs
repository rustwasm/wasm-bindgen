#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = FileSystemEntryCallback)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `FileSystemEntryCallback` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemEntryCallback`*"]
    pub type FileSystemEntryCallback;
    #[wasm_bindgen(method, getter = "handleEvent")]
    fn handle_event_shim(this: &FileSystemEntryCallback) -> &::js_sys::Function;
    #[wasm_bindgen(method, setter = "handleEvent")]
    fn set_handle_event_shim(this: &FileSystemEntryCallback, val: &::js_sys::Function);
}
#[doc = "The trait to access properties on the `FileSystemEntryCallback` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `FileSystemEntryCallback`*"]
pub trait FileSystemEntryCallbackGetters {
    #[doc = "Get the `handleEvent` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemEntryCallback`*"]
    fn handle_event(&self) -> &::js_sys::Function;
}
impl FileSystemEntryCallbackGetters for FileSystemEntryCallback {
    fn handle_event(&self) -> &::js_sys::Function {
        self.handle_event_shim()
    }
}
impl FileSystemEntryCallback {
    #[doc = "Construct a new `FileSystemEntryCallback`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemEntryCallback`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `handleEvent` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemEntryCallback`*"]
    pub fn handle_event(&mut self, val: &::js_sys::Function) -> &mut Self {
        self.set_handle_event_shim(val);
        self
    }
}
impl Default for FileSystemEntryCallback {
    fn default() -> Self {
        Self::new()
    }
}
