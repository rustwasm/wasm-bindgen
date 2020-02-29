use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = FileReaderSync , typescript_name = FileReaderSync ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `FileReaderSync` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileReaderSync)
    ///
    ///*This API requires the following crate features to be activated: `FileReaderSync`*
    pub type FileReaderSync;

    #[wasm_bindgen(catch, constructor, js_class = "FileReaderSync")]
    ///The `new FileReaderSync(..)` constructor, creating a new instance of `FileReaderSync`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileReaderSync/FileReaderSync)
    ///
    ///*This API requires the following crate features to be activated: `FileReaderSync`*
    pub fn new() -> Result<FileReaderSync, JsValue>;

    #[cfg(feature = "Blob")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "FileReaderSync" , js_name = readAsArrayBuffer ) ]
    ///The `readAsArrayBuffer()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileReaderSync/readAsArrayBuffer)
    ///
    ///*This API requires the following crate features to be activated: `Blob`, `FileReaderSync`*
    pub fn read_as_array_buffer(
        this: &FileReaderSync,
        blob: &Blob,
    ) -> Result<::js_sys::ArrayBuffer, JsValue>;

    #[cfg(feature = "Blob")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "FileReaderSync" , js_name = readAsBinaryString ) ]
    ///The `readAsBinaryString()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileReaderSync/readAsBinaryString)
    ///
    ///*This API requires the following crate features to be activated: `Blob`, `FileReaderSync`*
    pub fn read_as_binary_string(this: &FileReaderSync, blob: &Blob) -> Result<String, JsValue>;

    #[cfg(feature = "Blob")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "FileReaderSync" , js_name = readAsDataURL ) ]
    ///The `readAsDataURL()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileReaderSync/readAsDataURL)
    ///
    ///*This API requires the following crate features to be activated: `Blob`, `FileReaderSync`*
    pub fn read_as_data_url(this: &FileReaderSync, blob: &Blob) -> Result<String, JsValue>;

    #[cfg(feature = "Blob")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "FileReaderSync" , js_name = readAsText ) ]
    ///The `readAsText()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileReaderSync/readAsText)
    ///
    ///*This API requires the following crate features to be activated: `Blob`, `FileReaderSync`*
    pub fn read_as_text(this: &FileReaderSync, blob: &Blob) -> Result<String, JsValue>;

    #[cfg(feature = "Blob")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "FileReaderSync" , js_name = readAsText ) ]
    ///The `readAsText()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileReaderSync/readAsText)
    ///
    ///*This API requires the following crate features to be activated: `Blob`, `FileReaderSync`*
    pub fn read_as_text_with_encoding(
        this: &FileReaderSync,
        blob: &Blob,
        encoding: &str,
    ) -> Result<String, JsValue>;

}
