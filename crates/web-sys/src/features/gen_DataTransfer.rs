use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = DataTransfer , typescript_name = DataTransfer ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `DataTransfer` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DataTransfer)\n\n*This API requires the following crate features to be activated: `DataTransfer`*"]
    pub type DataTransfer;
    # [ wasm_bindgen ( structural , method , getter , js_class = "DataTransfer" , js_name = dropEffect ) ]
    #[doc = "Getter for the `dropEffect` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DataTransfer/dropEffect)\n\n*This API requires the following crate features to be activated: `DataTransfer`*"]
    pub fn drop_effect(this: &DataTransfer) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_class = "DataTransfer" , js_name = dropEffect ) ]
    #[doc = "Setter for the `dropEffect` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DataTransfer/dropEffect)\n\n*This API requires the following crate features to be activated: `DataTransfer`*"]
    pub fn set_drop_effect(this: &DataTransfer, value: &str);
    # [ wasm_bindgen ( structural , method , getter , js_class = "DataTransfer" , js_name = effectAllowed ) ]
    #[doc = "Getter for the `effectAllowed` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DataTransfer/effectAllowed)\n\n*This API requires the following crate features to be activated: `DataTransfer`*"]
    pub fn effect_allowed(this: &DataTransfer) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_class = "DataTransfer" , js_name = effectAllowed ) ]
    #[doc = "Setter for the `effectAllowed` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DataTransfer/effectAllowed)\n\n*This API requires the following crate features to be activated: `DataTransfer`*"]
    pub fn set_effect_allowed(this: &DataTransfer, value: &str);
    # [ wasm_bindgen ( structural , method , getter , js_class = "DataTransfer" , js_name = items ) ]
    #[cfg(feature = "DataTransferItemList")]
    #[doc = "Getter for the `items` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DataTransfer/items)\n\n*This API requires the following crate features to be activated: `DataTransfer`, `DataTransferItemList`*"]
    pub fn items(this: &DataTransfer) -> DataTransferItemList;
    # [ wasm_bindgen ( structural , method , getter , js_class = "DataTransfer" , js_name = types ) ]
    #[doc = "Getter for the `types` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DataTransfer/types)\n\n*This API requires the following crate features to be activated: `DataTransfer`*"]
    pub fn types(this: &DataTransfer) -> ::js_sys::Array;
    # [ wasm_bindgen ( structural , method , getter , js_class = "DataTransfer" , js_name = files ) ]
    #[cfg(feature = "FileList")]
    #[doc = "Getter for the `files` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DataTransfer/files)\n\n*This API requires the following crate features to be activated: `DataTransfer`, `FileList`*"]
    pub fn files(this: &DataTransfer) -> Option<FileList>;
    #[wasm_bindgen(catch, js_class = "DataTransfer", constructor)]
    #[doc = "The `new DataTransfer(..)` constructor, creating a new instance of `DataTransfer`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DataTransfer/DataTransfer)\n\n*This API requires the following crate features to be activated: `DataTransfer`*"]
    pub fn new(this: &DataTransfer) -> Result<DataTransfer, JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_class = "DataTransfer" , js_name = clearData ) ]
    #[doc = "The `clearData()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DataTransfer/clearData)\n\n*This API requires the following crate features to be activated: `DataTransfer`*"]
    pub fn clear_data(this: &DataTransfer) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_class = "DataTransfer" , js_name = clearData ) ]
    #[doc = "The `clearData()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DataTransfer/clearData)\n\n*This API requires the following crate features to be activated: `DataTransfer`*"]
    pub fn clear_data_with_format(this: &DataTransfer, format: &str) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_class = "DataTransfer" , js_name = getData ) ]
    #[doc = "The `getData()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DataTransfer/getData)\n\n*This API requires the following crate features to be activated: `DataTransfer`*"]
    pub fn get_data(this: &DataTransfer, format: &str) -> Result<String, JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_class = "DataTransfer" , js_name = getFiles ) ]
    #[doc = "The `getFiles()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DataTransfer/getFiles)\n\n*This API requires the following crate features to be activated: `DataTransfer`*"]
    pub fn get_files(this: &DataTransfer) -> Result<::js_sys::Promise, JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_class = "DataTransfer" , js_name = getFiles ) ]
    #[doc = "The `getFiles()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DataTransfer/getFiles)\n\n*This API requires the following crate features to be activated: `DataTransfer`*"]
    pub fn get_files_with_recursive_flag(
        this: &DataTransfer,
        recursive_flag: bool,
    ) -> Result<::js_sys::Promise, JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_class = "DataTransfer" , js_name = getFilesAndDirectories ) ]
    #[doc = "The `getFilesAndDirectories()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DataTransfer/getFilesAndDirectories)\n\n*This API requires the following crate features to be activated: `DataTransfer`*"]
    pub fn get_files_and_directories(this: &DataTransfer) -> Result<::js_sys::Promise, JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_class = "DataTransfer" , js_name = setData ) ]
    #[doc = "The `setData()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DataTransfer/setData)\n\n*This API requires the following crate features to be activated: `DataTransfer`*"]
    pub fn set_data(this: &DataTransfer, format: &str, data: &str) -> Result<(), JsValue>;
    #[cfg(feature = "Element")]
    # [ wasm_bindgen ( method , structural , js_class = "DataTransfer" , js_name = setDragImage ) ]
    #[doc = "The `setDragImage()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DataTransfer/setDragImage)\n\n*This API requires the following crate features to be activated: `DataTransfer`, `Element`*"]
    pub fn set_drag_image(this: &DataTransfer, image: &Element, x: i32, y: i32);
}
