use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = IDBObjectStore , typescript_type = "IDBObjectStore" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `IdbObjectStore` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore)
    ///
    ///*This API requires the following crate features to be activated: `IdbObjectStore`*
    pub type IdbObjectStore;

    # [ wasm_bindgen ( structural , method , getter , js_class = "IDBObjectStore" , js_name = name ) ]
    ///Getter for the `name` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore/name)
    ///
    ///*This API requires the following crate features to be activated: `IdbObjectStore`*
    pub fn name(this: &IdbObjectStore) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "IDBObjectStore" , js_name = name ) ]
    ///Setter for the `name` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore/name)
    ///
    ///*This API requires the following crate features to be activated: `IdbObjectStore`*
    pub fn set_name(this: &IdbObjectStore, value: &str);

    # [ wasm_bindgen ( structural , catch , method , getter , js_class = "IDBObjectStore" , js_name = keyPath ) ]
    ///Getter for the `keyPath` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore/keyPath)
    ///
    ///*This API requires the following crate features to be activated: `IdbObjectStore`*
    pub fn key_path(this: &IdbObjectStore) -> Result<::wasm_bindgen::JsValue, JsValue>;

    #[cfg(feature = "DomStringList")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "IDBObjectStore" , js_name = indexNames ) ]
    ///Getter for the `indexNames` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore/indexNames)
    ///
    ///*This API requires the following crate features to be activated: `DomStringList`, `IdbObjectStore`*
    pub fn index_names(this: &IdbObjectStore) -> DomStringList;

    #[cfg(feature = "IdbTransaction")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "IDBObjectStore" , js_name = transaction ) ]
    ///Getter for the `transaction` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore/transaction)
    ///
    ///*This API requires the following crate features to be activated: `IdbObjectStore`, `IdbTransaction`*
    pub fn transaction(this: &IdbObjectStore) -> IdbTransaction;

    # [ wasm_bindgen ( structural , method , getter , js_class = "IDBObjectStore" , js_name = autoIncrement ) ]
    ///Getter for the `autoIncrement` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore/autoIncrement)
    ///
    ///*This API requires the following crate features to be activated: `IdbObjectStore`*
    pub fn auto_increment(this: &IdbObjectStore) -> bool;

    #[cfg(feature = "IdbRequest")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "IDBObjectStore" , js_name = add ) ]
    ///The `add()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore/add)
    ///
    ///*This API requires the following crate features to be activated: `IdbObjectStore`, `IdbRequest`*
    pub fn add(
        this: &IdbObjectStore,
        value: &::wasm_bindgen::JsValue,
    ) -> Result<IdbRequest, JsValue>;

    #[cfg(feature = "IdbRequest")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "IDBObjectStore" , js_name = add ) ]
    ///The `add()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore/add)
    ///
    ///*This API requires the following crate features to be activated: `IdbObjectStore`, `IdbRequest`*
    pub fn add_with_key(
        this: &IdbObjectStore,
        value: &::wasm_bindgen::JsValue,
        key: &::wasm_bindgen::JsValue,
    ) -> Result<IdbRequest, JsValue>;

    #[cfg(feature = "IdbRequest")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "IDBObjectStore" , js_name = clear ) ]
    ///The `clear()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore/clear)
    ///
    ///*This API requires the following crate features to be activated: `IdbObjectStore`, `IdbRequest`*
    pub fn clear(this: &IdbObjectStore) -> Result<IdbRequest, JsValue>;

    #[cfg(feature = "IdbRequest")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "IDBObjectStore" , js_name = count ) ]
    ///The `count()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore/count)
    ///
    ///*This API requires the following crate features to be activated: `IdbObjectStore`, `IdbRequest`*
    pub fn count(this: &IdbObjectStore) -> Result<IdbRequest, JsValue>;

    #[cfg(feature = "IdbRequest")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "IDBObjectStore" , js_name = count ) ]
    ///The `count()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore/count)
    ///
    ///*This API requires the following crate features to be activated: `IdbObjectStore`, `IdbRequest`*
    pub fn count_with_key(
        this: &IdbObjectStore,
        key: &::wasm_bindgen::JsValue,
    ) -> Result<IdbRequest, JsValue>;

    #[cfg(feature = "IdbIndex")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "IDBObjectStore" , js_name = createIndex ) ]
    ///The `createIndex()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore/createIndex)
    ///
    ///*This API requires the following crate features to be activated: `IdbIndex`, `IdbObjectStore`*
    pub fn create_index_with_str(
        this: &IdbObjectStore,
        name: &str,
        key_path: &str,
    ) -> Result<IdbIndex, JsValue>;

    #[cfg(feature = "IdbIndex")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "IDBObjectStore" , js_name = createIndex ) ]
    ///The `createIndex()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore/createIndex)
    ///
    ///*This API requires the following crate features to be activated: `IdbIndex`, `IdbObjectStore`*
    pub fn create_index_with_str_sequence(
        this: &IdbObjectStore,
        name: &str,
        key_path: &::wasm_bindgen::JsValue,
    ) -> Result<IdbIndex, JsValue>;

    #[cfg(all(feature = "IdbIndex", feature = "IdbIndexParameters",))]
    # [ wasm_bindgen ( catch , method , structural , js_class = "IDBObjectStore" , js_name = createIndex ) ]
    ///The `createIndex()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore/createIndex)
    ///
    ///*This API requires the following crate features to be activated: `IdbIndex`, `IdbIndexParameters`, `IdbObjectStore`*
    pub fn create_index_with_str_and_optional_parameters(
        this: &IdbObjectStore,
        name: &str,
        key_path: &str,
        optional_parameters: &IdbIndexParameters,
    ) -> Result<IdbIndex, JsValue>;

    #[cfg(all(feature = "IdbIndex", feature = "IdbIndexParameters",))]
    # [ wasm_bindgen ( catch , method , structural , js_class = "IDBObjectStore" , js_name = createIndex ) ]
    ///The `createIndex()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore/createIndex)
    ///
    ///*This API requires the following crate features to be activated: `IdbIndex`, `IdbIndexParameters`, `IdbObjectStore`*
    pub fn create_index_with_str_sequence_and_optional_parameters(
        this: &IdbObjectStore,
        name: &str,
        key_path: &::wasm_bindgen::JsValue,
        optional_parameters: &IdbIndexParameters,
    ) -> Result<IdbIndex, JsValue>;

    #[cfg(feature = "IdbRequest")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "IDBObjectStore" , js_name = delete ) ]
    ///The `delete()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore/delete)
    ///
    ///*This API requires the following crate features to be activated: `IdbObjectStore`, `IdbRequest`*
    pub fn delete(
        this: &IdbObjectStore,
        key: &::wasm_bindgen::JsValue,
    ) -> Result<IdbRequest, JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "IDBObjectStore" , js_name = deleteIndex ) ]
    ///The `deleteIndex()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore/deleteIndex)
    ///
    ///*This API requires the following crate features to be activated: `IdbObjectStore`*
    pub fn delete_index(this: &IdbObjectStore, index_name: &str) -> Result<(), JsValue>;

    #[cfg(feature = "IdbRequest")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "IDBObjectStore" , js_name = get ) ]
    ///The `get()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore/get)
    ///
    ///*This API requires the following crate features to be activated: `IdbObjectStore`, `IdbRequest`*
    pub fn get(this: &IdbObjectStore, key: &::wasm_bindgen::JsValue)
        -> Result<IdbRequest, JsValue>;

    #[cfg(feature = "IdbRequest")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "IDBObjectStore" , js_name = getAll ) ]
    ///The `getAll()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore/getAll)
    ///
    ///*This API requires the following crate features to be activated: `IdbObjectStore`, `IdbRequest`*
    pub fn get_all(this: &IdbObjectStore) -> Result<IdbRequest, JsValue>;

    #[cfg(feature = "IdbRequest")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "IDBObjectStore" , js_name = getAll ) ]
    ///The `getAll()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore/getAll)
    ///
    ///*This API requires the following crate features to be activated: `IdbObjectStore`, `IdbRequest`*
    pub fn get_all_with_key(
        this: &IdbObjectStore,
        key: &::wasm_bindgen::JsValue,
    ) -> Result<IdbRequest, JsValue>;

    #[cfg(feature = "IdbRequest")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "IDBObjectStore" , js_name = getAll ) ]
    ///The `getAll()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore/getAll)
    ///
    ///*This API requires the following crate features to be activated: `IdbObjectStore`, `IdbRequest`*
    pub fn get_all_with_key_and_limit(
        this: &IdbObjectStore,
        key: &::wasm_bindgen::JsValue,
        limit: u32,
    ) -> Result<IdbRequest, JsValue>;

    #[cfg(feature = "IdbRequest")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "IDBObjectStore" , js_name = getAllKeys ) ]
    ///The `getAllKeys()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore/getAllKeys)
    ///
    ///*This API requires the following crate features to be activated: `IdbObjectStore`, `IdbRequest`*
    pub fn get_all_keys(this: &IdbObjectStore) -> Result<IdbRequest, JsValue>;

    #[cfg(feature = "IdbRequest")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "IDBObjectStore" , js_name = getAllKeys ) ]
    ///The `getAllKeys()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore/getAllKeys)
    ///
    ///*This API requires the following crate features to be activated: `IdbObjectStore`, `IdbRequest`*
    pub fn get_all_keys_with_key(
        this: &IdbObjectStore,
        key: &::wasm_bindgen::JsValue,
    ) -> Result<IdbRequest, JsValue>;

    #[cfg(feature = "IdbRequest")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "IDBObjectStore" , js_name = getAllKeys ) ]
    ///The `getAllKeys()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore/getAllKeys)
    ///
    ///*This API requires the following crate features to be activated: `IdbObjectStore`, `IdbRequest`*
    pub fn get_all_keys_with_key_and_limit(
        this: &IdbObjectStore,
        key: &::wasm_bindgen::JsValue,
        limit: u32,
    ) -> Result<IdbRequest, JsValue>;

    #[cfg(feature = "IdbRequest")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "IDBObjectStore" , js_name = getKey ) ]
    ///The `getKey()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore/getKey)
    ///
    ///*This API requires the following crate features to be activated: `IdbObjectStore`, `IdbRequest`*
    pub fn get_key(
        this: &IdbObjectStore,
        key: &::wasm_bindgen::JsValue,
    ) -> Result<IdbRequest, JsValue>;

    #[cfg(feature = "IdbIndex")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "IDBObjectStore" , js_name = index ) ]
    ///The `index()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore/index)
    ///
    ///*This API requires the following crate features to be activated: `IdbIndex`, `IdbObjectStore`*
    pub fn index(this: &IdbObjectStore, name: &str) -> Result<IdbIndex, JsValue>;

    #[cfg(feature = "IdbRequest")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "IDBObjectStore" , js_name = openCursor ) ]
    ///The `openCursor()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore/openCursor)
    ///
    ///*This API requires the following crate features to be activated: `IdbObjectStore`, `IdbRequest`*
    pub fn open_cursor(this: &IdbObjectStore) -> Result<IdbRequest, JsValue>;

    #[cfg(feature = "IdbRequest")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "IDBObjectStore" , js_name = openCursor ) ]
    ///The `openCursor()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore/openCursor)
    ///
    ///*This API requires the following crate features to be activated: `IdbObjectStore`, `IdbRequest`*
    pub fn open_cursor_with_range(
        this: &IdbObjectStore,
        range: &::wasm_bindgen::JsValue,
    ) -> Result<IdbRequest, JsValue>;

    #[cfg(all(feature = "IdbCursorDirection", feature = "IdbRequest",))]
    # [ wasm_bindgen ( catch , method , structural , js_class = "IDBObjectStore" , js_name = openCursor ) ]
    ///The `openCursor()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore/openCursor)
    ///
    ///*This API requires the following crate features to be activated: `IdbCursorDirection`, `IdbObjectStore`, `IdbRequest`*
    pub fn open_cursor_with_range_and_direction(
        this: &IdbObjectStore,
        range: &::wasm_bindgen::JsValue,
        direction: IdbCursorDirection,
    ) -> Result<IdbRequest, JsValue>;

    #[cfg(feature = "IdbRequest")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "IDBObjectStore" , js_name = openKeyCursor ) ]
    ///The `openKeyCursor()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore/openKeyCursor)
    ///
    ///*This API requires the following crate features to be activated: `IdbObjectStore`, `IdbRequest`*
    pub fn open_key_cursor(this: &IdbObjectStore) -> Result<IdbRequest, JsValue>;

    #[cfg(feature = "IdbRequest")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "IDBObjectStore" , js_name = openKeyCursor ) ]
    ///The `openKeyCursor()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore/openKeyCursor)
    ///
    ///*This API requires the following crate features to be activated: `IdbObjectStore`, `IdbRequest`*
    pub fn open_key_cursor_with_range(
        this: &IdbObjectStore,
        range: &::wasm_bindgen::JsValue,
    ) -> Result<IdbRequest, JsValue>;

    #[cfg(all(feature = "IdbCursorDirection", feature = "IdbRequest",))]
    # [ wasm_bindgen ( catch , method , structural , js_class = "IDBObjectStore" , js_name = openKeyCursor ) ]
    ///The `openKeyCursor()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore/openKeyCursor)
    ///
    ///*This API requires the following crate features to be activated: `IdbCursorDirection`, `IdbObjectStore`, `IdbRequest`*
    pub fn open_key_cursor_with_range_and_direction(
        this: &IdbObjectStore,
        range: &::wasm_bindgen::JsValue,
        direction: IdbCursorDirection,
    ) -> Result<IdbRequest, JsValue>;

    #[cfg(feature = "IdbRequest")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "IDBObjectStore" , js_name = put ) ]
    ///The `put()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore/put)
    ///
    ///*This API requires the following crate features to be activated: `IdbObjectStore`, `IdbRequest`*
    pub fn put(
        this: &IdbObjectStore,
        value: &::wasm_bindgen::JsValue,
    ) -> Result<IdbRequest, JsValue>;

    #[cfg(feature = "IdbRequest")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "IDBObjectStore" , js_name = put ) ]
    ///The `put()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore/put)
    ///
    ///*This API requires the following crate features to be activated: `IdbObjectStore`, `IdbRequest`*
    pub fn put_with_key(
        this: &IdbObjectStore,
        value: &::wasm_bindgen::JsValue,
        key: &::wasm_bindgen::JsValue,
    ) -> Result<IdbRequest, JsValue>;

}
