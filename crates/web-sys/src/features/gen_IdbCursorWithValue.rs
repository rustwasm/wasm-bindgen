use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = IdbCursor , extends = :: js_sys :: Object , js_name = IDBCursorWithValue , typescript_name = IDBCursorWithValue ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `IdbCursorWithValue` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBCursorWithValue)\n\n*This API requires the following crate features to be activated: `IdbCursorWithValue`*"]
    pub type IdbCursorWithValue;
    # [ wasm_bindgen ( structural , catch , method , getter , js_name = value ) ]
    #[doc = "Getter for the `value` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBCursorWithValue/value)\n\n*This API requires the following crate features to be activated: `IdbCursorWithValue`*"]
    pub fn value(this: &IdbCursorWithValue) -> Result<::wasm_bindgen::JsValue, JsValue>;
}
