#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = BasicCardResponse)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `BasicCardResponse` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BasicCardResponse`*"]
    pub type BasicCardResponse;
    #[cfg(feature = "PaymentAddress")]
    #[wasm_bindgen(method, setter = "billingAddress")]
    fn billing_address_shim(this: &BasicCardResponse, val: Option<&PaymentAddress>);
    #[wasm_bindgen(method, setter = "cardNumber")]
    fn card_number_shim(this: &BasicCardResponse, val: &str);
    #[wasm_bindgen(method, setter = "cardSecurityCode")]
    fn card_security_code_shim(this: &BasicCardResponse, val: &str);
    #[wasm_bindgen(method, setter = "cardholderName")]
    fn cardholder_name_shim(this: &BasicCardResponse, val: &str);
    #[wasm_bindgen(method, setter = "expiryMonth")]
    fn expiry_month_shim(this: &BasicCardResponse, val: &str);
    #[wasm_bindgen(method, setter = "expiryYear")]
    fn expiry_year_shim(this: &BasicCardResponse, val: &str);
}
impl BasicCardResponse {
    #[doc = "Construct a new `BasicCardResponse`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BasicCardResponse`*"]
    pub fn new(card_number: &str) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.card_number(card_number);
        ret
    }
    #[cfg(feature = "PaymentAddress")]
    #[doc = "Change the `billingAddress` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BasicCardResponse`, `PaymentAddress`*"]
    pub fn billing_address(&mut self, val: Option<&PaymentAddress>) -> &mut Self {
        self.billing_address_shim(val);
        self
    }
    #[doc = "Change the `cardNumber` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BasicCardResponse`*"]
    pub fn card_number(&mut self, val: &str) -> &mut Self {
        self.card_number_shim(val);
        self
    }
    #[doc = "Change the `cardSecurityCode` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BasicCardResponse`*"]
    pub fn card_security_code(&mut self, val: &str) -> &mut Self {
        self.card_security_code_shim(val);
        self
    }
    #[doc = "Change the `cardholderName` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BasicCardResponse`*"]
    pub fn cardholder_name(&mut self, val: &str) -> &mut Self {
        self.cardholder_name_shim(val);
        self
    }
    #[doc = "Change the `expiryMonth` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BasicCardResponse`*"]
    pub fn expiry_month(&mut self, val: &str) -> &mut Self {
        self.expiry_month_shim(val);
        self
    }
    #[doc = "Change the `expiryYear` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BasicCardResponse`*"]
    pub fn expiry_year(&mut self, val: &str) -> &mut Self {
        self.expiry_year_shim(val);
        self
    }
}
