use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = DOMException , typescript_name = DOMException ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `DomException` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMException)
    ///
    ///*This API requires the following crate features to be activated: `DomException`*
    pub type DomException;

    # [ wasm_bindgen ( structural , method , getter , js_class = "DOMException" , js_name = name ) ]
    ///Getter for the `name` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMException/name)
    ///
    ///*This API requires the following crate features to be activated: `DomException`*
    pub fn name(this: &DomException) -> String;

    # [ wasm_bindgen ( structural , method , getter , js_class = "DOMException" , js_name = message ) ]
    ///Getter for the `message` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMException/message)
    ///
    ///*This API requires the following crate features to be activated: `DomException`*
    pub fn message(this: &DomException) -> String;

    # [ wasm_bindgen ( structural , method , getter , js_class = "DOMException" , js_name = code ) ]
    ///Getter for the `code` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMException/code)
    ///
    ///*This API requires the following crate features to be activated: `DomException`*
    pub fn code(this: &DomException) -> u16;

    # [ wasm_bindgen ( structural , method , getter , js_class = "DOMException" , js_name = result ) ]
    ///Getter for the `result` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMException/result)
    ///
    ///*This API requires the following crate features to be activated: `DomException`*
    pub fn result(this: &DomException) -> u32;

    # [ wasm_bindgen ( structural , method , getter , js_class = "DOMException" , js_name = filename ) ]
    ///Getter for the `filename` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMException/filename)
    ///
    ///*This API requires the following crate features to be activated: `DomException`*
    pub fn filename(this: &DomException) -> String;

    # [ wasm_bindgen ( structural , method , getter , js_class = "DOMException" , js_name = lineNumber ) ]
    ///Getter for the `lineNumber` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMException/lineNumber)
    ///
    ///*This API requires the following crate features to be activated: `DomException`*
    pub fn line_number(this: &DomException) -> u32;

    # [ wasm_bindgen ( structural , method , getter , js_class = "DOMException" , js_name = columnNumber ) ]
    ///Getter for the `columnNumber` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMException/columnNumber)
    ///
    ///*This API requires the following crate features to be activated: `DomException`*
    pub fn column_number(this: &DomException) -> u32;

    # [ wasm_bindgen ( structural , method , getter , js_class = "DOMException" , js_name = data ) ]
    ///Getter for the `data` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMException/data)
    ///
    ///*This API requires the following crate features to be activated: `DomException`*
    pub fn data(this: &DomException) -> Option<::js_sys::Object>;

    # [ wasm_bindgen ( structural , method , getter , js_class = "DOMException" , js_name = stack ) ]
    ///Getter for the `stack` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMException/stack)
    ///
    ///*This API requires the following crate features to be activated: `DomException`*
    pub fn stack(this: &DomException) -> String;

    #[wasm_bindgen(catch, constructor, js_class = "DOMException")]
    ///The `new DomException(..)` constructor, creating a new instance of `DomException`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMException/DOMException)
    ///
    ///*This API requires the following crate features to be activated: `DomException`*
    pub fn new() -> Result<DomException, JsValue>;

    #[wasm_bindgen(catch, constructor, js_class = "DOMException")]
    ///The `new DomException(..)` constructor, creating a new instance of `DomException`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMException/DOMException)
    ///
    ///*This API requires the following crate features to be activated: `DomException`*
    pub fn new_with_message(message: &str) -> Result<DomException, JsValue>;

    #[wasm_bindgen(catch, constructor, js_class = "DOMException")]
    ///The `new DomException(..)` constructor, creating a new instance of `DomException`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMException/DOMException)
    ///
    ///*This API requires the following crate features to be activated: `DomException`*
    pub fn new_with_message_and_name(message: &str, name: &str) -> Result<DomException, JsValue>;

}

impl DomException {
    ///The `DOMException.INDEX_SIZE_ERR` const.
    ///
    ///*This API requires the following crate features to be activated: `DomException`*

    pub const INDEX_SIZE_ERR: u16 = 1u64 as u16;

    ///The `DOMException.DOMSTRING_SIZE_ERR` const.
    ///
    ///*This API requires the following crate features to be activated: `DomException`*

    pub const DOMSTRING_SIZE_ERR: u16 = 2u64 as u16;

    ///The `DOMException.HIERARCHY_REQUEST_ERR` const.
    ///
    ///*This API requires the following crate features to be activated: `DomException`*

    pub const HIERARCHY_REQUEST_ERR: u16 = 3u64 as u16;

    ///The `DOMException.WRONG_DOCUMENT_ERR` const.
    ///
    ///*This API requires the following crate features to be activated: `DomException`*

    pub const WRONG_DOCUMENT_ERR: u16 = 4u64 as u16;

