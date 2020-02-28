use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = CssRule , extends = :: js_sys :: Object , js_name = CSSFontFaceRule , typescript_name = CSSFontFaceRule ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `CssFontFaceRule` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSFontFaceRule)\n\n*This API requires the following crate features to be activated: `CssFontFaceRule`*"]
    pub type CssFontFaceRule;
    # [ wasm_bindgen ( structural , method , getter , js_name = style ) ]
    #[cfg(feature = "CssStyleDeclaration")]
    #[doc = "Getter for the `style` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSFontFaceRule/style)\n\n*This API requires the following crate features to be activated: `CssFontFaceRule`, `CssStyleDeclaration`*"]
    pub fn style(this: &CssFontFaceRule) -> CssStyleDeclaration;
}
