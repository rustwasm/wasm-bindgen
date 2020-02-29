use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = XPathResult , typescript_name = XPathResult ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `XPathResult` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XPathResult)
    ///
    ///*This API requires the following crate features to be activated: `XPathResult`*
    pub type XPathResult;

    # [ wasm_bindgen ( structural , method , getter , js_class = "XPathResult" , js_name = resultType ) ]
    ///Getter for the `resultType` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XPathResult/resultType)
    ///
    ///*This API requires the following crate features to be activated: `XPathResult`*
    pub fn result_type(this: &XPathResult) -> u16;

    # [ wasm_bindgen ( structural , catch , method , getter , js_class = "XPathResult" , js_name = numberValue ) ]
    ///Getter for the `numberValue` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XPathResult/numberValue)
    ///
    ///*This API requires the following crate features to be activated: `XPathResult`*
    pub fn number_value(this: &XPathResult) -> Result<f64, JsValue>;

    # [ wasm_bindgen ( structural , catch , method , getter , js_class = "XPathResult" , js_name = stringValue ) ]
    ///Getter for the `stringValue` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XPathResult/stringValue)
    ///
    ///*This API requires the following crate features to be activated: `XPathResult`*
    pub fn string_value(this: &XPathResult) -> Result<String, JsValue>;

    # [ wasm_bindgen ( structural , catch , method , getter , js_class = "XPathResult" , js_name = booleanValue ) ]
    ///Getter for the `booleanValue` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XPathResult/booleanValue)
    ///
    ///*This API requires the following crate features to be activated: `XPathResult`*
    pub fn boolean_value(this: &XPathResult) -> Result<bool, JsValue>;

    #[cfg(feature = "Node")]
    # [ wasm_bindgen ( structural , catch , method , getter , js_class = "XPathResult" , js_name = singleNodeValue ) ]
    ///Getter for the `singleNodeValue` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XPathResult/singleNodeValue)
    ///
    ///*This API requires the following crate features to be activated: `Node`, `XPathResult`*
    pub fn single_node_value(this: &XPathResult) -> Result<Option<Node>, JsValue>;

    # [ wasm_bindgen ( structural , method , getter , js_class = "XPathResult" , js_name = invalidIteratorState ) ]
    ///Getter for the `invalidIteratorState` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XPathResult/invalidIteratorState)
    ///
    ///*This API requires the following crate features to be activated: `XPathResult`*
    pub fn invalid_iterator_state(this: &XPathResult) -> bool;

    # [ wasm_bindgen ( structural , catch , method , getter , js_class = "XPathResult" , js_name = snapshotLength ) ]
    ///Getter for the `snapshotLength` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XPathResult/snapshotLength)
    ///
    ///*This API requires the following crate features to be activated: `XPathResult`*
    pub fn snapshot_length(this: &XPathResult) -> Result<u32, JsValue>;

    #[cfg(feature = "Node")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "XPathResult" , js_name = iterateNext ) ]
    ///The `iterateNext()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XPathResult/iterateNext)
    ///
    ///*This API requires the following crate features to be activated: `Node`, `XPathResult`*
    pub fn iterate_next(this: &XPathResult) -> Result<Option<Node>, JsValue>;

    #[cfg(feature = "Node")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "XPathResult" , js_name = snapshotItem ) ]
    ///The `snapshotItem()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XPathResult/snapshotItem)
    ///
    ///*This API requires the following crate features to be activated: `Node`, `XPathResult`*
    pub fn snapshot_item(this: &XPathResult, index: u32) -> Result<Option<Node>, JsValue>;

}

impl XPathResult {
    ///The `XPathResult.ANY_TYPE` const.
    ///
    ///*This API requires the following crate features to be activated: `XPathResult`*

    pub const ANY_TYPE: u16 = 0i64 as u16;

    ///The `XPathResult.NUMBER_TYPE` const.
    ///
    ///*This API requires the following crate features to be activated: `XPathResult`*

    pub const NUMBER_TYPE: u16 = 1u64 as u16;

    ///The `XPathResult.STRING_TYPE` const.
    ///
    ///*This API requires the following crate features to be activated: `XPathResult`*

    pub const STRING_TYPE: u16 = 2u64 as u16;

    ///The `XPathResult.BOOLEAN_TYPE` const.
    ///
    ///*This API requires the following crate features to be activated: `XPathResult`*

    pub const BOOLEAN_TYPE: u16 = 3u64 as u16;

    ///The `XPathResult.UNORDERED_NODE_ITERATOR_TYPE` const.
    ///
    ///*This API requires the following crate features to be activated: `XPathResult`*

    pub const UNORDERED_NODE_ITERATOR_TYPE: u16 = 4u64 as u16;

    ///The `XPathResult.ORDERED_NODE_ITERATOR_TYPE` const.
    ///
    ///*This API requires the following crate features to be activated: `XPathResult`*

    pub const ORDERED_NODE_ITERATOR_TYPE: u16 = 5u64 as u16;

    ///The `XPathResult.UNORDERED_NODE_SNAPSHOT_TYPE` const.
    ///
    ///*This API requires the following crate features to be activated: `XPathResult`*

    pub const UNORDERED_NODE_SNAPSHOT_TYPE: u16 = 6u64 as u16;

    ///The `XPathResult.ORDERED_NODE_SNAPSHOT_TYPE` const.
    ///
    ///*This API requires the following crate features to be activated: `XPathResult`*

    pub const ORDERED_NODE_SNAPSHOT_TYPE: u16 = 7u64 as u16;

    ///The `XPathResult.ANY_UNORDERED_NODE_TYPE` const.
    ///
    ///*This API requires the following crate features to be activated: `XPathResult`*

    pub const ANY_UNORDERED_NODE_TYPE: u16 = 8u64 as u16;

    ///The `XPathResult.FIRST_ORDERED_NODE_TYPE` const.
    ///
    ///*This API requires the following crate features to be activated: `XPathResult`*

    pub const FIRST_ORDERED_NODE_TYPE: u16 = 9u64 as u16;
}
