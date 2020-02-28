use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = CSSRuleList , typescript_name = CSSRuleList ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `CssRuleList` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSRuleList)\n\n*This API requires the following crate features to be activated: `CssRuleList`*"]
    pub type CssRuleList;
    # [ wasm_bindgen ( structural , method , getter , js_class = "CSSRuleList" , js_name = length ) ]
    #[doc = "Getter for the `length` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSRuleList/length)\n\n*This API requires the following crate features to be activated: `CssRuleList`*"]
    pub fn length(this: &CssRuleList) -> u32;
    #[cfg(feature = "CssRule")]
    # [ wasm_bindgen ( method , structural , js_class = "CSSRuleList" , js_name = item ) ]
    #[doc = "The `item()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSRuleList/item)\n\n*This API requires the following crate features to be activated: `CssRule`, `CssRuleList`*"]
    pub fn item(this: &CssRuleList, index: u32) -> Option<CssRule>;
    #[cfg(feature = "CssRule")]
    #[wasm_bindgen(method, structural, js_class = "CSSRuleList", indexing_getter)]
    #[doc = "Indexing getter.\n\n\n\n*This API requires the following crate features to be activated: `CssRule`, `CssRuleList`*"]
    pub fn get(this: &CssRuleList, index: u32) -> Option<CssRule>;
}
