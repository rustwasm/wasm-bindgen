use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = DataTransferItem , typescript_name = DataTransferItem ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `DataTransferItem` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DataTransferItem)
    ///
    ///*This API requires the following crate features to be activated: `DataTransferItem`*
    pub type DataTransferItem;

    # [ wasm_bindgen ( structural , method , getter , js_class = "DataTransferItem" , js_name = kind ) ]
    ///Getter for the `kind` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DataTransferItem/kind)
    ///
    ///*This API requires the following crate features to be activated: `DataTransferItem`*
    pub fn kind(this: &DataTransferItem) -> String;

    # [ wasm_bindgen ( structural , method , getter , js_class = "DataTransferItem" , js_name = type ) ]
    ///Getter for the `type` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DataTransferItem/type)
    ///
    ///*This API requires the following crate features to be activated: `DataTransferItem`*
    pub fn type_(this: &DataTransferItem) -> String;

    #[cfg(feature = "File")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "DataTransferItem" , js_name = getAsFile ) ]
    ///The `getAsFile()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DataTransferItem/getAsFile)
    ///
    ///*This API requires the following crate features to be activated: `DataTransferItem`, `File`*
    pub fn get_as_file(this: &DataTransferItem) -> Result<Option<File>, JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "DataTransferItem" , js_name = getAsString ) ]
    ///The `getAsString()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DataTransferItem/getAsString)
    ///
    ///*This API requires the following crate features to be activated: `DataTransferItem`*
    pub fn get_as_string(
        this: &DataTransferItem,
        callback: Option<&::js_sys::Function>,
    ) -> Result<(), JsValue>;

    #[cfg(feature = "FileSystemEntry")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "DataTransferItem" , js_name = webkitGetAsEntry ) ]
    ///The `webkitGetAsEntry()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DataTransferItem/webkitGetAsEntry)
    ///
    ///*This API requires the following crate features to be activated: `DataTransferItem`, `FileSystemEntry`*
    pub fn webkit_get_as_entry(this: &DataTransferItem)
        -> Result<Option<FileSystemEntry>, JsValue>;

}
