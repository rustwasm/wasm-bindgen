use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = IDBIndex , typescript_type = "IDBIndex" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `IdbIndex` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBIndex)
    ///
    ///*This API requires the following crate features to be activated: `IdbIndex`*
    pub type IdbIndex;

    # [ wasm_bindgen ( structural , method , getter , js_class = "IDBIndex" , js_name = name ) ]
    ///Getter for the `name` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBIndex/name)
    ///
    ///*This API requires the following crate features to be activated: `IdbIndex`*
    pub fn name(this: &IdbIndex) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "IDBIndex" , js_name = name ) ]
    ///Setter for the `name` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBIndex/name)
    ///
    ///*This API requires the following crate features to be activated: `IdbIndex`*
    pub fn set_name(this: &IdbIndex, value: &str);

    #[cfg(feature = "IdbObjectStore")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "IDBIndex" , js_name = objectStore ) ]
    ///Getter for the `objectStore` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBIndex/objectStore)
    ///
    ///*This API requires the following crate features to be activated: `IdbIndex`, `IdbObjectStore`*
    pub fn object_store(this: &IdbIndex) -> IdbObjectStore;

    # [ wasm_bindgen ( structural , catch , method , getter , js_class = "IDBIndex" , js_name = keyPath ) ]
    ///Getter for the `keyPath` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBIndex/keyPath)
    ///
    ///*This API requires the following crate features to be activated: `IdbIndex`*
    pub fn key_path(this: &IdbIndex) -> Result<::wasm_bindgen::JsValue, JsValue>;

    # [ wasm_bindgen ( structural , method , getter , js_class = "IDBIndex" , js_name = multiEntry ) ]
    ///Getter for the `multiEntry` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBIndex/multiEntry)
    ///
    ///*This API requires the following crate features to be activated: `IdbIndex`*
    pub fn multi_entry(this: &IdbIndex) -> bool;

    # [ wasm_bindgen ( structural , method , getter , js_class = "IDBIndex" , js_name = unique ) ]
    ///Getter for the `unique` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBIndex/unique)
    ///
    ///*This API requires the following crate features to be activated: `IdbIndex`*
    pub fn unique(this: &IdbIndex) -> bool;

    # [ wasm_bindgen ( structural , method , getter , js_class = "IDBIndex" , js_name = locale ) ]
    ///Getter for the `locale` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBIndex/locale)
    ///
    ///*This API requires the following crate features to be activated: `IdbIndex`*
    pub fn locale(this: &IdbIndex) -> Option<String>;

    # [ wasm_bindgen ( structural , method , getter , js_class = "IDBIndex" , js_name = isAutoLocale ) ]
    ///Getter for the `isAutoLocale` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBIndex/isAutoLocale)
    ///
    ///*This API requires the following crate features to be activated: `IdbIndex`*
    pub fn is_auto_locale(this: &IdbIndex) -> bool;

    #[cfg(feature = "IdbRequest")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "IDBIndex" , js_name = count ) ]
    ///The `count()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBIndex/count)
    ///
    ///*This API requires the following crate features to be activated: `IdbIndex`, `IdbRequest`*
    pub fn count(this: &IdbIndex) -> Result<IdbRequest, JsValue>;

    #[cfg(feature = "IdbRequest")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "IDBIndex" , js_name = count ) ]
    ///The `count()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBIndex/count)
    ///
    ///*This API requires the following crate features to be activated: `IdbIndex`, `IdbRequest`*
    pub fn count_with_key(
        this: &IdbIndex,
        key: &::wasm_bindgen::JsValue,
    ) -> Result<IdbRequest, JsValue>;

    #[cfg(feature = "IdbRequest")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "IDBIndex" , js_name = get ) ]
    ///The `get()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBIndex/get)
    ///
    ///*This API requires the following crate features to be activated: `IdbIndex`, `IdbRequest`*
    pub fn get(this: &IdbIndex, key: &::wasm_bindgen::JsValue) -> Result<IdbRequest, JsValue>;

    #[cfg(feature = "IdbRequest")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "IDBIndex" , js_name = getAll ) ]
    ///The `getAll()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBIndex/getAll)
    ///
    ///*This API requires the following crate features to be activated: `IdbIndex`, `IdbRequest`*
    pub fn get_all(this: &IdbIndex) -> Result<IdbRequest, JsValue>;

    #[cfg(feature = "IdbRequest")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "IDBIndex" , js_name = getAll ) ]
    ///The `getAll()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBIndex/getAll)
    ///
    ///*This API requires the following crate features to be activated: `IdbIndex`, `IdbRequest`*
    pub fn get_all_with_key(
        this: &IdbIndex,
        key: &::wasm_bindgen::JsValue,
    ) -> Result<IdbRequest, JsValue>;

    #[cfg(feature = "IdbRequest")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "IDBIndex" , js_name = getAll ) ]
    ///The `getAll()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBIndex/getAll)
    ///
    ///*This API requires the following crate features to be activated: `IdbIndex`, `IdbRequest`*
    pub fn get_all_with_key_and_limit(
        this: &IdbIndex,
        key: &::wasm_bindgen::JsValue,
        limit: u32,
    ) -> Result<IdbRequest, JsValue>;

    #[cfg(feature = "IdbRequest")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "IDBIndex" , js_name = getAllKeys ) ]
    ///The `getAllKeys()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBIndex/getAllKeys)
    ///
    ///*This API requires the following crate features to be activated: `IdbIndex`, `IdbRequest`*
    pub fn get_all_keys(this: &IdbIndex) -> Result<IdbRequest, JsValue>;

    #[cfg(feature = "IdbRequest")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "IDBIndex" , js_name = getAllKeys ) ]
    ///The `getAllKeys()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBIndex/getAllKeys)
    ///
    ///*This API requires the following crate features to be activated: `IdbIndex`, `IdbRequest`*
    pub fn get_all_keys_with_key(
        this: &IdbIndex,
        key: &::wasm_bindgen::JsValue,
    ) -> Result<IdbRequest, JsValue>;

    #[cfg(feature = "IdbRequest")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "IDBIndex" , js_name = getAllKeys ) ]
    ///The `getAllKeys()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBIndex/getAllKeys)
    ///
    ///*This API requires the following crate features to be activated: `IdbIndex`, `IdbRequest`*
    pub fn get_all_keys_with_key_and_limit(
        this: &IdbIndex,
        key: &::wasm_bindgen::JsValue,
        limit: u32,
    ) -> Result<IdbRequest, JsValue>;

    #[cfg(feature = "IdbRequest")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "IDBIndex" , js_name = getKey ) ]
    ///The `getKey()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBIndex/getKey)
    ///
    ///*This API requires the following crate features to be activated: `IdbIndex`, `IdbRequest`*
    pub fn get_key(this: &IdbIndex, key: &::wasm_bindgen::JsValue) -> Result<IdbRequest, JsValue>;

    #[cfg(feature = "IdbRequest")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "IDBIndex" , js_name = openCursor ) ]
    ///The `openCursor()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBIndex/openCursor)
    ///
    ///*This API requires the following crate features to be activated: `IdbIndex`, `IdbRequest`*
    pub fn open_cursor(this: &IdbIndex) -> Result<IdbRequest, JsValue>;

    #[cfg(feature = "IdbRequest")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "IDBIndex" , js_name = openCursor ) ]
    ///The `openCursor()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBIndex/openCursor)
    ///
    ///*This API requires the following crate features to be activated: `IdbIndex`, `IdbRequest`*
    pub fn open_cursor_with_range(
        this: &IdbIndex,
        range: &::wasm_bindgen::JsValue,
    ) -> Result<IdbRequest, JsValue>;

    #[cfg(all(feature = "IdbCursorDirection", feature = "IdbRequest",))]
    # [ wasm_bindgen ( catch , method , structural , js_class = "IDBIndex" , js_name = openCursor ) ]
    ///The `openCursor()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBIndex/openCursor)
    ///
    ///*This API requires the following crate features to be activated: `IdbCursorDirection`, `IdbIndex`, `IdbRequest`*
    pub fn open_cursor_with_range_and_direction(
        this: &IdbIndex,
        range: &::wasm_bindgen::JsValue,
        direction: IdbCursorDirection,
    ) -> Result<IdbRequest, JsValue>;

    #[cfg(feature = "IdbRequest")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "IDBIndex" , js_name = openKeyCursor ) ]
    ///The `openKeyCursor()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBIndex/openKeyCursor)
    ///
    ///*This API requires the following crate features to be activated: `IdbIndex`, `IdbRequest`*
    pub fn open_key_cursor(this: &IdbIndex) -> Result<IdbRequest, JsValue>;

    #[cfg(feature = "IdbRequest")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "IDBIndex" , js_name = openKeyCursor ) ]
    ///The `openKeyCursor()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBIndex/openKeyCursor)
    ///
    ///*This API requires the following crate features to be activated: `IdbIndex`, `IdbRequest`*
    pub fn open_key_cursor_with_range(
        this: &IdbIndex,
        range: &::wasm_bindgen::JsValue,
    ) -> Result<IdbRequest, JsValue>;

    #[cfg(all(feature = "IdbCursorDirection", feature = "IdbRequest",))]
    # [ wasm_bindgen ( catch , method , structural , js_class = "IDBIndex" , js_name = openKeyCursor ) ]
    ///The `openKeyCursor()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBIndex/openKeyCursor)
    ///
    ///*This API requires the following crate features to be activated: `IdbCursorDirection`, `IdbIndex`, `IdbRequest`*
    pub fn open_key_cursor_with_range_and_direction(
        this: &IdbIndex,
        range: &::wasm_bindgen::JsValue,
        direction: IdbCursorDirection,
    ) -> Result<IdbRequest, JsValue>;

}
