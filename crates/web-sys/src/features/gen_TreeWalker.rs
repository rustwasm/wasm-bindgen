use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = TreeWalker , typescript_name = TreeWalker ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `TreeWalker` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeWalker)
    ///
    ///*This API requires the following crate features to be activated: `TreeWalker`*
    pub type TreeWalker;

    #[cfg(feature = "Node")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "TreeWalker" , js_name = root ) ]
    ///Getter for the `root` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeWalker/root)
    ///
    ///*This API requires the following crate features to be activated: `Node`, `TreeWalker`*
    pub fn root(this: &TreeWalker) -> Node;

    # [ wasm_bindgen ( structural , method , getter , js_class = "TreeWalker" , js_name = whatToShow ) ]
    ///Getter for the `whatToShow` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeWalker/whatToShow)
    ///
    ///*This API requires the following crate features to be activated: `TreeWalker`*
    pub fn what_to_show(this: &TreeWalker) -> u32;

    #[cfg(feature = "NodeFilter")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "TreeWalker" , js_name = filter ) ]
    ///Getter for the `filter` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeWalker/filter)
    ///
    ///*This API requires the following crate features to be activated: `NodeFilter`, `TreeWalker`*
    pub fn filter(this: &TreeWalker) -> Option<NodeFilter>;

    #[cfg(feature = "Node")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "TreeWalker" , js_name = currentNode ) ]
    ///Getter for the `currentNode` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeWalker/currentNode)
    ///
    ///*This API requires the following crate features to be activated: `Node`, `TreeWalker`*
    pub fn current_node(this: &TreeWalker) -> Node;

    #[cfg(feature = "Node")]
    # [ wasm_bindgen ( structural , method , setter , js_class = "TreeWalker" , js_name = currentNode ) ]
    ///Setter for the `currentNode` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeWalker/currentNode)
    ///
    ///*This API requires the following crate features to be activated: `Node`, `TreeWalker`*
    pub fn set_current_node(this: &TreeWalker, value: &Node);

    #[cfg(feature = "Node")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "TreeWalker" , js_name = firstChild ) ]
    ///The `firstChild()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeWalker/firstChild)
    ///
    ///*This API requires the following crate features to be activated: `Node`, `TreeWalker`*
    pub fn first_child(this: &TreeWalker) -> Result<Option<Node>, JsValue>;

    #[cfg(feature = "Node")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "TreeWalker" , js_name = lastChild ) ]
    ///The `lastChild()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeWalker/lastChild)
    ///
    ///*This API requires the following crate features to be activated: `Node`, `TreeWalker`*
    pub fn last_child(this: &TreeWalker) -> Result<Option<Node>, JsValue>;

    #[cfg(feature = "Node")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "TreeWalker" , js_name = nextNode ) ]
    ///The `nextNode()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeWalker/nextNode)
    ///
    ///*This API requires the following crate features to be activated: `Node`, `TreeWalker`*
    pub fn next_node(this: &TreeWalker) -> Result<Option<Node>, JsValue>;

    #[cfg(feature = "Node")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "TreeWalker" , js_name = nextSibling ) ]
    ///The `nextSibling()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeWalker/nextSibling)
    ///
    ///*This API requires the following crate features to be activated: `Node`, `TreeWalker`*
    pub fn next_sibling(this: &TreeWalker) -> Result<Option<Node>, JsValue>;

    #[cfg(feature = "Node")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "TreeWalker" , js_name = parentNode ) ]
    ///The `parentNode()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeWalker/parentNode)
    ///
    ///*This API requires the following crate features to be activated: `Node`, `TreeWalker`*
    pub fn parent_node(this: &TreeWalker) -> Result<Option<Node>, JsValue>;

    #[cfg(feature = "Node")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "TreeWalker" , js_name = previousNode ) ]
    ///The `previousNode()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeWalker/previousNode)
    ///
    ///*This API requires the following crate features to be activated: `Node`, `TreeWalker`*
    pub fn previous_node(this: &TreeWalker) -> Result<Option<Node>, JsValue>;

    #[cfg(feature = "Node")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "TreeWalker" , js_name = previousSibling ) ]
    ///The `previousSibling()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeWalker/previousSibling)
    ///
    ///*This API requires the following crate features to be activated: `Node`, `TreeWalker`*
    pub fn previous_sibling(this: &TreeWalker) -> Result<Option<Node>, JsValue>;

}
