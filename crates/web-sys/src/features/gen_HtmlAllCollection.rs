use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = HTMLAllCollection , typescript_name = HTMLAllCollection ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `HtmlAllCollection` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAllCollection)\n\n*This API requires the following crate features to be activated: `HtmlAllCollection`*"]
    pub type HtmlAllCollection;
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLAllCollection" , js_name = length ) ]
    #[doc = "Getter for the `length` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAllCollection/length)\n\n*This API requires the following crate features to be activated: `HtmlAllCollection`*"]
    pub fn length(this: &HtmlAllCollection) -> u32;
    #[cfg(feature = "Node")]
    # [ wasm_bindgen ( method , structural , js_class = "HTMLAllCollection" , js_name = item ) ]
    #[doc = "The `item()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAllCollection/item)\n\n*This API requires the following crate features to be activated: `HtmlAllCollection`, `Node`*"]
    pub fn item_with_index(this: &HtmlAllCollection, index: u32) -> Option<Node>;
    # [ wasm_bindgen ( method , structural , js_class = "HTMLAllCollection" , js_name = item ) ]
    #[doc = "The `item()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAllCollection/item)\n\n*This API requires the following crate features to be activated: `HtmlAllCollection`*"]
    pub fn item_with_name(this: &HtmlAllCollection, name: &str) -> Option<::js_sys::Object>;
    # [ wasm_bindgen ( method , structural , js_class = "HTMLAllCollection" , js_name = namedItem ) ]
    #[doc = "The `namedItem()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAllCollection/namedItem)\n\n*This API requires the following crate features to be activated: `HtmlAllCollection`*"]
    pub fn named_item(this: &HtmlAllCollection, name: &str) -> Option<::js_sys::Object>;
    #[cfg(feature = "Node")]
    #[wasm_bindgen(method, structural, js_class = "HTMLAllCollection", indexing_getter)]
    #[doc = "Indexing getter.\n\n\n\n*This API requires the following crate features to be activated: `HtmlAllCollection`, `Node`*"]
    pub fn get_with_index(this: &HtmlAllCollection, index: u32) -> Option<Node>;
    #[wasm_bindgen(method, structural, js_class = "HTMLAllCollection", indexing_getter)]
    #[doc = "Indexing getter.\n\n\n\n*This API requires the following crate features to be activated: `HtmlAllCollection`*"]
    pub fn get_with_name(this: &HtmlAllCollection, name: &str) -> Option<::js_sys::Object>;
}
