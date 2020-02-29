use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = Event , extends = :: js_sys :: Object , js_name = TransitionEvent , typescript_type = "TransitionEvent" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `TransitionEvent` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TransitionEvent)
    ///
    ///*This API requires the following crate features to be activated: `TransitionEvent`*
    pub type TransitionEvent;

    # [ wasm_bindgen ( structural , method , getter , js_class = "TransitionEvent" , js_name = propertyName ) ]
    ///Getter for the `propertyName` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TransitionEvent/propertyName)
    ///
    ///*This API requires the following crate features to be activated: `TransitionEvent`*
    pub fn property_name(this: &TransitionEvent) -> String;

    # [ wasm_bindgen ( structural , method , getter , js_class = "TransitionEvent" , js_name = elapsedTime ) ]
    ///Getter for the `elapsedTime` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TransitionEvent/elapsedTime)
    ///
    ///*This API requires the following crate features to be activated: `TransitionEvent`*
    pub fn elapsed_time(this: &TransitionEvent) -> f32;

    # [ wasm_bindgen ( structural , method , getter , js_class = "TransitionEvent" , js_name = pseudoElement ) ]
    ///Getter for the `pseudoElement` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TransitionEvent/pseudoElement)
    ///
    ///*This API requires the following crate features to be activated: `TransitionEvent`*
    pub fn pseudo_element(this: &TransitionEvent) -> String;

    #[wasm_bindgen(catch, constructor, js_class = "TransitionEvent")]
    ///The `new TransitionEvent(..)` constructor, creating a new instance of `TransitionEvent`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TransitionEvent/TransitionEvent)
    ///
    ///*This API requires the following crate features to be activated: `TransitionEvent`*
    pub fn new(type_: &str) -> Result<TransitionEvent, JsValue>;

    #[cfg(feature = "TransitionEventInit")]
    #[wasm_bindgen(catch, constructor, js_class = "TransitionEvent")]
    ///The `new TransitionEvent(..)` constructor, creating a new instance of `TransitionEvent`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TransitionEvent/TransitionEvent)
    ///
    ///*This API requires the following crate features to be activated: `TransitionEvent`, `TransitionEventInit`*
    pub fn new_with_event_init_dict(
        type_: &str,
        event_init_dict: &TransitionEventInit,
    ) -> Result<TransitionEvent, JsValue>;

}
