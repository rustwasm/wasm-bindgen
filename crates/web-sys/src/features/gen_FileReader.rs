use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = EventTarget , extends = :: js_sys :: Object , js_name = FileReader , typescript_name = FileReader ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `FileReader` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileReader)\n\n*This API requires the following crate features to be activated: `FileReader`*"]
    pub type FileReader;
    # [ wasm_bindgen ( structural , method , getter , js_name = readyState ) ]
    #[doc = "Getter for the `readyState` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileReader/readyState)\n\n*This API requires the following crate features to be activated: `FileReader`*"]
    pub fn ready_state(this: &FileReader) -> u16;
    # [ wasm_bindgen ( structural , catch , method , getter , js_name = result ) ]
    #[doc = "Getter for the `result` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileReader/result)\n\n*This API requires the following crate features to be activated: `FileReader`*"]
    pub fn result(this: &FileReader) -> Result<::wasm_bindgen::JsValue, JsValue>;
    # [ wasm_bindgen ( structural , method , getter , js_name = error ) ]
    #[cfg(feature = "DomException")]
    #[doc = "Getter for the `error` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileReader/error)\n\n*This API requires the following crate features to be activated: `DomException`, `FileReader`*"]
    pub fn error(this: &FileReader) -> Option<DomException>;
    # [ wasm_bindgen ( structural , method , getter , js_name = onloadstart ) ]
    #[doc = "Getter for the `onloadstart` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileReader/onloadstart)\n\n*This API requires the following crate features to be activated: `FileReader`*"]
    pub fn onloadstart(this: &FileReader) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onloadstart ) ]
    #[doc = "Setter for the `onloadstart` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileReader/onloadstart)\n\n*This API requires the following crate features to be activated: `FileReader`*"]
    pub fn set_onloadstart(this: &FileReader, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = onprogress ) ]
    #[doc = "Getter for the `onprogress` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileReader/onprogress)\n\n*This API requires the following crate features to be activated: `FileReader`*"]
    pub fn onprogress(this: &FileReader) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onprogress ) ]
    #[doc = "Setter for the `onprogress` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileReader/onprogress)\n\n*This API requires the following crate features to be activated: `FileReader`*"]
    pub fn set_onprogress(this: &FileReader, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = onload ) ]
    #[doc = "Getter for the `onload` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileReader/onload)\n\n*This API requires the following crate features to be activated: `FileReader`*"]
    pub fn onload(this: &FileReader) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onload ) ]
    #[doc = "Setter for the `onload` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileReader/onload)\n\n*This API requires the following crate features to be activated: `FileReader`*"]
    pub fn set_onload(this: &FileReader, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = onabort ) ]
    #[doc = "Getter for the `onabort` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileReader/onabort)\n\n*This API requires the following crate features to be activated: `FileReader`*"]
    pub fn onabort(this: &FileReader) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onabort ) ]
    #[doc = "Setter for the `onabort` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileReader/onabort)\n\n*This API requires the following crate features to be activated: `FileReader`*"]
    pub fn set_onabort(this: &FileReader, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = onerror ) ]
    #[doc = "Getter for the `onerror` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileReader/onerror)\n\n*This API requires the following crate features to be activated: `FileReader`*"]
    pub fn onerror(this: &FileReader) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onerror ) ]
    #[doc = "Setter for the `onerror` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileReader/onerror)\n\n*This API requires the following crate features to be activated: `FileReader`*"]
    pub fn set_onerror(this: &FileReader, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = onloadend ) ]
    #[doc = "Getter for the `onloadend` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileReader/onloadend)\n\n*This API requires the following crate features to be activated: `FileReader`*"]
    pub fn onloadend(this: &FileReader) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onloadend ) ]
    #[doc = "Setter for the `onloadend` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileReader/onloadend)\n\n*This API requires the following crate features to be activated: `FileReader`*"]
    pub fn set_onloadend(this: &FileReader, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(catch, constructor)]
    #[doc = "The `new FileReader(..)` constructor, creating a new instance of `FileReader`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileReader/FileReader)\n\n*This API requires the following crate features to be activated: `FileReader`*"]
    pub fn new(this: &FileReader) -> Result<FileReader, JsValue>;
    # [ wasm_bindgen ( method , structural , js_name = abort ) ]
    #[doc = "The `abort()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileReader/abort)\n\n*This API requires the following crate features to be activated: `FileReader`*"]
    pub fn abort(this: &FileReader);
    #[cfg(feature = "Blob")]
    # [ wasm_bindgen ( catch , method , structural , js_name = readAsArrayBuffer ) ]
    #[doc = "The `readAsArrayBuffer()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileReader/readAsArrayBuffer)\n\n*This API requires the following crate features to be activated: `Blob`, `FileReader`*"]
    pub fn read_as_array_buffer(this: &FileReader, blob: &Blob) -> Result<(), JsValue>;
    #[cfg(feature = "Blob")]
    # [ wasm_bindgen ( catch , method , structural , js_name = readAsBinaryString ) ]
    #[doc = "The `readAsBinaryString()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileReader/readAsBinaryString)\n\n*This API requires the following crate features to be activated: `Blob`, `FileReader`*"]
    pub fn read_as_binary_string(this: &FileReader, filedata: &Blob) -> Result<(), JsValue>;
    #[cfg(feature = "Blob")]
    # [ wasm_bindgen ( catch , method , structural , js_name = readAsDataURL ) ]
    #[doc = "The `readAsDataURL()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileReader/readAsDataURL)\n\n*This API requires the following crate features to be activated: `Blob`, `FileReader`*"]
    pub fn read_as_data_url(this: &FileReader, blob: &Blob) -> Result<(), JsValue>;
    #[cfg(feature = "Blob")]
    # [ wasm_bindgen ( catch , method , structural , js_name = readAsText ) ]
    #[doc = "The `readAsText()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileReader/readAsText)\n\n*This API requires the following crate features to be activated: `Blob`, `FileReader`*"]
    pub fn read_as_text(this: &FileReader, blob: &Blob) -> Result<(), JsValue>;
    #[cfg(feature = "Blob")]
    # [ wasm_bindgen ( catch , method , structural , js_name = readAsText ) ]
    #[doc = "The `readAsText()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileReader/readAsText)\n\n*This API requires the following crate features to be activated: `Blob`, `FileReader`*"]
    pub fn read_as_text_with_label(
        this: &FileReader,
        blob: &Blob,
        label: &str,
    ) -> Result<(), JsValue>;
}
impl FileReader {
    pub const EMPTY: u16 = 0i64 as u16;
    pub const LOADING: u16 = 1u64 as u16;
    pub const DONE: u16 = 2u64 as u16;
}
