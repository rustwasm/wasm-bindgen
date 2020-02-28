use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = Event , extends = :: js_sys :: Object , js_name = SpeechRecognitionEvent , typescript_name = SpeechRecognitionEvent ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SpeechRecognitionEvent` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognitionEvent)\n\n*This API requires the following crate features to be activated: `SpeechRecognitionEvent`*"]
    pub type SpeechRecognitionEvent;
    # [ wasm_bindgen ( structural , method , getter , js_name = resultIndex ) ]
    #[doc = "Getter for the `resultIndex` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognitionEvent/resultIndex)\n\n*This API requires the following crate features to be activated: `SpeechRecognitionEvent`*"]
    pub fn result_index(this: &SpeechRecognitionEvent) -> u32;
    # [ wasm_bindgen ( structural , method , getter , js_name = results ) ]
    #[cfg(feature = "SpeechRecognitionResultList")]
    #[doc = "Getter for the `results` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognitionEvent/results)\n\n*This API requires the following crate features to be activated: `SpeechRecognitionEvent`, `SpeechRecognitionResultList`*"]
    pub fn results(this: &SpeechRecognitionEvent) -> Option<SpeechRecognitionResultList>;
    # [ wasm_bindgen ( structural , method , getter , js_name = interpretation ) ]
    #[doc = "Getter for the `interpretation` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognitionEvent/interpretation)\n\n*This API requires the following crate features to be activated: `SpeechRecognitionEvent`*"]
    pub fn interpretation(this: &SpeechRecognitionEvent) -> ::wasm_bindgen::JsValue;
    # [ wasm_bindgen ( structural , method , getter , js_name = emma ) ]
    #[cfg(feature = "Document")]
    #[doc = "Getter for the `emma` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognitionEvent/emma)\n\n*This API requires the following crate features to be activated: `Document`, `SpeechRecognitionEvent`*"]
    pub fn emma(this: &SpeechRecognitionEvent) -> Option<Document>;
    #[wasm_bindgen(catch, constructor)]
    #[doc = "The `new SpeechRecognitionEvent(..)` constructor, creating a new instance of `SpeechRecognitionEvent`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognitionEvent/SpeechRecognitionEvent)\n\n*This API requires the following crate features to be activated: `SpeechRecognitionEvent`*"]
    pub fn new(
        this: &SpeechRecognitionEvent,
        type_: &str,
    ) -> Result<SpeechRecognitionEvent, JsValue>;
    #[cfg(feature = "SpeechRecognitionEventInit")]
    #[wasm_bindgen(catch, constructor)]
    #[doc = "The `new SpeechRecognitionEvent(..)` constructor, creating a new instance of `SpeechRecognitionEvent`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognitionEvent/SpeechRecognitionEvent)\n\n*This API requires the following crate features to be activated: `SpeechRecognitionEvent`, `SpeechRecognitionEventInit`*"]
    pub fn new_with_event_init_dict(
        this: &SpeechRecognitionEvent,
        type_: &str,
        event_init_dict: &SpeechRecognitionEventInit,
    ) -> Result<SpeechRecognitionEvent, JsValue>;
}
