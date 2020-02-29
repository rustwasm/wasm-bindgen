use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = CssRule , extends = :: js_sys :: Object , js_name = CSSFontFaceRule , typescript_type = "CSSFontFaceRule" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `CssFontFaceRule` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSFontFaceRule)
    ///
    ///*This API requires the following crate features to be activated: `CssFontFaceRule`*
    pub type CssFontFaceRule;

    #[cfg(feature = "CssStyleDeclaration")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "CSSFontFaceRule" , js_name = style ) ]
    ///Getter for the `style` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSFontFaceRule/style)
    ///
    ///*This API requires the following crate features to be activated: `CssFontFaceRule`, `CssStyleDeclaration`*
    pub fn style(this: &CssFontFaceRule) -> CssStyleDeclaration;

}
