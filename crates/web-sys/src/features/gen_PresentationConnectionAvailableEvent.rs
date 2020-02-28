use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = Event , extends = :: js_sys :: Object , js_name = PresentationConnectionAvailableEvent , typescript_name = PresentationConnectionAvailableEvent ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `PresentationConnectionAvailableEvent` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PresentationConnectionAvailableEvent)\n\n*This API requires the following crate features to be activated: `PresentationConnectionAvailableEvent`*"]
    pub type PresentationConnectionAvailableEvent;
    # [ wasm_bindgen ( structural , method , getter , js_name = connection ) ]
    #[cfg(feature = "PresentationConnection")]
    #[doc = "Getter for the `connection` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PresentationConnectionAvailableEvent/connection)\n\n*This API requires the following crate features to be activated: `PresentationConnection`, `PresentationConnectionAvailableEvent`*"]
    pub fn connection(this: &PresentationConnectionAvailableEvent) -> PresentationConnection;
    #[cfg(feature = "PresentationConnectionAvailableEventInit")]
    #[wasm_bindgen(catch, constructor)]
    #[doc = "The `new PresentationConnectionAvailableEvent(..)` constructor, creating a new instance of `PresentationConnectionAvailableEvent`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PresentationConnectionAvailableEvent/PresentationConnectionAvailableEvent)\n\n*This API requires the following crate features to be activated: `PresentationConnectionAvailableEvent`, `PresentationConnectionAvailableEventInit`*"]
    pub fn new(
        this: &PresentationConnectionAvailableEvent,
        type_: &str,
        event_init_dict: &PresentationConnectionAvailableEventInit,
    ) -> Result<PresentationConnectionAvailableEvent, JsValue>;
}
