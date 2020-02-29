use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = EventTarget , extends = :: js_sys :: Object , js_name = FileReader , typescript_type = "FileReader" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `FileReader` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileReader)
    ///
    ///*This API requires the following crate features to be activated: `FileReader`*
    pub type FileReader;

    # [ wasm_bindgen ( structural , method , getter , js_class = "FileReader" , js_name = readyState ) ]
    ///Getter for the `readyState` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileReader/readyState)
    ///
    ///*This API requires the following crate features to be activated: `FileReader`*
    pub fn ready_state(this: &FileReader) -> u16;

    # [ wasm_bindgen ( structural , catch , method , getter , js_class = "FileReader" , js_name = result ) ]
    ///Getter for the `result` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileReader/result)
    ///
    ///*This API requires the following crate features to be activated: `FileReader`*
    pub fn result(this: &FileReader) -> Result<::wasm_bindgen::JsValue, JsValue>;

    #[cfg(feature = "DomException")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "FileReader" , js_name = error ) ]
    ///Getter for the `error` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileReader/error)
    ///
    ///*This API requires the following crate features to be activated: `DomException`, `FileReader`*
    pub fn error(this: &FileReader) -> Option<DomException>;

    # [ wasm_bindgen ( structural , method , getter , js_class = "FileReader" , js_name = onloadstart ) ]
    ///Getter for the `onloadstart` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileReader/onloadstart)
    ///
    ///*This API requires the following crate features to be activated: `FileReader`*
    pub fn onloadstart(this: &FileReader) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "FileReader" , js_name = onloadstart ) ]
    ///Setter for the `onloadstart` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileReader/onloadstart)
    ///
    ///*This API requires the following crate features to be activated: `FileReader`*
    pub fn set_onloadstart(this: &FileReader, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "FileReader" , js_name = onprogress ) ]
    ///Getter for the `onprogress` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileReader/onprogress)
    ///
    ///*This API requires the following crate features to be activated: `FileReader`*
    pub fn onprogress(this: &FileReader) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "FileReader" , js_name = onprogress ) ]
    ///Setter for the `onprogress` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileReader/onprogress)
    ///
    ///*This API requires the following crate features to be activated: `FileReader`*
    pub fn set_onprogress(this: &FileReader, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "FileReader" , js_name = onload ) ]
    ///Getter for the `onload` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileReader/onload)
    ///
    ///*This API requires the following crate features to be activated: `FileReader`*
    pub fn onload(this: &FileReader) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "FileReader" , js_name = onload ) ]
    ///Setter for the `onload` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileReader/onload)
    ///
    ///*This API requires the following crate features to be activated: `FileReader`*
    pub fn set_onload(this: &FileReader, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "FileReader" , js_name = onabort ) ]
    ///Getter for the `onabort` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileReader/onabort)
    ///
    ///*This API requires the following crate features to be activated: `FileReader`*
    pub fn onabort(this: &FileReader) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "FileReader" , js_name = onabort ) ]
    ///Setter for the `onabort` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileReader/onabort)
    ///
    ///*This API requires the following crate features to be activated: `FileReader`*
    pub fn set_onabort(this: &FileReader, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "FileReader" , js_name = onerror ) ]
    ///Getter for the `onerror` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileReader/onerror)
    ///
    ///*This API requires the following crate features to be activated: `FileReader`*
    pub fn onerror(this: &FileReader) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "FileReader" , js_name = onerror ) ]
    ///Setter for the `onerror` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileReader/onerror)
    ///
    ///*This API requires the following crate features to be activated: `FileReader`*
    pub fn set_onerror(this: &FileReader, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "FileReader" , js_name = onloadend ) ]
    ///Getter for the `onloadend` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileReader/onloadend)
    ///
    ///*This API requires the following crate features to be activated: `FileReader`*
    pub fn onloadend(this: &FileReader) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "FileReader" , js_name = onloadend ) ]
    ///Setter for the `onloadend` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileReader/onloadend)
    ///
    ///*This API requires the following crate features to be activated: `FileReader`*
    pub fn set_onloadend(this: &FileReader, value: Option<&::js_sys::Function>);

    #[wasm_bindgen(catch, constructor, js_class = "FileReader")]
    ///The `new FileReader(..)` constructor, creating a new instance of `FileReader`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileReader/FileReader)
    ///
    ///*This API requires the following crate features to be activated: `FileReader`*
    pub fn new() -> Result<FileReader, JsValue>;

    # [ wasm_bindgen ( method , structural , js_class = "FileReader" , js_name = abort ) ]
    ///The `abort()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileReader/abort)
    ///
    ///*This API requires the following crate features to be activated: `FileReader`*
    pub fn abort(this: &FileReader);

    #[cfg(feature = "Blob")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "FileReader" , js_name = readAsArrayBuffer ) ]
    ///The `readAsArrayBuffer()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileReader/readAsArrayBuffer)
    ///
    ///*This API requires the following crate features to be activated: `Blob`, `FileReader`*
    pub fn read_as_array_buffer(this: &FileReader, blob: &Blob) -> Result<(), JsValue>;

    #[cfg(feature = "Blob")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "FileReader" , js_name = readAsBinaryString ) ]
    ///The `readAsBinaryString()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileReader/readAsBinaryString)
    ///
    ///*This API requires the following crate features to be activated: `Blob`, `FileReader`*
    pub fn read_as_binary_string(this: &FileReader, filedata: &Blob) -> Result<(), JsValue>;

    #[cfg(feature = "Blob")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "FileReader" , js_name = readAsDataURL ) ]
    ///The `readAsDataURL()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileReader/readAsDataURL)
    ///
    ///*This API requires the following crate features to be activated: `Blob`, `FileReader`*
    pub fn read_as_data_url(this: &FileReader, blob: &Blob) -> Result<(), JsValue>;

    #[cfg(feature = "Blob")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "FileReader" , js_name = readAsText ) ]
    ///The `readAsText()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileReader/readAsText)
    ///
    ///*This API requires the following crate features to be activated: `Blob`, `FileReader`*
    pub fn read_as_text(this: &FileReader, blob: &Blob) -> Result<(), JsValue>;

    #[cfg(feature = "Blob")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "FileReader" , js_name = readAsText ) ]
    ///The `readAsText()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileReader/readAsText)
    ///
    ///*This API requires the following crate features to be activated: `Blob`, `FileReader`*
    pub fn read_as_text_with_label(
        this: &FileReader,
        blob: &Blob,
        label: &str,
    ) -> Result<(), JsValue>;

}

impl FileReader {
    ///The `FileReader.EMPTY` const.
    ///
    ///*This API requires the following crate features to be activated: `FileReader`*

    pub const EMPTY: u16 = 0i64 as u16;

    ///The `FileReader.LOADING` const.
    ///
    ///*This API requires the following crate features to be activated: `FileReader`*

    pub const LOADING: u16 = 1u64 as u16;

    ///The `FileReader.DONE` const.
    ///
    ///*This API requires the following crate features to be activated: `FileReader`*

    pub const DONE: u16 = 2u64 as u16;
}
