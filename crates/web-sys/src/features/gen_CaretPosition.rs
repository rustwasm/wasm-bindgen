use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = CaretPosition , typescript_name = CaretPosition ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `CaretPosition` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CaretPosition)\n\n*This API requires the following crate features to be activated: `CaretPosition`*"]
    pub type CaretPosition;
    # [ wasm_bindgen ( structural , method , getter , js_class = "CaretPosition" , js_name = offsetNode ) ]
    #[cfg(feature = "Node")]
    #[doc = "Getter for the `offsetNode` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CaretPosition/offsetNode)\n\n*This API requires the following crate features to be activated: `CaretPosition`, `Node`*"]
    pub fn offset_node(this: &CaretPosition) -> Option<Node>;
    # [ wasm_bindgen ( structural , method , getter , js_class = "CaretPosition" , js_name = offset ) ]
    #[doc = "Getter for the `offset` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CaretPosition/offset)\n\n*This API requires the following crate features to be activated: `CaretPosition`*"]
    pub fn offset(this: &CaretPosition) -> u32;
    #[cfg(feature = "DomRect")]
    # [ wasm_bindgen ( method , structural , js_class = "CaretPosition" , js_name = getClientRect ) ]
    #[doc = "The `getClientRect()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CaretPosition/getClientRect)\n\n*This API requires the following crate features to be activated: `CaretPosition`, `DomRect`*"]
    pub fn get_client_rect(this: &CaretPosition) -> Option<DomRect>;
}
