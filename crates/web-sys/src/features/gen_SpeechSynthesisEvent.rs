use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = Event , extends = :: js_sys :: Object , js_name = SpeechSynthesisEvent , typescript_name = SpeechSynthesisEvent ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `SpeechSynthesisEvent` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesisEvent)
    ///
    ///*This API requires the following crate features to be activated: `SpeechSynthesisEvent`*
    pub type SpeechSynthesisEvent;

    #[cfg(feature = "SpeechSynthesisUtterance")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "SpeechSynthesisEvent" , js_name = utterance ) ]
    ///Getter for the `utterance` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesisEvent/utterance)
    ///
    ///*This API requires the following crate features to be activated: `SpeechSynthesisEvent`, `SpeechSynthesisUtterance`*
    pub fn utterance(this: &SpeechSynthesisEvent) -> SpeechSynthesisUtterance;

    # [ wasm_bindgen ( structural , method , getter , js_class = "SpeechSynthesisEvent" , js_name = charIndex ) ]
    ///Getter for the `charIndex` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesisEvent/charIndex)
    ///
    ///*This API requires the following crate features to be activated: `SpeechSynthesisEvent`*
    pub fn char_index(this: &SpeechSynthesisEvent) -> u32;

    # [ wasm_bindgen ( structural , method , getter , js_class = "SpeechSynthesisEvent" , js_name = charLength ) ]
    ///Getter for the `charLength` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesisEvent/charLength)
    ///
    ///*This API requires the following crate features to be activated: `SpeechSynthesisEvent`*
    pub fn char_length(this: &SpeechSynthesisEvent) -> Option<u32>;

    # [ wasm_bindgen ( structural , method , getter , js_class = "SpeechSynthesisEvent" , js_name = elapsedTime ) ]
    ///Getter for the `elapsedTime` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesisEvent/elapsedTime)
    ///
    ///*This API requires the following crate features to be activated: `SpeechSynthesisEvent`*
    pub fn elapsed_time(this: &SpeechSynthesisEvent) -> f32;

    # [ wasm_bindgen ( structural , method , getter , js_class = "SpeechSynthesisEvent" , js_name = name ) ]
    ///Getter for the `name` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesisEvent/name)
    ///
    ///*This API requires the following crate features to be activated: `SpeechSynthesisEvent`*
    pub fn name(this: &SpeechSynthesisEvent) -> Option<String>;

    #[cfg(feature = "SpeechSynthesisEventInit")]
    #[wasm_bindgen(catch, constructor, js_class = "SpeechSynthesisEvent")]
    ///The `new SpeechSynthesisEvent(..)` constructor, creating a new instance of `SpeechSynthesisEvent`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesisEvent/SpeechSynthesisEvent)
    ///
    ///*This API requires the following crate features to be activated: `SpeechSynthesisEvent`, `SpeechSynthesisEventInit`*
    pub fn new(
        type_: &str,
        event_init_dict: &SpeechSynthesisEventInit,
    ) -> Result<SpeechSynthesisEvent, JsValue>;

}
