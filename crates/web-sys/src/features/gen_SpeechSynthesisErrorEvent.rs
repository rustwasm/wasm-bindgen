use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = SpeechSynthesisEvent , extends = Event , extends = :: js_sys :: Object , js_name = SpeechSynthesisErrorEvent , typescript_name = SpeechSynthesisErrorEvent ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SpeechSynthesisErrorEvent` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesisErrorEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechSynthesisErrorEvent`*"]
    pub type SpeechSynthesisErrorEvent;
    # [ wasm_bindgen ( structural , method , getter , js_class = "SpeechSynthesisErrorEvent" , js_name = error ) ]
    #[cfg(feature = "SpeechSynthesisErrorCode")]
    #[doc = "Getter for the `error` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesisErrorEvent/error)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechSynthesisErrorCode`, `SpeechSynthesisErrorEvent`*"]
    pub fn error(this: &SpeechSynthesisErrorEvent) -> SpeechSynthesisErrorCode;
    #[cfg(feature = "SpeechSynthesisErrorEventInit")]
    #[wasm_bindgen(catch, js_class = "SpeechSynthesisErrorEvent", constructor)]
    #[doc = "The `new SpeechSynthesisErrorEvent(..)` constructor, creating a new instance of `SpeechSynthesisErrorEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesisErrorEvent/SpeechSynthesisErrorEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechSynthesisErrorEvent`, `SpeechSynthesisErrorEventInit`*"]
    pub fn new(
        this: &SpeechSynthesisErrorEvent,
        type_: &str,
        event_init_dict: &SpeechSynthesisErrorEventInit,
    ) -> Result<SpeechSynthesisErrorEvent, JsValue>;
}
