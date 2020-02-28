use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = EventTarget , extends = :: js_sys :: Object , js_name = Node , typescript_name = Node ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `Node` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Node)\n\n*This API requires the following crate features to be activated: `Node`*"]
    pub type Node;
    # [ wasm_bindgen ( structural , method , getter , js_name = nodeType ) ]
    #[doc = "Getter for the `nodeType` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Node/nodeType)\n\n*This API requires the following crate features to be activated: `Node`*"]
    pub fn node_type(this: &Node) -> u16;
    # [ wasm_bindgen ( structural , method , getter , js_name = nodeName ) ]
    #[doc = "Getter for the `nodeName` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Node/nodeName)\n\n*This API requires the following crate features to be activated: `Node`*"]
    pub fn node_name(this: &Node) -> String;
    # [ wasm_bindgen ( structural , catch , method , getter , js_name = baseURI ) ]
    #[doc = "Getter for the `baseURI` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Node/baseURI)\n\n*This API requires the following crate features to be activated: `Node`*"]
    pub fn base_uri(this: &Node) -> Result<Option<String>, JsValue>;
    # [ wasm_bindgen ( structural , method , getter , js_name = isConnected ) ]
    #[doc = "Getter for the `isConnected` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Node/isConnected)\n\n*This API requires the following crate features to be activated: `Node`*"]
    pub fn is_connected(this: &Node) -> bool;
    # [ wasm_bindgen ( structural , method , getter , js_name = ownerDocument ) ]
    #[cfg(feature = "Document")]
    #[doc = "Getter for the `ownerDocument` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Node/ownerDocument)\n\n*This API requires the following crate features to be activated: `Document`, `Node`*"]
    pub fn owner_document(this: &Node) -> Option<Document>;
    # [ wasm_bindgen ( structural , method , getter , js_name = parentNode ) ]
    #[doc = "Getter for the `parentNode` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Node/parentNode)\n\n*This API requires the following crate features to be activated: `Node`*"]
    pub fn parent_node(this: &Node) -> Option<Node>;
    # [ wasm_bindgen ( structural , method , getter , js_name = parentElement ) ]
    #[cfg(feature = "Element")]
    #[doc = "Getter for the `parentElement` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Node/parentElement)\n\n*This API requires the following crate features to be activated: `Element`, `Node`*"]
    pub fn parent_element(this: &Node) -> Option<Element>;
    # [ wasm_bindgen ( structural , method , getter , js_name = childNodes ) ]
    #[cfg(feature = "NodeList")]
    #[doc = "Getter for the `childNodes` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Node/childNodes)\n\n*This API requires the following crate features to be activated: `Node`, `NodeList`*"]
    pub fn child_nodes(this: &Node) -> NodeList;
    # [ wasm_bindgen ( structural , method , getter , js_name = firstChild ) ]
    #[doc = "Getter for the `firstChild` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Node/firstChild)\n\n*This API requires the following crate features to be activated: `Node`*"]
    pub fn first_child(this: &Node) -> Option<Node>;
    # [ wasm_bindgen ( structural , method , getter , js_name = lastChild ) ]
    #[doc = "Getter for the `lastChild` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Node/lastChild)\n\n*This API requires the following crate features to be activated: `Node`*"]
    pub fn last_child(this: &Node) -> Option<Node>;
    # [ wasm_bindgen ( structural , method , getter , js_name = previousSibling ) ]
    #[doc = "Getter for the `previousSibling` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Node/previousSibling)\n\n*This API requires the following crate features to be activated: `Node`*"]
    pub fn previous_sibling(this: &Node) -> Option<Node>;
    # [ wasm_bindgen ( structural , method , getter , js_name = nextSibling ) ]
    #[doc = "Getter for the `nextSibling` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Node/nextSibling)\n\n*This API requires the following crate features to be activated: `Node`*"]
    pub fn next_sibling(this: &Node) -> Option<Node>;
    # [ wasm_bindgen ( structural , method , getter , js_name = nodeValue ) ]
    #[doc = "Getter for the `nodeValue` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Node/nodeValue)\n\n*This API requires the following crate features to be activated: `Node`*"]
    pub fn node_value(this: &Node) -> Option<String>;
    # [ wasm_bindgen ( structural , method , setter , js_name = nodeValue ) ]
    #[doc = "Setter for the `nodeValue` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Node/nodeValue)\n\n*This API requires the following crate features to be activated: `Node`*"]
    pub fn set_node_value(this: &Node, value: Option<&str>);
    # [ wasm_bindgen ( structural , method , getter , js_name = textContent ) ]
    #[doc = "Getter for the `textContent` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Node/textContent)\n\n*This API requires the following crate features to be activated: `Node`*"]
    pub fn text_content(this: &Node) -> Option<String>;
    # [ wasm_bindgen ( structural , method , setter , js_name = textContent ) ]
    #[doc = "Setter for the `textContent` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Node/textContent)\n\n*This API requires the following crate features to be activated: `Node`*"]
    pub fn set_text_content(this: &Node, value: Option<&str>);
    # [ wasm_bindgen ( catch , method , structural , js_name = appendChild ) ]
    #[doc = "The `appendChild()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Node/appendChild)\n\n*This API requires the following crate features to be activated: `Node`*"]
    pub fn append_child(this: &Node, node: &Node) -> Result<Node, JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = cloneNode ) ]
    #[doc = "The `cloneNode()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Node/cloneNode)\n\n*This API requires the following crate features to be activated: `Node`*"]
    pub fn clone_node(this: &Node) -> Result<Node, JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = cloneNode ) ]
    #[doc = "The `cloneNode()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Node/cloneNode)\n\n*This API requires the following crate features to be activated: `Node`*"]
    pub fn clone_node_with_deep(this: &Node, deep: bool) -> Result<Node, JsValue>;
    # [ wasm_bindgen ( method , structural , js_name = compareDocumentPosition ) ]
    #[doc = "The `compareDocumentPosition()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Node/compareDocumentPosition)\n\n*This API requires the following crate features to be activated: `Node`*"]
    pub fn compare_document_position(this: &Node, other: &Node) -> u16;
    # [ wasm_bindgen ( method , structural , js_name = contains ) ]
    #[doc = "The `contains()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Node/contains)\n\n*This API requires the following crate features to be activated: `Node`*"]
    pub fn contains(this: &Node, other: Option<&Node>) -> bool;
    # [ wasm_bindgen ( method , structural , js_name = getRootNode ) ]
    #[doc = "The `getRootNode()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Node/getRootNode)\n\n*This API requires the following crate features to be activated: `Node`*"]
    pub fn get_root_node(this: &Node) -> Node;
    #[cfg(feature = "GetRootNodeOptions")]
    # [ wasm_bindgen ( method , structural , js_name = getRootNode ) ]
    #[doc = "The `getRootNode()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Node/getRootNode)\n\n*This API requires the following crate features to be activated: `GetRootNodeOptions`, `Node`*"]
    pub fn get_root_node_with_options(this: &Node, options: &GetRootNodeOptions) -> Node;
    # [ wasm_bindgen ( method , structural , js_name = hasChildNodes ) ]
    #[doc = "The `hasChildNodes()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Node/hasChildNodes)\n\n*This API requires the following crate features to be activated: `Node`*"]
    pub fn has_child_nodes(this: &Node) -> bool;
    # [ wasm_bindgen ( catch , method , structural , js_name = insertBefore ) ]
    #[doc = "The `insertBefore()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Node/insertBefore)\n\n*This API requires the following crate features to be activated: `Node`*"]
    pub fn insert_before(this: &Node, node: &Node, child: Option<&Node>) -> Result<Node, JsValue>;
    # [ wasm_bindgen ( method , structural , js_name = isDefaultNamespace ) ]
    #[doc = "The `isDefaultNamespace()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Node/isDefaultNamespace)\n\n*This API requires the following crate features to be activated: `Node`*"]
    pub fn is_default_namespace(this: &Node, namespace: Option<&str>) -> bool;
    # [ wasm_bindgen ( method , structural , js_name = isEqualNode ) ]
    #[doc = "The `isEqualNode()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Node/isEqualNode)\n\n*This API requires the following crate features to be activated: `Node`*"]
    pub fn is_equal_node(this: &Node, node: Option<&Node>) -> bool;
    # [ wasm_bindgen ( method , structural , js_name = isSameNode ) ]
    #[doc = "The `isSameNode()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Node/isSameNode)\n\n*This API requires the following crate features to be activated: `Node`*"]
    pub fn is_same_node(this: &Node, node: Option<&Node>) -> bool;
    # [ wasm_bindgen ( method , structural , js_name = lookupNamespaceURI ) ]
    #[doc = "The `lookupNamespaceURI()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Node/lookupNamespaceURI)\n\n*This API requires the following crate features to be activated: `Node`*"]
    pub fn lookup_namespace_uri(this: &Node, prefix: Option<&str>) -> Option<String>;
    # [ wasm_bindgen ( method , structural , js_name = lookupPrefix ) ]
    #[doc = "The `lookupPrefix()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Node/lookupPrefix)\n\n*This API requires the following crate features to be activated: `Node`*"]
    pub fn lookup_prefix(this: &Node, namespace: Option<&str>) -> Option<String>;
    # [ wasm_bindgen ( method , structural , js_name = normalize ) ]
    #[doc = "The `normalize()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Node/normalize)\n\n*This API requires the following crate features to be activated: `Node`*"]
    pub fn normalize(this: &Node);
    # [ wasm_bindgen ( catch , method , structural , js_name = removeChild ) ]
    #[doc = "The `removeChild()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Node/removeChild)\n\n*This API requires the following crate features to be activated: `Node`*"]
    pub fn remove_child(this: &Node, child: &Node) -> Result<Node, JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = replaceChild ) ]
    #[doc = "The `replaceChild()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Node/replaceChild)\n\n*This API requires the following crate features to be activated: `Node`*"]
    pub fn replace_child(this: &Node, node: &Node, child: &Node) -> Result<Node, JsValue>;
}
impl Node {
    pub const ELEMENT_NODE: u16 = 1u64 as u16;
    pub const ATTRIBUTE_NODE: u16 = 2u64 as u16;
    pub const TEXT_NODE: u16 = 3u64 as u16;
    pub const CDATA_SECTION_NODE: u16 = 4u64 as u16;
    pub const ENTITY_REFERENCE_NODE: u16 = 5u64 as u16;
    pub const ENTITY_NODE: u16 = 6u64 as u16;
    pub const PROCESSING_INSTRUCTION_NODE: u16 = 7u64 as u16;
    pub const COMMENT_NODE: u16 = 8u64 as u16;
    pub const DOCUMENT_NODE: u16 = 9u64 as u16;
    pub const DOCUMENT_TYPE_NODE: u16 = 10u64 as u16;
    pub const DOCUMENT_FRAGMENT_NODE: u16 = 11u64 as u16;
    pub const NOTATION_NODE: u16 = 12u64 as u16;
    pub const DOCUMENT_POSITION_DISCONNECTED: u16 = 1u64 as u16;
    pub const DOCUMENT_POSITION_PRECEDING: u16 = 2u64 as u16;
    pub const DOCUMENT_POSITION_FOLLOWING: u16 = 4u64 as u16;
    pub const DOCUMENT_POSITION_CONTAINS: u16 = 8u64 as u16;
    pub const DOCUMENT_POSITION_CONTAINED_BY: u16 = 16u64 as u16;
    pub const DOCUMENT_POSITION_IMPLEMENTATION_SPECIFIC: u16 = 32u64 as u16;
}
