use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = DataTransfer , typescript_name = DataTransfer ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `DataTransfer` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DataTransfer)
    ///
    ///*This API requires the following crate features to be activated: `DataTransfer`*
    pub type DataTransfer;

    # [ wasm_bindgen ( structural , method , getter , js_class = "DataTransfer" , js_name = dropEffect ) ]
    ///Getter for the `dropEffect` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DataTransfer/dropEffect)
    ///
    ///*This API requires the following crate features to be activated: `DataTransfer`*
    pub fn drop_effect(this: &DataTransfer) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "DataTransfer" , js_name = dropEffect ) ]
    ///Setter for the `dropEffect` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DataTransfer/dropEffect)
    ///
    ///*This API requires the following crate features to be activated: `DataTransfer`*
    pub fn set_drop_effect(this: &DataTransfer, value: &str);

    # [ wasm_bindgen ( structural , method , getter , js_class = "DataTransfer" , js_name = effectAllowed ) ]
    ///Getter for the `effectAllowed` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DataTransfer/effectAllowed)
    ///
    ///*This API requires the following crate features to be activated: `DataTransfer`*
    pub fn effect_allowed(this: &DataTransfer) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "DataTransfer" , js_name = effectAllowed ) ]
    ///Setter for the `effectAllowed` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DataTransfer/effectAllowed)
    ///
    ///*This API requires the following crate features to be activated: `DataTransfer`*
    pub fn set_effect_allowed(this: &DataTransfer, value: &str);

    #[cfg(feature = "DataTransferItemList")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "DataTransfer" , js_name = items ) ]
    ///Getter for the `items` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DataTransfer/items)
    ///
    ///*This API requires the following crate features to be activated: `DataTransfer`, `DataTransferItemList`*
    pub fn items(this: &DataTransfer) -> DataTransferItemList;

    # [ wasm_bindgen ( structural , method , getter , js_class = "DataTransfer" , js_name = types ) ]
    ///Getter for the `types` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DataTransfer/types)
    ///
    ///*This API requires the following crate features to be activated: `DataTransfer`*
    pub fn types(this: &DataTransfer) -> ::js_sys::Array;

    #[cfg(feature = "FileList")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "DataTransfer" , js_name = files ) ]
    ///Getter for the `files` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DataTransfer/files)
    ///
    ///*This API requires the following crate features to be activated: `DataTransfer`, `FileList`*
    pub fn files(this: &DataTransfer) -> Option<FileList>;

    #[wasm_bindgen(catch, constructor, js_class = "DataTransfer")]
    ///The `new DataTransfer(..)` constructor, creating a new instance of `DataTransfer`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DataTransfer/DataTransfer)
    ///
    ///*This API requires the following crate features to be activated: `DataTransfer`*
    pub fn new() -> Result<DataTransfer, JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "DataTransfer" , js_name = clearData ) ]
    ///The `clearData()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DataTransfer/clearData)
    ///
    ///*This API requires the following crate features to be activated: `DataTransfer`*
    pub fn clear_data(this: &DataTransfer) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "DataTransfer" , js_name = clearData ) ]
    ///The `clearData()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DataTransfer/clearData)
    ///
    ///*This API requires the following crate features to be activated: `DataTransfer`*
    pub fn clear_data_with_format(this: &DataTransfer, format: &str) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "DataTransfer" , js_name = getData ) ]
    ///The `getData()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DataTransfer/getData)
    ///
    ///*This API requires the following crate features to be activated: `DataTransfer`*
    pub fn get_data(this: &DataTransfer, format: &str) -> Result<String, JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "DataTransfer" , js_name = getFiles ) ]
    ///The `getFiles()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DataTransfer/getFiles)
    ///
    ///*This API requires the following crate features to be activated: `DataTransfer`*
    pub fn get_files(this: &DataTransfer) -> Result<::js_sys::Promise, JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "DataTransfer" , js_name = getFiles ) ]
    ///The `getFiles()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DataTransfer/getFiles)
    ///
    ///*This API requires the following crate features to be activated: `DataTransfer`*
    pub fn get_files_with_recursive_flag(
        this: &DataTransfer,
        recursive_flag: bool,
    ) -> Result<::js_sys::Promise, JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "DataTransfer" , js_name = getFilesAndDirectories ) ]
    ///The `getFilesAndDirectories()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DataTransfer/getFilesAndDirectories)
    ///
    ///*This API requires the following crate features to be activated: `DataTransfer`*
    pub fn get_files_and_directories(this: &DataTransfer) -> Result<::js_sys::Promise, JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "DataTransfer" , js_name = setData ) ]
    ///The `setData()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DataTransfer/setData)
    ///
    ///*This API requires the following crate features to be activated: `DataTransfer`*
    pub fn set_data(this: &DataTransfer, format: &str, data: &str) -> Result<(), JsValue>;

    #[cfg(feature = "Element")]
    # [ wasm_bindgen ( method , structural , js_class = "DataTransfer" , js_name = setDragImage ) ]
    ///The `setDragImage()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DataTransfer/setDragImage)
    ///
    ///*This API requires the following crate features to be activated: `DataTransfer`, `Element`*
    pub fn set_drag_image(this: &DataTransfer, image: &Element, x: i32, y: i32);

}
