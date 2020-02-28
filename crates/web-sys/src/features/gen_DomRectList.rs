use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = DOMRectList , typescript_name = DOMRectList ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `DomRectList` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMRectList)\n\n*This API requires the following crate features to be activated: `DomRectList`*"]
    pub type DomRectList;
    # [ wasm_bindgen ( structural , method , getter , js_name = length ) ]
    #[doc = "Getter for the `length` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMRectList/length)\n\n*This API requires the following crate features to be activated: `DomRectList`*"]
    pub fn length(this: &DomRectList) -> u32;
    #[cfg(feature = "DomRect")]
    # [ wasm_bindgen ( method , structural , js_name = item ) ]
    #[doc = "The `item()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMRectList/item)\n\n*This API requires the following crate features to be activated: `DomRect`, `DomRectList`*"]
    pub fn item(this: &DomRectList, index: u32) -> Option<DomRect>;
    #[cfg(feature = "DomRect")]
    #[wasm_bindgen(method, structural, indexing_getter)]
    #[doc = "Indexing getter.\n\n\n\n*This API requires the following crate features to be activated: `DomRect`, `DomRectList`*"]
    pub fn get(this: &DomRectList, index: u32) -> Option<DomRect>;
}
