use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = AuthenticatorResponse , extends = :: js_sys :: Object , js_name = AuthenticatorAttestationResponse , typescript_type = "AuthenticatorAttestationResponse" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `AuthenticatorAttestationResponse` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AuthenticatorAttestationResponse)
    ///
    ///*This API requires the following crate features to be activated: `AuthenticatorAttestationResponse`*
    pub type AuthenticatorAttestationResponse;

    # [ wasm_bindgen ( structural , method , getter , js_class = "AuthenticatorAttestationResponse" , js_name = attestationObject ) ]
    ///Getter for the `attestationObject` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AuthenticatorAttestationResponse/attestationObject)
    ///
    ///*This API requires the following crate features to be activated: `AuthenticatorAttestationResponse`*
    pub fn attestation_object(this: &AuthenticatorAttestationResponse) -> ::js_sys::ArrayBuffer;

}
