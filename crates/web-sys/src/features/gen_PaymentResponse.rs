use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = PaymentResponse , typescript_name = PaymentResponse ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `PaymentResponse` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PaymentResponse)\n\n*This API requires the following crate features to be activated: `PaymentResponse`*"]
    pub type PaymentResponse;
    # [ wasm_bindgen ( structural , method , getter , js_name = requestId ) ]
    #[doc = "Getter for the `requestId` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PaymentResponse/requestId)\n\n*This API requires the following crate features to be activated: `PaymentResponse`*"]
    pub fn request_id(this: &PaymentResponse) -> String;
    # [ wasm_bindgen ( structural , method , getter , js_name = methodName ) ]
    #[doc = "Getter for the `methodName` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PaymentResponse/methodName)\n\n*This API requires the following crate features to be activated: `PaymentResponse`*"]
    pub fn method_name(this: &PaymentResponse) -> String;
    # [ wasm_bindgen ( structural , method , getter , js_name = details ) ]
    #[doc = "Getter for the `details` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PaymentResponse/details)\n\n*This API requires the following crate features to be activated: `PaymentResponse`*"]
    pub fn details(this: &PaymentResponse) -> ::js_sys::Object;
    # [ wasm_bindgen ( structural , method , getter , js_name = shippingAddress ) ]
    #[cfg(feature = "PaymentAddress")]
    #[doc = "Getter for the `shippingAddress` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PaymentResponse/shippingAddress)\n\n*This API requires the following crate features to be activated: `PaymentAddress`, `PaymentResponse`*"]
    pub fn shipping_address(this: &PaymentResponse) -> Option<PaymentAddress>;
    # [ wasm_bindgen ( structural , method , getter , js_name = shippingOption ) ]
    #[doc = "Getter for the `shippingOption` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PaymentResponse/shippingOption)\n\n*This API requires the following crate features to be activated: `PaymentResponse`*"]
    pub fn shipping_option(this: &PaymentResponse) -> Option<String>;
    # [ wasm_bindgen ( structural , method , getter , js_name = payerName ) ]
    #[doc = "Getter for the `payerName` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PaymentResponse/payerName)\n\n*This API requires the following crate features to be activated: `PaymentResponse`*"]
    pub fn payer_name(this: &PaymentResponse) -> Option<String>;
    # [ wasm_bindgen ( structural , method , getter , js_name = payerEmail ) ]
    #[doc = "Getter for the `payerEmail` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PaymentResponse/payerEmail)\n\n*This API requires the following crate features to be activated: `PaymentResponse`*"]
    pub fn payer_email(this: &PaymentResponse) -> Option<String>;
    # [ wasm_bindgen ( structural , method , getter , js_name = payerPhone ) ]
    #[doc = "Getter for the `payerPhone` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PaymentResponse/payerPhone)\n\n*This API requires the following crate features to be activated: `PaymentResponse`*"]
    pub fn payer_phone(this: &PaymentResponse) -> Option<String>;
    # [ wasm_bindgen ( method , structural , js_name = complete ) ]
    #[doc = "The `complete()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PaymentResponse/complete)\n\n*This API requires the following crate features to be activated: `PaymentResponse`*"]
    pub fn complete(this: &PaymentResponse) -> ::js_sys::Promise;
    #[cfg(feature = "PaymentComplete")]
    # [ wasm_bindgen ( method , structural , js_name = complete ) ]
    #[doc = "The `complete()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PaymentResponse/complete)\n\n*This API requires the following crate features to be activated: `PaymentComplete`, `PaymentResponse`*"]
    pub fn complete_with_result(
        this: &PaymentResponse,
        result: PaymentComplete,
    ) -> ::js_sys::Promise;
    # [ wasm_bindgen ( method , structural , js_name = toJSON ) ]
    #[doc = "The `toJSON()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PaymentResponse/toJSON)\n\n*This API requires the following crate features to be activated: `PaymentResponse`*"]
    pub fn to_json(this: &PaymentResponse) -> ::js_sys::Object;
}
