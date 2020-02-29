use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = SpeechRecognitionAlternative , typescript_type = "SpeechRecognitionAlternative" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `SpeechRecognitionAlternative` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognitionAlternative)
    ///
    ///*This API requires the following crate features to be activated: `SpeechRecognitionAlternative`*
    pub type SpeechRecognitionAlternative;

    # [ wasm_bindgen ( structural , method , getter , js_class = "SpeechRecognitionAlternative" , js_name = transcript ) ]
    ///Getter for the `transcript` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognitionAlternative/transcript)
    ///
    ///*This API requires the following crate features to be activated: `SpeechRecognitionAlternative`*
    pub fn transcript(this: &SpeechRecognitionAlternative) -> String;

    # [ wasm_bindgen ( structural , method , getter , js_class = "SpeechRecognitionAlternative" , js_name = confidence ) ]
    ///Getter for the `confidence` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognitionAlternative/confidence)
    ///
    ///*This API requires the following crate features to be activated: `SpeechRecognitionAlternative`*
    pub fn confidence(this: &SpeechRecognitionAlternative) -> f32;

}
