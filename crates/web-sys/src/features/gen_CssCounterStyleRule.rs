use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = CssRule , extends = :: js_sys :: Object , js_name = CSSCounterStyleRule , typescript_name = CSSCounterStyleRule ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `CssCounterStyleRule` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSCounterStyleRule)\n\n*This API requires the following crate features to be activated: `CssCounterStyleRule`*"]
    pub type CssCounterStyleRule;
    # [ wasm_bindgen ( structural , method , getter , js_name = name ) ]
    #[doc = "Getter for the `name` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSCounterStyleRule/name)\n\n*This API requires the following crate features to be activated: `CssCounterStyleRule`*"]
    pub fn name(this: &CssCounterStyleRule) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_name = name ) ]
    #[doc = "Setter for the `name` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSCounterStyleRule/name)\n\n*This API requires the following crate features to be activated: `CssCounterStyleRule`*"]
    pub fn set_name(this: &CssCounterStyleRule, value: &str);
    # [ wasm_bindgen ( structural , method , getter , js_name = system ) ]
    #[doc = "Getter for the `system` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSCounterStyleRule/system)\n\n*This API requires the following crate features to be activated: `CssCounterStyleRule`*"]
    pub fn system(this: &CssCounterStyleRule) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_name = system ) ]
    #[doc = "Setter for the `system` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSCounterStyleRule/system)\n\n*This API requires the following crate features to be activated: `CssCounterStyleRule`*"]
    pub fn set_system(this: &CssCounterStyleRule, value: &str);
    # [ wasm_bindgen ( structural , method , getter , js_name = symbols ) ]
    #[doc = "Getter for the `symbols` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSCounterStyleRule/symbols)\n\n*This API requires the following crate features to be activated: `CssCounterStyleRule`*"]
    pub fn symbols(this: &CssCounterStyleRule) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_name = symbols ) ]
    #[doc = "Setter for the `symbols` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSCounterStyleRule/symbols)\n\n*This API requires the following crate features to be activated: `CssCounterStyleRule`*"]
    pub fn set_symbols(this: &CssCounterStyleRule, value: &str);
    # [ wasm_bindgen ( structural , method , getter , js_name = additiveSymbols ) ]
    #[doc = "Getter for the `additiveSymbols` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSCounterStyleRule/additiveSymbols)\n\n*This API requires the following crate features to be activated: `CssCounterStyleRule`*"]
    pub fn additive_symbols(this: &CssCounterStyleRule) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_name = additiveSymbols ) ]
    #[doc = "Setter for the `additiveSymbols` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSCounterStyleRule/additiveSymbols)\n\n*This API requires the following crate features to be activated: `CssCounterStyleRule`*"]
    pub fn set_additive_symbols(this: &CssCounterStyleRule, value: &str);
    # [ wasm_bindgen ( structural , method , getter , js_name = negative ) ]
    #[doc = "Getter for the `negative` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSCounterStyleRule/negative)\n\n*This API requires the following crate features to be activated: `CssCounterStyleRule`*"]
    pub fn negative(this: &CssCounterStyleRule) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_name = negative ) ]
    #[doc = "Setter for the `negative` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSCounterStyleRule/negative)\n\n*This API requires the following crate features to be activated: `CssCounterStyleRule`*"]
    pub fn set_negative(this: &CssCounterStyleRule, value: &str);
    # [ wasm_bindgen ( structural , method , getter , js_name = prefix ) ]
    #[doc = "Getter for the `prefix` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSCounterStyleRule/prefix)\n\n*This API requires the following crate features to be activated: `CssCounterStyleRule`*"]
    pub fn prefix(this: &CssCounterStyleRule) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_name = prefix ) ]
    #[doc = "Setter for the `prefix` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSCounterStyleRule/prefix)\n\n*This API requires the following crate features to be activated: `CssCounterStyleRule`*"]
    pub fn set_prefix(this: &CssCounterStyleRule, value: &str);
    # [ wasm_bindgen ( structural , method , getter , js_name = suffix ) ]
    #[doc = "Getter for the `suffix` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSCounterStyleRule/suffix)\n\n*This API requires the following crate features to be activated: `CssCounterStyleRule`*"]
    pub fn suffix(this: &CssCounterStyleRule) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_name = suffix ) ]
    #[doc = "Setter for the `suffix` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSCounterStyleRule/suffix)\n\n*This API requires the following crate features to be activated: `CssCounterStyleRule`*"]
    pub fn set_suffix(this: &CssCounterStyleRule, value: &str);
    # [ wasm_bindgen ( structural , method , getter , js_name = range ) ]
    #[doc = "Getter for the `range` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSCounterStyleRule/range)\n\n*This API requires the following crate features to be activated: `CssCounterStyleRule`*"]
    pub fn range(this: &CssCounterStyleRule) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_name = range ) ]
    #[doc = "Setter for the `range` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSCounterStyleRule/range)\n\n*This API requires the following crate features to be activated: `CssCounterStyleRule`*"]
    pub fn set_range(this: &CssCounterStyleRule, value: &str);
    # [ wasm_bindgen ( structural , method , getter , js_name = pad ) ]
    #[doc = "Getter for the `pad` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSCounterStyleRule/pad)\n\n*This API requires the following crate features to be activated: `CssCounterStyleRule`*"]
    pub fn pad(this: &CssCounterStyleRule) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_name = pad ) ]
    #[doc = "Setter for the `pad` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSCounterStyleRule/pad)\n\n*This API requires the following crate features to be activated: `CssCounterStyleRule`*"]
    pub fn set_pad(this: &CssCounterStyleRule, value: &str);
    # [ wasm_bindgen ( structural , method , getter , js_name = speakAs ) ]
    #[doc = "Getter for the `speakAs` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSCounterStyleRule/speakAs)\n\n*This API requires the following crate features to be activated: `CssCounterStyleRule`*"]
    pub fn speak_as(this: &CssCounterStyleRule) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_name = speakAs ) ]
    #[doc = "Setter for the `speakAs` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSCounterStyleRule/speakAs)\n\n*This API requires the following crate features to be activated: `CssCounterStyleRule`*"]
    pub fn set_speak_as(this: &CssCounterStyleRule, value: &str);
    # [ wasm_bindgen ( structural , method , getter , js_name = fallback ) ]
    #[doc = "Getter for the `fallback` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSCounterStyleRule/fallback)\n\n*This API requires the following crate features to be activated: `CssCounterStyleRule`*"]
    pub fn fallback(this: &CssCounterStyleRule) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_name = fallback ) ]
    #[doc = "Setter for the `fallback` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSCounterStyleRule/fallback)\n\n*This API requires the following crate features to be activated: `CssCounterStyleRule`*"]
    pub fn set_fallback(this: &CssCounterStyleRule, value: &str);
}
