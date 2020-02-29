use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = CssRule , extends = :: js_sys :: Object , js_name = CSSCounterStyleRule , typescript_name = CSSCounterStyleRule ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `CssCounterStyleRule` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSCounterStyleRule)
    ///
    ///*This API requires the following crate features to be activated: `CssCounterStyleRule`*
    pub type CssCounterStyleRule;

    # [ wasm_bindgen ( structural , method , getter , js_class = "CSSCounterStyleRule" , js_name = name ) ]
    ///Getter for the `name` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSCounterStyleRule/name)
    ///
    ///*This API requires the following crate features to be activated: `CssCounterStyleRule`*
    pub fn name(this: &CssCounterStyleRule) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "CSSCounterStyleRule" , js_name = name ) ]
    ///Setter for the `name` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSCounterStyleRule/name)
    ///
    ///*This API requires the following crate features to be activated: `CssCounterStyleRule`*
    pub fn set_name(this: &CssCounterStyleRule, value: &str);

    # [ wasm_bindgen ( structural , method , getter , js_class = "CSSCounterStyleRule" , js_name = system ) ]
    ///Getter for the `system` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSCounterStyleRule/system)
    ///
    ///*This API requires the following crate features to be activated: `CssCounterStyleRule`*
    pub fn system(this: &CssCounterStyleRule) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "CSSCounterStyleRule" , js_name = system ) ]
    ///Setter for the `system` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSCounterStyleRule/system)
    ///
    ///*This API requires the following crate features to be activated: `CssCounterStyleRule`*
    pub fn set_system(this: &CssCounterStyleRule, value: &str);

    # [ wasm_bindgen ( structural , method , getter , js_class = "CSSCounterStyleRule" , js_name = symbols ) ]
    ///Getter for the `symbols` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSCounterStyleRule/symbols)
    ///
    ///*This API requires the following crate features to be activated: `CssCounterStyleRule`*
    pub fn symbols(this: &CssCounterStyleRule) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "CSSCounterStyleRule" , js_name = symbols ) ]
    ///Setter for the `symbols` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSCounterStyleRule/symbols)
    ///
    ///*This API requires the following crate features to be activated: `CssCounterStyleRule`*
    pub fn set_symbols(this: &CssCounterStyleRule, value: &str);

    # [ wasm_bindgen ( structural , method , getter , js_class = "CSSCounterStyleRule" , js_name = additiveSymbols ) ]
    ///Getter for the `additiveSymbols` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSCounterStyleRule/additiveSymbols)
    ///
    ///*This API requires the following crate features to be activated: `CssCounterStyleRule`*
    pub fn additive_symbols(this: &CssCounterStyleRule) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "CSSCounterStyleRule" , js_name = additiveSymbols ) ]
    ///Setter for the `additiveSymbols` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSCounterStyleRule/additiveSymbols)
    ///
    ///*This API requires the following crate features to be activated: `CssCounterStyleRule`*
    pub fn set_additive_symbols(this: &CssCounterStyleRule, value: &str);

    # [ wasm_bindgen ( structural , method , getter , js_class = "CSSCounterStyleRule" , js_name = negative ) ]
    ///Getter for the `negative` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSCounterStyleRule/negative)
    ///
    ///*This API requires the following crate features to be activated: `CssCounterStyleRule`*
    pub fn negative(this: &CssCounterStyleRule) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "CSSCounterStyleRule" , js_name = negative ) ]
    ///Setter for the `negative` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSCounterStyleRule/negative)
    ///
    ///*This API requires the following crate features to be activated: `CssCounterStyleRule`*
    pub fn set_negative(this: &CssCounterStyleRule, value: &str);

    # [ wasm_bindgen ( structural , method , getter , js_class = "CSSCounterStyleRule" , js_name = prefix ) ]
    ///Getter for the `prefix` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSCounterStyleRule/prefix)
    ///
    ///*This API requires the following crate features to be activated: `CssCounterStyleRule`*
    pub fn prefix(this: &CssCounterStyleRule) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "CSSCounterStyleRule" , js_name = prefix ) ]
    ///Setter for the `prefix` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSCounterStyleRule/prefix)
    ///
    ///*This API requires the following crate features to be activated: `CssCounterStyleRule`*
    pub fn set_prefix(this: &CssCounterStyleRule, value: &str);

    # [ wasm_bindgen ( structural , method , getter , js_class = "CSSCounterStyleRule" , js_name = suffix ) ]
    ///Getter for the `suffix` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSCounterStyleRule/suffix)
    ///
    ///*This API requires the following crate features to be activated: `CssCounterStyleRule`*
    pub fn suffix(this: &CssCounterStyleRule) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "CSSCounterStyleRule" , js_name = suffix ) ]
    ///Setter for the `suffix` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSCounterStyleRule/suffix)
    ///
    ///*This API requires the following crate features to be activated: `CssCounterStyleRule`*
    pub fn set_suffix(this: &CssCounterStyleRule, value: &str);

    # [ wasm_bindgen ( structural , method , getter , js_class = "CSSCounterStyleRule" , js_name = range ) ]
    ///Getter for the `range` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSCounterStyleRule/range)
    ///
    ///*This API requires the following crate features to be activated: `CssCounterStyleRule`*
    pub fn range(this: &CssCounterStyleRule) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "CSSCounterStyleRule" , js_name = range ) ]
    ///Setter for the `range` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSCounterStyleRule/range)
    ///
    ///*This API requires the following crate features to be activated: `CssCounterStyleRule`*
    pub fn set_range(this: &CssCounterStyleRule, value: &str);

    # [ wasm_bindgen ( structural , method , getter , js_class = "CSSCounterStyleRule" , js_name = pad ) ]
    ///Getter for the `pad` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSCounterStyleRule/pad)
    ///
    ///*This API requires the following crate features to be activated: `CssCounterStyleRule`*
    pub fn pad(this: &CssCounterStyleRule) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "CSSCounterStyleRule" , js_name = pad ) ]
    ///Setter for the `pad` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSCounterStyleRule/pad)
    ///
    ///*This API requires the following crate features to be activated: `CssCounterStyleRule`*
    pub fn set_pad(this: &CssCounterStyleRule, value: &str);

    # [ wasm_bindgen ( structural , method , getter , js_class = "CSSCounterStyleRule" , js_name = speakAs ) ]
    ///Getter for the `speakAs` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSCounterStyleRule/speakAs)
    ///
    ///*This API requires the following crate features to be activated: `CssCounterStyleRule`*
    pub fn speak_as(this: &CssCounterStyleRule) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "CSSCounterStyleRule" , js_name = speakAs ) ]
    ///Setter for the `speakAs` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSCounterStyleRule/speakAs)
    ///
    ///*This API requires the following crate features to be activated: `CssCounterStyleRule`*
    pub fn set_speak_as(this: &CssCounterStyleRule, value: &str);

    # [ wasm_bindgen ( structural , method , getter , js_class = "CSSCounterStyleRule" , js_name = fallback ) ]
    ///Getter for the `fallback` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSCounterStyleRule/fallback)
    ///
    ///*This API requires the following crate features to be activated: `CssCounterStyleRule`*
    pub fn fallback(this: &CssCounterStyleRule) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "CSSCounterStyleRule" , js_name = fallback ) ]
    ///Setter for the `fallback` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSCounterStyleRule/fallback)
    ///
    ///*This API requires the following crate features to be activated: `CssCounterStyleRule`*
    pub fn set_fallback(this: &CssCounterStyleRule, value: &str);

}
