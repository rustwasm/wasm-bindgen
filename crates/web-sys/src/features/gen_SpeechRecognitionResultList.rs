use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = SpeechRecognitionResultList , typescript_name = SpeechRecognitionResultList ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SpeechRecognitionResultList` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognitionResultList)\n\n*This API requires the following crate features to be activated: `SpeechRecognitionResultList`*"]
    pub type SpeechRecognitionResultList;
    # [ wasm_bindgen ( structural , method , getter , js_name = length ) ]
    #[doc = "Getter for the `length` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognitionResultList/length)\n\n*This API requires the following crate features to be activated: `SpeechRecognitionResultList`*"]
    pub fn length(this: &SpeechRecognitionResultList) -> u32;
    #[cfg(feature = "SpeechRecognitionResult")]
    # [ wasm_bindgen ( method , structural , js_name = item ) ]
    #[doc = "The `item()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognitionResultList/item)\n\n*This API requires the following crate features to be activated: `SpeechRecognitionResult`, `SpeechRecognitionResultList`*"]
    pub fn item(this: &SpeechRecognitionResultList, index: u32) -> SpeechRecognitionResult;
    #[cfg(feature = "SpeechRecognitionResult")]
    #[wasm_bindgen(method, structural, indexing_getter)]
    #[doc = "Indexing getter.\n\n\n\n*This API requires the following crate features to be activated: `SpeechRecognitionResult`, `SpeechRecognitionResultList`*"]
    pub fn get(this: &SpeechRecognitionResultList, index: u32) -> Option<SpeechRecognitionResult>;
}
