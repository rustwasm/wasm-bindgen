use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = FileReaderSync , typescript_name = FileReaderSync ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `FileReaderSync` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileReaderSync)\n\n*This API requires the following crate features to be activated: `FileReaderSync`*"]
    pub type FileReaderSync;
    #[wasm_bindgen(catch, js_class = "FileReaderSync", constructor)]
    #[doc = "The `new FileReaderSync(..)` constructor, creating a new instance of `FileReaderSync`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileReaderSync/FileReaderSync)\n\n*This API requires the following crate features to be activated: `FileReaderSync`*"]
    pub fn new(this: &FileReaderSync) -> Result<FileReaderSync, JsValue>;
    #[cfg(feature = "Blob")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "FileReaderSync" , js_name = readAsArrayBuffer ) ]
    #[doc = "The `readAsArrayBuffer()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileReaderSync/readAsArrayBuffer)\n\n*This API requires the following crate features to be activated: `Blob`, `FileReaderSync`*"]
    pub fn read_as_array_buffer(
        this: &FileReaderSync,
        blob: &Blob,
    ) -> Result<::js_sys::ArrayBuffer, JsValue>;
    #[cfg(feature = "Blob")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "FileReaderSync" , js_name = readAsBinaryString ) ]
    #[doc = "The `readAsBinaryString()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileReaderSync/readAsBinaryString)\n\n*This API requires the following crate features to be activated: `Blob`, `FileReaderSync`*"]
    pub fn read_as_binary_string(this: &FileReaderSync, blob: &Blob) -> Result<String, JsValue>;
    #[cfg(feature = "Blob")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "FileReaderSync" , js_name = readAsDataURL ) ]
    #[doc = "The `readAsDataURL()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileReaderSync/readAsDataURL)\n\n*This API requires the following crate features to be activated: `Blob`, `FileReaderSync`*"]
    pub fn read_as_data_url(this: &FileReaderSync, blob: &Blob) -> Result<String, JsValue>;
    #[cfg(feature = "Blob")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "FileReaderSync" , js_name = readAsText ) ]
    #[doc = "The `readAsText()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileReaderSync/readAsText)\n\n*This API requires the following crate features to be activated: `Blob`, `FileReaderSync`*"]
    pub fn read_as_text(this: &FileReaderSync, blob: &Blob) -> Result<String, JsValue>;
    #[cfg(feature = "Blob")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "FileReaderSync" , js_name = readAsText ) ]
    #[doc = "The `readAsText()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileReaderSync/readAsText)\n\n*This API requires the following crate features to be activated: `Blob`, `FileReaderSync`*"]
    pub fn read_as_text_with_encoding(
        this: &FileReaderSync,
        blob: &Blob,
        encoding: &str,
    ) -> Result<String, JsValue>;
}
