use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = EventTarget , extends = :: js_sys :: Object , js_name = IDBDatabase , typescript_name = IDBDatabase ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `IdbDatabase` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBDatabase)\n\n*This API requires the following crate features to be activated: `IdbDatabase`*"]
    pub type IdbDatabase;
    # [ wasm_bindgen ( structural , method , getter , js_name = name ) ]
    #[doc = "Getter for the `name` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBDatabase/name)\n\n*This API requires the following crate features to be activated: `IdbDatabase`*"]
    pub fn name(this: &IdbDatabase) -> String;
    # [ wasm_bindgen ( structural , method , getter , js_name = version ) ]
    #[doc = "Getter for the `version` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBDatabase/version)\n\n*This API requires the following crate features to be activated: `IdbDatabase`*"]
    pub fn version(this: &IdbDatabase) -> f64;
    # [ wasm_bindgen ( structural , method , getter , js_name = objectStoreNames ) ]
    #[cfg(feature = "DomStringList")]
    #[doc = "Getter for the `objectStoreNames` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBDatabase/objectStoreNames)\n\n*This API requires the following crate features to be activated: `DomStringList`, `IdbDatabase`*"]
    pub fn object_store_names(this: &IdbDatabase) -> DomStringList;
    # [ wasm_bindgen ( structural , method , getter , js_name = onabort ) ]
    #[doc = "Getter for the `onabort` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBDatabase/onabort)\n\n*This API requires the following crate features to be activated: `IdbDatabase`*"]
    pub fn onabort(this: &IdbDatabase) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onabort ) ]
    #[doc = "Setter for the `onabort` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBDatabase/onabort)\n\n*This API requires the following crate features to be activated: `IdbDatabase`*"]
    pub fn set_onabort(this: &IdbDatabase, value: Option<::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = onclose ) ]
    #[doc = "Getter for the `onclose` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBDatabase/onclose)\n\n*This API requires the following crate features to be activated: `IdbDatabase`*"]
    pub fn onclose(this: &IdbDatabase) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onclose ) ]
    #[doc = "Setter for the `onclose` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBDatabase/onclose)\n\n*This API requires the following crate features to be activated: `IdbDatabase`*"]
    pub fn set_onclose(this: &IdbDatabase, value: Option<::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = onerror ) ]
    #[doc = "Getter for the `onerror` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBDatabase/onerror)\n\n*This API requires the following crate features to be activated: `IdbDatabase`*"]
    pub fn onerror(this: &IdbDatabase) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onerror ) ]
    #[doc = "Setter for the `onerror` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBDatabase/onerror)\n\n*This API requires the following crate features to be activated: `IdbDatabase`*"]
    pub fn set_onerror(this: &IdbDatabase, value: Option<::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = onversionchange ) ]
    #[doc = "Getter for the `onversionchange` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBDatabase/onversionchange)\n\n*This API requires the following crate features to be activated: `IdbDatabase`*"]
    pub fn onversionchange(this: &IdbDatabase) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onversionchange ) ]
    #[doc = "Setter for the `onversionchange` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBDatabase/onversionchange)\n\n*This API requires the following crate features to be activated: `IdbDatabase`*"]
    pub fn set_onversionchange(this: &IdbDatabase, value: Option<::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = storage ) ]
    #[cfg(feature = "StorageType")]
    #[doc = "Getter for the `storage` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBDatabase/storage)\n\n*This API requires the following crate features to be activated: `IdbDatabase`, `StorageType`*"]
    pub fn storage(this: &IdbDatabase) -> StorageType;
    # [ wasm_bindgen ( method , structural , js_name = close ) ]
    #[doc = "The `close()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBDatabase/close)\n\n*This API requires the following crate features to be activated: `IdbDatabase`*"]
    pub fn close(this: &IdbDatabase);
    #[cfg(feature = "IdbRequest")]
    # [ wasm_bindgen ( catch , method , structural , js_name = createMutableFile ) ]
    #[doc = "The `createMutableFile()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBDatabase/createMutableFile)\n\n*This API requires the following crate features to be activated: `IdbDatabase`, `IdbRequest`*"]
    pub fn create_mutable_file(this: &IdbDatabase, name: &str) -> Result<IdbRequest, JsValue>;
    #[cfg(feature = "IdbRequest")]
    # [ wasm_bindgen ( catch , method , structural , js_name = createMutableFile ) ]
    #[doc = "The `createMutableFile()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBDatabase/createMutableFile)\n\n*This API requires the following crate features to be activated: `IdbDatabase`, `IdbRequest`*"]
    pub fn create_mutable_file_with_type(
        this: &IdbDatabase,
        name: &str,
        type_: &str,
    ) -> Result<IdbRequest, JsValue>;
    #[cfg(feature = "IdbObjectStore")]
    # [ wasm_bindgen ( catch , method , structural , js_name = createObjectStore ) ]
    #[doc = "The `createObjectStore()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBDatabase/createObjectStore)\n\n*This API requires the following crate features to be activated: `IdbDatabase`, `IdbObjectStore`*"]
    pub fn create_object_store(this: &IdbDatabase, name: &str) -> Result<IdbObjectStore, JsValue>;
    #[cfg(all(feature = "IdbObjectStore", feature = "IdbObjectStoreParameters",))]
    # [ wasm_bindgen ( catch , method , structural , js_name = createObjectStore ) ]
    #[doc = "The `createObjectStore()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBDatabase/createObjectStore)\n\n*This API requires the following crate features to be activated: `IdbDatabase`, `IdbObjectStore`, `IdbObjectStoreParameters`*"]
    pub fn create_object_store_with_optional_parameters(
        this: &IdbDatabase,
        name: &str,
        optional_parameters: &IdbObjectStoreParameters,
    ) -> Result<IdbObjectStore, JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = deleteObjectStore ) ]
    #[doc = "The `deleteObjectStore()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBDatabase/deleteObjectStore)\n\n*This API requires the following crate features to be activated: `IdbDatabase`*"]
    pub fn delete_object_store(this: &IdbDatabase, name: &str) -> Result<(), JsValue>;
    #[cfg(feature = "IdbTransaction")]
    # [ wasm_bindgen ( catch , method , structural , js_name = transaction ) ]
    #[doc = "The `transaction()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBDatabase/transaction)\n\n*This API requires the following crate features to be activated: `IdbDatabase`, `IdbTransaction`*"]
    pub fn transaction_with_str(
        this: &IdbDatabase,
        store_names: &str,
    ) -> Result<IdbTransaction, JsValue>;
    #[cfg(feature = "IdbTransaction")]
    # [ wasm_bindgen ( catch , method , structural , js_name = transaction ) ]
    #[doc = "The `transaction()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBDatabase/transaction)\n\n*This API requires the following crate features to be activated: `IdbDatabase`, `IdbTransaction`*"]
    pub fn transaction_with_str_sequence(
        this: &IdbDatabase,
        store_names: &::wasm_bindgen::JsValue,
    ) -> Result<IdbTransaction, JsValue>;
    #[cfg(all(feature = "IdbTransaction", feature = "IdbTransactionMode",))]
    # [ wasm_bindgen ( catch , method , structural , js_name = transaction ) ]
    #[doc = "The `transaction()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBDatabase/transaction)\n\n*This API requires the following crate features to be activated: `IdbDatabase`, `IdbTransaction`, `IdbTransactionMode`*"]
    pub fn transaction_with_str_and_mode(
        this: &IdbDatabase,
        store_names: &str,
        mode: IdbTransactionMode,
    ) -> Result<IdbTransaction, JsValue>;
    #[cfg(all(feature = "IdbTransaction", feature = "IdbTransactionMode",))]
    # [ wasm_bindgen ( catch , method , structural , js_name = transaction ) ]
    #[doc = "The `transaction()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBDatabase/transaction)\n\n*This API requires the following crate features to be activated: `IdbDatabase`, `IdbTransaction`, `IdbTransactionMode`*"]
    pub fn transaction_with_str_sequence_and_mode(
        this: &IdbDatabase,
        store_names: &::wasm_bindgen::JsValue,
        mode: IdbTransactionMode,
    ) -> Result<IdbTransaction, JsValue>;
}
