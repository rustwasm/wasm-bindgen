use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = Event , extends = :: js_sys :: Object , js_name = PresentationConnectionAvailableEvent , typescript_type = "PresentationConnectionAvailableEvent" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `PresentationConnectionAvailableEvent` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PresentationConnectionAvailableEvent)
    ///
    ///*This API requires the following crate features to be activated: `PresentationConnectionAvailableEvent`*
    pub type PresentationConnectionAvailableEvent;

    #[cfg(feature = "PresentationConnection")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "PresentationConnectionAvailableEvent" , js_name = connection ) ]
    ///Getter for the `connection` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PresentationConnectionAvailableEvent/connection)
    ///
    ///*This API requires the following crate features to be activated: `PresentationConnection`, `PresentationConnectionAvailableEvent`*
    pub fn connection(this: &PresentationConnectionAvailableEvent) -> PresentationConnection;

    #[cfg(feature = "PresentationConnectionAvailableEventInit")]
    #[wasm_bindgen(catch, constructor, js_class = "PresentationConnectionAvailableEvent")]
    ///The `new PresentationConnectionAvailableEvent(..)` constructor, creating a new instance of `PresentationConnectionAvailableEvent`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PresentationConnectionAvailableEvent/PresentationConnectionAvailableEvent)
    ///
    ///*This API requires the following crate features to be activated: `PresentationConnectionAvailableEvent`, `PresentationConnectionAvailableEventInit`*
    pub fn new(
        type_: &str,
        event_init_dict: &PresentationConnectionAvailableEventInit,
    ) -> Result<PresentationConnectionAvailableEvent, JsValue>;

}
