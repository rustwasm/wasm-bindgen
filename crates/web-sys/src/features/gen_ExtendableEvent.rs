use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = Event , extends = :: js_sys :: Object , js_name = ExtendableEvent , typescript_name = ExtendableEvent ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ExtendableEvent` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ExtendableEvent)\n\n*This API requires the following crate features to be activated: `ExtendableEvent`*"]
    pub type ExtendableEvent;
    #[wasm_bindgen(catch, js_class = "ExtendableEvent", constructor)]
    #[doc = "The `new ExtendableEvent(..)` constructor, creating a new instance of `ExtendableEvent`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ExtendableEvent/ExtendableEvent)\n\n*This API requires the following crate features to be activated: `ExtendableEvent`*"]
    pub fn new(this: &ExtendableEvent, type_: &str) -> Result<ExtendableEvent, JsValue>;
    #[cfg(feature = "ExtendableEventInit")]
    #[wasm_bindgen(catch, js_class = "ExtendableEvent", constructor)]
    #[doc = "The `new ExtendableEvent(..)` constructor, creating a new instance of `ExtendableEvent`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ExtendableEvent/ExtendableEvent)\n\n*This API requires the following crate features to be activated: `ExtendableEvent`, `ExtendableEventInit`*"]
    pub fn new_with_event_init_dict(
        this: &ExtendableEvent,
        type_: &str,
        event_init_dict: &ExtendableEventInit,
    ) -> Result<ExtendableEvent, JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_class = "ExtendableEvent" , js_name = waitUntil ) ]
    #[doc = "The `waitUntil()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ExtendableEvent/waitUntil)\n\n*This API requires the following crate features to be activated: `ExtendableEvent`*"]
    pub fn wait_until(this: &ExtendableEvent, p: &::js_sys::Promise) -> Result<(), JsValue>;
}
