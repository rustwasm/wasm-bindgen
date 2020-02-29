use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = NodeIterator , typescript_type = "NodeIterator" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `NodeIterator` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/NodeIterator)
    ///
    ///*This API requires the following crate features to be activated: `NodeIterator`*
    pub type NodeIterator;

    #[cfg(feature = "Node")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "NodeIterator" , js_name = root ) ]
    ///Getter for the `root` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/NodeIterator/root)
    ///
    ///*This API requires the following crate features to be activated: `Node`, `NodeIterator`*
    pub fn root(this: &NodeIterator) -> Node;

    #[cfg(feature = "Node")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "NodeIterator" , js_name = referenceNode ) ]
    ///Getter for the `referenceNode` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/NodeIterator/referenceNode)
    ///
    ///*This API requires the following crate features to be activated: `Node`, `NodeIterator`*
    pub fn reference_node(this: &NodeIterator) -> Option<Node>;

    # [ wasm_bindgen ( structural , method , getter , js_class = "NodeIterator" , js_name = pointerBeforeReferenceNode ) ]
    ///Getter for the `pointerBeforeReferenceNode` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/NodeIterator/pointerBeforeReferenceNode)
    ///
    ///*This API requires the following crate features to be activated: `NodeIterator`*
    pub fn pointer_before_reference_node(this: &NodeIterator) -> bool;

    # [ wasm_bindgen ( structural , method , getter , js_class = "NodeIterator" , js_name = whatToShow ) ]
    ///Getter for the `whatToShow` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/NodeIterator/whatToShow)
    ///
    ///*This API requires the following crate features to be activated: `NodeIterator`*
    pub fn what_to_show(this: &NodeIterator) -> u32;

    #[cfg(feature = "NodeFilter")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "NodeIterator" , js_name = filter ) ]
    ///Getter for the `filter` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/NodeIterator/filter)
    ///
    ///*This API requires the following crate features to be activated: `NodeFilter`, `NodeIterator`*
    pub fn filter(this: &NodeIterator) -> Option<NodeFilter>;

    # [ wasm_bindgen ( method , structural , js_class = "NodeIterator" , js_name = detach ) ]
    ///The `detach()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/NodeIterator/detach)
    ///
    ///*This API requires the following crate features to be activated: `NodeIterator`*
    pub fn detach(this: &NodeIterator);

    #[cfg(feature = "Node")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "NodeIterator" , js_name = nextNode ) ]
    ///The `nextNode()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/NodeIterator/nextNode)
    ///
    ///*This API requires the following crate features to be activated: `Node`, `NodeIterator`*
    pub fn next_node(this: &NodeIterator) -> Result<Option<Node>, JsValue>;

    #[cfg(feature = "Node")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "NodeIterator" , js_name = previousNode ) ]
    ///The `previousNode()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/NodeIterator/previousNode)
    ///
    ///*This API requires the following crate features to be activated: `Node`, `NodeIterator`*
    pub fn previous_node(this: &NodeIterator) -> Result<Option<Node>, JsValue>;

}
