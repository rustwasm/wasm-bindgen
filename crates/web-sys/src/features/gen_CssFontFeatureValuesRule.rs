use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = CssRule , extends = :: js_sys :: Object , js_name = CSSFontFeatureValuesRule , typescript_type = "CSSFontFeatureValuesRule" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `CssFontFeatureValuesRule` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSFontFeatureValuesRule)
    ///
    ///*This API requires the following crate features to be activated: `CssFontFeatureValuesRule`*
    pub type CssFontFeatureValuesRule;

    # [ wasm_bindgen ( structural , method , getter , js_class = "CSSFontFeatureValuesRule" , js_name = fontFamily ) ]
    ///Getter for the `fontFamily` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSFontFeatureValuesRule/fontFamily)
    ///
    ///*This API requires the following crate features to be activated: `CssFontFeatureValuesRule`*
    pub fn font_family(this: &CssFontFeatureValuesRule) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "CSSFontFeatureValuesRule" , js_name = fontFamily ) ]
    ///Setter for the `fontFamily` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSFontFeatureValuesRule/fontFamily)
    ///
    ///*This API requires the following crate features to be activated: `CssFontFeatureValuesRule`*
    pub fn set_font_family(this: &CssFontFeatureValuesRule, value: &str);

    # [ wasm_bindgen ( structural , method , getter , js_class = "CSSFontFeatureValuesRule" , js_name = valueText ) ]
    ///Getter for the `valueText` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSFontFeatureValuesRule/valueText)
    ///
    ///*This API requires the following crate features to be activated: `CssFontFeatureValuesRule`*
    pub fn value_text(this: &CssFontFeatureValuesRule) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "CSSFontFeatureValuesRule" , js_name = valueText ) ]
    ///Setter for the `valueText` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSFontFeatureValuesRule/valueText)
    ///
    ///*This API requires the following crate features to be activated: `CssFontFeatureValuesRule`*
    pub fn set_value_text(this: &CssFontFeatureValuesRule, value: &str);

}
