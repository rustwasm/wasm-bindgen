use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = IdbCursor , extends = :: js_sys :: Object , js_name = IDBCursorWithValue , typescript_name = IDBCursorWithValue ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `IdbCursorWithValue` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBCursorWithValue)
    ///
    ///*This API requires the following crate features to be activated: `IdbCursorWithValue`*
    pub type IdbCursorWithValue;

    # [ wasm_bindgen ( structural , catch , method , getter , js_class = "IDBCursorWithValue" , js_name = value ) ]
    ///Getter for the `value` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBCursorWithValue/value)
    ///
    ///*This API requires the following crate features to be activated: `IdbCursorWithValue`*
    pub fn value(this: &IdbCursorWithValue) -> Result<::wasm_bindgen::JsValue, JsValue>;

}
