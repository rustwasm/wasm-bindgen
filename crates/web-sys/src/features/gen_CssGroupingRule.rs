use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = CssRule , extends = :: js_sys :: Object , js_name = CSSGroupingRule , typescript_name = CSSGroupingRule ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `CssGroupingRule` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSGroupingRule)\n\n*This API requires the following crate features to be activated: `CssGroupingRule`*"]
    pub type CssGroupingRule;
    # [ wasm_bindgen ( structural , method , getter , js_name = cssRules ) ]
    #[cfg(feature = "CssRuleList")]
    #[doc = "Getter for the `cssRules` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSGroupingRule/cssRules)\n\n*This API requires the following crate features to be activated: `CssGroupingRule`, `CssRuleList`*"]
    pub fn css_rules(this: &CssGroupingRule) -> CssRuleList;
    # [ wasm_bindgen ( catch , method , structural , js_name = deleteRule ) ]
    #[doc = "The `deleteRule()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSGroupingRule/deleteRule)\n\n*This API requires the following crate features to be activated: `CssGroupingRule`*"]
    pub fn delete_rule(this: &CssGroupingRule, index: u32) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = insertRule ) ]
    #[doc = "The `insertRule()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSGroupingRule/insertRule)\n\n*This API requires the following crate features to be activated: `CssGroupingRule`*"]
    pub fn insert_rule(this: &CssGroupingRule, rule: &str) -> Result<u32, JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = insertRule ) ]
    #[doc = "The `insertRule()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSGroupingRule/insertRule)\n\n*This API requires the following crate features to be activated: `CssGroupingRule`*"]
    pub fn insert_rule_with_index(
        this: &CssGroupingRule,
        rule: &str,
        index: u32,
    ) -> Result<u32, JsValue>;
}
