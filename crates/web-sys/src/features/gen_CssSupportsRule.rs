use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = CssConditionRule , extends = CssGroupingRule , extends = CssRule , extends = :: js_sys :: Object , js_name = CSSSupportsRule , typescript_name = CSSSupportsRule ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `CssSupportsRule` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSSupportsRule)\n\n*This API requires the following crate features to be activated: `CssSupportsRule`*"]
    pub type CssSupportsRule;
}
