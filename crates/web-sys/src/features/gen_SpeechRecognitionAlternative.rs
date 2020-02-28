use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = SpeechRecognitionAlternative , typescript_name = SpeechRecognitionAlternative ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SpeechRecognitionAlternative` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognitionAlternative)\n\n*This API requires the following crate features to be activated: `SpeechRecognitionAlternative`*"]
    pub type SpeechRecognitionAlternative;
    # [ wasm_bindgen ( structural , method , getter , js_class = "SpeechRecognitionAlternative" , js_name = transcript ) ]
    #[doc = "Getter for the `transcript` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognitionAlternative/transcript)\n\n*This API requires the following crate features to be activated: `SpeechRecognitionAlternative`*"]
    pub fn transcript(this: &SpeechRecognitionAlternative) -> String;
    # [ wasm_bindgen ( structural , method , getter , js_class = "SpeechRecognitionAlternative" , js_name = confidence ) ]
    #[doc = "Getter for the `confidence` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognitionAlternative/confidence)\n\n*This API requires the following crate features to be activated: `SpeechRecognitionAlternative`*"]
    pub fn confidence(this: &SpeechRecognitionAlternative) -> f32;
}
