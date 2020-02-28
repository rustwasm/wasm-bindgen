use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = CssRule , extends = :: js_sys :: Object , js_name = CSSPageRule , typescript_name = CSSPageRule ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `CssPageRule` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSPageRule)\n\n*This API requires the following crate features to be activated: `CssPageRule`*"]
    pub type CssPageRule;
    # [ wasm_bindgen ( structural , method , getter , js_class = "CSSPageRule" , js_name = style ) ]
    #[cfg(feature = "CssStyleDeclaration")]
    #[doc = "Getter for the `style` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSPageRule/style)\n\n*This API requires the following crate features to be activated: `CssPageRule`, `CssStyleDeclaration`*"]
    pub fn style(this: &CssPageRule) -> CssStyleDeclaration;
}
