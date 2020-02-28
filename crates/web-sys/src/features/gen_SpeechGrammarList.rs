use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = SpeechGrammarList , typescript_name = SpeechGrammarList ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SpeechGrammarList` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechGrammarList)\n\n*This API requires the following crate features to be activated: `SpeechGrammarList`*"]
    pub type SpeechGrammarList;
    # [ wasm_bindgen ( structural , method , getter , js_name = length ) ]
    #[doc = "Getter for the `length` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechGrammarList/length)\n\n*This API requires the following crate features to be activated: `SpeechGrammarList`*"]
    pub fn length(this: &SpeechGrammarList) -> u32;
    #[wasm_bindgen(catch, constructor)]
    #[doc = "The `new SpeechGrammarList(..)` constructor, creating a new instance of `SpeechGrammarList`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechGrammarList/SpeechGrammarList)\n\n*This API requires the following crate features to be activated: `SpeechGrammarList`*"]
    pub fn new(this: &SpeechGrammarList) -> Result<SpeechGrammarList, JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = addFromString ) ]
    #[doc = "The `addFromString()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechGrammarList/addFromString)\n\n*This API requires the following crate features to be activated: `SpeechGrammarList`*"]
    pub fn add_from_string(this: &SpeechGrammarList, string: &str) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = addFromString ) ]
    #[doc = "The `addFromString()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechGrammarList/addFromString)\n\n*This API requires the following crate features to be activated: `SpeechGrammarList`*"]
    pub fn add_from_string_with_weight(
        this: &SpeechGrammarList,
        string: &str,
        weight: f32,
    ) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = addFromURI ) ]
    #[doc = "The `addFromURI()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechGrammarList/addFromURI)\n\n*This API requires the following crate features to be activated: `SpeechGrammarList`*"]
    pub fn add_from_uri(this: &SpeechGrammarList, src: &str) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = addFromURI ) ]
    #[doc = "The `addFromURI()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechGrammarList/addFromURI)\n\n*This API requires the following crate features to be activated: `SpeechGrammarList`*"]
    pub fn add_from_uri_with_weight(
        this: &SpeechGrammarList,
        src: &str,
        weight: f32,
    ) -> Result<(), JsValue>;
    #[cfg(feature = "SpeechGrammar")]
    # [ wasm_bindgen ( catch , method , structural , js_name = item ) ]
    #[doc = "The `item()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechGrammarList/item)\n\n*This API requires the following crate features to be activated: `SpeechGrammar`, `SpeechGrammarList`*"]
    pub fn item(this: &SpeechGrammarList, index: u32) -> Result<SpeechGrammar, JsValue>;
    #[cfg(feature = "SpeechGrammar")]
    #[wasm_bindgen(catch, method, structural, indexing_getter)]
    #[doc = "Indexing getter.\n\n\n\n*This API requires the following crate features to be activated: `SpeechGrammar`, `SpeechGrammarList`*"]
    pub fn get(this: &SpeechGrammarList, index: u32) -> Result<SpeechGrammar, JsValue>;
}
