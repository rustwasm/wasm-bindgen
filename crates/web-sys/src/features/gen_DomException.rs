use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = DOMException , typescript_name = DOMException ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `DomException` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMException)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomException`*"]
    pub type DomException;
    # [ wasm_bindgen ( structural , method , getter , js_class = "DOMException" , js_name = name ) ]
    #[doc = "Getter for the `name` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMException/name)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomException`*"]
    pub fn name(this: &DomException) -> String;
    # [ wasm_bindgen ( structural , method , getter , js_class = "DOMException" , js_name = message ) ]
    #[doc = "Getter for the `message` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMException/message)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomException`*"]
    pub fn message(this: &DomException) -> String;
    # [ wasm_bindgen ( structural , method , getter , js_class = "DOMException" , js_name = code ) ]
    #[doc = "Getter for the `code` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMException/code)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomException`*"]
    pub fn code(this: &DomException) -> u16;
    # [ wasm_bindgen ( structural , method , getter , js_class = "DOMException" , js_name = result ) ]
    #[doc = "Getter for the `result` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMException/result)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomException`*"]
    pub fn result(this: &DomException) -> u32;
    # [ wasm_bindgen ( structural , method , getter , js_class = "DOMException" , js_name = filename ) ]
    #[doc = "Getter for the `filename` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMException/filename)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomException`*"]
    pub fn filename(this: &DomException) -> String;
    # [ wasm_bindgen ( structural , method , getter , js_class = "DOMException" , js_name = lineNumber ) ]
    #[doc = "Getter for the `lineNumber` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMException/lineNumber)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomException`*"]
    pub fn line_number(this: &DomException) -> u32;
    # [ wasm_bindgen ( structural , method , getter , js_class = "DOMException" , js_name = columnNumber ) ]
    #[doc = "Getter for the `columnNumber` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMException/columnNumber)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomException`*"]
    pub fn column_number(this: &DomException) -> u32;
    # [ wasm_bindgen ( structural , method , getter , js_class = "DOMException" , js_name = data ) ]
    #[doc = "Getter for the `data` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMException/data)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomException`*"]
    pub fn data(this: &DomException) -> Option<::js_sys::Object>;
    # [ wasm_bindgen ( structural , method , getter , js_class = "DOMException" , js_name = stack ) ]
    #[doc = "Getter for the `stack` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMException/stack)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomException`*"]
    pub fn stack(this: &DomException) -> String;
    #[wasm_bindgen(catch, js_class = "DOMException", constructor)]
    #[doc = "The `new DomException(..)` constructor, creating a new instance of `DomException`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMException/DOMException)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomException`*"]
    pub fn new(this: &DomException) -> Result<DomException, JsValue>;
    #[wasm_bindgen(catch, js_class = "DOMException", constructor)]
    #[doc = "The `new DomException(..)` constructor, creating a new instance of `DomException`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMException/DOMException)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomException`*"]
    pub fn new_with_message(this: &DomException, message: &str) -> Result<DomException, JsValue>;
    #[wasm_bindgen(catch, js_class = "DOMException", constructor)]
    #[doc = "The `new DomException(..)` constructor, creating a new instance of `DomException`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMException/DOMException)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomException`*"]
    pub fn new_with_message_and_name(
        this: &DomException,
        message: &str,
        name: &str,
    ) -> Result<DomException, JsValue>;
}
impl DomException {
    pub const INDEX_SIZE_ERR: u16 = 1u64 as u16;
    pub const DOMSTRING_SIZE_ERR: u16 = 2u64 as u16;
    pub const HIERARCHY_REQUEST_ERR: u16 = 3u64 as u16;
    pub const WRONG_DOCUMENT_ERR: u16 = 4u64 as u16;
    pub const INVALID_CHARACTER_ERR: u16 = 5u64 as u16;
    pub const NO_DATA_ALLOWED_ERR: u16 = 6u64 as u16;
    pub const NO_MODIFICATION_ALLOWED_ERR: u16 = 7u64 as u16;
    pub const NOT_FOUND_ERR: u16 = 8u64 as u16;
    pub const NOT_SUPPORTED_ERR: u16 = 9u64 as u16;
    pub const INUSE_ATTRIBUTE_ERR: u16 = 10u64 as u16;
    pub const INVALID_STATE_ERR: u16 = 11u64 as u16;
    pub const SYNTAX_ERR: u16 = 12u64 as u16;
    pub const INVALID_MODIFICATION_ERR: u16 = 13u64 as u16;
    pub const NAMESPACE_ERR: u16 = 14u64 as u16;
    pub const INVALID_ACCESS_ERR: u16 = 15u64 as u16;
    pub const VALIDATION_ERR: u16 = 16u64 as u16;
    pub const TYPE_MISMATCH_ERR: u16 = 17u64 as u16;
    pub const SECURITY_ERR: u16 = 18u64 as u16;
    pub const NETWORK_ERR: u16 = 19u64 as u16;
    pub const ABORT_ERR: u16 = 20u64 as u16;
    pub const URL_MISMATCH_ERR: u16 = 21u64 as u16;
    pub const QUOTA_EXCEEDED_ERR: u16 = 22u64 as u16;
    pub const TIMEOUT_ERR: u16 = 23u64 as u16;
    pub const INVALID_NODE_TYPE_ERR: u16 = 24u64 as u16;
    pub const DATA_CLONE_ERR: u16 = 25u64 as u16;
}
