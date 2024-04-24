#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = FileSystemRemoveOptions)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `FileSystemRemoveOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemRemoveOptions`*"]
    pub type FileSystemRemoveOptions;
    #[wasm_bindgen(method, getter = "recursive")]
    fn recursive_shim(this: &FileSystemRemoveOptions) -> bool;
    #[wasm_bindgen(method, setter = "recursive")]
    fn set_recursive_shim(this: &FileSystemRemoveOptions, val: bool);
}
#[doc = "The trait to access properties on the `FileSystemRemoveOptions` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `FileSystemRemoveOptions`*"]
pub trait FileSystemRemoveOptionsGetters {
    #[doc = "Get the `recursive` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemRemoveOptions`*"]
    fn recursive(&self) -> bool;
}
impl FileSystemRemoveOptionsGetters for FileSystemRemoveOptions {
    fn recursive(&self) -> bool {
        self.recursive_shim()
    }
}
impl FileSystemRemoveOptions {
    #[doc = "Construct a new `FileSystemRemoveOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemRemoveOptions`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `recursive` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemRemoveOptions`*"]
    pub fn recursive(&mut self, val: bool) -> &mut Self {
        self.set_recursive_shim(val);
        self
    }
}
impl Default for FileSystemRemoveOptions {
    fn default() -> Self {
        Self::new()
    }
}
