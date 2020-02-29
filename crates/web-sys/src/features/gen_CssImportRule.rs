use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = CssRule , extends = :: js_sys :: Object , js_name = CSSImportRule , typescript_type = "CSSImportRule" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `CssImportRule` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSImportRule)
    ///
    ///*This API requires the following crate features to be activated: `CssImportRule`*
    pub type CssImportRule;

    # [ wasm_bindgen ( structural , method , getter , js_class = "CSSImportRule" , js_name = href ) ]
    ///Getter for the `href` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSImportRule/href)
    ///
    ///*This API requires the following crate features to be activated: `CssImportRule`*
    pub fn href(this: &CssImportRule) -> String;

    #[cfg(feature = "MediaList")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "CSSImportRule" , js_name = media ) ]
    ///Getter for the `media` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSImportRule/media)
    ///
    ///*This API requires the following crate features to be activated: `CssImportRule`, `MediaList`*
    pub fn media(this: &CssImportRule) -> Option<MediaList>;

    #[cfg(feature = "CssStyleSheet")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "CSSImportRule" , js_name = styleSheet ) ]
    ///Getter for the `styleSheet` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSImportRule/styleSheet)
    ///
    ///*This API requires the following crate features to be activated: `CssImportRule`, `CssStyleSheet`*
    pub fn style_sheet(this: &CssImportRule) -> Option<CssStyleSheet>;

}
