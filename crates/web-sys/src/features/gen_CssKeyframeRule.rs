use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = CssRule , extends = :: js_sys :: Object , js_name = CSSKeyframeRule , typescript_name = CSSKeyframeRule ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `CssKeyframeRule` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSKeyframeRule)\n\n*This API requires the following crate features to be activated: `CssKeyframeRule`*"]
    pub type CssKeyframeRule;
    # [ wasm_bindgen ( structural , method , getter , js_class = "CSSKeyframeRule" , js_name = keyText ) ]
    #[doc = "Getter for the `keyText` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSKeyframeRule/keyText)\n\n*This API requires the following crate features to be activated: `CssKeyframeRule`*"]
    pub fn key_text(this: &CssKeyframeRule) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_class = "CSSKeyframeRule" , js_name = keyText ) ]
    #[doc = "Setter for the `keyText` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSKeyframeRule/keyText)\n\n*This API requires the following crate features to be activated: `CssKeyframeRule`*"]
    pub fn set_key_text(this: &CssKeyframeRule, value: &str);
    # [ wasm_bindgen ( structural , method , getter , js_class = "CSSKeyframeRule" , js_name = style ) ]
    #[cfg(feature = "CssStyleDeclaration")]
    #[doc = "Getter for the `style` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSKeyframeRule/style)\n\n*This API requires the following crate features to be activated: `CssKeyframeRule`, `CssStyleDeclaration`*"]
    pub fn style(this: &CssKeyframeRule) -> CssStyleDeclaration;
}
