use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = EventTarget , extends = :: js_sys :: Object , js_name = Node , typescript_type = "Node" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `Node` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Node)
    ///
    ///*This API requires the following crate features to be activated: `Node`*
    pub type Node;

    # [ wasm_bindgen ( structural , method , getter , js_class = "Node" , js_name = nodeType ) ]
    ///Getter for the `nodeType` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Node/nodeType)
    ///
    ///*This API requires the following crate features to be activated: `Node`*
    pub fn node_type(this: &Node) -> u16;

    # [ wasm_bindgen ( structural , method , getter , js_class = "Node" , js_name = nodeName ) ]
    ///Getter for the `nodeName` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Node/nodeName)
    ///
    ///*This API requires the following crate features to be activated: `Node`*
    pub fn node_name(this: &Node) -> String;

    # [ wasm_bindgen ( structural , catch , method , getter , js_class = "Node" , js_name = baseURI ) ]
    ///Getter for the `baseURI` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Node/baseURI)
    ///
    ///*This API requires the following crate features to be activated: `Node`*
    pub fn base_uri(this: &Node) -> Result<Option<String>, JsValue>;

    # [ wasm_bindgen ( structural , method , getter , js_class = "Node" , js_name = isConnected ) ]
    ///Getter for the `isConnected` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Node/isConnected)
    ///
    ///*This API requires the following crate features to be activated: `Node`*
    pub fn is_connected(this: &Node) -> bool;

    #[cfg(feature = "Document")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "Node" , js_name = ownerDocument ) ]
    ///Getter for the `ownerDocument` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Node/ownerDocument)
    ///
    ///*This API requires the following crate features to be activated: `Document`, `Node`*
    pub fn owner_document(this: &Node) -> Option<Document>;

    # [ wasm_bindgen ( structural , method , getter , js_class = "Node" , js_name = parentNode ) ]
    ///Getter for the `parentNode` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Node/parentNode)
    ///
    ///*This API requires the following crate features to be activated: `Node`*
    pub fn parent_node(this: &Node) -> Option<Node>;

    #[cfg(feature = "Element")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "Node" , js_name = parentElement ) ]
    ///Getter for the `parentElement` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Node/parentElement)
    ///
    ///*This API requires the following crate features to be activated: `Element`, `Node`*
    pub fn parent_element(this: &Node) -> Option<Element>;

    #[cfg(feature = "NodeList")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "Node" , js_name = childNodes ) ]
    ///Getter for the `childNodes` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Node/childNodes)
    ///
    ///*This API requires the following crate features to be activated: `Node`, `NodeList`*
    pub fn child_nodes(this: &Node) -> NodeList;

    # [ wasm_bindgen ( structural , method , getter , js_class = "Node" , js_name = firstChild ) ]
    ///Getter for the `firstChild` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Node/firstChild)
    ///
    ///*This API requires the following crate features to be activated: `Node`*
    pub fn first_child(this: &Node) -> Option<Node>;

    # [ wasm_bindgen ( structural , method , getter , js_class = "Node" , js_name = lastChild ) ]
    ///Getter for the `lastChild` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Node/lastChild)
    ///
    ///*This API requires the following crate features to be activated: `Node`*
    pub fn last_child(this: &Node) -> Option<Node>;

    # [ wasm_bindgen ( structural , method , getter , js_class = "Node" , js_name = previousSibling ) ]
    ///Getter for the `previousSibling` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Node/previousSibling)
    ///
    ///*This API requires the following crate features to be activated: `Node`*
    pub fn previous_sibling(this: &Node) -> Option<Node>;

    # [ wasm_bindgen ( structural , method , getter , js_class = "Node" , js_name = nextSibling ) ]
    ///Getter for the `nextSibling` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Node/nextSibling)
    ///
    ///*This API requires the following crate features to be activated: `Node`*
    pub fn next_sibling(this: &Node) -> Option<Node>;

    # [ wasm_bindgen ( structural , method , getter , js_class = "Node" , js_name = nodeValue ) ]
    ///Getter for the `nodeValue` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Node/nodeValue)
    ///
    ///*This API requires the following crate features to be activated: `Node`*
    pub fn node_value(this: &Node) -> Option<String>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "Node" , js_name = nodeValue ) ]
    ///Setter for the `nodeValue` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Node/nodeValue)
    ///
    ///*This API requires the following crate features to be activated: `Node`*
    pub fn set_node_value(this: &Node, value: Option<&str>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "Node" , js_name = textContent ) ]
    ///Getter for the `textContent` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Node/textContent)
    ///
    ///*This API requires the following crate features to be activated: `Node`*
    pub fn text_content(this: &Node) -> Option<String>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "Node" , js_name = textContent ) ]
    ///Setter for the `textContent` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Node/textContent)
    ///
    ///*This API requires the following crate features to be activated: `Node`*
    pub fn set_text_content(this: &Node, value: Option<&str>);

    # [ wasm_bindgen ( catch , method , structural , js_class = "Node" , js_name = appendChild ) ]
    ///The `appendChild()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Node/appendChild)
    ///
    ///*This API requires the following crate features to be activated: `Node`*
    pub fn append_child(this: &Node, node: &Node) -> Result<Node, JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "Node" , js_name = cloneNode ) ]
    ///The `cloneNode()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Node/cloneNode)
    ///
    ///*This API requires the following crate features to be activated: `Node`*
    pub fn clone_node(this: &Node) -> Result<Node, JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "Node" , js_name = cloneNode ) ]
    ///The `cloneNode()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Node/cloneNode)
    ///
    ///*This API requires the following crate features to be activated: `Node`*
    pub fn clone_node_with_deep(this: &Node, deep: bool) -> Result<Node, JsValue>;

    # [ wasm_bindgen ( method , structural , js_class = "Node" , js_name = compareDocumentPosition ) ]
    ///The `compareDocumentPosition()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Node/compareDocumentPosition)
    ///
    ///*This API requires the following crate features to be activated: `Node`*
    pub fn compare_document_position(this: &Node, other: &Node) -> u16;

    # [ wasm_bindgen ( method , structural , js_class = "Node" , js_name = contains ) ]
    ///The `contains()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Node/contains)
    ///
    ///*This API requires the following crate features to be activated: `Node`*
    pub fn contains(this: &Node, other: Option<&Node>) -> bool;

    # [ wasm_bindgen ( method , structural , js_class = "Node" , js_name = getRootNode ) ]
    ///The `getRootNode()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Node/getRootNode)
    ///
    ///*This API requires the following crate features to be activated: `Node`*
    pub fn get_root_node(this: &Node) -> Node;

    #[cfg(feature = "GetRootNodeOptions")]
    # [ wasm_bindgen ( method , structural , js_class = "Node" , js_name = getRootNode ) ]
    ///The `getRootNode()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Node/getRootNode)
    ///
    ///*This API requires the following crate features to be activated: `GetRootNodeOptions`, `Node`*
    pub fn get_root_node_with_options(this: &Node, options: &GetRootNodeOptions) -> Node;

    # [ wasm_bindgen ( method , structural , js_class = "Node" , js_name = hasChildNodes ) ]
    ///The `hasChildNodes()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Node/hasChildNodes)
    ///
    ///*This API requires the following crate features to be activated: `Node`*
    pub fn has_child_nodes(this: &Node) -> bool;

    # [ wasm_bindgen ( catch , method , structural , js_class = "Node" , js_name = insertBefore ) ]
    ///The `insertBefore()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Node/insertBefore)
    ///
    ///*This API requires the following crate features to be activated: `Node`*
    pub fn insert_before(this: &Node, node: &Node, child: Option<&Node>) -> Result<Node, JsValue>;

    # [ wasm_bindgen ( method , structural , js_class = "Node" , js_name = isDefaultNamespace ) ]
    ///The `isDefaultNamespace()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Node/isDefaultNamespace)
    ///
    ///*This API requires the following crate features to be activated: `Node`*
    pub fn is_default_namespace(this: &Node, namespace: Option<&str>) -> bool;

    # [ wasm_bindgen ( method , structural , js_class = "Node" , js_name = isEqualNode ) ]
    ///The `isEqualNode()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Node/isEqualNode)
    ///
    ///*This API requires the following crate features to be activated: `Node`*
    pub fn is_equal_node(this: &Node, node: Option<&Node>) -> bool;

    # [ wasm_bindgen ( method , structural , js_class = "Node" , js_name = isSameNode ) ]
    ///The `isSameNode()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Node/isSameNode)
    ///
    ///*This API requires the following crate features to be activated: `Node`*
    pub fn is_same_node(this: &Node, node: Option<&Node>) -> bool;

    # [ wasm_bindgen ( method , structural , js_class = "Node" , js_name = lookupNamespaceURI ) ]
    ///The `lookupNamespaceURI()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Node/lookupNamespaceURI)
    ///
    ///*This API requires the following crate features to be activated: `Node`*
    pub fn lookup_namespace_uri(this: &Node, prefix: Option<&str>) -> Option<String>;

    # [ wasm_bindgen ( method , structural , js_class = "Node" , js_name = lookupPrefix ) ]
    ///The `lookupPrefix()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Node/lookupPrefix)
    ///
    ///*This API requires the following crate features to be activated: `Node`*
    pub fn lookup_prefix(this: &Node, namespace: Option<&str>) -> Option<String>;

    # [ wasm_bindgen ( method , structural , js_class = "Node" , js_name = normalize ) ]
    ///The `normalize()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Node/normalize)
    ///
    ///*This API requires the following crate features to be activated: `Node`*
    pub fn normalize(this: &Node);

    # [ wasm_bindgen ( catch , method , structural , js_class = "Node" , js_name = removeChild ) ]
    ///The `removeChild()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Node/removeChild)
    ///
    ///*This API requires the following crate features to be activated: `Node`*
    pub fn remove_child(this: &Node, child: &Node) -> Result<Node, JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "Node" , js_name = replaceChild ) ]
    ///The `replaceChild()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Node/replaceChild)
    ///
    ///*This API requires the following crate features to be activated: `Node`*
    pub fn replace_child(this: &Node, node: &Node, child: &Node) -> Result<Node, JsValue>;

}

