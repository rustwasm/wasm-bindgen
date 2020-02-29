use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = CssRule , extends = :: js_sys :: Object , js_name = CSSGroupingRule , typescript_name = CSSGroupingRule ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `CssGroupingRule` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSGroupingRule)
    ///
    ///*This API requires the following crate features to be activated: `CssGroupingRule`*
    pub type CssGroupingRule;

    #[cfg(feature = "CssRuleList")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "CSSGroupingRule" , js_name = cssRules ) ]
    ///Getter for the `cssRules` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSGroupingRule/cssRules)
    ///
    ///*This API requires the following crate features to be activated: `CssGroupingRule`, `CssRuleList`*
    pub fn css_rules(this: &CssGroupingRule) -> CssRuleList;

    # [ wasm_bindgen ( catch , method , structural , js_class = "CSSGroupingRule" , js_name = deleteRule ) ]
    ///The `deleteRule()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSGroupingRule/deleteRule)
    ///
    ///*This API requires the following crate features to be activated: `CssGroupingRule`*
    pub fn delete_rule(this: &CssGroupingRule, index: u32) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "CSSGroupingRule" , js_name = insertRule ) ]
    ///The `insertRule()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSGroupingRule/insertRule)
    ///
    ///*This API requires the following crate features to be activated: `CssGroupingRule`*
    pub fn insert_rule(this: &CssGroupingRule, rule: &str) -> Result<u32, JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "CSSGroupingRule" , js_name = insertRule ) ]
    ///The `insertRule()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSGroupingRule/insertRule)
    ///
    ///*This API requires the following crate features to be activated: `CssGroupingRule`*
    pub fn insert_rule_with_index(
        this: &CssGroupingRule,
        rule: &str,
        index: u32,
    ) -> Result<u32, JsValue>;

}
