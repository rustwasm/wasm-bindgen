use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = Event , extends = :: js_sys :: Object , js_name = SpeechRecognitionError , typescript_name = SpeechRecognitionError ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `SpeechRecognitionError` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognitionError)
    ///
    ///*This API requires the following crate features to be activated: `SpeechRecognitionError`*
    pub type SpeechRecognitionError;

    #[cfg(feature = "SpeechRecognitionErrorCode")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "SpeechRecognitionError" , js_name = error ) ]
    ///Getter for the `error` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognitionError/error)
    ///
    ///*This API requires the following crate features to be activated: `SpeechRecognitionError`, `SpeechRecognitionErrorCode`*
    pub fn error(this: &SpeechRecognitionError) -> SpeechRecognitionErrorCode;

    # [ wasm_bindgen ( structural , method , getter , js_class = "SpeechRecognitionError" , js_name = message ) ]
    ///Getter for the `message` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognitionError/message)
    ///
    ///*This API requires the following crate features to be activated: `SpeechRecognitionError`*
    pub fn message(this: &SpeechRecognitionError) -> Option<String>;

    #[wasm_bindgen(catch, constructor, js_class = "SpeechRecognitionError")]
    ///The `new SpeechRecognitionError(..)` constructor, creating a new instance of `SpeechRecognitionError`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognitionError/SpeechRecognitionError)
    ///
    ///*This API requires the following crate features to be activated: `SpeechRecognitionError`*
    pub fn new(type_: &str) -> Result<SpeechRecognitionError, JsValue>;

    #[cfg(feature = "SpeechRecognitionErrorInit")]
    #[wasm_bindgen(catch, constructor, js_class = "SpeechRecognitionError")]
    ///The `new SpeechRecognitionError(..)` constructor, creating a new instance of `SpeechRecognitionError`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognitionError/SpeechRecognitionError)
    ///
    ///*This API requires the following crate features to be activated: `SpeechRecognitionError`, `SpeechRecognitionErrorInit`*
    pub fn new_with_event_init_dict(
        type_: &str,
        event_init_dict: &SpeechRecognitionErrorInit,
    ) -> Result<SpeechRecognitionError, JsValue>;

}
