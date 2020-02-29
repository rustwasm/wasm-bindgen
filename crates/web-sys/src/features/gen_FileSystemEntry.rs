use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = FileSystemEntry , typescript_name = FileSystemEntry ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `FileSystemEntry` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemEntry)
    ///
    ///*This API requires the following crate features to be activated: `FileSystemEntry`*
    pub type FileSystemEntry;

    # [ wasm_bindgen ( structural , method , getter , js_class = "FileSystemEntry" , js_name = isFile ) ]
    ///Getter for the `isFile` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemEntry/isFile)
    ///
    ///*This API requires the following crate features to be activated: `FileSystemEntry`*
    pub fn is_file(this: &FileSystemEntry) -> bool;

    # [ wasm_bindgen ( structural , method , getter , js_class = "FileSystemEntry" , js_name = isDirectory ) ]
    ///Getter for the `isDirectory` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemEntry/isDirectory)
    ///
    ///*This API requires the following crate features to be activated: `FileSystemEntry`*
    pub fn is_directory(this: &FileSystemEntry) -> bool;

    # [ wasm_bindgen ( structural , method , getter , js_class = "FileSystemEntry" , js_name = name ) ]
    ///Getter for the `name` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemEntry/name)
    ///
    ///*This API requires the following crate features to be activated: `FileSystemEntry`*
    pub fn name(this: &FileSystemEntry) -> String;

    # [ wasm_bindgen ( structural , method , getter , js_class = "FileSystemEntry" , js_name = fullPath ) ]
    ///Getter for the `fullPath` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemEntry/fullPath)
    ///
    ///*This API requires the following crate features to be activated: `FileSystemEntry`*
    pub fn full_path(this: &FileSystemEntry) -> String;

    #[cfg(feature = "FileSystem")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "FileSystemEntry" , js_name = filesystem ) ]
    ///Getter for the `filesystem` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemEntry/filesystem)
    ///
    ///*This API requires the following crate features to be activated: `FileSystem`, `FileSystemEntry`*
    pub fn filesystem(this: &FileSystemEntry) -> FileSystem;

    # [ wasm_bindgen ( method , structural , js_class = "FileSystemEntry" , js_name = getParent ) ]
    ///The `getParent()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemEntry/getParent)
    ///
    ///*This API requires the following crate features to be activated: `FileSystemEntry`*
    pub fn get_parent(this: &FileSystemEntry);

    # [ wasm_bindgen ( method , structural , js_class = "FileSystemEntry" , js_name = getParent ) ]
    ///The `getParent()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemEntry/getParent)
    ///
    ///*This API requires the following crate features to be activated: `FileSystemEntry`*
    pub fn get_parent_with_callback(this: &FileSystemEntry, success_callback: &::js_sys::Function);

    #[cfg(feature = "FileSystemEntryCallback")]
    # [ wasm_bindgen ( method , structural , js_class = "FileSystemEntry" , js_name = getParent ) ]
    ///The `getParent()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemEntry/getParent)
    ///
    ///*This API requires the following crate features to be activated: `FileSystemEntry`, `FileSystemEntryCallback`*
    pub fn get_parent_with_file_system_entry_callback(
        this: &FileSystemEntry,
        success_callback: &FileSystemEntryCallback,
    );

    # [ wasm_bindgen ( method , structural , js_class = "FileSystemEntry" , js_name = getParent ) ]
    ///The `getParent()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemEntry/getParent)
    ///
    ///*This API requires the following crate features to be activated: `FileSystemEntry`*
    pub fn get_parent_with_callback_and_callback(
        this: &FileSystemEntry,
        success_callback: &::js_sys::Function,
        error_callback: &::js_sys::Function,
    );

    #[cfg(feature = "FileSystemEntryCallback")]
    # [ wasm_bindgen ( method , structural , js_class = "FileSystemEntry" , js_name = getParent ) ]
    ///The `getParent()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemEntry/getParent)
    ///
    ///*This API requires the following crate features to be activated: `FileSystemEntry`, `FileSystemEntryCallback`*
    pub fn get_parent_with_file_system_entry_callback_and_callback(
        this: &FileSystemEntry,
        success_callback: &FileSystemEntryCallback,
        error_callback: &::js_sys::Function,
    );

    #[cfg(feature = "ErrorCallback")]
    # [ wasm_bindgen ( method , structural , js_class = "FileSystemEntry" , js_name = getParent ) ]
    ///The `getParent()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemEntry/getParent)
    ///
    ///*This API requires the following crate features to be activated: `ErrorCallback`, `FileSystemEntry`*
    pub fn get_parent_with_callback_and_error_callback(
        this: &FileSystemEntry,
        success_callback: &::js_sys::Function,
        error_callback: &ErrorCallback,
    );

    #[cfg(all(feature = "ErrorCallback", feature = "FileSystemEntryCallback",))]
    # [ wasm_bindgen ( method , structural , js_class = "FileSystemEntry" , js_name = getParent ) ]
    ///The `getParent()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemEntry/getParent)
    ///
    ///*This API requires the following crate features to be activated: `ErrorCallback`, `FileSystemEntry`, `FileSystemEntryCallback`*
    pub fn get_parent_with_file_system_entry_callback_and_error_callback(
        this: &FileSystemEntry,
        success_callback: &FileSystemEntryCallback,
        error_callback: &ErrorCallback,
    );

}
