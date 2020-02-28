use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = PaymentRequestUpdateEvent , extends = Event , extends = :: js_sys :: Object , js_name = PaymentMethodChangeEvent , typescript_name = PaymentMethodChangeEvent ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `PaymentMethodChangeEvent` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PaymentMethodChangeEvent)\n\n*This API requires the following crate features to be activated: `PaymentMethodChangeEvent`*"]
    pub type PaymentMethodChangeEvent;
    # [ wasm_bindgen ( structural , method , getter , js_class = "PaymentMethodChangeEvent" , js_name = methodName ) ]
    #[doc = "Getter for the `methodName` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PaymentMethodChangeEvent/methodName)\n\n*This API requires the following crate features to be activated: `PaymentMethodChangeEvent`*"]
    pub fn method_name(this: &PaymentMethodChangeEvent) -> String;
    # [ wasm_bindgen ( structural , method , getter , js_class = "PaymentMethodChangeEvent" , js_name = methodDetails ) ]
    #[doc = "Getter for the `methodDetails` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PaymentMethodChangeEvent/methodDetails)\n\n*This API requires the following crate features to be activated: `PaymentMethodChangeEvent`*"]
    pub fn method_details(this: &PaymentMethodChangeEvent) -> Option<::js_sys::Object>;
    #[wasm_bindgen(catch, js_class = "PaymentMethodChangeEvent", constructor)]
    #[doc = "The `new PaymentMethodChangeEvent(..)` constructor, creating a new instance of `PaymentMethodChangeEvent`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PaymentMethodChangeEvent/PaymentMethodChangeEvent)\n\n*This API requires the following crate features to be activated: `PaymentMethodChangeEvent`*"]
    pub fn new(
        this: &PaymentMethodChangeEvent,
        type_: &str,
    ) -> Result<PaymentMethodChangeEvent, JsValue>;
    #[cfg(feature = "PaymentMethodChangeEventInit")]
    #[wasm_bindgen(catch, js_class = "PaymentMethodChangeEvent", constructor)]
    #[doc = "The `new PaymentMethodChangeEvent(..)` constructor, creating a new instance of `PaymentMethodChangeEvent`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PaymentMethodChangeEvent/PaymentMethodChangeEvent)\n\n*This API requires the following crate features to be activated: `PaymentMethodChangeEvent`, `PaymentMethodChangeEventInit`*"]
    pub fn new_with_event_init_dict(
        this: &PaymentMethodChangeEvent,
        type_: &str,
        event_init_dict: &PaymentMethodChangeEventInit,
    ) -> Result<PaymentMethodChangeEvent, JsValue>;
}
