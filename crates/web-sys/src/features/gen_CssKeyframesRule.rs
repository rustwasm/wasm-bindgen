use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = CssRule , extends = :: js_sys :: Object , js_name = CSSKeyframesRule , typescript_type = "CSSKeyframesRule" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `CssKeyframesRule` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSKeyframesRule)
    ///
    ///*This API requires the following crate features to be activated: `CssKeyframesRule`*
    pub type CssKeyframesRule;

    # [ wasm_bindgen ( structural , method , getter , js_class = "CSSKeyframesRule" , js_name = name ) ]
    ///Getter for the `name` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSKeyframesRule/name)
    ///
    ///*This API requires the following crate features to be activated: `CssKeyframesRule`*
    pub fn name(this: &CssKeyframesRule) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "CSSKeyframesRule" , js_name = name ) ]
    ///Setter for the `name` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSKeyframesRule/name)
    ///
    ///*This API requires the following crate features to be activated: `CssKeyframesRule`*
    pub fn set_name(this: &CssKeyframesRule, value: &str);

    #[cfg(feature = "CssRuleList")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "CSSKeyframesRule" , js_name = cssRules ) ]
    ///Getter for the `cssRules` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSKeyframesRule/cssRules)
    ///
    ///*This API requires the following crate features to be activated: `CssKeyframesRule`, `CssRuleList`*
    pub fn css_rules(this: &CssKeyframesRule) -> CssRuleList;

    # [ wasm_bindgen ( method , structural , js_class = "CSSKeyframesRule" , js_name = appendRule ) ]
    ///The `appendRule()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSKeyframesRule/appendRule)
    ///
    ///*This API requires the following crate features to be activated: `CssKeyframesRule`*
    pub fn append_rule(this: &CssKeyframesRule, rule: &str);

    # [ wasm_bindgen ( method , structural , js_class = "CSSKeyframesRule" , js_name = deleteRule ) ]
    ///The `deleteRule()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSKeyframesRule/deleteRule)
    ///
    ///*This API requires the following crate features to be activated: `CssKeyframesRule`*
    pub fn delete_rule(this: &CssKeyframesRule, select: &str);

    #[cfg(feature = "CssKeyframeRule")]
    # [ wasm_bindgen ( method , structural , js_class = "CSSKeyframesRule" , js_name = findRule ) ]
    ///The `findRule()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSKeyframesRule/findRule)
    ///
    ///*This API requires the following crate features to be activated: `CssKeyframeRule`, `CssKeyframesRule`*
    pub fn find_rule(this: &CssKeyframesRule, select: &str) -> Option<CssKeyframeRule>;

}
