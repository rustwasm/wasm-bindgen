use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = SpeechRecognitionResult , typescript_type = "SpeechRecognitionResult" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `SpeechRecognitionResult` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognitionResult)
    ///
    ///*This API requires the following crate features to be activated: `SpeechRecognitionResult`*
    pub type SpeechRecognitionResult;

    # [ wasm_bindgen ( structural , method , getter , js_class = "SpeechRecognitionResult" , js_name = length ) ]
    ///Getter for the `length` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognitionResult/length)
    ///
    ///*This API requires the following crate features to be activated: `SpeechRecognitionResult`*
    pub fn length(this: &SpeechRecognitionResult) -> u32;

    # [ wasm_bindgen ( structural , method , getter , js_class = "SpeechRecognitionResult" , js_name = isFinal ) ]
    ///Getter for the `isFinal` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognitionResult/isFinal)
    ///
    ///*This API requires the following crate features to be activated: `SpeechRecognitionResult`*
    pub fn is_final(this: &SpeechRecognitionResult) -> bool;

    #[cfg(feature = "SpeechRecognitionAlternative")]
    # [ wasm_bindgen ( method , structural , js_class = "SpeechRecognitionResult" , js_name = item ) ]
    ///The `item()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognitionResult/item)
    ///
    ///*This API requires the following crate features to be activated: `SpeechRecognitionAlternative`, `SpeechRecognitionResult`*
    pub fn item(this: &SpeechRecognitionResult, index: u32) -> SpeechRecognitionAlternative;

    #[cfg(feature = "SpeechRecognitionAlternative")]
    #[wasm_bindgen(
        method,
        structural,
        js_class = "SpeechRecognitionResult",
        indexing_getter
    )]
    ///Indexing getter.
    ///
    ///
    ///
    ///*This API requires the following crate features to be activated: `SpeechRecognitionAlternative`, `SpeechRecognitionResult`*
    pub fn get(this: &SpeechRecognitionResult, index: u32) -> Option<SpeechRecognitionAlternative>;

}
