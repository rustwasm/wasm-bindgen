use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = SpeechGrammar , typescript_name = SpeechGrammar ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `SpeechGrammar` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechGrammar)
    ///
    ///*This API requires the following crate features to be activated: `SpeechGrammar`*
    pub type SpeechGrammar;

    # [ wasm_bindgen ( structural , catch , method , getter , js_class = "SpeechGrammar" , js_name = src ) ]
    ///Getter for the `src` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechGrammar/src)
    ///
    ///*This API requires the following crate features to be activated: `SpeechGrammar`*
    pub fn src(this: &SpeechGrammar) -> Result<String, JsValue>;

    # [ wasm_bindgen ( structural , catch , method , setter , js_class = "SpeechGrammar" , js_name = src ) ]
    ///Setter for the `src` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechGrammar/src)
    ///
    ///*This API requires the following crate features to be activated: `SpeechGrammar`*
    pub fn set_src(this: &SpeechGrammar, value: &str) -> Result<(), JsValue>;

    # [ wasm_bindgen ( structural , catch , method , getter , js_class = "SpeechGrammar" , js_name = weight ) ]
    ///Getter for the `weight` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechGrammar/weight)
    ///
    ///*This API requires the following crate features to be activated: `SpeechGrammar`*
    pub fn weight(this: &SpeechGrammar) -> Result<f32, JsValue>;

    # [ wasm_bindgen ( structural , catch , method , setter , js_class = "SpeechGrammar" , js_name = weight ) ]
    ///Setter for the `weight` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechGrammar/weight)
    ///
    ///*This API requires the following crate features to be activated: `SpeechGrammar`*
    pub fn set_weight(this: &SpeechGrammar, value: f32) -> Result<(), JsValue>;

    #[wasm_bindgen(catch, constructor, js_class = "SpeechGrammar")]
    ///The `new SpeechGrammar(..)` constructor, creating a new instance of `SpeechGrammar`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechGrammar/SpeechGrammar)
    ///
    ///*This API requires the following crate features to be activated: `SpeechGrammar`*
    pub fn new() -> Result<SpeechGrammar, JsValue>;

}
