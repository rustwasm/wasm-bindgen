use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = PaymentResponse , typescript_name = PaymentResponse ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `PaymentResponse` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PaymentResponse)
    ///
    ///*This API requires the following crate features to be activated: `PaymentResponse`*
    pub type PaymentResponse;

    # [ wasm_bindgen ( structural , method , getter , js_class = "PaymentResponse" , js_name = requestId ) ]
    ///Getter for the `requestId` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PaymentResponse/requestId)
    ///
    ///*This API requires the following crate features to be activated: `PaymentResponse`*
    pub fn request_id(this: &PaymentResponse) -> String;

    # [ wasm_bindgen ( structural , method , getter , js_class = "PaymentResponse" , js_name = methodName ) ]
    ///Getter for the `methodName` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PaymentResponse/methodName)
    ///
    ///*This API requires the following crate features to be activated: `PaymentResponse`*
    pub fn method_name(this: &PaymentResponse) -> String;

    # [ wasm_bindgen ( structural , method , getter , js_class = "PaymentResponse" , js_name = details ) ]
    ///Getter for the `details` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PaymentResponse/details)
    ///
    ///*This API requires the following crate features to be activated: `PaymentResponse`*
    pub fn details(this: &PaymentResponse) -> ::js_sys::Object;

    #[cfg(feature = "PaymentAddress")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "PaymentResponse" , js_name = shippingAddress ) ]
    ///Getter for the `shippingAddress` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PaymentResponse/shippingAddress)
    ///
    ///*This API requires the following crate features to be activated: `PaymentAddress`, `PaymentResponse`*
    pub fn shipping_address(this: &PaymentResponse) -> Option<PaymentAddress>;

    # [ wasm_bindgen ( structural , method , getter , js_class = "PaymentResponse" , js_name = shippingOption ) ]
    ///Getter for the `shippingOption` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PaymentResponse/shippingOption)
    ///
    ///*This API requires the following crate features to be activated: `PaymentResponse`*
    pub fn shipping_option(this: &PaymentResponse) -> Option<String>;

    # [ wasm_bindgen ( structural , method , getter , js_class = "PaymentResponse" , js_name = payerName ) ]
    ///Getter for the `payerName` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PaymentResponse/payerName)
    ///
    ///*This API requires the following crate features to be activated: `PaymentResponse`*
    pub fn payer_name(this: &PaymentResponse) -> Option<String>;

    # [ wasm_bindgen ( structural , method , getter , js_class = "PaymentResponse" , js_name = payerEmail ) ]
    ///Getter for the `payerEmail` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PaymentResponse/payerEmail)
    ///
    ///*This API requires the following crate features to be activated: `PaymentResponse`*
    pub fn payer_email(this: &PaymentResponse) -> Option<String>;

    # [ wasm_bindgen ( structural , method , getter , js_class = "PaymentResponse" , js_name = payerPhone ) ]
    ///Getter for the `payerPhone` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PaymentResponse/payerPhone)
    ///
    ///*This API requires the following crate features to be activated: `PaymentResponse`*
    pub fn payer_phone(this: &PaymentResponse) -> Option<String>;

    # [ wasm_bindgen ( method , structural , js_class = "PaymentResponse" , js_name = complete ) ]
    ///The `complete()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PaymentResponse/complete)
    ///
    ///*This API requires the following crate features to be activated: `PaymentResponse`*
    pub fn complete(this: &PaymentResponse) -> ::js_sys::Promise;

    #[cfg(feature = "PaymentComplete")]
    # [ wasm_bindgen ( method , structural , js_class = "PaymentResponse" , js_name = complete ) ]
    ///The `complete()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PaymentResponse/complete)
    ///
    ///*This API requires the following crate features to be activated: `PaymentComplete`, `PaymentResponse`*
    pub fn complete_with_result(
        this: &PaymentResponse,
        result: PaymentComplete,
    ) -> ::js_sys::Promise;

    # [ wasm_bindgen ( method , structural , js_class = "PaymentResponse" , js_name = toJSON ) ]
    ///The `toJSON()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PaymentResponse/toJSON)
    ///
    ///*This API requires the following crate features to be activated: `PaymentResponse`*
    pub fn to_json(this: &PaymentResponse) -> ::js_sys::Object;

}
