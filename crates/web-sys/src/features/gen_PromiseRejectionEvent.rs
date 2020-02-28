use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = Event , extends = :: js_sys :: Object , js_name = PromiseRejectionEvent , typescript_name = PromiseRejectionEvent ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `PromiseRejectionEvent` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PromiseRejectionEvent)\n\n*This API requires the following crate features to be activated: `PromiseRejectionEvent`*"]
    pub type PromiseRejectionEvent;
    # [ wasm_bindgen ( structural , method , getter , js_name = promise ) ]
    #[doc = "Getter for the `promise` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PromiseRejectionEvent/promise)\n\n*This API requires the following crate features to be activated: `PromiseRejectionEvent`*"]
    pub fn promise(this: &PromiseRejectionEvent) -> ::js_sys::Promise;
    # [ wasm_bindgen ( structural , method , getter , js_name = reason ) ]
    #[doc = "Getter for the `reason` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PromiseRejectionEvent/reason)\n\n*This API requires the following crate features to be activated: `PromiseRejectionEvent`*"]
    pub fn reason(this: &PromiseRejectionEvent) -> ::wasm_bindgen::JsValue;
    #[cfg(feature = "PromiseRejectionEventInit")]
    #[wasm_bindgen(catch, constructor)]
    #[doc = "The `new PromiseRejectionEvent(..)` constructor, creating a new instance of `PromiseRejectionEvent`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PromiseRejectionEvent/PromiseRejectionEvent)\n\n*This API requires the following crate features to be activated: `PromiseRejectionEvent`, `PromiseRejectionEventInit`*"]
    pub fn new(
        this: &PromiseRejectionEvent,
        type_: &str,
        event_init_dict: &PromiseRejectionEventInit,
    ) -> Result<PromiseRejectionEvent, JsValue>;
}
