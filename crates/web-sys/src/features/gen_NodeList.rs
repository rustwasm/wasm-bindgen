use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = NodeList , typescript_name = NodeList ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `NodeList` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/NodeList)
    ///
    ///*This API requires the following crate features to be activated: `NodeList`*
    pub type NodeList;

    # [ wasm_bindgen ( structural , method , getter , js_class = "NodeList" , js_name = length ) ]
    ///Getter for the `length` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/NodeList/length)
    ///
    ///*This API requires the following crate features to be activated: `NodeList`*
    pub fn length(this: &NodeList) -> u32;

    #[cfg(feature = "Node")]
    # [ wasm_bindgen ( method , structural , js_class = "NodeList" , js_name = item ) ]
    ///The `item()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/NodeList/item)
    ///
    ///*This API requires the following crate features to be activated: `Node`, `NodeList`*
    pub fn item(this: &NodeList, index: u32) -> Option<Node>;

    #[cfg(feature = "Node")]
    #[wasm_bindgen(method, structural, js_class = "NodeList", indexing_getter)]
    ///Indexing getter.
    ///
    ///
    ///
    ///*This API requires the following crate features to be activated: `Node`, `NodeList`*
    pub fn get(this: &NodeList, index: u32) -> Option<Node>;

}
