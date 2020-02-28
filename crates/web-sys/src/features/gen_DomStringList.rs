use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = DOMStringList , typescript_name = DOMStringList ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `DomStringList` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMStringList)\n\n*This API requires the following crate features to be activated: `DomStringList`*"]
    pub type DomStringList;
    # [ wasm_bindgen ( structural , method , getter , js_name = length ) ]
    #[doc = "Getter for the `length` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMStringList/length)\n\n*This API requires the following crate features to be activated: `DomStringList`*"]
    pub fn length(this: &DomStringList) -> u32;
    # [ wasm_bindgen ( method , structural , js_name = contains ) ]
    #[doc = "The `contains()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMStringList/contains)\n\n*This API requires the following crate features to be activated: `DomStringList`*"]
    pub fn contains(this: &DomStringList, string: &str) -> bool;
    # [ wasm_bindgen ( method , structural , js_name = item ) ]
    #[doc = "The `item()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMStringList/item)\n\n*This API requires the following crate features to be activated: `DomStringList`*"]
    pub fn item(this: &DomStringList, index: u32) -> Option<String>;
    #[wasm_bindgen(method, structural, indexing_getter)]
    #[doc = "Indexing getter.\n\n\n\n*This API requires the following crate features to be activated: `DomStringList`*"]
    pub fn get(this: &DomStringList, index: u32) -> Option<String>;
}
