use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = DOMStringList , typescript_type = "DOMStringList" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `DomStringList` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMStringList)
    ///
    ///*This API requires the following crate features to be activated: `DomStringList`*
    pub type DomStringList;

    # [ wasm_bindgen ( structural , method , getter , js_class = "DOMStringList" , js_name = length ) ]
    ///Getter for the `length` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMStringList/length)
    ///
    ///*This API requires the following crate features to be activated: `DomStringList`*
    pub fn length(this: &DomStringList) -> u32;

    # [ wasm_bindgen ( method , structural , js_class = "DOMStringList" , js_name = contains ) ]
    ///The `contains()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMStringList/contains)
    ///
    ///*This API requires the following crate features to be activated: `DomStringList`*
    pub fn contains(this: &DomStringList, string: &str) -> bool;

    # [ wasm_bindgen ( method , structural , js_class = "DOMStringList" , js_name = item ) ]
    ///The `item()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMStringList/item)
    ///
    ///*This API requires the following crate features to be activated: `DomStringList`*
    pub fn item(this: &DomStringList, index: u32) -> Option<String>;

    #[wasm_bindgen(method, structural, js_class = "DOMStringList", indexing_getter)]
    ///Indexing getter.
    ///
    ///
    ///
    ///*This API requires the following crate features to be activated: `DomStringList`*
    pub fn get(this: &DomStringList, index: u32) -> Option<String>;

}
