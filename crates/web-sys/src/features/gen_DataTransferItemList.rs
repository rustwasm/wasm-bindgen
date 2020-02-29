use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = DataTransferItemList , typescript_type = "DataTransferItemList" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `DataTransferItemList` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DataTransferItemList)
    ///
    ///*This API requires the following crate features to be activated: `DataTransferItemList`*
    pub type DataTransferItemList;

    # [ wasm_bindgen ( structural , method , getter , js_class = "DataTransferItemList" , js_name = length ) ]
    ///Getter for the `length` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DataTransferItemList/length)
    ///
    ///*This API requires the following crate features to be activated: `DataTransferItemList`*
    pub fn length(this: &DataTransferItemList) -> u32;

    #[cfg(feature = "DataTransferItem")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "DataTransferItemList" , js_name = add ) ]
    ///The `add()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DataTransferItemList/add)
    ///
    ///*This API requires the following crate features to be activated: `DataTransferItem`, `DataTransferItemList`*
    pub fn add_with_str_and_type(
        this: &DataTransferItemList,
        data: &str,
        type_: &str,
    ) -> Result<Option<DataTransferItem>, JsValue>;

    #[cfg(all(feature = "DataTransferItem", feature = "File",))]
    # [ wasm_bindgen ( catch , method , structural , js_class = "DataTransferItemList" , js_name = add ) ]
    ///The `add()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DataTransferItemList/add)
    ///
    ///*This API requires the following crate features to be activated: `DataTransferItem`, `DataTransferItemList`, `File`*
    pub fn add_with_file(
        this: &DataTransferItemList,
        data: &File,
    ) -> Result<Option<DataTransferItem>, JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "DataTransferItemList" , js_name = clear ) ]
    ///The `clear()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DataTransferItemList/clear)
    ///
    ///*This API requires the following crate features to be activated: `DataTransferItemList`*
    pub fn clear(this: &DataTransferItemList) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "DataTransferItemList" , js_name = remove ) ]
    ///The `remove()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DataTransferItemList/remove)
    ///
    ///*This API requires the following crate features to be activated: `DataTransferItemList`*
    pub fn remove(this: &DataTransferItemList, index: u32) -> Result<(), JsValue>;

    #[cfg(feature = "DataTransferItem")]
    #[wasm_bindgen(method, structural, js_class = "DataTransferItemList", indexing_getter)]
    ///Indexing getter.
    ///
    ///
    ///
    ///*This API requires the following crate features to be activated: `DataTransferItem`, `DataTransferItemList`*
    pub fn get(this: &DataTransferItemList, index: u32) -> Option<DataTransferItem>;

}
