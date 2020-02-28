use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = Event , extends = :: js_sys :: Object , js_name = SpeechRecognitionError , typescript_name = SpeechRecognitionError ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SpeechRecognitionError` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognitionError)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechRecognitionError`*"]
    pub type SpeechRecognitionError;
    # [ wasm_bindgen ( structural , method , getter , js_class = "SpeechRecognitionError" , js_name = error ) ]
    #[cfg(feature = "SpeechRecognitionErrorCode")]
    #[doc = "Getter for the `error` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognitionError/error)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechRecognitionError`, `SpeechRecognitionErrorCode`*"]
    pub fn error(this: &SpeechRecognitionError) -> SpeechRecognitionErrorCode;
    # [ wasm_bindgen ( structural , method , getter , js_class = "SpeechRecognitionError" , js_name = message ) ]
    #[doc = "Getter for the `message` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognitionError/message)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechRecognitionError`*"]
    pub fn message(this: &SpeechRecognitionError) -> Option<String>;
    #[wasm_bindgen(catch, js_class = "SpeechRecognitionError", constructor)]
    #[doc = "The `new SpeechRecognitionError(..)` constructor, creating a new instance of `SpeechRecognitionError`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognitionError/SpeechRecognitionError)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechRecognitionError`*"]
    pub fn new(
        this: &SpeechRecognitionError,
        type_: &str,
    ) -> Result<SpeechRecognitionError, JsValue>;
    #[cfg(feature = "SpeechRecognitionErrorInit")]
    #[wasm_bindgen(catch, js_class = "SpeechRecognitionError", constructor)]
    #[doc = "The `new SpeechRecognitionError(..)` constructor, creating a new instance of `SpeechRecognitionError`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognitionError/SpeechRecognitionError)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechRecognitionError`, `SpeechRecognitionErrorInit`*"]
    pub fn new_with_event_init_dict(
        this: &SpeechRecognitionError,
        type_: &str,
        event_init_dict: &SpeechRecognitionErrorInit,
    ) -> Result<SpeechRecognitionError, JsValue>;
}
