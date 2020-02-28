use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = ExtendableEvent , extends = Event , extends = :: js_sys :: Object , js_name = PushEvent , typescript_name = PushEvent ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `PushEvent` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PushEvent)\n\n*This API requires the following crate features to be activated: `PushEvent`*"]
    pub type PushEvent;
    # [ wasm_bindgen ( structural , method , getter , js_class = "PushEvent" , js_name = data ) ]
    #[cfg(feature = "PushMessageData")]
    #[doc = "Getter for the `data` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PushEvent/data)\n\n*This API requires the following crate features to be activated: `PushEvent`, `PushMessageData`*"]
    pub fn data(this: &PushEvent) -> Option<PushMessageData>;
    #[wasm_bindgen(catch, js_class = "PushEvent", constructor)]
    #[doc = "The `new PushEvent(..)` constructor, creating a new instance of `PushEvent`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PushEvent/PushEvent)\n\n*This API requires the following crate features to be activated: `PushEvent`*"]
    pub fn new(this: &PushEvent, type_: &str) -> Result<PushEvent, JsValue>;
    #[cfg(feature = "PushEventInit")]
    #[wasm_bindgen(catch, js_class = "PushEvent", constructor)]
    #[doc = "The `new PushEvent(..)` constructor, creating a new instance of `PushEvent`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PushEvent/PushEvent)\n\n*This API requires the following crate features to be activated: `PushEvent`, `PushEventInit`*"]
    pub fn new_with_event_init_dict(
        this: &PushEvent,
        type_: &str,
        event_init_dict: &PushEventInit,
    ) -> Result<PushEvent, JsValue>;
}
