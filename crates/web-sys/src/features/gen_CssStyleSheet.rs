use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = StyleSheet , extends = :: js_sys :: Object , js_name = CSSStyleSheet , typescript_name = CSSStyleSheet ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `CssStyleSheet` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSStyleSheet)\n\n*This API requires the following crate features to be activated: `CssStyleSheet`*"]
    pub type CssStyleSheet;
    # [ wasm_bindgen ( structural , method , getter , js_name = ownerRule ) ]
    #[cfg(feature = "CssRule")]
    #[doc = "Getter for the `ownerRule` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSStyleSheet/ownerRule)\n\n*This API requires the following crate features to be activated: `CssRule`, `CssStyleSheet`*"]
    pub fn owner_rule(this: &CssStyleSheet) -> Option<CssRule>;
    # [ wasm_bindgen ( structural , catch , method , getter , js_name = cssRules ) ]
    #[cfg(feature = "CssRuleList")]
    #[doc = "Getter for the `cssRules` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSStyleSheet/cssRules)\n\n*This API requires the following crate features to be activated: `CssRuleList`, `CssStyleSheet`*"]
    pub fn css_rules(this: &CssStyleSheet) -> Result<CssRuleList, JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = deleteRule ) ]
    #[doc = "The `deleteRule()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSStyleSheet/deleteRule)\n\n*This API requires the following crate features to be activated: `CssStyleSheet`*"]
    pub fn delete_rule(this: &CssStyleSheet, index: u32) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = insertRule ) ]
    #[doc = "The `insertRule()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSStyleSheet/insertRule)\n\n*This API requires the following crate features to be activated: `CssStyleSheet`*"]
    pub fn insert_rule(this: &CssStyleSheet, rule: &str) -> Result<u32, JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = insertRule ) ]
    #[doc = "The `insertRule()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSStyleSheet/insertRule)\n\n*This API requires the following crate features to be activated: `CssStyleSheet`*"]
    pub fn insert_rule_with_index(
        this: &CssStyleSheet,
        rule: &str,
        index: u32,
    ) -> Result<u32, JsValue>;
}
