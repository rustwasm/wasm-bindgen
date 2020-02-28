use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = Event , extends = :: js_sys :: Object , js_name = SpeechSynthesisEvent , typescript_name = SpeechSynthesisEvent ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SpeechSynthesisEvent` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesisEvent)\n\n*This API requires the following crate features to be activated: `SpeechSynthesisEvent`*"]
    pub type SpeechSynthesisEvent;
    # [ wasm_bindgen ( structural , method , getter , js_name = utterance ) ]
    #[cfg(feature = "SpeechSynthesisUtterance")]
    #[doc = "Getter for the `utterance` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesisEvent/utterance)\n\n*This API requires the following crate features to be activated: `SpeechSynthesisEvent`, `SpeechSynthesisUtterance`*"]
    pub fn utterance(this: &SpeechSynthesisEvent) -> SpeechSynthesisUtterance;
    # [ wasm_bindgen ( structural , method , getter , js_name = charIndex ) ]
    #[doc = "Getter for the `charIndex` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesisEvent/charIndex)\n\n*This API requires the following crate features to be activated: `SpeechSynthesisEvent`*"]
    pub fn char_index(this: &SpeechSynthesisEvent) -> u32;
    # [ wasm_bindgen ( structural , method , getter , js_name = charLength ) ]
    #[doc = "Getter for the `charLength` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesisEvent/charLength)\n\n*This API requires the following crate features to be activated: `SpeechSynthesisEvent`*"]
    pub fn char_length(this: &SpeechSynthesisEvent) -> Option<u32>;
    # [ wasm_bindgen ( structural , method , getter , js_name = elapsedTime ) ]
    #[doc = "Getter for the `elapsedTime` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesisEvent/elapsedTime)\n\n*This API requires the following crate features to be activated: `SpeechSynthesisEvent`*"]
    pub fn elapsed_time(this: &SpeechSynthesisEvent) -> f32;
    # [ wasm_bindgen ( structural , method , getter , js_name = name ) ]
    #[doc = "Getter for the `name` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesisEvent/name)\n\n*This API requires the following crate features to be activated: `SpeechSynthesisEvent`*"]
    pub fn name(this: &SpeechSynthesisEvent) -> Option<String>;
    #[cfg(feature = "SpeechSynthesisEventInit")]
    #[wasm_bindgen(catch, constructor)]
    #[doc = "The `new SpeechSynthesisEvent(..)` constructor, creating a new instance of `SpeechSynthesisEvent`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesisEvent/SpeechSynthesisEvent)\n\n*This API requires the following crate features to be activated: `SpeechSynthesisEvent`, `SpeechSynthesisEventInit`*"]
    pub fn new(
        this: &SpeechSynthesisEvent,
        type_: &str,
        event_init_dict: &SpeechSynthesisEventInit,
    ) -> Result<SpeechSynthesisEvent, JsValue>;
}
