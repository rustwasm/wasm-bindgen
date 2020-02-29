use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = EventTarget , extends = :: js_sys :: Object , js_name = IDBDatabase , typescript_type = "IDBDatabase" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `IdbDatabase` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBDatabase)
    ///
    ///*This API requires the following crate features to be activated: `IdbDatabase`*
    pub type IdbDatabase;

    # [ wasm_bindgen ( structural , method , getter , js_class = "IDBDatabase" , js_name = name ) ]
    ///Getter for the `name` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBDatabase/name)
    ///
    ///*This API requires the following crate features to be activated: `IdbDatabase`*
    pub fn name(this: &IdbDatabase) -> String;

    # [ wasm_bindgen ( structural , method , getter , js_class = "IDBDatabase" , js_name = version ) ]
    ///Getter for the `version` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBDatabase/version)
    ///
    ///*This API requires the following crate features to be activated: `IdbDatabase`*
    pub fn version(this: &IdbDatabase) -> f64;

    #[cfg(feature = "DomStringList")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "IDBDatabase" , js_name = objectStoreNames ) ]
    ///Getter for the `objectStoreNames` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBDatabase/objectStoreNames)
    ///
    ///*This API requires the following crate features to be activated: `DomStringList`, `IdbDatabase`*
    pub fn object_store_names(this: &IdbDatabase) -> DomStringList;

    # [ wasm_bindgen ( structural , method , getter , js_class = "IDBDatabase" , js_name = onabort ) ]
    ///Getter for the `onabort` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBDatabase/onabort)
    ///
    ///*This API requires the following crate features to be activated: `IdbDatabase`*
    pub fn onabort(this: &IdbDatabase) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "IDBDatabase" , js_name = onabort ) ]
    ///Setter for the `onabort` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBDatabase/onabort)
    ///
    ///*This API requires the following crate features to be activated: `IdbDatabase`*
    pub fn set_onabort(this: &IdbDatabase, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "IDBDatabase" , js_name = onclose ) ]
    ///Getter for the `onclose` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBDatabase/onclose)
    ///
    ///*This API requires the following crate features to be activated: `IdbDatabase`*
    pub fn onclose(this: &IdbDatabase) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "IDBDatabase" , js_name = onclose ) ]
    ///Setter for the `onclose` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBDatabase/onclose)
    ///
    ///*This API requires the following crate features to be activated: `IdbDatabase`*
    pub fn set_onclose(this: &IdbDatabase, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "IDBDatabase" , js_name = onerror ) ]
    ///Getter for the `onerror` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBDatabase/onerror)
    ///
    ///*This API requires the following crate features to be activated: `IdbDatabase`*
    pub fn onerror(this: &IdbDatabase) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "IDBDatabase" , js_name = onerror ) ]
    ///Setter for the `onerror` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBDatabase/onerror)
    ///
    ///*This API requires the following crate features to be activated: `IdbDatabase`*
    pub fn set_onerror(this: &IdbDatabase, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "IDBDatabase" , js_name = onversionchange ) ]
    ///Getter for the `onversionchange` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBDatabase/onversionchange)
    ///
    ///*This API requires the following crate features to be activated: `IdbDatabase`*
    pub fn onversionchange(this: &IdbDatabase) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "IDBDatabase" , js_name = onversionchange ) ]
    ///Setter for the `onversionchange` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBDatabase/onversionchange)
    ///
    ///*This API requires the following crate features to be activated: `IdbDatabase`*
    pub fn set_onversionchange(this: &IdbDatabase, value: Option<&::js_sys::Function>);

    #[cfg(feature = "StorageType")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "IDBDatabase" , js_name = storage ) ]
    ///Getter for the `storage` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBDatabase/storage)
    ///
    ///*This API requires the following crate features to be activated: `IdbDatabase`, `StorageType`*
    pub fn storage(this: &IdbDatabase) -> StorageType;

    # [ wasm_bindgen ( method , structural , js_class = "IDBDatabase" , js_name = close ) ]
    ///The `close()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBDatabase/close)
    ///
    ///*This API requires the following crate features to be activated: `IdbDatabase`*
    pub fn close(this: &IdbDatabase);

    #[cfg(feature = "IdbRequest")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "IDBDatabase" , js_name = createMutableFile ) ]
    ///The `createMutableFile()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBDatabase/createMutableFile)
    ///
    ///*This API requires the following crate features to be activated: `IdbDatabase`, `IdbRequest`*
    pub fn create_mutable_file(this: &IdbDatabase, name: &str) -> Result<IdbRequest, JsValue>;

    #[cfg(feature = "IdbRequest")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "IDBDatabase" , js_name = createMutableFile ) ]
    ///The `createMutableFile()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBDatabase/createMutableFile)
    ///
    ///*This API requires the following crate features to be activated: `IdbDatabase`, `IdbRequest`*
    pub fn create_mutable_file_with_type(
        this: &IdbDatabase,
        name: &str,
        type_: &str,
    ) -> Result<IdbRequest, JsValue>;

    #[cfg(feature = "IdbObjectStore")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "IDBDatabase" , js_name = createObjectStore ) ]
    ///The `createObjectStore()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBDatabase/createObjectStore)
    ///
    ///*This API requires the following crate features to be activated: `IdbDatabase`, `IdbObjectStore`*
    pub fn create_object_store(this: &IdbDatabase, name: &str) -> Result<IdbObjectStore, JsValue>;

    #[cfg(all(feature = "IdbObjectStore", feature = "IdbObjectStoreParameters",))]
    # [ wasm_bindgen ( catch , method , structural , js_class = "IDBDatabase" , js_name = createObjectStore ) ]
    ///The `createObjectStore()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBDatabase/createObjectStore)
    ///
    ///*This API requires the following crate features to be activated: `IdbDatabase`, `IdbObjectStore`, `IdbObjectStoreParameters`*
    pub fn create_object_store_with_optional_parameters(
        this: &IdbDatabase,
        name: &str,
        optional_parameters: &IdbObjectStoreParameters,
    ) -> Result<IdbObjectStore, JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "IDBDatabase" , js_name = deleteObjectStore ) ]
    ///The `deleteObjectStore()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBDatabase/deleteObjectStore)
    ///
    ///*This API requires the following crate features to be activated: `IdbDatabase`*
    pub fn delete_object_store(this: &IdbDatabase, name: &str) -> Result<(), JsValue>;

    #[cfg(feature = "IdbTransaction")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "IDBDatabase" , js_name = transaction ) ]
    ///The `transaction()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBDatabase/transaction)
    ///
    ///*This API requires the following crate features to be activated: `IdbDatabase`, `IdbTransaction`*
    pub fn transaction_with_str(
        this: &IdbDatabase,
        store_names: &str,
    ) -> Result<IdbTransaction, JsValue>;

    #[cfg(feature = "IdbTransaction")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "IDBDatabase" , js_name = transaction ) ]
    ///The `transaction()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBDatabase/transaction)
    ///
    ///*This API requires the following crate features to be activated: `IdbDatabase`, `IdbTransaction`*
    pub fn transaction_with_str_sequence(
        this: &IdbDatabase,
        store_names: &::wasm_bindgen::JsValue,
    ) -> Result<IdbTransaction, JsValue>;

    #[cfg(all(feature = "IdbTransaction", feature = "IdbTransactionMode",))]
    # [ wasm_bindgen ( catch , method , structural , js_class = "IDBDatabase" , js_name = transaction ) ]
    ///The `transaction()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBDatabase/transaction)
    ///
    ///*This API requires the following crate features to be activated: `IdbDatabase`, `IdbTransaction`, `IdbTransactionMode`*
    pub fn transaction_with_str_and_mode(
        this: &IdbDatabase,
        store_names: &str,
        mode: IdbTransactionMode,
    ) -> Result<IdbTransaction, JsValue>;

    #[cfg(all(feature = "IdbTransaction", feature = "IdbTransactionMode",))]
    # [ wasm_bindgen ( catch , method , structural , js_class = "IDBDatabase" , js_name = transaction ) ]
    ///The `transaction()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBDatabase/transaction)
    ///
    ///*This API requires the following crate features to be activated: `IdbDatabase`, `IdbTransaction`, `IdbTransactionMode`*
    pub fn transaction_with_str_sequence_and_mode(
        this: &IdbDatabase,
        store_names: &::wasm_bindgen::JsValue,
        mode: IdbTransactionMode,
    ) -> Result<IdbTransaction, JsValue>;

}
