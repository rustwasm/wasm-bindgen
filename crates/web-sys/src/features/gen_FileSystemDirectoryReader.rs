#![allow(unused_imports)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = FileSystemDirectoryReader , typescript_type = "FileSystemDirectoryReader")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `FileSystemDirectoryReader` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemDirectoryReader)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemDirectoryReader`*"]
    pub type FileSystemDirectoryReader;
    # [wasm_bindgen (catch , method , structural , js_class = "FileSystemDirectoryReader" , js_name = readEntries)]
    #[doc = "The `readEntries()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemDirectoryReader/readEntries)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemDirectoryReader`*"]
    #[doc = ""]
    #[doc = "Argument `success_callback`: See the referenced MDN documentation or the IDL files for the signature of the callback."]
    pub fn read_entries_with_callback(
        this: &FileSystemDirectoryReader,
        success_callback: &::js_sys::Function,
    ) -> Result<(), JsValue>;
    #[cfg(feature = "FileSystemEntriesCallback")]
    # [wasm_bindgen (catch , method , structural , js_class = "FileSystemDirectoryReader" , js_name = readEntries)]
    #[doc = "The `readEntries()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemDirectoryReader/readEntries)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemDirectoryReader`, `FileSystemEntriesCallback`*"]
    pub fn read_entries_with_file_system_entries_callback(
        this: &FileSystemDirectoryReader,
        success_callback: &FileSystemEntriesCallback,
    ) -> Result<(), JsValue>;
    # [wasm_bindgen (catch , method , structural , js_class = "FileSystemDirectoryReader" , js_name = readEntries)]
    #[doc = "The `readEntries()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemDirectoryReader/readEntries)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemDirectoryReader`*"]
    #[doc = ""]
    #[doc = "Argument `success_callback`: See the referenced MDN documentation or the IDL files for the signature of the callback."]
    #[doc = ""]
    #[doc = "Argument `error_callback`: See the referenced MDN documentation or the IDL files for the signature of the callback."]
    pub fn read_entries_with_callback_and_callback(
        this: &FileSystemDirectoryReader,
        success_callback: &::js_sys::Function,
        error_callback: &::js_sys::Function,
    ) -> Result<(), JsValue>;
    #[cfg(feature = "FileSystemEntriesCallback")]
    # [wasm_bindgen (catch , method , structural , js_class = "FileSystemDirectoryReader" , js_name = readEntries)]
    #[doc = "The `readEntries()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemDirectoryReader/readEntries)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemDirectoryReader`, `FileSystemEntriesCallback`*"]
    #[doc = ""]
    #[doc = "Argument `error_callback`: See the referenced MDN documentation or the IDL files for the signature of the callback."]
    pub fn read_entries_with_file_system_entries_callback_and_callback(
        this: &FileSystemDirectoryReader,
        success_callback: &FileSystemEntriesCallback,
        error_callback: &::js_sys::Function,
    ) -> Result<(), JsValue>;
    #[cfg(feature = "ErrorCallback")]
    # [wasm_bindgen (catch , method , structural , js_class = "FileSystemDirectoryReader" , js_name = readEntries)]
    #[doc = "The `readEntries()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemDirectoryReader/readEntries)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ErrorCallback`, `FileSystemDirectoryReader`*"]
    #[doc = ""]
    #[doc = "Argument `success_callback`: See the referenced MDN documentation or the IDL files for the signature of the callback."]
    pub fn read_entries_with_callback_and_error_callback(
        this: &FileSystemDirectoryReader,
        success_callback: &::js_sys::Function,
        error_callback: &ErrorCallback,
    ) -> Result<(), JsValue>;
    #[cfg(all(feature = "ErrorCallback", feature = "FileSystemEntriesCallback",))]
    # [wasm_bindgen (catch , method , structural , js_class = "FileSystemDirectoryReader" , js_name = readEntries)]
    #[doc = "The `readEntries()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemDirectoryReader/readEntries)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ErrorCallback`, `FileSystemDirectoryReader`, `FileSystemEntriesCallback`*"]
    pub fn read_entries_with_file_system_entries_callback_and_error_callback(
        this: &FileSystemDirectoryReader,
        success_callback: &FileSystemEntriesCallback,
        error_callback: &ErrorCallback,
    ) -> Result<(), JsValue>;
}
