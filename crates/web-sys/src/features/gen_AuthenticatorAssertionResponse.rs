use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = AuthenticatorResponse , extends = :: js_sys :: Object , js_name = AuthenticatorAssertionResponse , typescript_name = AuthenticatorAssertionResponse ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `AuthenticatorAssertionResponse` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AuthenticatorAssertionResponse)\n\n*This API requires the following crate features to be activated: `AuthenticatorAssertionResponse`*"]
    pub type AuthenticatorAssertionResponse;
    # [ wasm_bindgen ( structural , method , getter , js_name = authenticatorData ) ]
    #[doc = "Getter for the `authenticatorData` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AuthenticatorAssertionResponse/authenticatorData)\n\n*This API requires the following crate features to be activated: `AuthenticatorAssertionResponse`*"]
    pub fn authenticator_data(this: &AuthenticatorAssertionResponse) -> ::js_sys::ArrayBuffer;
    # [ wasm_bindgen ( structural , method , getter , js_name = signature ) ]
    #[doc = "Getter for the `signature` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AuthenticatorAssertionResponse/signature)\n\n*This API requires the following crate features to be activated: `AuthenticatorAssertionResponse`*"]
    pub fn signature(this: &AuthenticatorAssertionResponse) -> ::js_sys::ArrayBuffer;
    # [ wasm_bindgen ( structural , method , getter , js_name = userHandle ) ]
    #[doc = "Getter for the `userHandle` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AuthenticatorAssertionResponse/userHandle)\n\n*This API requires the following crate features to be activated: `AuthenticatorAssertionResponse`*"]
    pub fn user_handle(this: &AuthenticatorAssertionResponse) -> Option<::js_sys::ArrayBuffer>;
}
