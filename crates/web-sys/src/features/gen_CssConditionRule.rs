use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = CssGroupingRule , extends = CssRule , extends = :: js_sys :: Object , js_name = CSSConditionRule , typescript_name = CSSConditionRule ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `CssConditionRule` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSConditionRule)
    ///
    ///*This API requires the following crate features to be activated: `CssConditionRule`*
    pub type CssConditionRule;

    # [ wasm_bindgen ( structural , method , getter , js_class = "CSSConditionRule" , js_name = conditionText ) ]
    ///Getter for the `conditionText` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSConditionRule/conditionText)
    ///
    ///*This API requires the following crate features to be activated: `CssConditionRule`*
    pub fn condition_text(this: &CssConditionRule) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "CSSConditionRule" , js_name = conditionText ) ]
    ///Setter for the `conditionText` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSConditionRule/conditionText)
    ///
    ///*This API requires the following crate features to be activated: `CssConditionRule`*
    pub fn set_condition_text(this: &CssConditionRule, value: &str);

}
