#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = WritableStream , extends = :: js_sys :: Object , js_name = FileSystemWritableFileStream , typescript_type = "FileSystemWritableFileStream")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `FileSystemWritableFileStream` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemWritableFileStream)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemWritableFileStream`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type FileSystemWritableFileStream;
    #[cfg(web_sys_unstable_apis)]
    # [wasm_bindgen (catch , method , structural , js_class = "FileSystemWritableFileStream" , js_name = seek)]
    #[doc = "The `seek()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemWritableFileStream/seek)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemWritableFileStream`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn seek_with_u32(
        this: &FileSystemWritableFileStream,
        position: u32,
    ) -> Result<::js_sys::Promise, JsValue>;
    #[cfg(web_sys_unstable_apis)]
    # [wasm_bindgen (catch , method , structural , js_class = "FileSystemWritableFileStream" , js_name = seek)]
    #[doc = "The `seek()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemWritableFileStream/seek)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemWritableFileStream`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn seek_with_f64(
        this: &FileSystemWritableFileStream,
        position: f64,
    ) -> Result<::js_sys::Promise, JsValue>;
    #[cfg(web_sys_unstable_apis)]
    # [wasm_bindgen (catch , method , structural , js_class = "FileSystemWritableFileStream" , js_name = truncate)]
    #[doc = "The `truncate()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemWritableFileStream/truncate)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemWritableFileStream`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn truncate_with_u32(
        this: &FileSystemWritableFileStream,
        size: u32,
    ) -> Result<::js_sys::Promise, JsValue>;
    #[cfg(web_sys_unstable_apis)]
    # [wasm_bindgen (catch , method , structural , js_class = "FileSystemWritableFileStream" , js_name = truncate)]
    #[doc = "The `truncate()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemWritableFileStream/truncate)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemWritableFileStream`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn truncate_with_f64(
        this: &FileSystemWritableFileStream,
        size: f64,
    ) -> Result<::js_sys::Promise, JsValue>;
    #[cfg(web_sys_unstable_apis)]
    # [wasm_bindgen (catch , method , structural , js_class = "FileSystemWritableFileStream" , js_name = write)]
    #[doc = "The `write()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemWritableFileStream/write)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemWritableFileStream`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn write_with_buffer_source(
        this: &FileSystemWritableFileStream,
        data: &::js_sys::Object,
    ) -> Result<::js_sys::Promise, JsValue>;
    #[cfg(web_sys_unstable_apis)]
    # [wasm_bindgen (catch , method , structural , js_class = "FileSystemWritableFileStream" , js_name = write)]
    #[doc = "The `write()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemWritableFileStream/write)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemWritableFileStream`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn write_with_u8_array(
        this: &FileSystemWritableFileStream,
        data: &mut [u8],
    ) -> Result<::js_sys::Promise, JsValue>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "Blob")]
    # [wasm_bindgen (catch , method , structural , js_class = "FileSystemWritableFileStream" , js_name = write)]
    #[doc = "The `write()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemWritableFileStream/write)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Blob`, `FileSystemWritableFileStream`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn write_with_blob(
        this: &FileSystemWritableFileStream,
        data: &Blob,
    ) -> Result<::js_sys::Promise, JsValue>;
    #[cfg(web_sys_unstable_apis)]
    # [wasm_bindgen (catch , method , structural , js_class = "FileSystemWritableFileStream" , js_name = write)]
    #[doc = "The `write()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemWritableFileStream/write)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemWritableFileStream`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn write_with_str(
        this: &FileSystemWritableFileStream,
        data: &str,
    ) -> Result<::js_sys::Promise, JsValue>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "WriteParams")]
    # [wasm_bindgen (catch , method , structural , js_class = "FileSystemWritableFileStream" , js_name = write)]
    #[doc = "The `write()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemWritableFileStream/write)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemWritableFileStream`, `WriteParams`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn write_with_write_params(
        this: &FileSystemWritableFileStream,
        data: &WriteParams,
    ) -> Result<::js_sys::Promise, JsValue>;
}
