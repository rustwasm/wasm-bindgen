use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = StyleSheetList , typescript_name = StyleSheetList ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `StyleSheetList` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/StyleSheetList)
    ///
    ///*This API requires the following crate features to be activated: `StyleSheetList`*
    pub type StyleSheetList;

    # [ wasm_bindgen ( structural , method , getter , js_class = "StyleSheetList" , js_name = length ) ]
    ///Getter for the `length` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/StyleSheetList/length)
    ///
    ///*This API requires the following crate features to be activated: `StyleSheetList`*
    pub fn length(this: &StyleSheetList) -> u32;

    #[cfg(feature = "StyleSheet")]
    # [ wasm_bindgen ( method , structural , js_class = "StyleSheetList" , js_name = item ) ]
    ///The `item()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/StyleSheetList/item)
    ///
    ///*This API requires the following crate features to be activated: `StyleSheet`, `StyleSheetList`*
    pub fn item(this: &StyleSheetList, index: u32) -> Option<StyleSheet>;

    #[cfg(feature = "StyleSheet")]
    #[wasm_bindgen(method, structural, js_class = "StyleSheetList", indexing_getter)]
    ///Indexing getter.
    ///
    ///
    ///
    ///*This API requires the following crate features to be activated: `StyleSheet`, `StyleSheetList`*
    pub fn get(this: &StyleSheetList, index: u32) -> Option<StyleSheet>;

}