impl Node {
    ///The `Node.ELEMENT_NODE` const.
    ///
    ///*This API requires the following crate features to be activated: `Node`*

    pub const ELEMENT_NODE: u16 = 1u64 as u16;

    ///The `Node.ATTRIBUTE_NODE` const.
    ///
    ///*This API requires the following crate features to be activated: `Node`*

    pub const ATTRIBUTE_NODE: u16 = 2u64 as u16;

    ///The `Node.TEXT_NODE` const.
    ///
    ///*This API requires the following crate features to be activated: `Node`*

    pub const TEXT_NODE: u16 = 3u64 as u16;

    ///The `Node.CDATA_SECTION_NODE` const.
    ///
    ///*This API requires the following crate features to be activated: `Node`*

    pub const CDATA_SECTION_NODE: u16 = 4u64 as u16;

    ///The `Node.ENTITY_REFERENCE_NODE` const.
    ///
    ///*This API requires the following crate features to be activated: `Node`*

    pub const ENTITY_REFERENCE_NODE: u16 = 5u64 as u16;

    ///The `Node.ENTITY_NODE` const.
    ///
    ///*This API requires the following crate features to be activated: `Node`*

    pub const ENTITY_NODE: u16 = 6u64 as u16;

    ///The `Node.PROCESSING_INSTRUCTION_NODE` const.
    ///
    ///*This API requires the following crate features to be activated: `Node`*

