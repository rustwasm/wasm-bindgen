use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = HTMLAllCollection , typescript_type = "HTMLAllCollection" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `HtmlAllCollection` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAllCollection)
    ///
    ///*This API requires the following crate features to be activated: `HtmlAllCollection`*
    pub type HtmlAllCollection;

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLAllCollection" , js_name = length ) ]
    ///Getter for the `length` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAllCollection/length)
    ///
    ///*This API requires the following crate features to be activated: `HtmlAllCollection`*
    pub fn length(this: &HtmlAllCollection) -> u32;

    #[cfg(feature = "Node")]
    # [ wasm_bindgen ( method , structural , js_class = "HTMLAllCollection" , js_name = item ) ]
    ///The `item()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAllCollection/item)
    ///
    ///*This API requires the following crate features to be activated: `HtmlAllCollection`, `Node`*
    pub fn item_with_index(this: &HtmlAllCollection, index: u32) -> Option<Node>;

    # [ wasm_bindgen ( method , structural , js_class = "HTMLAllCollection" , js_name = item ) ]
    ///The `item()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAllCollection/item)
    ///
    ///*This API requires the following crate features to be activated: `HtmlAllCollection`*
    pub fn item_with_name(this: &HtmlAllCollection, name: &str) -> Option<::js_sys::Object>;

    # [ wasm_bindgen ( method , structural , js_class = "HTMLAllCollection" , js_name = namedItem ) ]
    ///The `namedItem()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAllCollection/namedItem)
    ///
    ///*This API requires the following crate features to be activated: `HtmlAllCollection`*
    pub fn named_item(this: &HtmlAllCollection, name: &str) -> Option<::js_sys::Object>;

    #[cfg(feature = "Node")]
    #[wasm_bindgen(method, structural, js_class = "HTMLAllCollection", indexing_getter)]
    ///Indexing getter.
    ///
    ///
    ///
    ///*This API requires the following crate features to be activated: `HtmlAllCollection`, `Node`*
    pub fn get_with_index(this: &HtmlAllCollection, index: u32) -> Option<Node>;

    #[wasm_bindgen(method, structural, js_class = "HTMLAllCollection", indexing_getter)]
    ///Indexing getter.
    ///
    ///
    ///
    ///*This API requires the following crate features to be activated: `HtmlAllCollection`*
    pub fn get_with_name(this: &HtmlAllCollection, name: &str) -> Option<::js_sys::Object>;

}
