use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = CssRule , extends = :: js_sys :: Object , js_name = CSSKeyframesRule , typescript_name = CSSKeyframesRule ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `CssKeyframesRule` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSKeyframesRule)\n\n*This API requires the following crate features to be activated: `CssKeyframesRule`*"]
    pub type CssKeyframesRule;
    # [ wasm_bindgen ( structural , method , getter , js_class = "CSSKeyframesRule" , js_name = name ) ]
    #[doc = "Getter for the `name` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSKeyframesRule/name)\n\n*This API requires the following crate features to be activated: `CssKeyframesRule`*"]
    pub fn name(this: &CssKeyframesRule) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_class = "CSSKeyframesRule" , js_name = name ) ]
    #[doc = "Setter for the `name` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSKeyframesRule/name)\n\n*This API requires the following crate features to be activated: `CssKeyframesRule`*"]
    pub fn set_name(this: &CssKeyframesRule, value: &str);
    # [ wasm_bindgen ( structural , method , getter , js_class = "CSSKeyframesRule" , js_name = cssRules ) ]
    #[cfg(feature = "CssRuleList")]
    #[doc = "Getter for the `cssRules` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSKeyframesRule/cssRules)\n\n*This API requires the following crate features to be activated: `CssKeyframesRule`, `CssRuleList`*"]
    pub fn css_rules(this: &CssKeyframesRule) -> CssRuleList;
    # [ wasm_bindgen ( method , structural , js_class = "CSSKeyframesRule" , js_name = appendRule ) ]
    #[doc = "The `appendRule()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSKeyframesRule/appendRule)\n\n*This API requires the following crate features to be activated: `CssKeyframesRule`*"]
    pub fn append_rule(this: &CssKeyframesRule, rule: &str);
    # [ wasm_bindgen ( method , structural , js_class = "CSSKeyframesRule" , js_name = deleteRule ) ]
    #[doc = "The `deleteRule()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSKeyframesRule/deleteRule)\n\n*This API requires the following crate features to be activated: `CssKeyframesRule`*"]
    pub fn delete_rule(this: &CssKeyframesRule, select: &str);
    #[cfg(feature = "CssKeyframeRule")]
    # [ wasm_bindgen ( method , structural , js_class = "CSSKeyframesRule" , js_name = findRule ) ]
    #[doc = "The `findRule()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSKeyframesRule/findRule)\n\n*This API requires the following crate features to be activated: `CssKeyframeRule`, `CssKeyframesRule`*"]
    pub fn find_rule(this: &CssKeyframesRule, select: &str) -> Option<CssKeyframeRule>;
}
