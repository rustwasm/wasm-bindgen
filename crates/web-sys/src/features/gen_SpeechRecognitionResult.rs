use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = SpeechRecognitionResult , typescript_name = SpeechRecognitionResult ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SpeechRecognitionResult` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognitionResult)\n\n*This API requires the following crate features to be activated: `SpeechRecognitionResult`*"]
    pub type SpeechRecognitionResult;
    # [ wasm_bindgen ( structural , method , getter , js_class = "SpeechRecognitionResult" , js_name = length ) ]
    #[doc = "Getter for the `length` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognitionResult/length)\n\n*This API requires the following crate features to be activated: `SpeechRecognitionResult`*"]
    pub fn length(this: &SpeechRecognitionResult) -> u32;
    # [ wasm_bindgen ( structural , method , getter , js_class = "SpeechRecognitionResult" , js_name = isFinal ) ]
    #[doc = "Getter for the `isFinal` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognitionResult/isFinal)\n\n*This API requires the following crate features to be activated: `SpeechRecognitionResult`*"]
    pub fn is_final(this: &SpeechRecognitionResult) -> bool;
    #[cfg(feature = "SpeechRecognitionAlternative")]
    # [ wasm_bindgen ( method , structural , js_class = "SpeechRecognitionResult" , js_name = item ) ]
    #[doc = "The `item()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognitionResult/item)\n\n*This API requires the following crate features to be activated: `SpeechRecognitionAlternative`, `SpeechRecognitionResult`*"]
    pub fn item(this: &SpeechRecognitionResult, index: u32) -> SpeechRecognitionAlternative;
    #[cfg(feature = "SpeechRecognitionAlternative")]
    #[wasm_bindgen(
        method,
        structural,
        js_class = "SpeechRecognitionResult",
        indexing_getter
    )]
    #[doc = "Indexing getter.\n\n\n\n*This API requires the following crate features to be activated: `SpeechRecognitionAlternative`, `SpeechRecognitionResult`*"]
    pub fn get(this: &SpeechRecognitionResult, index: u32) -> Option<SpeechRecognitionAlternative>;
}
