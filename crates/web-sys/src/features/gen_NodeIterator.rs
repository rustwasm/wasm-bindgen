use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = NodeIterator , typescript_name = NodeIterator ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `NodeIterator` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/NodeIterator)\n\n*This API requires the following crate features to be activated: `NodeIterator`*"]
    pub type NodeIterator;
    # [ wasm_bindgen ( structural , method , getter , js_name = root ) ]
    #[cfg(feature = "Node")]
    #[doc = "Getter for the `root` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/NodeIterator/root)\n\n*This API requires the following crate features to be activated: `Node`, `NodeIterator`*"]
    pub fn root(this: &NodeIterator) -> Node;
    # [ wasm_bindgen ( structural , method , getter , js_name = referenceNode ) ]
    #[cfg(feature = "Node")]
    #[doc = "Getter for the `referenceNode` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/NodeIterator/referenceNode)\n\n*This API requires the following crate features to be activated: `Node`, `NodeIterator`*"]
    pub fn reference_node(this: &NodeIterator) -> Option<Node>;
    # [ wasm_bindgen ( structural , method , getter , js_name = pointerBeforeReferenceNode ) ]
    #[doc = "Getter for the `pointerBeforeReferenceNode` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/NodeIterator/pointerBeforeReferenceNode)\n\n*This API requires the following crate features to be activated: `NodeIterator`*"]
    pub fn pointer_before_reference_node(this: &NodeIterator) -> bool;
    # [ wasm_bindgen ( structural , method , getter , js_name = whatToShow ) ]
    #[doc = "Getter for the `whatToShow` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/NodeIterator/whatToShow)\n\n*This API requires the following crate features to be activated: `NodeIterator`*"]
    pub fn what_to_show(this: &NodeIterator) -> u32;
    # [ wasm_bindgen ( structural , method , getter , js_name = filter ) ]
    #[cfg(feature = "NodeFilter")]
    #[doc = "Getter for the `filter` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/NodeIterator/filter)\n\n*This API requires the following crate features to be activated: `NodeFilter`, `NodeIterator`*"]
    pub fn filter(this: &NodeIterator) -> Option<NodeFilter>;
    # [ wasm_bindgen ( method , structural , js_name = detach ) ]
    #[doc = "The `detach()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/NodeIterator/detach)\n\n*This API requires the following crate features to be activated: `NodeIterator`*"]
    pub fn detach(this: &NodeIterator);
    #[cfg(feature = "Node")]
    # [ wasm_bindgen ( catch , method , structural , js_name = nextNode ) ]
    #[doc = "The `nextNode()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/NodeIterator/nextNode)\n\n*This API requires the following crate features to be activated: `Node`, `NodeIterator`*"]
    pub fn next_node(this: &NodeIterator) -> Result<Option<Node>, JsValue>;
    #[cfg(feature = "Node")]
    # [ wasm_bindgen ( catch , method , structural , js_name = previousNode ) ]
    #[doc = "The `previousNode()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/NodeIterator/previousNode)\n\n*This API requires the following crate features to be activated: `Node`, `NodeIterator`*"]
    pub fn previous_node(this: &NodeIterator) -> Result<Option<Node>, JsValue>;
}
