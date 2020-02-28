use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = FileSystemEntry , typescript_name = FileSystemEntry ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `FileSystemEntry` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemEntry)\n\n*This API requires the following crate features to be activated: `FileSystemEntry`*"]
    pub type FileSystemEntry;
    # [ wasm_bindgen ( structural , method , getter , js_class = "FileSystemEntry" , js_name = isFile ) ]
    #[doc = "Getter for the `isFile` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemEntry/isFile)\n\n*This API requires the following crate features to be activated: `FileSystemEntry`*"]
    pub fn is_file(this: &FileSystemEntry) -> bool;
    # [ wasm_bindgen ( structural , method , getter , js_class = "FileSystemEntry" , js_name = isDirectory ) ]
    #[doc = "Getter for the `isDirectory` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemEntry/isDirectory)\n\n*This API requires the following crate features to be activated: `FileSystemEntry`*"]
    pub fn is_directory(this: &FileSystemEntry) -> bool;
    # [ wasm_bindgen ( structural , method , getter , js_class = "FileSystemEntry" , js_name = name ) ]
    #[doc = "Getter for the `name` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemEntry/name)\n\n*This API requires the following crate features to be activated: `FileSystemEntry`*"]
    pub fn name(this: &FileSystemEntry) -> String;
    # [ wasm_bindgen ( structural , method , getter , js_class = "FileSystemEntry" , js_name = fullPath ) ]
    #[doc = "Getter for the `fullPath` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemEntry/fullPath)\n\n*This API requires the following crate features to be activated: `FileSystemEntry`*"]
    pub fn full_path(this: &FileSystemEntry) -> String;
    # [ wasm_bindgen ( structural , method , getter , js_class = "FileSystemEntry" , js_name = filesystem ) ]
    #[cfg(feature = "FileSystem")]
    #[doc = "Getter for the `filesystem` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemEntry/filesystem)\n\n*This API requires the following crate features to be activated: `FileSystem`, `FileSystemEntry`*"]
    pub fn filesystem(this: &FileSystemEntry) -> FileSystem;
    # [ wasm_bindgen ( method , structural , js_class = "FileSystemEntry" , js_name = getParent ) ]
    #[doc = "The `getParent()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemEntry/getParent)\n\n*This API requires the following crate features to be activated: `FileSystemEntry`*"]
    pub fn get_parent(this: &FileSystemEntry);
    # [ wasm_bindgen ( method , structural , js_class = "FileSystemEntry" , js_name = getParent ) ]
    #[doc = "The `getParent()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemEntry/getParent)\n\n*This API requires the following crate features to be activated: `FileSystemEntry`*"]
    pub fn get_parent_with_callback(this: &FileSystemEntry, success_callback: &::js_sys::Function);
    #[cfg(feature = "FileSystemEntryCallback")]
    # [ wasm_bindgen ( method , structural , js_class = "FileSystemEntry" , js_name = getParent ) ]
    #[doc = "The `getParent()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemEntry/getParent)\n\n*This API requires the following crate features to be activated: `FileSystemEntry`, `FileSystemEntryCallback`*"]
    pub fn get_parent_with_file_system_entry_callback(
        this: &FileSystemEntry,
        success_callback: &FileSystemEntryCallback,
    );
    # [ wasm_bindgen ( method , structural , js_class = "FileSystemEntry" , js_name = getParent ) ]
    #[doc = "The `getParent()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemEntry/getParent)\n\n*This API requires the following crate features to be activated: `FileSystemEntry`*"]
    pub fn get_parent_with_callback_and_callback(
        this: &FileSystemEntry,
        success_callback: &::js_sys::Function,
        error_callback: &::js_sys::Function,
    );
    #[cfg(feature = "FileSystemEntryCallback")]
    # [ wasm_bindgen ( method , structural , js_class = "FileSystemEntry" , js_name = getParent ) ]
    #[doc = "The `getParent()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemEntry/getParent)\n\n*This API requires the following crate features to be activated: `FileSystemEntry`, `FileSystemEntryCallback`*"]
    pub fn get_parent_with_file_system_entry_callback_and_callback(
        this: &FileSystemEntry,
        success_callback: &FileSystemEntryCallback,
        error_callback: &::js_sys::Function,
    );
    #[cfg(feature = "ErrorCallback")]
    # [ wasm_bindgen ( method , structural , js_class = "FileSystemEntry" , js_name = getParent ) ]
    #[doc = "The `getParent()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemEntry/getParent)\n\n*This API requires the following crate features to be activated: `ErrorCallback`, `FileSystemEntry`*"]
    pub fn get_parent_with_callback_and_error_callback(
        this: &FileSystemEntry,
        success_callback: &::js_sys::Function,
        error_callback: &ErrorCallback,
    );
    #[cfg(all(feature = "ErrorCallback", feature = "FileSystemEntryCallback",))]
    # [ wasm_bindgen ( method , structural , js_class = "FileSystemEntry" , js_name = getParent ) ]
    #[doc = "The `getParent()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemEntry/getParent)\n\n*This API requires the following crate features to be activated: `ErrorCallback`, `FileSystemEntry`, `FileSystemEntryCallback`*"]
    pub fn get_parent_with_file_system_entry_callback_and_error_callback(
        this: &FileSystemEntry,
        success_callback: &FileSystemEntryCallback,
        error_callback: &ErrorCallback,
    );
}
