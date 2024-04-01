#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = FileSystemGetDirectoryOptions)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `FileSystemGetDirectoryOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemGetDirectoryOptions`*"]
    pub type FileSystemGetDirectoryOptions;
    #[wasm_bindgen(method, setter = "create")]
    fn create_shim(this: &FileSystemGetDirectoryOptions, val: bool);
}
impl FileSystemGetDirectoryOptions {
    #[doc = "Construct a new `FileSystemGetDirectoryOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemGetDirectoryOptions`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `create` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemGetDirectoryOptions`*"]
    pub fn create(&mut self, val: bool) -> &mut Self {
        self.create_shim(val);
        self
    }
}
impl Default for FileSystemGetDirectoryOptions {
    fn default() -> Self {
        Self::new()
    }
}
