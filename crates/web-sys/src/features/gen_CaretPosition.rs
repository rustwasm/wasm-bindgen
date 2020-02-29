use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = CaretPosition , typescript_name = CaretPosition ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `CaretPosition` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CaretPosition)
    ///
    ///*This API requires the following crate features to be activated: `CaretPosition`*
    pub type CaretPosition;

    #[cfg(feature = "Node")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "CaretPosition" , js_name = offsetNode ) ]
    ///Getter for the `offsetNode` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CaretPosition/offsetNode)
    ///
    ///*This API requires the following crate features to be activated: `CaretPosition`, `Node`*
    pub fn offset_node(this: &CaretPosition) -> Option<Node>;

    # [ wasm_bindgen ( structural , method , getter , js_class = "CaretPosition" , js_name = offset ) ]
    ///Getter for the `offset` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CaretPosition/offset)
    ///
    ///*This API requires the following crate features to be activated: `CaretPosition`*
    pub fn offset(this: &CaretPosition) -> u32;

    #[cfg(feature = "DomRect")]
    # [ wasm_bindgen ( method , structural , js_class = "CaretPosition" , js_name = getClientRect ) ]
    ///The `getClientRect()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CaretPosition/getClientRect)
    ///
    ///*This API requires the following crate features to be activated: `CaretPosition`, `DomRect`*
    pub fn get_client_rect(this: &CaretPosition) -> Option<DomRect>;

}
