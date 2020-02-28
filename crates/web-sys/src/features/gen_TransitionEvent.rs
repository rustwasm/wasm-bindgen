use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = Event , extends = :: js_sys :: Object , js_name = TransitionEvent , typescript_name = TransitionEvent ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `TransitionEvent` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TransitionEvent)\n\n*This API requires the following crate features to be activated: `TransitionEvent`*"]
    pub type TransitionEvent;
    # [ wasm_bindgen ( structural , method , getter , js_name = propertyName ) ]
    #[doc = "Getter for the `propertyName` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TransitionEvent/propertyName)\n\n*This API requires the following crate features to be activated: `TransitionEvent`*"]
    pub fn property_name(this: &TransitionEvent) -> String;
    # [ wasm_bindgen ( structural , method , getter , js_name = elapsedTime ) ]
    #[doc = "Getter for the `elapsedTime` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TransitionEvent/elapsedTime)\n\n*This API requires the following crate features to be activated: `TransitionEvent`*"]
    pub fn elapsed_time(this: &TransitionEvent) -> f32;
    # [ wasm_bindgen ( structural , method , getter , js_name = pseudoElement ) ]
    #[doc = "Getter for the `pseudoElement` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TransitionEvent/pseudoElement)\n\n*This API requires the following crate features to be activated: `TransitionEvent`*"]
    pub fn pseudo_element(this: &TransitionEvent) -> String;
    #[wasm_bindgen(catch, constructor)]
    #[doc = "The `new TransitionEvent(..)` constructor, creating a new instance of `TransitionEvent`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TransitionEvent/TransitionEvent)\n\n*This API requires the following crate features to be activated: `TransitionEvent`*"]
    pub fn new(this: &TransitionEvent, type_: &str) -> Result<TransitionEvent, JsValue>;
    #[cfg(feature = "TransitionEventInit")]
    #[wasm_bindgen(catch, constructor)]
    #[doc = "The `new TransitionEvent(..)` constructor, creating a new instance of `TransitionEvent`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TransitionEvent/TransitionEvent)\n\n*This API requires the following crate features to be activated: `TransitionEvent`, `TransitionEventInit`*"]
    pub fn new_with_event_init_dict(
        this: &TransitionEvent,
        type_: &str,
        event_init_dict: &TransitionEventInit,
    ) -> Result<TransitionEvent, JsValue>;
}
