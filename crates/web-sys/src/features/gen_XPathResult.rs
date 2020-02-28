use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = XPathResult , typescript_name = XPathResult ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `XPathResult` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XPathResult)\n\n*This API requires the following crate features to be activated: `XPathResult`*"]
    pub type XPathResult;
    # [ wasm_bindgen ( structural , method , getter , js_class = "XPathResult" , js_name = resultType ) ]
    #[doc = "Getter for the `resultType` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XPathResult/resultType)\n\n*This API requires the following crate features to be activated: `XPathResult`*"]
    pub fn result_type(this: &XPathResult) -> u16;
    # [ wasm_bindgen ( structural , catch , method , getter , js_class = "XPathResult" , js_name = numberValue ) ]
    #[doc = "Getter for the `numberValue` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XPathResult/numberValue)\n\n*This API requires the following crate features to be activated: `XPathResult`*"]
    pub fn number_value(this: &XPathResult) -> Result<f64, JsValue>;
    # [ wasm_bindgen ( structural , catch , method , getter , js_class = "XPathResult" , js_name = stringValue ) ]
    #[doc = "Getter for the `stringValue` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XPathResult/stringValue)\n\n*This API requires the following crate features to be activated: `XPathResult`*"]
    pub fn string_value(this: &XPathResult) -> Result<String, JsValue>;
    # [ wasm_bindgen ( structural , catch , method , getter , js_class = "XPathResult" , js_name = booleanValue ) ]
    #[doc = "Getter for the `booleanValue` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XPathResult/booleanValue)\n\n*This API requires the following crate features to be activated: `XPathResult`*"]
    pub fn boolean_value(this: &XPathResult) -> Result<bool, JsValue>;
    # [ wasm_bindgen ( structural , catch , method , getter , js_class = "XPathResult" , js_name = singleNodeValue ) ]
    #[cfg(feature = "Node")]
    #[doc = "Getter for the `singleNodeValue` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XPathResult/singleNodeValue)\n\n*This API requires the following crate features to be activated: `Node`, `XPathResult`*"]
    pub fn single_node_value(this: &XPathResult) -> Result<Option<Node>, JsValue>;
    # [ wasm_bindgen ( structural , method , getter , js_class = "XPathResult" , js_name = invalidIteratorState ) ]
    #[doc = "Getter for the `invalidIteratorState` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XPathResult/invalidIteratorState)\n\n*This API requires the following crate features to be activated: `XPathResult`*"]
    pub fn invalid_iterator_state(this: &XPathResult) -> bool;
    # [ wasm_bindgen ( structural , catch , method , getter , js_class = "XPathResult" , js_name = snapshotLength ) ]
    #[doc = "Getter for the `snapshotLength` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XPathResult/snapshotLength)\n\n*This API requires the following crate features to be activated: `XPathResult`*"]
    pub fn snapshot_length(this: &XPathResult) -> Result<u32, JsValue>;
    #[cfg(feature = "Node")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "XPathResult" , js_name = iterateNext ) ]
    #[doc = "The `iterateNext()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XPathResult/iterateNext)\n\n*This API requires the following crate features to be activated: `Node`, `XPathResult`*"]
    pub fn iterate_next(this: &XPathResult) -> Result<Option<Node>, JsValue>;
    #[cfg(feature = "Node")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "XPathResult" , js_name = snapshotItem ) ]
    #[doc = "The `snapshotItem()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XPathResult/snapshotItem)\n\n*This API requires the following crate features to be activated: `Node`, `XPathResult`*"]
    pub fn snapshot_item(this: &XPathResult, index: u32) -> Result<Option<Node>, JsValue>;
}
impl XPathResult {
    pub const ANY_TYPE: u16 = 0i64 as u16;
    pub const NUMBER_TYPE: u16 = 1u64 as u16;
    pub const STRING_TYPE: u16 = 2u64 as u16;
    pub const BOOLEAN_TYPE: u16 = 3u64 as u16;
    pub const UNORDERED_NODE_ITERATOR_TYPE: u16 = 4u64 as u16;
    pub const ORDERED_NODE_ITERATOR_TYPE: u16 = 5u64 as u16;
    pub const UNORDERED_NODE_SNAPSHOT_TYPE: u16 = 6u64 as u16;
    pub const ORDERED_NODE_SNAPSHOT_TYPE: u16 = 7u64 as u16;
    pub const ANY_UNORDERED_NODE_TYPE: u16 = 8u64 as u16;
    pub const FIRST_ORDERED_NODE_TYPE: u16 = 9u64 as u16;
}
