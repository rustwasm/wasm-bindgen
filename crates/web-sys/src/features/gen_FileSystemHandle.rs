#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = FileSystemHandle , typescript_type = "FileSystemHandle")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `FileSystemHandle` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemHandle)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemHandle`*"]
    pub type FileSystemHandle;
    #[cfg(feature = "FileSystemHandleKind")]
    # [wasm_bindgen (structural , method , getter , js_class = "FileSystemHandle" , js_name = kind)]
    #[doc = "Getter for the `kind` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemHandle/kind)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemHandle`, `FileSystemHandleKind`*"]
    pub fn kind(this: &FileSystemHandle) -> FileSystemHandleKind;
    # [wasm_bindgen (structural , method , getter , js_class = "FileSystemHandle" , js_name = name)]
    #[doc = "Getter for the `name` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemHandle/name)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemHandle`*"]
    pub fn name(this: &FileSystemHandle) -> String;
    # [wasm_bindgen (method , structural , js_class = "FileSystemHandle" , js_name = isSameEntry)]
    #[doc = "The `isSameEntry()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemHandle/isSameEntry)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemHandle`*"]
    pub fn is_same_entry(this: &FileSystemHandle, other: &FileSystemHandle) -> ::js_sys::Promise;
}
