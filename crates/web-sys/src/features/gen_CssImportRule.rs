use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = CssRule , extends = :: js_sys :: Object , js_name = CSSImportRule , typescript_name = CSSImportRule ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `CssImportRule` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSImportRule)\n\n*This API requires the following crate features to be activated: `CssImportRule`*"]
    pub type CssImportRule;
    # [ wasm_bindgen ( structural , method , getter , js_name = href ) ]
    #[doc = "Getter for the `href` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSImportRule/href)\n\n*This API requires the following crate features to be activated: `CssImportRule`*"]
    pub fn href(this: &CssImportRule) -> String;
    # [ wasm_bindgen ( structural , method , getter , js_name = media ) ]
    #[cfg(feature = "MediaList")]
    #[doc = "Getter for the `media` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSImportRule/media)\n\n*This API requires the following crate features to be activated: `CssImportRule`, `MediaList`*"]
    pub fn media(this: &CssImportRule) -> Option<MediaList>;
    # [ wasm_bindgen ( structural , method , getter , js_name = styleSheet ) ]
    #[cfg(feature = "CssStyleSheet")]
    #[doc = "Getter for the `styleSheet` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSImportRule/styleSheet)\n\n*This API requires the following crate features to be activated: `CssImportRule`, `CssStyleSheet`*"]
    pub fn style_sheet(this: &CssImportRule) -> Option<CssStyleSheet>;
}
