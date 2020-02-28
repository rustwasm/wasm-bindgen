use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = HTMLCollection , typescript_name = HTMLCollection ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `HtmlCollection` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLCollection)\n\n*This API requires the following crate features to be activated: `HtmlCollection`*"]
    pub type HtmlCollection;
    # [ wasm_bindgen ( structural , method , getter , js_name = length ) ]
    #[doc = "Getter for the `length` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLCollection/length)\n\n*This API requires the following crate features to be activated: `HtmlCollection`*"]
    pub fn length(this: &HtmlCollection) -> u32;
    #[cfg(feature = "Element")]
    # [ wasm_bindgen ( method , structural , js_name = item ) ]
    #[doc = "The `item()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLCollection/item)\n\n*This API requires the following crate features to be activated: `Element`, `HtmlCollection`*"]
    pub fn item(this: &HtmlCollection, index: u32) -> Option<Element>;
    #[cfg(feature = "Element")]
    # [ wasm_bindgen ( method , structural , js_name = namedItem ) ]
    #[doc = "The `namedItem()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLCollection/namedItem)\n\n*This API requires the following crate features to be activated: `Element`, `HtmlCollection`*"]
    pub fn named_item(this: &HtmlCollection, name: &str) -> Option<Element>;
    #[cfg(feature = "Element")]
    #[wasm_bindgen(method, structural, indexing_getter)]
    #[doc = "Indexing getter.\n\n\n\n*This API requires the following crate features to be activated: `Element`, `HtmlCollection`*"]
    pub fn get_with_index(this: &HtmlCollection, index: u32) -> Option<Element>;
    #[cfg(feature = "Element")]
    #[wasm_bindgen(method, structural, indexing_getter)]
    #[doc = "Indexing getter.\n\n\n\n*This API requires the following crate features to be activated: `Element`, `HtmlCollection`*"]
    pub fn get_with_name(this: &HtmlCollection, name: &str) -> Option<Element>;
}
