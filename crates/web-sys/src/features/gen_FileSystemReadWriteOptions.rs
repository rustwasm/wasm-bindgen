#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = FileSystemReadWriteOptions)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `FileSystemReadWriteOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemReadWriteOptions`*"]
    pub type FileSystemReadWriteOptions;
    #[wasm_bindgen(method, getter = "at")]
    fn at_shim(this: &FileSystemReadWriteOptions) -> f64;
    #[wasm_bindgen(method, setter = "at")]
    fn set_at_shim(this: &FileSystemReadWriteOptions, val: f64);
}
#[doc = "The trait to access properties on the `FileSystemReadWriteOptions` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `FileSystemReadWriteOptions`*"]
pub trait FileSystemReadWriteOptionsGetters {
    #[doc = "Get the `at` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemReadWriteOptions`*"]
    fn at(&self) -> f64;
}
impl FileSystemReadWriteOptionsGetters for FileSystemReadWriteOptions {
    fn at(&self) -> f64 {
        self.at_shim()
    }
}
impl FileSystemReadWriteOptions {
    #[doc = "Construct a new `FileSystemReadWriteOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemReadWriteOptions`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `at` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemReadWriteOptions`*"]
    pub fn at(&mut self, val: f64) -> &mut Self {
        self.set_at_shim(val);
        self
    }
}
impl Default for FileSystemReadWriteOptions {
    fn default() -> Self {
        Self::new()
    }
}
