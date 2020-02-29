use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = Event , extends = :: js_sys :: Object , js_name = PresentationConnectionCloseEvent , typescript_name = PresentationConnectionCloseEvent ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `PresentationConnectionCloseEvent` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PresentationConnectionCloseEvent)
    ///
    ///*This API requires the following crate features to be activated: `PresentationConnectionCloseEvent`*
    pub type PresentationConnectionCloseEvent;

    #[cfg(feature = "PresentationConnectionClosedReason")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "PresentationConnectionCloseEvent" , js_name = reason ) ]
    ///Getter for the `reason` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PresentationConnectionCloseEvent/reason)
    ///
    ///*This API requires the following crate features to be activated: `PresentationConnectionCloseEvent`, `PresentationConnectionClosedReason`*
    pub fn reason(this: &PresentationConnectionCloseEvent) -> PresentationConnectionClosedReason;

    # [ wasm_bindgen ( structural , method , getter , js_class = "PresentationConnectionCloseEvent" , js_name = message ) ]
    ///Getter for the `message` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PresentationConnectionCloseEvent/message)
    ///
    ///*This API requires the following crate features to be activated: `PresentationConnectionCloseEvent`*
    pub fn message(this: &PresentationConnectionCloseEvent) -> String;

    #[cfg(feature = "PresentationConnectionCloseEventInit")]
    #[wasm_bindgen(catch, constructor, js_class = "PresentationConnectionCloseEvent")]
    ///The `new PresentationConnectionCloseEvent(..)` constructor, creating a new instance of `PresentationConnectionCloseEvent`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PresentationConnectionCloseEvent/PresentationConnectionCloseEvent)
    ///
    ///*This API requires the following crate features to be activated: `PresentationConnectionCloseEvent`, `PresentationConnectionCloseEventInit`*
    pub fn new(
        type_: &str,
        event_init_dict: &PresentationConnectionCloseEventInit,
    ) -> Result<PresentationConnectionCloseEvent, JsValue>;

}
