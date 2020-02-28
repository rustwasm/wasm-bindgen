use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = IDBObjectStore , typescript_name = IDBObjectStore ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `IdbObjectStore` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore)\n\n*This API requires the following crate features to be activated: `IdbObjectStore`*"]
    pub type IdbObjectStore;
    # [ wasm_bindgen ( structural , method , getter , js_class = "IDBObjectStore" , js_name = name ) ]
    #[doc = "Getter for the `name` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore/name)\n\n*This API requires the following crate features to be activated: `IdbObjectStore`*"]
    pub fn name(this: &IdbObjectStore) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_class = "IDBObjectStore" , js_name = name ) ]
    #[doc = "Setter for the `name` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore/name)\n\n*This API requires the following crate features to be activated: `IdbObjectStore`*"]
    pub fn set_name(this: &IdbObjectStore, value: &str);
    # [ wasm_bindgen ( structural , catch , method , getter , js_class = "IDBObjectStore" , js_name = keyPath ) ]
    #[doc = "Getter for the `keyPath` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore/keyPath)\n\n*This API requires the following crate features to be activated: `IdbObjectStore`*"]
    pub fn key_path(this: &IdbObjectStore) -> Result<::wasm_bindgen::JsValue, JsValue>;
    # [ wasm_bindgen ( structural , method , getter , js_class = "IDBObjectStore" , js_name = indexNames ) ]
    #[cfg(feature = "DomStringList")]
    #[doc = "Getter for the `indexNames` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore/indexNames)\n\n*This API requires the following crate features to be activated: `DomStringList`, `IdbObjectStore`*"]
    pub fn index_names(this: &IdbObjectStore) -> DomStringList;
    # [ wasm_bindgen ( structural , method , getter , js_class = "IDBObjectStore" , js_name = transaction ) ]
    #[cfg(feature = "IdbTransaction")]
    #[doc = "Getter for the `transaction` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore/transaction)\n\n*This API requires the following crate features to be activated: `IdbObjectStore`, `IdbTransaction`*"]
    pub fn transaction(this: &IdbObjectStore) -> IdbTransaction;
    # [ wasm_bindgen ( structural , method , getter , js_class = "IDBObjectStore" , js_name = autoIncrement ) ]
    #[doc = "Getter for the `autoIncrement` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore/autoIncrement)\n\n*This API requires the following crate features to be activated: `IdbObjectStore`*"]
    pub fn auto_increment(this: &IdbObjectStore) -> bool;
    #[cfg(feature = "IdbRequest")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "IDBObjectStore" , js_name = add ) ]
    #[doc = "The `add()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore/add)\n\n*This API requires the following crate features to be activated: `IdbObjectStore`, `IdbRequest`*"]
    pub fn add(
        this: &IdbObjectStore,
        value: &::wasm_bindgen::JsValue,
    ) -> Result<IdbRequest, JsValue>;
    #[cfg(feature = "IdbRequest")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "IDBObjectStore" , js_name = add ) ]
    #[doc = "The `add()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore/add)\n\n*This API requires the following crate features to be activated: `IdbObjectStore`, `IdbRequest`*"]
    pub fn add_with_key(
        this: &IdbObjectStore,
        value: &::wasm_bindgen::JsValue,
        key: &::wasm_bindgen::JsValue,
    ) -> Result<IdbRequest, JsValue>;
    #[cfg(feature = "IdbRequest")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "IDBObjectStore" , js_name = clear ) ]
    #[doc = "The `clear()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore/clear)\n\n*This API requires the following crate features to be activated: `IdbObjectStore`, `IdbRequest`*"]
    pub fn clear(this: &IdbObjectStore) -> Result<IdbRequest, JsValue>;
    #[cfg(feature = "IdbRequest")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "IDBObjectStore" , js_name = count ) ]
    #[doc = "The `count()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore/count)\n\n*This API requires the following crate features to be activated: `IdbObjectStore`, `IdbRequest`*"]
    pub fn count(this: &IdbObjectStore) -> Result<IdbRequest, JsValue>;
    #[cfg(feature = "IdbRequest")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "IDBObjectStore" , js_name = count ) ]
    #[doc = "The `count()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore/count)\n\n*This API requires the following crate features to be activated: `IdbObjectStore`, `IdbRequest`*"]
    pub fn count_with_key(
        this: &IdbObjectStore,
        key: &::wasm_bindgen::JsValue,
    ) -> Result<IdbRequest, JsValue>;
    #[cfg(feature = "IdbIndex")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "IDBObjectStore" , js_name = createIndex ) ]
    #[doc = "The `createIndex()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore/createIndex)\n\n*This API requires the following crate features to be activated: `IdbIndex`, `IdbObjectStore`*"]
    pub fn create_index_with_str(
        this: &IdbObjectStore,
        name: &str,
        key_path: &str,
    ) -> Result<IdbIndex, JsValue>;
    #[cfg(feature = "IdbIndex")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "IDBObjectStore" , js_name = createIndex ) ]
    #[doc = "The `createIndex()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore/createIndex)\n\n*This API requires the following crate features to be activated: `IdbIndex`, `IdbObjectStore`*"]
    pub fn create_index_with_str_sequence(
        this: &IdbObjectStore,
        name: &str,
        key_path: &::wasm_bindgen::JsValue,
    ) -> Result<IdbIndex, JsValue>;
    #[cfg(all(feature = "IdbIndex", feature = "IdbIndexParameters",))]
    # [ wasm_bindgen ( catch , method , structural , js_class = "IDBObjectStore" , js_name = createIndex ) ]
    #[doc = "The `createIndex()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore/createIndex)\n\n*This API requires the following crate features to be activated: `IdbIndex`, `IdbIndexParameters`, `IdbObjectStore`*"]
    pub fn create_index_with_str_and_optional_parameters(
        this: &IdbObjectStore,
        name: &str,
        key_path: &str,
        optional_parameters: &IdbIndexParameters,
    ) -> Result<IdbIndex, JsValue>;
    #[cfg(all(feature = "IdbIndex", feature = "IdbIndexParameters",))]
    # [ wasm_bindgen ( catch , method , structural , js_class = "IDBObjectStore" , js_name = createIndex ) ]
    #[doc = "The `createIndex()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore/createIndex)\n\n*This API requires the following crate features to be activated: `IdbIndex`, `IdbIndexParameters`, `IdbObjectStore`*"]
    pub fn create_index_with_str_sequence_and_optional_parameters(
        this: &IdbObjectStore,
        name: &str,
        key_path: &::wasm_bindgen::JsValue,
        optional_parameters: &IdbIndexParameters,
    ) -> Result<IdbIndex, JsValue>;
    #[cfg(feature = "IdbRequest")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "IDBObjectStore" , js_name = delete ) ]
    #[doc = "The `delete()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore/delete)\n\n*This API requires the following crate features to be activated: `IdbObjectStore`, `IdbRequest`*"]
    pub fn delete(
        this: &IdbObjectStore,
        key: &::wasm_bindgen::JsValue,
    ) -> Result<IdbRequest, JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_class = "IDBObjectStore" , js_name = deleteIndex ) ]
    #[doc = "The `deleteIndex()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore/deleteIndex)\n\n*This API requires the following crate features to be activated: `IdbObjectStore`*"]
    pub fn delete_index(this: &IdbObjectStore, index_name: &str) -> Result<(), JsValue>;
    #[cfg(feature = "IdbRequest")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "IDBObjectStore" , js_name = get ) ]
    #[doc = "The `get()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore/get)\n\n*This API requires the following crate features to be activated: `IdbObjectStore`, `IdbRequest`*"]
    pub fn get(this: &IdbObjectStore, key: &::wasm_bindgen::JsValue)
        -> Result<IdbRequest, JsValue>;
    #[cfg(feature = "IdbRequest")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "IDBObjectStore" , js_name = getAll ) ]
    #[doc = "The `getAll()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore/getAll)\n\n*This API requires the following crate features to be activated: `IdbObjectStore`, `IdbRequest`*"]
    pub fn get_all(this: &IdbObjectStore) -> Result<IdbRequest, JsValue>;
    #[cfg(feature = "IdbRequest")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "IDBObjectStore" , js_name = getAll ) ]
    #[doc = "The `getAll()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore/getAll)\n\n*This API requires the following crate features to be activated: `IdbObjectStore`, `IdbRequest`*"]
    pub fn get_all_with_key(
        this: &IdbObjectStore,
        key: &::wasm_bindgen::JsValue,
    ) -> Result<IdbRequest, JsValue>;
    #[cfg(feature = "IdbRequest")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "IDBObjectStore" , js_name = getAll ) ]
    #[doc = "The `getAll()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore/getAll)\n\n*This API requires the following crate features to be activated: `IdbObjectStore`, `IdbRequest`*"]
    pub fn get_all_with_key_and_limit(
        this: &IdbObjectStore,
        key: &::wasm_bindgen::JsValue,
        limit: u32,
    ) -> Result<IdbRequest, JsValue>;
    #[cfg(feature = "IdbRequest")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "IDBObjectStore" , js_name = getAllKeys ) ]
    #[doc = "The `getAllKeys()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore/getAllKeys)\n\n*This API requires the following crate features to be activated: `IdbObjectStore`, `IdbRequest`*"]
    pub fn get_all_keys(this: &IdbObjectStore) -> Result<IdbRequest, JsValue>;
    #[cfg(feature = "IdbRequest")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "IDBObjectStore" , js_name = getAllKeys ) ]
    #[doc = "The `getAllKeys()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore/getAllKeys)\n\n*This API requires the following crate features to be activated: `IdbObjectStore`, `IdbRequest`*"]
    pub fn get_all_keys_with_key(
        this: &IdbObjectStore,
        key: &::wasm_bindgen::JsValue,
    ) -> Result<IdbRequest, JsValue>;
    #[cfg(feature = "IdbRequest")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "IDBObjectStore" , js_name = getAllKeys ) ]
    #[doc = "The `getAllKeys()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore/getAllKeys)\n\n*This API requires the following crate features to be activated: `IdbObjectStore`, `IdbRequest`*"]
    pub fn get_all_keys_with_key_and_limit(
        this: &IdbObjectStore,
        key: &::wasm_bindgen::JsValue,
        limit: u32,
    ) -> Result<IdbRequest, JsValue>;
    #[cfg(feature = "IdbRequest")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "IDBObjectStore" , js_name = getKey ) ]
    #[doc = "The `getKey()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore/getKey)\n\n*This API requires the following crate features to be activated: `IdbObjectStore`, `IdbRequest`*"]
    pub fn get_key(
        this: &IdbObjectStore,
        key: &::wasm_bindgen::JsValue,
    ) -> Result<IdbRequest, JsValue>;
    #[cfg(feature = "IdbIndex")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "IDBObjectStore" , js_name = index ) ]
    #[doc = "The `index()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore/index)\n\n*This API requires the following crate features to be activated: `IdbIndex`, `IdbObjectStore`*"]
    pub fn index(this: &IdbObjectStore, name: &str) -> Result<IdbIndex, JsValue>;
    #[cfg(feature = "IdbRequest")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "IDBObjectStore" , js_name = openCursor ) ]
    #[doc = "The `openCursor()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore/openCursor)\n\n*This API requires the following crate features to be activated: `IdbObjectStore`, `IdbRequest`*"]
    pub fn open_cursor(this: &IdbObjectStore) -> Result<IdbRequest, JsValue>;
    #[cfg(feature = "IdbRequest")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "IDBObjectStore" , js_name = openCursor ) ]
    #[doc = "The `openCursor()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore/openCursor)\n\n*This API requires the following crate features to be activated: `IdbObjectStore`, `IdbRequest`*"]
    pub fn open_cursor_with_range(
        this: &IdbObjectStore,
        range: &::wasm_bindgen::JsValue,
    ) -> Result<IdbRequest, JsValue>;
    #[cfg(all(feature = "IdbCursorDirection", feature = "IdbRequest",))]
    # [ wasm_bindgen ( catch , method , structural , js_class = "IDBObjectStore" , js_name = openCursor ) ]
    #[doc = "The `openCursor()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore/openCursor)\n\n*This API requires the following crate features to be activated: `IdbCursorDirection`, `IdbObjectStore`, `IdbRequest`*"]
    pub fn open_cursor_with_range_and_direction(
        this: &IdbObjectStore,
        range: &::wasm_bindgen::JsValue,
        direction: IdbCursorDirection,
    ) -> Result<IdbRequest, JsValue>;
    #[cfg(feature = "IdbRequest")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "IDBObjectStore" , js_name = openKeyCursor ) ]
    #[doc = "The `openKeyCursor()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore/openKeyCursor)\n\n*This API requires the following crate features to be activated: `IdbObjectStore`, `IdbRequest`*"]
    pub fn open_key_cursor(this: &IdbObjectStore) -> Result<IdbRequest, JsValue>;
    #[cfg(feature = "IdbRequest")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "IDBObjectStore" , js_name = openKeyCursor ) ]
    #[doc = "The `openKeyCursor()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore/openKeyCursor)\n\n*This API requires the following crate features to be activated: `IdbObjectStore`, `IdbRequest`*"]
    pub fn open_key_cursor_with_range(
        this: &IdbObjectStore,
        range: &::wasm_bindgen::JsValue,
    ) -> Result<IdbRequest, JsValue>;
    #[cfg(all(feature = "IdbCursorDirection", feature = "IdbRequest",))]
    # [ wasm_bindgen ( catch , method , structural , js_class = "IDBObjectStore" , js_name = openKeyCursor ) ]
    #[doc = "The `openKeyCursor()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore/openKeyCursor)\n\n*This API requires the following crate features to be activated: `IdbCursorDirection`, `IdbObjectStore`, `IdbRequest`*"]
    pub fn open_key_cursor_with_range_and_direction(
        this: &IdbObjectStore,
        range: &::wasm_bindgen::JsValue,
        direction: IdbCursorDirection,
    ) -> Result<IdbRequest, JsValue>;
    #[cfg(feature = "IdbRequest")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "IDBObjectStore" , js_name = put ) ]
    #[doc = "The `put()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore/put)\n\n*This API requires the following crate features to be activated: `IdbObjectStore`, `IdbRequest`*"]
    pub fn put(
        this: &IdbObjectStore,
        value: &::wasm_bindgen::JsValue,
    ) -> Result<IdbRequest, JsValue>;
    #[cfg(feature = "IdbRequest")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "IDBObjectStore" , js_name = put ) ]
    #[doc = "The `put()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore/put)\n\n*This API requires the following crate features to be activated: `IdbObjectStore`, `IdbRequest`*"]
    pub fn put_with_key(
        this: &IdbObjectStore,
        value: &::wasm_bindgen::JsValue,
        key: &::wasm_bindgen::JsValue,
    ) -> Result<IdbRequest, JsValue>;
}
