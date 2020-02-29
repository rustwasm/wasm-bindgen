use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = PaymentAddress , typescript_name = PaymentAddress ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `PaymentAddress` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PaymentAddress)
    ///
    ///*This API requires the following crate features to be activated: `PaymentAddress`*
    pub type PaymentAddress;

    # [ wasm_bindgen ( structural , method , getter , js_class = "PaymentAddress" , js_name = country ) ]
    ///Getter for the `country` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PaymentAddress/country)
    ///
    ///*This API requires the following crate features to be activated: `PaymentAddress`*
    pub fn country(this: &PaymentAddress) -> String;

    # [ wasm_bindgen ( structural , method , getter , js_class = "PaymentAddress" , js_name = addressLine ) ]
    ///Getter for the `addressLine` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PaymentAddress/addressLine)
    ///
    ///*This API requires the following crate features to be activated: `PaymentAddress`*
    pub fn address_line(this: &PaymentAddress) -> ::js_sys::Array;

    # [ wasm_bindgen ( structural , method , getter , js_class = "PaymentAddress" , js_name = region ) ]
    ///Getter for the `region` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PaymentAddress/region)
    ///
    ///*This API requires the following crate features to be activated: `PaymentAddress`*
    pub fn region(this: &PaymentAddress) -> String;

    # [ wasm_bindgen ( structural , method , getter , js_class = "PaymentAddress" , js_name = city ) ]
    ///Getter for the `city` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PaymentAddress/city)
    ///
    ///*This API requires the following crate features to be activated: `PaymentAddress`*
    pub fn city(this: &PaymentAddress) -> String;

    # [ wasm_bindgen ( structural , method , getter , js_class = "PaymentAddress" , js_name = dependentLocality ) ]
    ///Getter for the `dependentLocality` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PaymentAddress/dependentLocality)
    ///
    ///*This API requires the following crate features to be activated: `PaymentAddress`*
    pub fn dependent_locality(this: &PaymentAddress) -> String;

    # [ wasm_bindgen ( structural , method , getter , js_class = "PaymentAddress" , js_name = postalCode ) ]
    ///Getter for the `postalCode` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PaymentAddress/postalCode)
    ///
    ///*This API requires the following crate features to be activated: `PaymentAddress`*
    pub fn postal_code(this: &PaymentAddress) -> String;

    # [ wasm_bindgen ( structural , method , getter , js_class = "PaymentAddress" , js_name = sortingCode ) ]
    ///Getter for the `sortingCode` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PaymentAddress/sortingCode)
    ///
    ///*This API requires the following crate features to be activated: `PaymentAddress`*
    pub fn sorting_code(this: &PaymentAddress) -> String;

    # [ wasm_bindgen ( structural , method , getter , js_class = "PaymentAddress" , js_name = languageCode ) ]
    ///Getter for the `languageCode` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PaymentAddress/languageCode)
    ///
    ///*This API requires the following crate features to be activated: `PaymentAddress`*
    pub fn language_code(this: &PaymentAddress) -> String;

    # [ wasm_bindgen ( structural , method , getter , js_class = "PaymentAddress" , js_name = organization ) ]
    ///Getter for the `organization` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PaymentAddress/organization)
    ///
    ///*This API requires the following crate features to be activated: `PaymentAddress`*
    pub fn organization(this: &PaymentAddress) -> String;

    # [ wasm_bindgen ( structural , method , getter , js_class = "PaymentAddress" , js_name = recipient ) ]
    ///Getter for the `recipient` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PaymentAddress/recipient)
    ///
    ///*This API requires the following crate features to be activated: `PaymentAddress`*
    pub fn recipient(this: &PaymentAddress) -> String;

    # [ wasm_bindgen ( structural , method , getter , js_class = "PaymentAddress" , js_name = phone ) ]
    ///Getter for the `phone` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PaymentAddress/phone)
    ///
    ///*This API requires the following crate features to be activated: `PaymentAddress`*
    pub fn phone(this: &PaymentAddress) -> String;

    # [ wasm_bindgen ( method , structural , js_class = "PaymentAddress" , js_name = toJSON ) ]
    ///The `toJSON()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PaymentAddress/toJSON)
    ///
    ///*This API requires the following crate features to be activated: `PaymentAddress`*
    pub fn to_json(this: &PaymentAddress) -> ::js_sys::Object;

}
