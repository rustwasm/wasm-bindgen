use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = Event , extends = :: js_sys :: Object , js_name = PromiseRejectionEvent , typescript_name = PromiseRejectionEvent ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `PromiseRejectionEvent` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PromiseRejectionEvent)
    ///
    ///*This API requires the following crate features to be activated: `PromiseRejectionEvent`*
    pub type PromiseRejectionEvent;

    # [ wasm_bindgen ( structural , method , getter , js_class = "PromiseRejectionEvent" , js_name = promise ) ]
    ///Getter for the `promise` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PromiseRejectionEvent/promise)
    ///
    ///*This API requires the following crate features to be activated: `PromiseRejectionEvent`*
    pub fn promise(this: &PromiseRejectionEvent) -> ::js_sys::Promise;

    # [ wasm_bindgen ( structural , method , getter , js_class = "PromiseRejectionEvent" , js_name = reason ) ]
    ///Getter for the `reason` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PromiseRejectionEvent/reason)
    ///
    ///*This API requires the following crate features to be activated: `PromiseRejectionEvent`*
    pub fn reason(this: &PromiseRejectionEvent) -> ::wasm_bindgen::JsValue;

    #[cfg(feature = "PromiseRejectionEventInit")]
    #[wasm_bindgen(catch, constructor, js_class = "PromiseRejectionEvent")]
    ///The `new PromiseRejectionEvent(..)` constructor, creating a new instance of `PromiseRejectionEvent`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PromiseRejectionEvent/PromiseRejectionEvent)
    ///
    ///*This API requires the following crate features to be activated: `PromiseRejectionEvent`, `PromiseRejectionEventInit`*
    pub fn new(
        type_: &str,
        event_init_dict: &PromiseRejectionEventInit,
    ) -> Result<PromiseRejectionEvent, JsValue>;

}
