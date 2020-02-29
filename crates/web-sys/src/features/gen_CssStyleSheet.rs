use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = StyleSheet , extends = :: js_sys :: Object , js_name = CSSStyleSheet , typescript_type = "CSSStyleSheet" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `CssStyleSheet` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSStyleSheet)
    ///
    ///*This API requires the following crate features to be activated: `CssStyleSheet`*
    pub type CssStyleSheet;

    #[cfg(feature = "CssRule")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "CSSStyleSheet" , js_name = ownerRule ) ]
    ///Getter for the `ownerRule` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSStyleSheet/ownerRule)
    ///
    ///*This API requires the following crate features to be activated: `CssRule`, `CssStyleSheet`*
    pub fn owner_rule(this: &CssStyleSheet) -> Option<CssRule>;

    #[cfg(feature = "CssRuleList")]
    # [ wasm_bindgen ( structural , catch , method , getter , js_class = "CSSStyleSheet" , js_name = cssRules ) ]
    ///Getter for the `cssRules` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSStyleSheet/cssRules)
    ///
    ///*This API requires the following crate features to be activated: `CssRuleList`, `CssStyleSheet`*
    pub fn css_rules(this: &CssStyleSheet) -> Result<CssRuleList, JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "CSSStyleSheet" , js_name = deleteRule ) ]
    ///The `deleteRule()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSStyleSheet/deleteRule)
    ///
    ///*This API requires the following crate features to be activated: `CssStyleSheet`*
    pub fn delete_rule(this: &CssStyleSheet, index: u32) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "CSSStyleSheet" , js_name = insertRule ) ]
    ///The `insertRule()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSStyleSheet/insertRule)
    ///
    ///*This API requires the following crate features to be activated: `CssStyleSheet`*
    pub fn insert_rule(this: &CssStyleSheet, rule: &str) -> Result<u32, JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "CSSStyleSheet" , js_name = insertRule ) ]
    ///The `insertRule()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSStyleSheet/insertRule)
    ///
    ///*This API requires the following crate features to be activated: `CssStyleSheet`*
    pub fn insert_rule_with_index(
        this: &CssStyleSheet,
        rule: &str,
        index: u32,
    ) -> Result<u32, JsValue>;

}
