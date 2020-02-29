use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = AuthenticatorResponse , typescript_name = AuthenticatorResponse ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `AuthenticatorResponse` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AuthenticatorResponse)
    ///
    ///*This API requires the following crate features to be activated: `AuthenticatorResponse`*
    pub type AuthenticatorResponse;

    # [ wasm_bindgen ( structural , method , getter , js_class = "AuthenticatorResponse" , js_name = clientDataJSON ) ]
    ///Getter for the `clientDataJSON` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AuthenticatorResponse/clientDataJSON)
    ///
    ///*This API requires the following crate features to be activated: `AuthenticatorResponse`*
    pub fn client_data_json(this: &AuthenticatorResponse) -> ::js_sys::ArrayBuffer;

}
