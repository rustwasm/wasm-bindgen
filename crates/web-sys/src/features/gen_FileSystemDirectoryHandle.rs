#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = FileSystemHandle , extends = :: js_sys :: Object , js_name = FileSystemDirectoryHandle , typescript_type = "FileSystemDirectoryHandle")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `FileSystemDirectoryHandle` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemDirectoryHandle)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemDirectoryHandle`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type FileSystemDirectoryHandle;
    #[cfg(web_sys_unstable_apis)]
    # [wasm_bindgen (method , structural , js_class = "FileSystemDirectoryHandle" , js_name = getDirectoryHandle)]
    #[doc = "The `getDirectoryHandle()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemDirectoryHandle/getDirectoryHandle)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemDirectoryHandle`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn get_directory_handle(this: &FileSystemDirectoryHandle, name: &str) -> ::js_sys::Promise;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "FileSystemGetDirectoryOptions")]
    # [wasm_bindgen (method , structural , js_class = "FileSystemDirectoryHandle" , js_name = getDirectoryHandle)]
    #[doc = "The `getDirectoryHandle()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemDirectoryHandle/getDirectoryHandle)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemDirectoryHandle`, `FileSystemGetDirectoryOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn get_directory_handle_with_options(
        this: &FileSystemDirectoryHandle,
        name: &str,
        options: &FileSystemGetDirectoryOptions,
    ) -> ::js_sys::Promise;
    #[cfg(web_sys_unstable_apis)]
    # [wasm_bindgen (method , structural , js_class = "FileSystemDirectoryHandle" , js_name = getFileHandle)]
    #[doc = "The `getFileHandle()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemDirectoryHandle/getFileHandle)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemDirectoryHandle`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn get_file_handle(this: &FileSystemDirectoryHandle, name: &str) -> ::js_sys::Promise;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "FileSystemGetFileOptions")]
    # [wasm_bindgen (method , structural , js_class = "FileSystemDirectoryHandle" , js_name = getFileHandle)]
    #[doc = "The `getFileHandle()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemDirectoryHandle/getFileHandle)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemDirectoryHandle`, `FileSystemGetFileOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn get_file_handle_with_options(
        this: &FileSystemDirectoryHandle,
        name: &str,
        options: &FileSystemGetFileOptions,
    ) -> ::js_sys::Promise;
    #[cfg(web_sys_unstable_apis)]
    # [wasm_bindgen (method , structural , js_class = "FileSystemDirectoryHandle" , js_name = removeEntry)]
    #[doc = "The `removeEntry()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemDirectoryHandle/removeEntry)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemDirectoryHandle`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn remove_entry(this: &FileSystemDirectoryHandle, name: &str) -> ::js_sys::Promise;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "FileSystemRemoveOptions")]
    # [wasm_bindgen (method , structural , js_class = "FileSystemDirectoryHandle" , js_name = removeEntry)]
    #[doc = "The `removeEntry()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemDirectoryHandle/removeEntry)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemDirectoryHandle`, `FileSystemRemoveOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn remove_entry_with_options(
        this: &FileSystemDirectoryHandle,
        name: &str,
        options: &FileSystemRemoveOptions,
    ) -> ::js_sys::Promise;
    #[cfg(web_sys_unstable_apis)]
    # [wasm_bindgen (method , structural , js_class = "FileSystemDirectoryHandle" , js_name = resolve)]
    #[doc = "The `resolve()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemDirectoryHandle/resolve)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemDirectoryHandle`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn resolve(
        this: &FileSystemDirectoryHandle,
        possible_descendant: &FileSystemHandle,
    ) -> ::js_sys::Promise;
}
