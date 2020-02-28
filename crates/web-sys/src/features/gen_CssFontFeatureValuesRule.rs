use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = CssRule , extends = :: js_sys :: Object , js_name = CSSFontFeatureValuesRule , typescript_name = CSSFontFeatureValuesRule ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `CssFontFeatureValuesRule` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSFontFeatureValuesRule)\n\n*This API requires the following crate features to be activated: `CssFontFeatureValuesRule`*"]
    pub type CssFontFeatureValuesRule;
    # [ wasm_bindgen ( structural , method , getter , js_name = fontFamily ) ]
    #[doc = "Getter for the `fontFamily` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSFontFeatureValuesRule/fontFamily)\n\n*This API requires the following crate features to be activated: `CssFontFeatureValuesRule`*"]
    pub fn font_family(this: &CssFontFeatureValuesRule) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_name = fontFamily ) ]
    #[doc = "Setter for the `fontFamily` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSFontFeatureValuesRule/fontFamily)\n\n*This API requires the following crate features to be activated: `CssFontFeatureValuesRule`*"]
    pub fn set_font_family(this: &CssFontFeatureValuesRule, value: &str);
    # [ wasm_bindgen ( structural , method , getter , js_name = valueText ) ]
    #[doc = "Getter for the `valueText` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSFontFeatureValuesRule/valueText)\n\n*This API requires the following crate features to be activated: `CssFontFeatureValuesRule`*"]
    pub fn value_text(this: &CssFontFeatureValuesRule) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_name = valueText ) ]
    #[doc = "Setter for the `valueText` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSFontFeatureValuesRule/valueText)\n\n*This API requires the following crate features to be activated: `CssFontFeatureValuesRule`*"]
    pub fn set_value_text(this: &CssFontFeatureValuesRule, value: &str);
}
