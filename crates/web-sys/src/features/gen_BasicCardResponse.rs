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
    #[wasm_bindgen(method, getter = "billingAddress")]
    fn billing_address_shim(this: &BasicCardResponse) -> Option<PaymentAddress>;
    #[cfg(feature = "PaymentAddress")]
    #[wasm_bindgen(method, setter = "billingAddress")]
    fn set_billing_address_shim(this: &BasicCardResponse, val: Option<&PaymentAddress>);
    #[wasm_bindgen(method, getter = "cardNumber")]
    fn card_number_shim(this: &BasicCardResponse) -> String;
    #[wasm_bindgen(method, setter = "cardNumber")]
    fn set_card_number_shim(this: &BasicCardResponse, val: &str);
    #[wasm_bindgen(method, getter = "cardSecurityCode")]
    fn card_security_code_shim(this: &BasicCardResponse) -> String;
    #[wasm_bindgen(method, setter = "cardSecurityCode")]
    fn set_card_security_code_shim(this: &BasicCardResponse, val: &str);
    #[wasm_bindgen(method, getter = "cardholderName")]
    fn cardholder_name_shim(this: &BasicCardResponse) -> String;
    #[wasm_bindgen(method, setter = "cardholderName")]
    fn set_cardholder_name_shim(this: &BasicCardResponse, val: &str);
    #[wasm_bindgen(method, getter = "expiryMonth")]
    fn expiry_month_shim(this: &BasicCardResponse) -> String;
    #[wasm_bindgen(method, setter = "expiryMonth")]
    fn set_expiry_month_shim(this: &BasicCardResponse, val: &str);
    #[wasm_bindgen(method, getter = "expiryYear")]
    fn expiry_year_shim(this: &BasicCardResponse) -> String;
    #[wasm_bindgen(method, setter = "expiryYear")]
    fn set_expiry_year_shim(this: &BasicCardResponse, val: &str);
}
#[doc = "The trait to access properties on the `BasicCardResponse` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `BasicCardResponse`*"]
pub trait BasicCardResponseGetters {
    #[cfg(feature = "PaymentAddress")]
    #[doc = "Get the `billingAddress` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BasicCardResponse`, `PaymentAddress`*"]
    fn billing_address(&self) -> Option<PaymentAddress>;
    #[doc = "Get the `cardNumber` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BasicCardResponse`*"]
    fn card_number(&self) -> String;
    #[doc = "Get the `cardSecurityCode` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BasicCardResponse`*"]
    fn card_security_code(&self) -> String;
    #[doc = "Get the `cardholderName` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BasicCardResponse`*"]
    fn cardholder_name(&self) -> String;
    #[doc = "Get the `expiryMonth` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BasicCardResponse`*"]
    fn expiry_month(&self) -> String;
    #[doc = "Get the `expiryYear` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BasicCardResponse`*"]
    fn expiry_year(&self) -> String;
}
impl BasicCardResponseGetters for BasicCardResponse {
    #[cfg(feature = "PaymentAddress")]
    fn billing_address(&self) -> Option<PaymentAddress> {
        self.billing_address_shim()
    }
    fn card_number(&self) -> String {
        self.card_number_shim()
    }
    fn card_security_code(&self) -> String {
        self.card_security_code_shim()
    }
    fn cardholder_name(&self) -> String {
        self.cardholder_name_shim()
    }
    fn expiry_month(&self) -> String {
        self.expiry_month_shim()
    }
    fn expiry_year(&self) -> String {
        self.expiry_year_shim()
    }
}
impl BasicCardResponse {
    #[doc = "Construct a new `BasicCardResponse`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BasicCardResponse`*"]
    pub fn new(card_number: &str) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        Self::card_number(&mut ret, card_number);
        ret
    }
    #[cfg(feature = "PaymentAddress")]
    #[doc = "Change the `billingAddress` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BasicCardResponse`, `PaymentAddress`*"]
    pub fn billing_address(&mut self, val: Option<&PaymentAddress>) -> &mut Self {
        self.set_billing_address_shim(val);
        self
    }
    #[doc = "Change the `cardNumber` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BasicCardResponse`*"]
    pub fn card_number(&mut self, val: &str) -> &mut Self {
        self.set_card_number_shim(val);
        self
    }
    #[doc = "Change the `cardSecurityCode` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BasicCardResponse`*"]
    pub fn card_security_code(&mut self, val: &str) -> &mut Self {
        self.set_card_security_code_shim(val);
        self
    }
    #[doc = "Change the `cardholderName` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BasicCardResponse`*"]
    pub fn cardholder_name(&mut self, val: &str) -> &mut Self {
        self.set_cardholder_name_shim(val);
        self
    }
    #[doc = "Change the `expiryMonth` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BasicCardResponse`*"]
    pub fn expiry_month(&mut self, val: &str) -> &mut Self {
        self.set_expiry_month_shim(val);
        self
    }
    #[doc = "Change the `expiryYear` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BasicCardResponse`*"]
    pub fn expiry_year(&mut self, val: &str) -> &mut Self {
        self.set_expiry_year_shim(val);
        self
    }
}
