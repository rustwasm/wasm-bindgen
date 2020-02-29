use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = EventTarget , extends = :: js_sys :: Object , js_name = IDBFileHandle , typescript_type = "IDBFileHandle" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `IdbFileHandle` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBFileHandle)
    ///
    ///*This API requires the following crate features to be activated: `IdbFileHandle`*
    pub type IdbFileHandle;

    #[cfg(feature = "IdbMutableFile")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "IDBFileHandle" , js_name = mutableFile ) ]
    ///Getter for the `mutableFile` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBFileHandle/mutableFile)
    ///
    ///*This API requires the following crate features to be activated: `IdbFileHandle`, `IdbMutableFile`*
    pub fn mutable_file(this: &IdbFileHandle) -> Option<IdbMutableFile>;

    #[cfg(feature = "IdbMutableFile")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "IDBFileHandle" , js_name = fileHandle ) ]
    ///Getter for the `fileHandle` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBFileHandle/fileHandle)
    ///
    ///*This API requires the following crate features to be activated: `IdbFileHandle`, `IdbMutableFile`*
    pub fn file_handle(this: &IdbFileHandle) -> Option<IdbMutableFile>;

    # [ wasm_bindgen ( structural , method , getter , js_class = "IDBFileHandle" , js_name = active ) ]
    ///Getter for the `active` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBFileHandle/active)
    ///
    ///*This API requires the following crate features to be activated: `IdbFileHandle`*
    pub fn active(this: &IdbFileHandle) -> bool;

    # [ wasm_bindgen ( structural , method , getter , js_class = "IDBFileHandle" , js_name = location ) ]
    ///Getter for the `location` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBFileHandle/location)
    ///
    ///*This API requires the following crate features to be activated: `IdbFileHandle`*
    pub fn location(this: &IdbFileHandle) -> Option<f64>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "IDBFileHandle" , js_name = location ) ]
    ///Setter for the `location` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBFileHandle/location)
    ///
    ///*This API requires the following crate features to be activated: `IdbFileHandle`*
    pub fn set_location(this: &IdbFileHandle, value: Option<f64>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "IDBFileHandle" , js_name = oncomplete ) ]
    ///Getter for the `oncomplete` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBFileHandle/oncomplete)
    ///
    ///*This API requires the following crate features to be activated: `IdbFileHandle`*
    pub fn oncomplete(this: &IdbFileHandle) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "IDBFileHandle" , js_name = oncomplete ) ]
    ///Setter for the `oncomplete` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBFileHandle/oncomplete)
    ///
    ///*This API requires the following crate features to be activated: `IdbFileHandle`*
    pub fn set_oncomplete(this: &IdbFileHandle, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "IDBFileHandle" , js_name = onabort ) ]
    ///Getter for the `onabort` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBFileHandle/onabort)
    ///
    ///*This API requires the following crate features to be activated: `IdbFileHandle`*
    pub fn onabort(this: &IdbFileHandle) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "IDBFileHandle" , js_name = onabort ) ]
    ///Setter for the `onabort` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBFileHandle/onabort)
    ///
    ///*This API requires the following crate features to be activated: `IdbFileHandle`*
    pub fn set_onabort(this: &IdbFileHandle, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "IDBFileHandle" , js_name = onerror ) ]
    ///Getter for the `onerror` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBFileHandle/onerror)
    ///
    ///*This API requires the following crate features to be activated: `IdbFileHandle`*
    pub fn onerror(this: &IdbFileHandle) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "IDBFileHandle" , js_name = onerror ) ]
    ///Setter for the `onerror` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBFileHandle/onerror)
    ///
    ///*This API requires the following crate features to be activated: `IdbFileHandle`*
    pub fn set_onerror(this: &IdbFileHandle, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( catch , method , structural , js_class = "IDBFileHandle" , js_name = abort ) ]
    ///The `abort()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBFileHandle/abort)
    ///
    ///*This API requires the following crate features to be activated: `IdbFileHandle`*
    pub fn abort(this: &IdbFileHandle) -> Result<(), JsValue>;

    #[cfg(feature = "IdbFileRequest")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "IDBFileHandle" , js_name = append ) ]
    ///The `append()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBFileHandle/append)
    ///
    ///*This API requires the following crate features to be activated: `IdbFileHandle`, `IdbFileRequest`*
    pub fn append_with_str(
        this: &IdbFileHandle,
        value: &str,
    ) -> Result<Option<IdbFileRequest>, JsValue>;

    #[cfg(feature = "IdbFileRequest")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "IDBFileHandle" , js_name = append ) ]
    ///The `append()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBFileHandle/append)
    ///
    ///*This API requires the following crate features to be activated: `IdbFileHandle`, `IdbFileRequest`*
    pub fn append_with_array_buffer(
        this: &IdbFileHandle,
        value: &::js_sys::ArrayBuffer,
    ) -> Result<Option<IdbFileRequest>, JsValue>;

    #[cfg(feature = "IdbFileRequest")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "IDBFileHandle" , js_name = append ) ]
    ///The `append()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBFileHandle/append)
    ///
    ///*This API requires the following crate features to be activated: `IdbFileHandle`, `IdbFileRequest`*
    pub fn append_with_array_buffer_view(
        this: &IdbFileHandle,
        value: &::js_sys::Object,
    ) -> Result<Option<IdbFileRequest>, JsValue>;

    #[cfg(feature = "IdbFileRequest")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "IDBFileHandle" , js_name = append ) ]
    ///The `append()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBFileHandle/append)
    ///
    ///*This API requires the following crate features to be activated: `IdbFileHandle`, `IdbFileRequest`*
    pub fn append_with_u8_array(
        this: &IdbFileHandle,
        value: &mut [u8],
    ) -> Result<Option<IdbFileRequest>, JsValue>;

    #[cfg(all(feature = "Blob", feature = "IdbFileRequest",))]
    # [ wasm_bindgen ( catch , method , structural , js_class = "IDBFileHandle" , js_name = append ) ]
    ///The `append()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBFileHandle/append)
    ///
    ///*This API requires the following crate features to be activated: `Blob`, `IdbFileHandle`, `IdbFileRequest`*
    pub fn append_with_blob(
        this: &IdbFileHandle,
        value: &Blob,
    ) -> Result<Option<IdbFileRequest>, JsValue>;

    #[cfg(feature = "IdbFileRequest")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "IDBFileHandle" , js_name = flush ) ]
    ///The `flush()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBFileHandle/flush)
    ///
    ///*This API requires the following crate features to be activated: `IdbFileHandle`, `IdbFileRequest`*
    pub fn flush(this: &IdbFileHandle) -> Result<Option<IdbFileRequest>, JsValue>;

    #[cfg(feature = "IdbFileRequest")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "IDBFileHandle" , js_name = getMetadata ) ]
    ///The `getMetadata()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBFileHandle/getMetadata)
    ///
    ///*This API requires the following crate features to be activated: `IdbFileHandle`, `IdbFileRequest`*
    pub fn get_metadata(this: &IdbFileHandle) -> Result<Option<IdbFileRequest>, JsValue>;

    #[cfg(all(feature = "IdbFileMetadataParameters", feature = "IdbFileRequest",))]
    # [ wasm_bindgen ( catch , method , structural , js_class = "IDBFileHandle" , js_name = getMetadata ) ]
    ///The `getMetadata()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBFileHandle/getMetadata)
    ///
    ///*This API requires the following crate features to be activated: `IdbFileHandle`, `IdbFileMetadataParameters`, `IdbFileRequest`*
    pub fn get_metadata_with_parameters(
        this: &IdbFileHandle,
        parameters: &IdbFileMetadataParameters,
    ) -> Result<Option<IdbFileRequest>, JsValue>;

    #[cfg(feature = "IdbFileRequest")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "IDBFileHandle" , js_name = readAsArrayBuffer ) ]
    ///The `readAsArrayBuffer()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBFileHandle/readAsArrayBuffer)
    ///
    ///*This API requires the following crate features to be activated: `IdbFileHandle`, `IdbFileRequest`*
    pub fn read_as_array_buffer_with_u32(
        this: &IdbFileHandle,
        size: u32,
    ) -> Result<Option<IdbFileRequest>, JsValue>;

    #[cfg(feature = "IdbFileRequest")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "IDBFileHandle" , js_name = readAsArrayBuffer ) ]
    ///The `readAsArrayBuffer()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBFileHandle/readAsArrayBuffer)
    ///
    ///*This API requires the following crate features to be activated: `IdbFileHandle`, `IdbFileRequest`*
    pub fn read_as_array_buffer_with_f64(
        this: &IdbFileHandle,
        size: f64,
    ) -> Result<Option<IdbFileRequest>, JsValue>;

    #[cfg(feature = "IdbFileRequest")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "IDBFileHandle" , js_name = readAsText ) ]
    ///The `readAsText()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBFileHandle/readAsText)
    ///
    ///*This API requires the following crate features to be activated: `IdbFileHandle`, `IdbFileRequest`*
    pub fn read_as_text_with_u32(
        this: &IdbFileHandle,
        size: u32,
    ) -> Result<Option<IdbFileRequest>, JsValue>;

    #[cfg(feature = "IdbFileRequest")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "IDBFileHandle" , js_name = readAsText ) ]
    ///The `readAsText()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBFileHandle/readAsText)
    ///
    ///*This API requires the following crate features to be activated: `IdbFileHandle`, `IdbFileRequest`*
    pub fn read_as_text_with_f64(
        this: &IdbFileHandle,
        size: f64,
    ) -> Result<Option<IdbFileRequest>, JsValue>;

    #[cfg(feature = "IdbFileRequest")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "IDBFileHandle" , js_name = readAsText ) ]
    ///The `readAsText()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBFileHandle/readAsText)
    ///
    ///*This API requires the following crate features to be activated: `IdbFileHandle`, `IdbFileRequest`*
    pub fn read_as_text_with_u32_and_encoding(
        this: &IdbFileHandle,
        size: u32,
        encoding: Option<&str>,
    ) -> Result<Option<IdbFileRequest>, JsValue>;

    #[cfg(feature = "IdbFileRequest")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "IDBFileHandle" , js_name = readAsText ) ]
    ///The `readAsText()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBFileHandle/readAsText)
    ///
    ///*This API requires the following crate features to be activated: `IdbFileHandle`, `IdbFileRequest`*
    pub fn read_as_text_with_f64_and_encoding(
        this: &IdbFileHandle,
        size: f64,
        encoding: Option<&str>,
    ) -> Result<Option<IdbFileRequest>, JsValue>;

    #[cfg(feature = "IdbFileRequest")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "IDBFileHandle" , js_name = truncate ) ]
    ///The `truncate()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBFileHandle/truncate)
    ///
    ///*This API requires the following crate features to be activated: `IdbFileHandle`, `IdbFileRequest`*
    pub fn truncate(this: &IdbFileHandle) -> Result<Option<IdbFileRequest>, JsValue>;

    #[cfg(feature = "IdbFileRequest")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "IDBFileHandle" , js_name = truncate ) ]
    ///The `truncate()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBFileHandle/truncate)
    ///
    ///*This API requires the following crate features to be activated: `IdbFileHandle`, `IdbFileRequest`*
    pub fn truncate_with_u32(
        this: &IdbFileHandle,
        size: u32,
    ) -> Result<Option<IdbFileRequest>, JsValue>;

    #[cfg(feature = "IdbFileRequest")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "IDBFileHandle" , js_name = truncate ) ]
    ///The `truncate()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBFileHandle/truncate)
    ///
    ///*This API requires the following crate features to be activated: `IdbFileHandle`, `IdbFileRequest`*
    pub fn truncate_with_f64(
        this: &IdbFileHandle,
        size: f64,
    ) -> Result<Option<IdbFileRequest>, JsValue>;

    #[cfg(feature = "IdbFileRequest")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "IDBFileHandle" , js_name = write ) ]
    ///The `write()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBFileHandle/write)
    ///
    ///*This API requires the following crate features to be activated: `IdbFileHandle`, `IdbFileRequest`*
    pub fn write_with_str(
        this: &IdbFileHandle,
        value: &str,
    ) -> Result<Option<IdbFileRequest>, JsValue>;

    #[cfg(feature = "IdbFileRequest")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "IDBFileHandle" , js_name = write ) ]
    ///The `write()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBFileHandle/write)
    ///
    ///*This API requires the following crate features to be activated: `IdbFileHandle`, `IdbFileRequest`*
    pub fn write_with_array_buffer(
        this: &IdbFileHandle,
        value: &::js_sys::ArrayBuffer,
    ) -> Result<Option<IdbFileRequest>, JsValue>;

    #[cfg(feature = "IdbFileRequest")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "IDBFileHandle" , js_name = write ) ]
    ///The `write()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBFileHandle/write)
    ///
    ///*This API requires the following crate features to be activated: `IdbFileHandle`, `IdbFileRequest`*
    pub fn write_with_array_buffer_view(
        this: &IdbFileHandle,
        value: &::js_sys::Object,
    ) -> Result<Option<IdbFileRequest>, JsValue>;

    #[cfg(feature = "IdbFileRequest")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "IDBFileHandle" , js_name = write ) ]
    ///The `write()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBFileHandle/write)
    ///
    ///*This API requires the following crate features to be activated: `IdbFileHandle`, `IdbFileRequest`*
    pub fn write_with_u8_array(
        this: &IdbFileHandle,
        value: &mut [u8],
    ) -> Result<Option<IdbFileRequest>, JsValue>;

    #[cfg(all(feature = "Blob", feature = "IdbFileRequest",))]
    # [ wasm_bindgen ( catch , method , structural , js_class = "IDBFileHandle" , js_name = write ) ]
    ///The `write()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBFileHandle/write)
    ///
    ///*This API requires the following crate features to be activated: `Blob`, `IdbFileHandle`, `IdbFileRequest`*
    pub fn write_with_blob(
        this: &IdbFileHandle,
        value: &Blob,
    ) -> Result<Option<IdbFileRequest>, JsValue>;

}
