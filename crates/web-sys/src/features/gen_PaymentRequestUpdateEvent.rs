use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = Event , extends = :: js_sys :: Object , js_name = PaymentRequestUpdateEvent , typescript_name = PaymentRequestUpdateEvent ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `PaymentRequestUpdateEvent` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PaymentRequestUpdateEvent)\n\n*This API requires the following crate features to be activated: `PaymentRequestUpdateEvent`*"]
    pub type PaymentRequestUpdateEvent;
    #[wasm_bindgen(catch, js_class = "PaymentRequestUpdateEvent", constructor)]
    #[doc = "The `new PaymentRequestUpdateEvent(..)` constructor, creating a new instance of `PaymentRequestUpdateEvent`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PaymentRequestUpdateEvent/PaymentRequestUpdateEvent)\n\n*This API requires the following crate features to be activated: `PaymentRequestUpdateEvent`*"]
    pub fn new(
        this: &PaymentRequestUpdateEvent,
        type_: &str,
    ) -> Result<PaymentRequestUpdateEvent, JsValue>;
    #[cfg(feature = "PaymentRequestUpdateEventInit")]
    #[wasm_bindgen(catch, js_class = "PaymentRequestUpdateEvent", constructor)]
    #[doc = "The `new PaymentRequestUpdateEvent(..)` constructor, creating a new instance of `PaymentRequestUpdateEvent`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PaymentRequestUpdateEvent/PaymentRequestUpdateEvent)\n\n*This API requires the following crate features to be activated: `PaymentRequestUpdateEvent`, `PaymentRequestUpdateEventInit`*"]
    pub fn new_with_event_init_dict(
        this: &PaymentRequestUpdateEvent,
        type_: &str,
        event_init_dict: &PaymentRequestUpdateEventInit,
    ) -> Result<PaymentRequestUpdateEvent, JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_class = "PaymentRequestUpdateEvent" , js_name = updateWith ) ]
    #[doc = "The `updateWith()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PaymentRequestUpdateEvent/updateWith)\n\n*This API requires the following crate features to be activated: `PaymentRequestUpdateEvent`*"]
    pub fn update_with(
        this: &PaymentRequestUpdateEvent,
        details_promise: &::js_sys::Promise,
    ) -> Result<(), JsValue>;
}
