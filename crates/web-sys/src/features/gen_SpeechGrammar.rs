use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = SpeechGrammar , typescript_name = SpeechGrammar ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SpeechGrammar` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechGrammar)\n\n*This API requires the following crate features to be activated: `SpeechGrammar`*"]
    pub type SpeechGrammar;
    # [ wasm_bindgen ( structural , catch , method , getter , js_name = src ) ]
    #[doc = "Getter for the `src` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechGrammar/src)\n\n*This API requires the following crate features to be activated: `SpeechGrammar`*"]
    pub fn src(this: &SpeechGrammar) -> Result<String, JsValue>;
    # [ wasm_bindgen ( structural , catch , method , setter , js_name = src ) ]
    #[doc = "Setter for the `src` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechGrammar/src)\n\n*This API requires the following crate features to be activated: `SpeechGrammar`*"]
    pub fn set_src(this: &SpeechGrammar, value: &str) -> Result<(), JsValue>;
    # [ wasm_bindgen ( structural , catch , method , getter , js_name = weight ) ]
    #[doc = "Getter for the `weight` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechGrammar/weight)\n\n*This API requires the following crate features to be activated: `SpeechGrammar`*"]
    pub fn weight(this: &SpeechGrammar) -> Result<f32, JsValue>;
    # [ wasm_bindgen ( structural , catch , method , setter , js_name = weight ) ]
    #[doc = "Setter for the `weight` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechGrammar/weight)\n\n*This API requires the following crate features to be activated: `SpeechGrammar`*"]
    pub fn set_weight(this: &SpeechGrammar, value: f32) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, constructor)]
    #[doc = "The `new SpeechGrammar(..)` constructor, creating a new instance of `SpeechGrammar`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechGrammar/SpeechGrammar)\n\n*This API requires the following crate features to be activated: `SpeechGrammar`*"]
    pub fn new(this: &SpeechGrammar) -> Result<SpeechGrammar, JsValue>;
}
