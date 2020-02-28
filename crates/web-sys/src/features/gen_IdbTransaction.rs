use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = EventTarget , extends = :: js_sys :: Object , js_name = IDBTransaction , typescript_name = IDBTransaction ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `IdbTransaction` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBTransaction)\n\n*This API requires the following crate features to be activated: `IdbTransaction`*"]
    pub type IdbTransaction;
    # [ wasm_bindgen ( structural , catch , method , getter , js_name = mode ) ]
    #[cfg(feature = "IdbTransactionMode")]
    #[doc = "Getter for the `mode` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBTransaction/mode)\n\n*This API requires the following crate features to be activated: `IdbTransaction`, `IdbTransactionMode`*"]
    pub fn mode(this: &IdbTransaction) -> Result<IdbTransactionMode, JsValue>;
    # [ wasm_bindgen ( structural , method , getter , js_name = db ) ]
    #[cfg(feature = "IdbDatabase")]
    #[doc = "Getter for the `db` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBTransaction/db)\n\n*This API requires the following crate features to be activated: `IdbDatabase`, `IdbTransaction`*"]
    pub fn db(this: &IdbTransaction) -> IdbDatabase;
    # [ wasm_bindgen ( structural , method , getter , js_name = error ) ]
    #[cfg(feature = "DomException")]
    #[doc = "Getter for the `error` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBTransaction/error)\n\n*This API requires the following crate features to be activated: `DomException`, `IdbTransaction`*"]
    pub fn error(this: &IdbTransaction) -> Option<DomException>;
    # [ wasm_bindgen ( structural , method , getter , js_name = onabort ) ]
    #[doc = "Getter for the `onabort` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBTransaction/onabort)\n\n*This API requires the following crate features to be activated: `IdbTransaction`*"]
    pub fn onabort(this: &IdbTransaction) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onabort ) ]
    #[doc = "Setter for the `onabort` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBTransaction/onabort)\n\n*This API requires the following crate features to be activated: `IdbTransaction`*"]
    pub fn set_onabort(this: &IdbTransaction, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = oncomplete ) ]
    #[doc = "Getter for the `oncomplete` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBTransaction/oncomplete)\n\n*This API requires the following crate features to be activated: `IdbTransaction`*"]
    pub fn oncomplete(this: &IdbTransaction) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = oncomplete ) ]
    #[doc = "Setter for the `oncomplete` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBTransaction/oncomplete)\n\n*This API requires the following crate features to be activated: `IdbTransaction`*"]
    pub fn set_oncomplete(this: &IdbTransaction, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = onerror ) ]
    #[doc = "Getter for the `onerror` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBTransaction/onerror)\n\n*This API requires the following crate features to be activated: `IdbTransaction`*"]
    pub fn onerror(this: &IdbTransaction) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onerror ) ]
    #[doc = "Setter for the `onerror` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBTransaction/onerror)\n\n*This API requires the following crate features to be activated: `IdbTransaction`*"]
    pub fn set_onerror(this: &IdbTransaction, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = objectStoreNames ) ]
    #[cfg(feature = "DomStringList")]
    #[doc = "Getter for the `objectStoreNames` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBTransaction/objectStoreNames)\n\n*This API requires the following crate features to be activated: `DomStringList`, `IdbTransaction`*"]
    pub fn object_store_names(this: &IdbTransaction) -> DomStringList;
    # [ wasm_bindgen ( catch , method , structural , js_name = abort ) ]
    #[doc = "The `abort()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBTransaction/abort)\n\n*This API requires the following crate features to be activated: `IdbTransaction`*"]
    pub fn abort(this: &IdbTransaction) -> Result<(), JsValue>;
    #[cfg(feature = "IdbObjectStore")]
    # [ wasm_bindgen ( catch , method , structural , js_name = objectStore ) ]
    #[doc = "The `objectStore()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBTransaction/objectStore)\n\n*This API requires the following crate features to be activated: `IdbObjectStore`, `IdbTransaction`*"]
    pub fn object_store(this: &IdbTransaction, name: &str) -> Result<IdbObjectStore, JsValue>;
}
