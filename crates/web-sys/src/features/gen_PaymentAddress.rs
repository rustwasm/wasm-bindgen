use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = PaymentAddress , typescript_name = PaymentAddress ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `PaymentAddress` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PaymentAddress)\n\n*This API requires the following crate features to be activated: `PaymentAddress`*"]
    pub type PaymentAddress;
    # [ wasm_bindgen ( structural , method , getter , js_name = country ) ]
    #[doc = "Getter for the `country` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PaymentAddress/country)\n\n*This API requires the following crate features to be activated: `PaymentAddress`*"]
    pub fn country(this: &PaymentAddress) -> String;
    # [ wasm_bindgen ( structural , method , getter , js_name = addressLine ) ]
    #[doc = "Getter for the `addressLine` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PaymentAddress/addressLine)\n\n*This API requires the following crate features to be activated: `PaymentAddress`*"]
    pub fn address_line(this: &PaymentAddress) -> ::js_sys::Array;
    # [ wasm_bindgen ( structural , method , getter , js_name = region ) ]
    #[doc = "Getter for the `region` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PaymentAddress/region)\n\n*This API requires the following crate features to be activated: `PaymentAddress`*"]
    pub fn region(this: &PaymentAddress) -> String;
    # [ wasm_bindgen ( structural , method , getter , js_name = city ) ]
    #[doc = "Getter for the `city` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PaymentAddress/city)\n\n*This API requires the following crate features to be activated: `PaymentAddress`*"]
    pub fn city(this: &PaymentAddress) -> String;
    # [ wasm_bindgen ( structural , method , getter , js_name = dependentLocality ) ]
    #[doc = "Getter for the `dependentLocality` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PaymentAddress/dependentLocality)\n\n*This API requires the following crate features to be activated: `PaymentAddress`*"]
    pub fn dependent_locality(this: &PaymentAddress) -> String;
    # [ wasm_bindgen ( structural , method , getter , js_name = postalCode ) ]
    #[doc = "Getter for the `postalCode` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PaymentAddress/postalCode)\n\n*This API requires the following crate features to be activated: `PaymentAddress`*"]
    pub fn postal_code(this: &PaymentAddress) -> String;
    # [ wasm_bindgen ( structural , method , getter , js_name = sortingCode ) ]
    #[doc = "Getter for the `sortingCode` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PaymentAddress/sortingCode)\n\n*This API requires the following crate features to be activated: `PaymentAddress`*"]
    pub fn sorting_code(this: &PaymentAddress) -> String;
    # [ wasm_bindgen ( structural , method , getter , js_name = languageCode ) ]
    #[doc = "Getter for the `languageCode` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PaymentAddress/languageCode)\n\n*This API requires the following crate features to be activated: `PaymentAddress`*"]
    pub fn language_code(this: &PaymentAddress) -> String;
    # [ wasm_bindgen ( structural , method , getter , js_name = organization ) ]
    #[doc = "Getter for the `organization` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PaymentAddress/organization)\n\n*This API requires the following crate features to be activated: `PaymentAddress`*"]
    pub fn organization(this: &PaymentAddress) -> String;
    # [ wasm_bindgen ( structural , method , getter , js_name = recipient ) ]
    #[doc = "Getter for the `recipient` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PaymentAddress/recipient)\n\n*This API requires the following crate features to be activated: `PaymentAddress`*"]
    pub fn recipient(this: &PaymentAddress) -> String;
    # [ wasm_bindgen ( structural , method , getter , js_name = phone ) ]
    #[doc = "Getter for the `phone` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PaymentAddress/phone)\n\n*This API requires the following crate features to be activated: `PaymentAddress`*"]
    pub fn phone(this: &PaymentAddress) -> String;
    # [ wasm_bindgen ( method , structural , js_name = toJSON ) ]
    #[doc = "The `toJSON()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PaymentAddress/toJSON)\n\n*This API requires the following crate features to be activated: `PaymentAddress`*"]
    pub fn to_json(this: &PaymentAddress) -> ::js_sys::Object;
}
