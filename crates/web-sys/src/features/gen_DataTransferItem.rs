use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = DataTransferItem , typescript_name = DataTransferItem ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `DataTransferItem` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DataTransferItem)\n\n*This API requires the following crate features to be activated: `DataTransferItem`*"]
    pub type DataTransferItem;
    # [ wasm_bindgen ( structural , method , getter , js_name = kind ) ]
    #[doc = "Getter for the `kind` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DataTransferItem/kind)\n\n*This API requires the following crate features to be activated: `DataTransferItem`*"]
    pub fn kind(this: &DataTransferItem) -> String;
    # [ wasm_bindgen ( structural , method , getter , js_name = type ) ]
    #[doc = "Getter for the `type` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DataTransferItem/type)\n\n*This API requires the following crate features to be activated: `DataTransferItem`*"]
    pub fn type_(this: &DataTransferItem) -> String;
    #[cfg(feature = "File")]
    # [ wasm_bindgen ( catch , method , structural , js_name = getAsFile ) ]
    #[doc = "The `getAsFile()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DataTransferItem/getAsFile)\n\n*This API requires the following crate features to be activated: `DataTransferItem`, `File`*"]
    pub fn get_as_file(this: &DataTransferItem) -> Result<Option<File>, JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = getAsString ) ]
    #[doc = "The `getAsString()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DataTransferItem/getAsString)\n\n*This API requires the following crate features to be activated: `DataTransferItem`*"]
    pub fn get_as_string(
        this: &DataTransferItem,
        callback: Option<&::js_sys::Function>,
    ) -> Result<(), JsValue>;
    #[cfg(feature = "FileSystemEntry")]
    # [ wasm_bindgen ( catch , method , structural , js_name = webkitGetAsEntry ) ]
    #[doc = "The `webkitGetAsEntry()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DataTransferItem/webkitGetAsEntry)\n\n*This API requires the following crate features to be activated: `DataTransferItem`, `FileSystemEntry`*"]
    pub fn webkit_get_as_entry(this: &DataTransferItem)
        -> Result<Option<FileSystemEntry>, JsValue>;
}
