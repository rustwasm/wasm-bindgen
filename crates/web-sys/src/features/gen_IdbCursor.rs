use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = IDBCursor , typescript_name = IDBCursor ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `IdbCursor` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBCursor)
    ///
    ///*This API requires the following crate features to be activated: `IdbCursor`*
    pub type IdbCursor;

    # [ wasm_bindgen ( structural , method , getter , js_class = "IDBCursor" , js_name = source ) ]
    ///Getter for the `source` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBCursor/source)
    ///
    ///*This API requires the following crate features to be activated: `IdbCursor`*
    pub fn source(this: &IdbCursor) -> ::js_sys::Object;

    #[cfg(feature = "IdbCursorDirection")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "IDBCursor" , js_name = direction ) ]
    ///Getter for the `direction` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBCursor/direction)
    ///
    ///*This API requires the following crate features to be activated: `IdbCursor`, `IdbCursorDirection`*
    pub fn direction(this: &IdbCursor) -> IdbCursorDirection;

    # [ wasm_bindgen ( structural , catch , method , getter , js_class = "IDBCursor" , js_name = key ) ]
    ///Getter for the `key` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBCursor/key)
    ///
    ///*This API requires the following crate features to be activated: `IdbCursor`*
    pub fn key(this: &IdbCursor) -> Result<::wasm_bindgen::JsValue, JsValue>;

    # [ wasm_bindgen ( structural , catch , method , getter , js_class = "IDBCursor" , js_name = primaryKey ) ]
    ///Getter for the `primaryKey` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBCursor/primaryKey)
    ///
    ///*This API requires the following crate features to be activated: `IdbCursor`*
    pub fn primary_key(this: &IdbCursor) -> Result<::wasm_bindgen::JsValue, JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "IDBCursor" , js_name = advance ) ]
    ///The `advance()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBCursor/advance)
    ///
    ///*This API requires the following crate features to be activated: `IdbCursor`*
    pub fn advance(this: &IdbCursor, count: u32) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "IDBCursor" , js_name = continue ) ]
    ///The `continue()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBCursor/continue)
    ///
    ///*This API requires the following crate features to be activated: `IdbCursor`*
    pub fn continue_(this: &IdbCursor) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "IDBCursor" , js_name = continue ) ]
    ///The `continue()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBCursor/continue)
    ///
    ///*This API requires the following crate features to be activated: `IdbCursor`*
    pub fn continue_with_key(
        this: &IdbCursor,
        key: &::wasm_bindgen::JsValue,
    ) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "IDBCursor" , js_name = continuePrimaryKey ) ]
    ///The `continuePrimaryKey()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBCursor/continuePrimaryKey)
    ///
    ///*This API requires the following crate features to be activated: `IdbCursor`*
    pub fn continue_primary_key(
        this: &IdbCursor,
        key: &::wasm_bindgen::JsValue,
        primary_key: &::wasm_bindgen::JsValue,
    ) -> Result<(), JsValue>;

    #[cfg(feature = "IdbRequest")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "IDBCursor" , js_name = delete ) ]
    ///The `delete()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBCursor/delete)
    ///
    ///*This API requires the following crate features to be activated: `IdbCursor`, `IdbRequest`*
    pub fn delete(this: &IdbCursor) -> Result<IdbRequest, JsValue>;

    #[cfg(feature = "IdbRequest")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "IDBCursor" , js_name = update ) ]
    ///The `update()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBCursor/update)
    ///
    ///*This API requires the following crate features to be activated: `IdbCursor`, `IdbRequest`*
    pub fn update(this: &IdbCursor, value: &::wasm_bindgen::JsValue)
        -> Result<IdbRequest, JsValue>;

}
