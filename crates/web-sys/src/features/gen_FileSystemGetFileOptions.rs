#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = FileSystemGetFileOptions)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `FileSystemGetFileOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemGetFileOptions`*"]
    pub type FileSystemGetFileOptions;
    #[wasm_bindgen(method, getter = "create")]
    fn create_shim(this: &FileSystemGetFileOptions) -> bool;
    #[wasm_bindgen(method, setter = "create")]
    fn set_create_shim(this: &FileSystemGetFileOptions, val: bool);
}
#[doc = "The trait to access properties on the `FileSystemGetFileOptions` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `FileSystemGetFileOptions`*"]
pub trait FileSystemGetFileOptionsGetters {
    #[doc = "Get the `create` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemGetFileOptions`*"]
    fn create(&self) -> bool;
}
impl FileSystemGetFileOptionsGetters for FileSystemGetFileOptions {
    fn create(&self) -> bool {
        self.create_shim()
    }
}
impl FileSystemGetFileOptions {
    #[doc = "Construct a new `FileSystemGetFileOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemGetFileOptions`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `create` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemGetFileOptions`*"]
    pub fn create(&mut self, val: bool) -> &mut Self {
        self.set_create_shim(val);
        self
    }
}
impl Default for FileSystemGetFileOptions {
    fn default() -> Self {
        Self::new()
    }
}
