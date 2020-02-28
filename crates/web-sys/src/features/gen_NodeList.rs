use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = NodeList , typescript_name = NodeList ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `NodeList` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/NodeList)\n\n*This API requires the following crate features to be activated: `NodeList`*"]
    pub type NodeList;
    # [ wasm_bindgen ( structural , method , getter , js_name = length ) ]
    #[doc = "Getter for the `length` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/NodeList/length)\n\n*This API requires the following crate features to be activated: `NodeList`*"]
    pub fn length(this: &NodeList) -> u32;
    #[cfg(feature = "Node")]
    # [ wasm_bindgen ( method , structural , js_name = item ) ]
    #[doc = "The `item()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/NodeList/item)\n\n*This API requires the following crate features to be activated: `Node`, `NodeList`*"]
    pub fn item(this: &NodeList, index: u32) -> Option<Node>;
    #[cfg(feature = "Node")]
    #[wasm_bindgen(method, structural, indexing_getter)]
    #[doc = "Indexing getter.\n\n\n\n*This API requires the following crate features to be activated: `Node`, `NodeList`*"]
    pub fn get(this: &NodeList, index: u32) -> Option<Node>;
}
