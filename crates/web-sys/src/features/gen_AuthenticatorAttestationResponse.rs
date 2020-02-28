use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = AuthenticatorResponse , extends = :: js_sys :: Object , js_name = AuthenticatorAttestationResponse , typescript_name = AuthenticatorAttestationResponse ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `AuthenticatorAttestationResponse` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AuthenticatorAttestationResponse)\n\n*This API requires the following crate features to be activated: `AuthenticatorAttestationResponse`*"]
    pub type AuthenticatorAttestationResponse;
    # [ wasm_bindgen ( structural , method , getter , js_class = "AuthenticatorAttestationResponse" , js_name = attestationObject ) ]
    #[doc = "Getter for the `attestationObject` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AuthenticatorAttestationResponse/attestationObject)\n\n*This API requires the following crate features to be activated: `AuthenticatorAttestationResponse`*"]
    pub fn attestation_object(this: &AuthenticatorAttestationResponse) -> ::js_sys::ArrayBuffer;
}
