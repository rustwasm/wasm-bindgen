#![allow(unused_imports)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = FileSystemEntry , extends = :: js_sys :: Object , js_name = FileSystemFileEntry , typescript_type = "FileSystemFileEntry")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `FileSystemFileEntry` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemFileEntry)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemFileEntry`*"]
    pub type FileSystemFileEntry;
    # [wasm_bindgen (method , structural , js_class = "FileSystemFileEntry" , js_name = file)]
    #[doc = "The `file()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemFileEntry/file)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemFileEntry`*"]
    #[doc = ""]
    #[doc = "Argument `success_callback`: See the referenced MDN documentation or the IDL files for the signature of the callback."]
    pub fn file_with_callback(this: &FileSystemFileEntry, success_callback: &::js_sys::Function);
    #[cfg(feature = "FileCallback")]
    # [wasm_bindgen (method , structural , js_class = "FileSystemFileEntry" , js_name = file)]
    #[doc = "The `file()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemFileEntry/file)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileCallback`, `FileSystemFileEntry`*"]
    pub fn file_with_file_callback(this: &FileSystemFileEntry, success_callback: &FileCallback);
    # [wasm_bindgen (method , structural , js_class = "FileSystemFileEntry" , js_name = file)]
    #[doc = "The `file()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemFileEntry/file)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemFileEntry`*"]
    #[doc = ""]
    #[doc = "Argument `success_callback`: See the referenced MDN documentation or the IDL files for the signature of the callback."]
    #[doc = ""]
    #[doc = "Argument `error_callback`: See the referenced MDN documentation or the IDL files for the signature of the callback."]
    pub fn file_with_callback_and_callback(
        this: &FileSystemFileEntry,
        success_callback: &::js_sys::Function,
        error_callback: &::js_sys::Function,
    );
    #[cfg(feature = "FileCallback")]
    # [wasm_bindgen (method , structural , js_class = "FileSystemFileEntry" , js_name = file)]
    #[doc = "The `file()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemFileEntry/file)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileCallback`, `FileSystemFileEntry`*"]
    #[doc = ""]
    #[doc = "Argument `error_callback`: See the referenced MDN documentation or the IDL files for the signature of the callback."]
    pub fn file_with_file_callback_and_callback(
        this: &FileSystemFileEntry,
        success_callback: &FileCallback,
        error_callback: &::js_sys::Function,
    );
    #[cfg(feature = "ErrorCallback")]
    # [wasm_bindgen (method , structural , js_class = "FileSystemFileEntry" , js_name = file)]
    #[doc = "The `file()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemFileEntry/file)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ErrorCallback`, `FileSystemFileEntry`*"]
    #[doc = ""]
    #[doc = "Argument `success_callback`: See the referenced MDN documentation or the IDL files for the signature of the callback."]
    pub fn file_with_callback_and_error_callback(
        this: &FileSystemFileEntry,
        success_callback: &::js_sys::Function,
        error_callback: &ErrorCallback,
    );
    #[cfg(all(feature = "ErrorCallback", feature = "FileCallback",))]
    # [wasm_bindgen (method , structural , js_class = "FileSystemFileEntry" , js_name = file)]
    #[doc = "The `file()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemFileEntry/file)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ErrorCallback`, `FileCallback`, `FileSystemFileEntry`*"]
    pub fn file_with_file_callback_and_error_callback(
        this: &FileSystemFileEntry,
        success_callback: &FileCallback,
        error_callback: &ErrorCallback,
    );
}
