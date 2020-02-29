use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = HTMLCollection , typescript_type = "HTMLCollection" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `HtmlCollection` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLCollection)
    ///
    ///*This API requires the following crate features to be activated: `HtmlCollection`*
    pub type HtmlCollection;

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLCollection" , js_name = length ) ]
    ///Getter for the `length` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLCollection/length)
    ///
    ///*This API requires the following crate features to be activated: `HtmlCollection`*
    pub fn length(this: &HtmlCollection) -> u32;

    #[cfg(feature = "Element")]
    # [ wasm_bindgen ( method , structural , js_class = "HTMLCollection" , js_name = item ) ]
    ///The `item()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLCollection/item)
    ///
    ///*This API requires the following crate features to be activated: `Element`, `HtmlCollection`*
    pub fn item(this: &HtmlCollection, index: u32) -> Option<Element>;

    #[cfg(feature = "Element")]
    # [ wasm_bindgen ( method , structural , js_class = "HTMLCollection" , js_name = namedItem ) ]
    ///The `namedItem()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLCollection/namedItem)
    ///
    ///*This API requires the following crate features to be activated: `Element`, `HtmlCollection`*
    pub fn named_item(this: &HtmlCollection, name: &str) -> Option<Element>;

    #[cfg(feature = "Element")]
    #[wasm_bindgen(method, structural, js_class = "HTMLCollection", indexing_getter)]
    ///Indexing getter.
    ///
    ///
    ///
    ///*This API requires the following crate features to be activated: `Element`, `HtmlCollection`*
    pub fn get_with_index(this: &HtmlCollection, index: u32) -> Option<Element>;

    #[cfg(feature = "Element")]
    #[wasm_bindgen(method, structural, js_class = "HTMLCollection", indexing_getter)]
    ///Indexing getter.
    ///
    ///
    ///
    ///*This API requires the following crate features to be activated: `Element`, `HtmlCollection`*
    pub fn get_with_name(this: &HtmlCollection, name: &str) -> Option<Element>;

}
