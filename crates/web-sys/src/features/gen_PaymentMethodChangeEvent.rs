use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = PaymentRequestUpdateEvent , extends = Event , extends = :: js_sys :: Object , js_name = PaymentMethodChangeEvent , typescript_type = "PaymentMethodChangeEvent" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `PaymentMethodChangeEvent` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PaymentMethodChangeEvent)
    ///
    ///*This API requires the following crate features to be activated: `PaymentMethodChangeEvent`*
    pub type PaymentMethodChangeEvent;

    # [ wasm_bindgen ( structural , method , getter , js_class = "PaymentMethodChangeEvent" , js_name = methodName ) ]
    ///Getter for the `methodName` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PaymentMethodChangeEvent/methodName)
    ///
    ///*This API requires the following crate features to be activated: `PaymentMethodChangeEvent`*
    pub fn method_name(this: &PaymentMethodChangeEvent) -> String;

    # [ wasm_bindgen ( structural , method , getter , js_class = "PaymentMethodChangeEvent" , js_name = methodDetails ) ]
    ///Getter for the `methodDetails` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PaymentMethodChangeEvent/methodDetails)
    ///
    ///*This API requires the following crate features to be activated: `PaymentMethodChangeEvent`*
    pub fn method_details(this: &PaymentMethodChangeEvent) -> Option<::js_sys::Object>;

    #[wasm_bindgen(catch, constructor, js_class = "PaymentMethodChangeEvent")]
    ///The `new PaymentMethodChangeEvent(..)` constructor, creating a new instance of `PaymentMethodChangeEvent`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PaymentMethodChangeEvent/PaymentMethodChangeEvent)
    ///
    ///*This API requires the following crate features to be activated: `PaymentMethodChangeEvent`*
    pub fn new(type_: &str) -> Result<PaymentMethodChangeEvent, JsValue>;

    #[cfg(feature = "PaymentMethodChangeEventInit")]
    #[wasm_bindgen(catch, constructor, js_class = "PaymentMethodChangeEvent")]
    ///The `new PaymentMethodChangeEvent(..)` constructor, creating a new instance of `PaymentMethodChangeEvent`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PaymentMethodChangeEvent/PaymentMethodChangeEvent)
    ///
    ///*This API requires the following crate features to be activated: `PaymentMethodChangeEvent`, `PaymentMethodChangeEventInit`*
    pub fn new_with_event_init_dict(
        type_: &str,
        event_init_dict: &PaymentMethodChangeEventInit,
    ) -> Result<PaymentMethodChangeEvent, JsValue>;

}
