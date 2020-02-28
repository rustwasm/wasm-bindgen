use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = IDBCursor , typescript_name = IDBCursor ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `IdbCursor` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBCursor)\n\n*This API requires the following crate features to be activated: `IdbCursor`*"]
    pub type IdbCursor;
    # [ wasm_bindgen ( structural , method , getter , js_name = source ) ]
    #[doc = "Getter for the `source` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBCursor/source)\n\n*This API requires the following crate features to be activated: `IdbCursor`*"]
    pub fn source(this: &IdbCursor) -> ::js_sys::Object;
    # [ wasm_bindgen ( structural , method , getter , js_name = direction ) ]
    #[cfg(feature = "IdbCursorDirection")]
    #[doc = "Getter for the `direction` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBCursor/direction)\n\n*This API requires the following crate features to be activated: `IdbCursor`, `IdbCursorDirection`*"]
    pub fn direction(this: &IdbCursor) -> IdbCursorDirection;
    # [ wasm_bindgen ( structural , catch , method , getter , js_name = key ) ]
    #[doc = "Getter for the `key` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBCursor/key)\n\n*This API requires the following crate features to be activated: `IdbCursor`*"]
    pub fn key(this: &IdbCursor) -> Result<::wasm_bindgen::JsValue, JsValue>;
    # [ wasm_bindgen ( structural , catch , method , getter , js_name = primaryKey ) ]
    #[doc = "Getter for the `primaryKey` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBCursor/primaryKey)\n\n*This API requires the following crate features to be activated: `IdbCursor`*"]
    pub fn primary_key(this: &IdbCursor) -> Result<::wasm_bindgen::JsValue, JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = advance ) ]
    #[doc = "The `advance()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBCursor/advance)\n\n*This API requires the following crate features to be activated: `IdbCursor`*"]
    pub fn advance(this: &IdbCursor, count: u32) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = continue ) ]
    #[doc = "The `continue()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBCursor/continue)\n\n*This API requires the following crate features to be activated: `IdbCursor`*"]
    pub fn continue_(this: &IdbCursor) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = continue ) ]
    #[doc = "The `continue()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBCursor/continue)\n\n*This API requires the following crate features to be activated: `IdbCursor`*"]
    pub fn continue_with_key(
        this: &IdbCursor,
        key: &::wasm_bindgen::JsValue,
    ) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = continuePrimaryKey ) ]
    #[doc = "The `continuePrimaryKey()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBCursor/continuePrimaryKey)\n\n*This API requires the following crate features to be activated: `IdbCursor`*"]
    pub fn continue_primary_key(
        this: &IdbCursor,
        key: &::wasm_bindgen::JsValue,
        primary_key: &::wasm_bindgen::JsValue,
    ) -> Result<(), JsValue>;
    #[cfg(feature = "IdbRequest")]
    # [ wasm_bindgen ( catch , method , structural , js_name = delete ) ]
    #[doc = "The `delete()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBCursor/delete)\n\n*This API requires the following crate features to be activated: `IdbCursor`, `IdbRequest`*"]
    pub fn delete(this: &IdbCursor) -> Result<IdbRequest, JsValue>;
    #[cfg(feature = "IdbRequest")]
    # [ wasm_bindgen ( catch , method , structural , js_name = update ) ]
    #[doc = "The `update()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBCursor/update)\n\n*This API requires the following crate features to be activated: `IdbCursor`, `IdbRequest`*"]
    pub fn update(this: &IdbCursor, value: &::wasm_bindgen::JsValue)
        -> Result<IdbRequest, JsValue>;
}
