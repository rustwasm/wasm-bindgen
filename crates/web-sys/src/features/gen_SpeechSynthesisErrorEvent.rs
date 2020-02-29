use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = SpeechSynthesisEvent , extends = Event , extends = :: js_sys :: Object , js_name = SpeechSynthesisErrorEvent , typescript_type = "SpeechSynthesisErrorEvent" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `SpeechSynthesisErrorEvent` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesisErrorEvent)
    ///
    ///*This API requires the following crate features to be activated: `SpeechSynthesisErrorEvent`*
    pub type SpeechSynthesisErrorEvent;

    #[cfg(feature = "SpeechSynthesisErrorCode")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "SpeechSynthesisErrorEvent" , js_name = error ) ]
    ///Getter for the `error` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesisErrorEvent/error)
    ///
    ///*This API requires the following crate features to be activated: `SpeechSynthesisErrorCode`, `SpeechSynthesisErrorEvent`*
    pub fn error(this: &SpeechSynthesisErrorEvent) -> SpeechSynthesisErrorCode;

    #[cfg(feature = "SpeechSynthesisErrorEventInit")]
    #[wasm_bindgen(catch, constructor, js_class = "SpeechSynthesisErrorEvent")]
    ///The `new SpeechSynthesisErrorEvent(..)` constructor, creating a new instance of `SpeechSynthesisErrorEvent`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesisErrorEvent/SpeechSynthesisErrorEvent)
    ///
    ///*This API requires the following crate features to be activated: `SpeechSynthesisErrorEvent`, `SpeechSynthesisErrorEventInit`*
    pub fn new(
        type_: &str,
        event_init_dict: &SpeechSynthesisErrorEventInit,
    ) -> Result<SpeechSynthesisErrorEvent, JsValue>;

}
