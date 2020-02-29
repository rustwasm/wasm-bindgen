use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = CssConditionRule , extends = CssGroupingRule , extends = CssRule , extends = :: js_sys :: Object , js_name = CSSMediaRule , typescript_name = CSSMediaRule ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `CssMediaRule` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSMediaRule)
    ///
    ///*This API requires the following crate features to be activated: `CssMediaRule`*
    pub type CssMediaRule;

    #[cfg(feature = "MediaList")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "CSSMediaRule" , js_name = media ) ]
    ///Getter for the `media` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSMediaRule/media)
    ///
    ///*This API requires the following crate features to be activated: `CssMediaRule`, `MediaList`*
    pub fn media(this: &CssMediaRule) -> MediaList;

}
