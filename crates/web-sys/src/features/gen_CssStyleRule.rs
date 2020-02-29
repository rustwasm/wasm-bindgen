use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = CssRule , extends = :: js_sys :: Object , js_name = CSSStyleRule , typescript_name = CSSStyleRule ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `CssStyleRule` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSStyleRule)
    ///
    ///*This API requires the following crate features to be activated: `CssStyleRule`*
    pub type CssStyleRule;

    # [ wasm_bindgen ( structural , method , getter , js_class = "CSSStyleRule" , js_name = selectorText ) ]
    ///Getter for the `selectorText` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSStyleRule/selectorText)
    ///
    ///*This API requires the following crate features to be activated: `CssStyleRule`*
    pub fn selector_text(this: &CssStyleRule) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "CSSStyleRule" , js_name = selectorText ) ]
    ///Setter for the `selectorText` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSStyleRule/selectorText)
    ///
    ///*This API requires the following crate features to be activated: `CssStyleRule`*
    pub fn set_selector_text(this: &CssStyleRule, value: &str);

    #[cfg(feature = "CssStyleDeclaration")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "CSSStyleRule" , js_name = style ) ]
    ///Getter for the `style` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSStyleRule/style)
    ///
    ///*This API requires the following crate features to be activated: `CssStyleDeclaration`, `CssStyleRule`*
    pub fn style(this: &CssStyleRule) -> CssStyleDeclaration;

}
