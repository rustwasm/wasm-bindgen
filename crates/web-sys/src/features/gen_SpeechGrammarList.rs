use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = SpeechGrammarList , typescript_name = SpeechGrammarList ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `SpeechGrammarList` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechGrammarList)
    ///
    ///*This API requires the following crate features to be activated: `SpeechGrammarList`*
    pub type SpeechGrammarList;

    # [ wasm_bindgen ( structural , method , getter , js_class = "SpeechGrammarList" , js_name = length ) ]
    ///Getter for the `length` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechGrammarList/length)
    ///
    ///*This API requires the following crate features to be activated: `SpeechGrammarList`*
    pub fn length(this: &SpeechGrammarList) -> u32;

    #[wasm_bindgen(catch, constructor, js_class = "SpeechGrammarList")]
    ///The `new SpeechGrammarList(..)` constructor, creating a new instance of `SpeechGrammarList`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechGrammarList/SpeechGrammarList)
    ///
    ///*This API requires the following crate features to be activated: `SpeechGrammarList`*
    pub fn new() -> Result<SpeechGrammarList, JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "SpeechGrammarList" , js_name = addFromString ) ]
    ///The `addFromString()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechGrammarList/addFromString)
    ///
    ///*This API requires the following crate features to be activated: `SpeechGrammarList`*
    pub fn add_from_string(this: &SpeechGrammarList, string: &str) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "SpeechGrammarList" , js_name = addFromString ) ]
    ///The `addFromString()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechGrammarList/addFromString)
    ///
    ///*This API requires the following crate features to be activated: `SpeechGrammarList`*
    pub fn add_from_string_with_weight(
        this: &SpeechGrammarList,
        string: &str,
        weight: f32,
    ) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "SpeechGrammarList" , js_name = addFromURI ) ]
    ///The `addFromURI()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechGrammarList/addFromURI)
    ///
    ///*This API requires the following crate features to be activated: `SpeechGrammarList`*
    pub fn add_from_uri(this: &SpeechGrammarList, src: &str) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "SpeechGrammarList" , js_name = addFromURI ) ]
    ///The `addFromURI()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechGrammarList/addFromURI)
    ///
    ///*This API requires the following crate features to be activated: `SpeechGrammarList`*
    pub fn add_from_uri_with_weight(
        this: &SpeechGrammarList,
        src: &str,
        weight: f32,
    ) -> Result<(), JsValue>;

    #[cfg(feature = "SpeechGrammar")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "SpeechGrammarList" , js_name = item ) ]
    ///The `item()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechGrammarList/item)
    ///
    ///*This API requires the following crate features to be activated: `SpeechGrammar`, `SpeechGrammarList`*
    pub fn item(this: &SpeechGrammarList, index: u32) -> Result<SpeechGrammar, JsValue>;

    #[cfg(feature = "SpeechGrammar")]
    #[wasm_bindgen(
        catch,
        method,
        structural,
        js_class = "SpeechGrammarList",
        indexing_getter
    )]
    ///Indexing getter.
    ///
    ///
    ///
    ///*This API requires the following crate features to be activated: `SpeechGrammar`, `SpeechGrammarList`*
    pub fn get(this: &SpeechGrammarList, index: u32) -> Result<SpeechGrammar, JsValue>;

}