    ///The `DOMException.INVALID_CHARACTER_ERR` const.
    ///
    ///*This API requires the following crate features to be activated: `DomException`*

    pub const INVALID_CHARACTER_ERR: u16 = 5u64 as u16;

    ///The `DOMException.NO_DATA_ALLOWED_ERR` const.
    ///
    ///*This API requires the following crate features to be activated: `DomException`*

    pub const NO_DATA_ALLOWED_ERR: u16 = 6u64 as u16;

    ///The `DOMException.NO_MODIFICATION_ALLOWED_ERR` const.
    ///
    ///*This API requires the following crate features to be activated: `DomException`*

    pub const NO_MODIFICATION_ALLOWED_ERR: u16 = 7u64 as u16;

    ///The `DOMException.NOT_FOUND_ERR` const.
    ///
    ///*This API requires the following crate features to be activated: `DomException`*

    pub const NOT_FOUND_ERR: u16 = 8u64 as u16;

    ///The `DOMException.NOT_SUPPORTED_ERR` const.
    ///
    ///*This API requires the following crate features to be activated: `DomException`*

    pub const NOT_SUPPORTED_ERR: u16 = 9u64 as u16;

    ///The `DOMException.INUSE_ATTRIBUTE_ERR` const.
    ///
    ///*This API requires the following crate features to be activated: `DomException`*

    pub const INUSE_ATTRIBUTE_ERR: u16 = 10u64 as u16;

    ///The `DOMException.INVALID_STATE_ERR` const.
    ///
    ///*This API requires the following crate features to be activated: `DomException`*

    pub const INVALID_STATE_ERR: u16 = 11u64 as u16;

    ///The `DOMException.SYNTAX_ERR` const.
    ///
    ///*This API requires the following crate features to be activated: `DomException`*

    pub const SYNTAX_ERR: u16 = 12u64 as u16;

    ///The `DOMException.INVALID_MODIFICATION_ERR` const.
    ///
    ///*This API requires the following crate features to be activated: `DomException`*

    pub const INVALID_MODIFICATION_ERR: u16 = 13u64 as u16;

    ///The `DOMException.NAMESPACE_ERR` const.
    ///
    ///*This API requires the following crate features to be activated: `DomException`*

    pub const NAMESPACE_ERR: u16 = 14u64 as u16;

    ///The `DOMException.INVALID_ACCESS_ERR` const.
    ///
    ///*This API requires the following crate features to be activated: `DomException`*

    pub const INVALID_ACCESS_ERR: u16 = 15u64 as u16;

    ///The `DOMException.VALIDATION_ERR` const.
    ///
    ///*This API requires the following crate features to be activated: `DomException`*

    pub const VALIDATION_ERR: u16 = 16u64 as u16;

    ///The `DOMException.TYPE_MISMATCH_ERR` const.
    ///
    ///*This API requires the following crate features to be activated: `DomException`*

    pub const TYPE_MISMATCH_ERR: u16 = 17u64 as u16;

    ///The `DOMException.SECURITY_ERR` const.
    ///
    ///*This API requires the following crate features to be activated: `DomException`*

    pub const SECURITY_ERR: u16 = 18u64 as u16;

    ///The `DOMException.NETWORK_ERR` const.
    ///
    ///*This API requires the following crate features to be activated: `DomException`*

    pub const NETWORK_ERR: u16 = 19u64 as u16;

    ///The `DOMException.ABORT_ERR` const.
    ///
    ///*This API requires the following crate features to be activated: `DomException`*

    pub const ABORT_ERR: u16 = 20u64 as u16;

    ///The `DOMException.URL_MISMATCH_ERR` const.
    ///
    ///*This API requires the following crate features to be activated: `DomException`*

    pub const URL_MISMATCH_ERR: u16 = 21u64 as u16;

    ///The `DOMException.QUOTA_EXCEEDED_ERR` const.
    ///
    ///*This API requires the following crate features to be activated: `DomException`*

    pub const QUOTA_EXCEEDED_ERR: u16 = 22u64 as u16;

    ///The `DOMException.TIMEOUT_ERR` const.
    ///
    ///*This API requires the following crate features to be activated: `DomException`*

    pub const TIMEOUT_ERR: u16 = 23u64 as u16;

    ///The `DOMException.INVALID_NODE_TYPE_ERR` const.
    ///
    ///*This API requires the following crate features to be activated: `DomException`*

    pub const INVALID_NODE_TYPE_ERR: u16 = 24u64 as u16;

    ///The `DOMException.DATA_CLONE_ERR` const.
    ///
    ///*This API requires the following crate features to be activated: `DomException`*

    pub const DATA_CLONE_ERR: u16 = 25u64 as u16;
}
