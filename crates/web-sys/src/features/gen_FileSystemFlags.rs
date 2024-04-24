#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = FileSystemFlags)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `FileSystemFlags` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemFlags`*"]
    pub type FileSystemFlags;
    #[wasm_bindgen(method, getter = "create")]
    fn create_shim(this: &FileSystemFlags) -> bool;
    #[wasm_bindgen(method, setter = "create")]
    fn set_create_shim(this: &FileSystemFlags, val: bool);
    #[wasm_bindgen(method, getter = "exclusive")]
    fn exclusive_shim(this: &FileSystemFlags) -> bool;
    #[wasm_bindgen(method, setter = "exclusive")]
    fn set_exclusive_shim(this: &FileSystemFlags, val: bool);
}
#[doc = "The trait to access properties on the `FileSystemFlags` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `FileSystemFlags`*"]
pub trait FileSystemFlagsGetters {
    #[doc = "Get the `create` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemFlags`*"]
    fn create(&self) -> bool;
    #[doc = "Get the `exclusive` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemFlags`*"]
    fn exclusive(&self) -> bool;
}
impl FileSystemFlagsGetters for FileSystemFlags {
    fn create(&self) -> bool {
        self.create_shim()
    }
    fn exclusive(&self) -> bool {
        self.exclusive_shim()
    }
}
impl FileSystemFlags {
    #[doc = "Construct a new `FileSystemFlags`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemFlags`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `create` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemFlags`*"]
    pub fn create(&mut self, val: bool) -> &mut Self {
        self.set_create_shim(val);
        self
    }
    #[doc = "Change the `exclusive` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemFlags`*"]
    pub fn exclusive(&mut self, val: bool) -> &mut Self {
        self.set_exclusive_shim(val);
        self
    }
}
impl Default for FileSystemFlags {
    fn default() -> Self {
        Self::new()
    }
}