    pub const PROCESSING_INSTRUCTION_NODE: u16 = 7u64 as u16;

    ///The `Node.COMMENT_NODE` const.
    ///
    ///*This API requires the following crate features to be activated: `Node`*

    pub const COMMENT_NODE: u16 = 8u64 as u16;

    ///The `Node.DOCUMENT_NODE` const.
    ///
    ///*This API requires the following crate features to be activated: `Node`*

    pub const DOCUMENT_NODE: u16 = 9u64 as u16;

    ///The `Node.DOCUMENT_TYPE_NODE` const.
    ///
    ///*This API requires the following crate features to be activated: `Node`*

    pub const DOCUMENT_TYPE_NODE: u16 = 10u64 as u16;

    ///The `Node.DOCUMENT_FRAGMENT_NODE` const.
    ///
    ///*This API requires the following crate features to be activated: `Node`*

    pub const DOCUMENT_FRAGMENT_NODE: u16 = 11u64 as u16;

    ///The `Node.NOTATION_NODE` const.
    ///
    ///*This API requires the following crate features to be activated: `Node`*

    pub const NOTATION_NODE: u16 = 12u64 as u16;

    ///The `Node.DOCUMENT_POSITION_DISCONNECTED` const.
    ///
    ///*This API requires the following crate features to be activated: `Node`*

    pub const DOCUMENT_POSITION_DISCONNECTED: u16 = 1u64 as u16;

    ///The `Node.DOCUMENT_POSITION_PRECEDING` const.
    ///
    ///*This API requires the following crate features to be activated: `Node`*

    pub const DOCUMENT_POSITION_PRECEDING: u16 = 2u64 as u16;

    ///The `Node.DOCUMENT_POSITION_FOLLOWING` const.
    ///
    ///*This API requires the following crate features to be activated: `Node`*

    pub const DOCUMENT_POSITION_FOLLOWING: u16 = 4u64 as u16;

    ///The `Node.DOCUMENT_POSITION_CONTAINS` const.
    ///
    ///*This API requires the following crate features to be activated: `Node`*

    pub const DOCUMENT_POSITION_CONTAINS: u16 = 8u64 as u16;

    ///The `Node.DOCUMENT_POSITION_CONTAINED_BY` const.
    ///
    ///*This API requires the following crate features to be activated: `Node`*

    pub const DOCUMENT_POSITION_CONTAINED_BY: u16 = 16u64 as u16;

    ///The `Node.DOCUMENT_POSITION_IMPLEMENTATION_SPECIFIC` const.
    ///
    ///*This API requires the following crate features to be activated: `Node`*

    pub const DOCUMENT_POSITION_IMPLEMENTATION_SPECIFIC: u16 = 32u64 as u16;
}
