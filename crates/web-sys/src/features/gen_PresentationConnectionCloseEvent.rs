use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = Event , extends = :: js_sys :: Object , js_name = PresentationConnectionCloseEvent , typescript_name = PresentationConnectionCloseEvent ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `PresentationConnectionCloseEvent` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PresentationConnectionCloseEvent)\n\n*This API requires the following crate features to be activated: `PresentationConnectionCloseEvent`*"]
    pub type PresentationConnectionCloseEvent;
    # [ wasm_bindgen ( structural , method , getter , js_name = reason ) ]
    #[cfg(feature = "PresentationConnectionClosedReason")]
    #[doc = "Getter for the `reason` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PresentationConnectionCloseEvent/reason)\n\n*This API requires the following crate features to be activated: `PresentationConnectionCloseEvent`, `PresentationConnectionClosedReason`*"]
    pub fn reason(this: &PresentationConnectionCloseEvent) -> PresentationConnectionClosedReason;
    # [ wasm_bindgen ( structural , method , getter , js_name = message ) ]
    #[doc = "Getter for the `message` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PresentationConnectionCloseEvent/message)\n\n*This API requires the following crate features to be activated: `PresentationConnectionCloseEvent`*"]
    pub fn message(this: &PresentationConnectionCloseEvent) -> String;
    #[cfg(feature = "PresentationConnectionCloseEventInit")]
    #[wasm_bindgen(catch, constructor)]
    #[doc = "The `new PresentationConnectionCloseEvent(..)` constructor, creating a new instance of `PresentationConnectionCloseEvent`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PresentationConnectionCloseEvent/PresentationConnectionCloseEvent)\n\n*This API requires the following crate features to be activated: `PresentationConnectionCloseEvent`, `PresentationConnectionCloseEventInit`*"]
    pub fn new(
        this: &PresentationConnectionCloseEvent,
        type_: &str,
        event_init_dict: &PresentationConnectionCloseEventInit,
    ) -> Result<PresentationConnectionCloseEvent, JsValue>;
}
