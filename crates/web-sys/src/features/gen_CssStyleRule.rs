use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = CssRule , extends = :: js_sys :: Object , js_name = CSSStyleRule , typescript_name = CSSStyleRule ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `CssStyleRule` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSStyleRule)\n\n*This API requires the following crate features to be activated: `CssStyleRule`*"]
    pub type CssStyleRule;
    # [ wasm_bindgen ( structural , method , getter , js_name = selectorText ) ]
    #[doc = "Getter for the `selectorText` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSStyleRule/selectorText)\n\n*This API requires the following crate features to be activated: `CssStyleRule`*"]
    pub fn selector_text(this: &CssStyleRule) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_name = selectorText ) ]
    #[doc = "Setter for the `selectorText` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSStyleRule/selectorText)\n\n*This API requires the following crate features to be activated: `CssStyleRule`*"]
    pub fn set_selector_text(this: &CssStyleRule, value: &str);
    # [ wasm_bindgen ( structural , method , getter , js_name = style ) ]
    #[cfg(feature = "CssStyleDeclaration")]
    #[doc = "Getter for the `style` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSStyleRule/style)\n\n*This API requires the following crate features to be activated: `CssStyleDeclaration`, `CssStyleRule`*"]
    pub fn style(this: &CssStyleRule) -> CssStyleDeclaration;
}
