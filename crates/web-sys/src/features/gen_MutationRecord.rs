use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = MutationRecord , typescript_name = MutationRecord ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `MutationRecord` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MutationRecord)\n\n*This API requires the following crate features to be activated: `MutationRecord`*"]
    pub type MutationRecord;
    # [ wasm_bindgen ( structural , method , getter , js_name = type ) ]
    #[doc = "Getter for the `type` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MutationRecord/type)\n\n*This API requires the following crate features to be activated: `MutationRecord`*"]
    pub fn type_(this: &MutationRecord) -> String;
    # [ wasm_bindgen ( structural , method , getter , js_name = target ) ]
    #[cfg(feature = "Node")]
    #[doc = "Getter for the `target` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MutationRecord/target)\n\n*This API requires the following crate features to be activated: `MutationRecord`, `Node`*"]
    pub fn target(this: &MutationRecord) -> Option<Node>;
    # [ wasm_bindgen ( structural , method , getter , js_name = addedNodes ) ]
    #[cfg(feature = "NodeList")]
    #[doc = "Getter for the `addedNodes` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MutationRecord/addedNodes)\n\n*This API requires the following crate features to be activated: `MutationRecord`, `NodeList`*"]
    pub fn added_nodes(this: &MutationRecord) -> NodeList;
    # [ wasm_bindgen ( structural , method , getter , js_name = removedNodes ) ]
    #[cfg(feature = "NodeList")]
    #[doc = "Getter for the `removedNodes` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MutationRecord/removedNodes)\n\n*This API requires the following crate features to be activated: `MutationRecord`, `NodeList`*"]
    pub fn removed_nodes(this: &MutationRecord) -> NodeList;
    # [ wasm_bindgen ( structural , method , getter , js_name = previousSibling ) ]
    #[cfg(feature = "Node")]
    #[doc = "Getter for the `previousSibling` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MutationRecord/previousSibling)\n\n*This API requires the following crate features to be activated: `MutationRecord`, `Node`*"]
    pub fn previous_sibling(this: &MutationRecord) -> Option<Node>;
    # [ wasm_bindgen ( structural , method , getter , js_name = nextSibling ) ]
    #[cfg(feature = "Node")]
    #[doc = "Getter for the `nextSibling` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MutationRecord/nextSibling)\n\n*This API requires the following crate features to be activated: `MutationRecord`, `Node`*"]
    pub fn next_sibling(this: &MutationRecord) -> Option<Node>;
    # [ wasm_bindgen ( structural , method , getter , js_name = attributeName ) ]
    #[doc = "Getter for the `attributeName` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MutationRecord/attributeName)\n\n*This API requires the following crate features to be activated: `MutationRecord`*"]
    pub fn attribute_name(this: &MutationRecord) -> Option<String>;
    # [ wasm_bindgen ( structural , method , getter , js_name = attributeNamespace ) ]
    #[doc = "Getter for the `attributeNamespace` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MutationRecord/attributeNamespace)\n\n*This API requires the following crate features to be activated: `MutationRecord`*"]
    pub fn attribute_namespace(this: &MutationRecord) -> Option<String>;
    # [ wasm_bindgen ( structural , method , getter , js_name = oldValue ) ]
    #[doc = "Getter for the `oldValue` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MutationRecord/oldValue)\n\n*This API requires the following crate features to be activated: `MutationRecord`*"]
    pub fn old_value(this: &MutationRecord) -> Option<String>;
}
