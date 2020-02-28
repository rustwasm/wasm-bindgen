use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = StyleSheetList , typescript_name = StyleSheetList ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `StyleSheetList` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/StyleSheetList)\n\n*This API requires the following crate features to be activated: `StyleSheetList`*"]
    pub type StyleSheetList;
    # [ wasm_bindgen ( structural , method , getter , js_class = "StyleSheetList" , js_name = length ) ]
    #[doc = "Getter for the `length` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/StyleSheetList/length)\n\n*This API requires the following crate features to be activated: `StyleSheetList`*"]
    pub fn length(this: &StyleSheetList) -> u32;
    #[cfg(feature = "StyleSheet")]
    # [ wasm_bindgen ( method , structural , js_class = "StyleSheetList" , js_name = item ) ]
    #[doc = "The `item()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/StyleSheetList/item)\n\n*This API requires the following crate features to be activated: `StyleSheet`, `StyleSheetList`*"]
    pub fn item(this: &StyleSheetList, index: u32) -> Option<StyleSheet>;
    #[cfg(feature = "StyleSheet")]
    #[wasm_bindgen(method, structural, js_class = "StyleSheetList", indexing_getter)]
    #[doc = "Indexing getter.\n\n\n\n*This API requires the following crate features to be activated: `StyleSheet`, `StyleSheetList`*"]
    pub fn get(this: &StyleSheetList, index: u32) -> Option<StyleSheet>;
}
