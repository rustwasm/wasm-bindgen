use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = TreeWalker , typescript_name = TreeWalker ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `TreeWalker` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeWalker)\n\n*This API requires the following crate features to be activated: `TreeWalker`*"]
    pub type TreeWalker;
    # [ wasm_bindgen ( structural , method , getter , js_name = root ) ]
    #[cfg(feature = "Node")]
    #[doc = "Getter for the `root` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeWalker/root)\n\n*This API requires the following crate features to be activated: `Node`, `TreeWalker`*"]
    pub fn root(this: &TreeWalker) -> Node;
    # [ wasm_bindgen ( structural , method , getter , js_name = whatToShow ) ]
    #[doc = "Getter for the `whatToShow` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeWalker/whatToShow)\n\n*This API requires the following crate features to be activated: `TreeWalker`*"]
    pub fn what_to_show(this: &TreeWalker) -> u32;
    # [ wasm_bindgen ( structural , method , getter , js_name = filter ) ]
    #[cfg(feature = "NodeFilter")]
    #[doc = "Getter for the `filter` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeWalker/filter)\n\n*This API requires the following crate features to be activated: `NodeFilter`, `TreeWalker`*"]
    pub fn filter(this: &TreeWalker) -> Option<NodeFilter>;
    # [ wasm_bindgen ( structural , method , getter , js_name = currentNode ) ]
    #[cfg(feature = "Node")]
    #[doc = "Getter for the `currentNode` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeWalker/currentNode)\n\n*This API requires the following crate features to be activated: `Node`, `TreeWalker`*"]
    pub fn current_node(this: &TreeWalker) -> Node;
    # [ wasm_bindgen ( structural , method , setter , js_name = currentNode ) ]
    #[cfg(feature = "Node")]
    #[doc = "Setter for the `currentNode` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeWalker/currentNode)\n\n*This API requires the following crate features to be activated: `Node`, `TreeWalker`*"]
    pub fn set_current_node(this: &TreeWalker, value: Node);
    #[cfg(feature = "Node")]
    # [ wasm_bindgen ( catch , method , structural , js_name = firstChild ) ]
    #[doc = "The `firstChild()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeWalker/firstChild)\n\n*This API requires the following crate features to be activated: `Node`, `TreeWalker`*"]
    pub fn first_child(this: &TreeWalker) -> Result<Option<Node>, JsValue>;
    #[cfg(feature = "Node")]
    # [ wasm_bindgen ( catch , method , structural , js_name = lastChild ) ]
    #[doc = "The `lastChild()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeWalker/lastChild)\n\n*This API requires the following crate features to be activated: `Node`, `TreeWalker`*"]
    pub fn last_child(this: &TreeWalker) -> Result<Option<Node>, JsValue>;
    #[cfg(feature = "Node")]
    # [ wasm_bindgen ( catch , method , structural , js_name = nextNode ) ]
    #[doc = "The `nextNode()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeWalker/nextNode)\n\n*This API requires the following crate features to be activated: `Node`, `TreeWalker`*"]
    pub fn next_node(this: &TreeWalker) -> Result<Option<Node>, JsValue>;
    #[cfg(feature = "Node")]
    # [ wasm_bindgen ( catch , method , structural , js_name = nextSibling ) ]
    #[doc = "The `nextSibling()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeWalker/nextSibling)\n\n*This API requires the following crate features to be activated: `Node`, `TreeWalker`*"]
    pub fn next_sibling(this: &TreeWalker) -> Result<Option<Node>, JsValue>;
    #[cfg(feature = "Node")]
    # [ wasm_bindgen ( catch , method , structural , js_name = parentNode ) ]
    #[doc = "The `parentNode()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeWalker/parentNode)\n\n*This API requires the following crate features to be activated: `Node`, `TreeWalker`*"]
    pub fn parent_node(this: &TreeWalker) -> Result<Option<Node>, JsValue>;
    #[cfg(feature = "Node")]
    # [ wasm_bindgen ( catch , method , structural , js_name = previousNode ) ]
    #[doc = "The `previousNode()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeWalker/previousNode)\n\n*This API requires the following crate features to be activated: `Node`, `TreeWalker`*"]
    pub fn previous_node(this: &TreeWalker) -> Result<Option<Node>, JsValue>;
    #[cfg(feature = "Node")]
    # [ wasm_bindgen ( catch , method , structural , js_name = previousSibling ) ]
    #[doc = "The `previousSibling()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeWalker/previousSibling)\n\n*This API requires the following crate features to be activated: `Node`, `TreeWalker`*"]
    pub fn previous_sibling(this: &TreeWalker) -> Result<Option<Node>, JsValue>;
}
