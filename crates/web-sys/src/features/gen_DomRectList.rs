use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = DOMRectList , typescript_name = DOMRectList ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `DomRectList` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMRectList)
    ///
    ///*This API requires the following crate features to be activated: `DomRectList`*
    pub type DomRectList;

    # [ wasm_bindgen ( structural , method , getter , js_class = "DOMRectList" , js_name = length ) ]
    ///Getter for the `length` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMRectList/length)
    ///
    ///*This API requires the following crate features to be activated: `DomRectList`*
    pub fn length(this: &DomRectList) -> u32;

    #[cfg(feature = "DomRect")]
    # [ wasm_bindgen ( method , structural , js_class = "DOMRectList" , js_name = item ) ]
    ///The `item()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMRectList/item)
    ///
    ///*This API requires the following crate features to be activated: `DomRect`, `DomRectList`*
    pub fn item(this: &DomRectList, index: u32) -> Option<DomRect>;

    #[cfg(feature = "DomRect")]
    #[wasm_bindgen(method, structural, js_class = "DOMRectList", indexing_getter)]
    ///Indexing getter.
    ///
    ///
    ///
    ///*This API requires the following crate features to be activated: `DomRect`, `DomRectList`*
    pub fn get(this: &DomRectList, index: u32) -> Option<DomRect>;

}
