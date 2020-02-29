use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = SpeechRecognitionResultList , typescript_name = SpeechRecognitionResultList ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `SpeechRecognitionResultList` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognitionResultList)
    ///
    ///*This API requires the following crate features to be activated: `SpeechRecognitionResultList`*
    pub type SpeechRecognitionResultList;

    # [ wasm_bindgen ( structural , method , getter , js_class = "SpeechRecognitionResultList" , js_name = length ) ]
    ///Getter for the `length` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognitionResultList/length)
    ///
    ///*This API requires the following crate features to be activated: `SpeechRecognitionResultList`*
    pub fn length(this: &SpeechRecognitionResultList) -> u32;

    #[cfg(feature = "SpeechRecognitionResult")]
    # [ wasm_bindgen ( method , structural , js_class = "SpeechRecognitionResultList" , js_name = item ) ]
    ///The `item()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognitionResultList/item)
    ///
    ///*This API requires the following crate features to be activated: `SpeechRecognitionResult`, `SpeechRecognitionResultList`*
    pub fn item(this: &SpeechRecognitionResultList, index: u32) -> SpeechRecognitionResult;

    #[cfg(feature = "SpeechRecognitionResult")]
    #[wasm_bindgen(
        method,
        structural,
        js_class = "SpeechRecognitionResultList",
        indexing_getter
    )]
    ///Indexing getter.
    ///
    ///
    ///
    ///*This API requires the following crate features to be activated: `SpeechRecognitionResult`, `SpeechRecognitionResultList`*
    pub fn get(this: &SpeechRecognitionResultList, index: u32) -> Option<SpeechRecognitionResult>;

}
