use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = CSSRuleList , typescript_type = "CSSRuleList" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `CssRuleList` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSRuleList)
    ///
    ///*This API requires the following crate features to be activated: `CssRuleList`*
    pub type CssRuleList;

    # [ wasm_bindgen ( structural , method , getter , js_class = "CSSRuleList" , js_name = length ) ]
    ///Getter for the `length` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSRuleList/length)
    ///
    ///*This API requires the following crate features to be activated: `CssRuleList`*
    pub fn length(this: &CssRuleList) -> u32;

    #[cfg(feature = "CssRule")]
    # [ wasm_bindgen ( method , structural , js_class = "CSSRuleList" , js_name = item ) ]
    ///The `item()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSRuleList/item)
    ///
    ///*This API requires the following crate features to be activated: `CssRule`, `CssRuleList`*
    pub fn item(this: &CssRuleList, index: u32) -> Option<CssRule>;

    #[cfg(feature = "CssRule")]
    #[wasm_bindgen(method, structural, js_class = "CSSRuleList", indexing_getter)]
    ///Indexing getter.
    ///
    ///
    ///
    ///*This API requires the following crate features to be activated: `CssRule`, `CssRuleList`*
    pub fn get(this: &CssRuleList, index: u32) -> Option<CssRule>;

}
