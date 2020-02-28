use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = Credential , extends = :: js_sys :: Object , js_name = PublicKeyCredential , typescript_name = PublicKeyCredential ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `PublicKeyCredential` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PublicKeyCredential)\n\n*This API requires the following crate features to be activated: `PublicKeyCredential`*"]
    pub type PublicKeyCredential;
    # [ wasm_bindgen ( structural , method , getter , js_name = rawId ) ]
    #[doc = "Getter for the `rawId` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PublicKeyCredential/rawId)\n\n*This API requires the following crate features to be activated: `PublicKeyCredential`*"]
    pub fn raw_id(this: &PublicKeyCredential) -> ::js_sys::ArrayBuffer;
    # [ wasm_bindgen ( structural , method , getter , js_name = response ) ]
    #[cfg(feature = "AuthenticatorResponse")]
    #[doc = "Getter for the `response` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PublicKeyCredential/response)\n\n*This API requires the following crate features to be activated: `AuthenticatorResponse`, `PublicKeyCredential`*"]
    pub fn response(this: &PublicKeyCredential) -> AuthenticatorResponse;
    #[cfg(feature = "AuthenticationExtensionsClientOutputs")]
    # [ wasm_bindgen ( method , structural , js_name = getClientExtensionResults ) ]
    #[doc = "The `getClientExtensionResults()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PublicKeyCredential/getClientExtensionResults)\n\n*This API requires the following crate features to be activated: `AuthenticationExtensionsClientOutputs`, `PublicKeyCredential`*"]
    pub fn get_client_extension_results(
        this: &PublicKeyCredential,
    ) -> AuthenticationExtensionsClientOutputs;
    # [ wasm_bindgen ( method , structural , static_method_of = PublicKeyCredential , js_name = isUserVerifyingPlatformAuthenticatorAvailable ) ]
    #[doc = "The `isUserVerifyingPlatformAuthenticatorAvailable()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PublicKeyCredential/isUserVerifyingPlatformAuthenticatorAvailable)\n\n*This API requires the following crate features to be activated: `PublicKeyCredential`*"]
    pub fn is_user_verifying_platform_authenticator_available() -> ::js_sys::Promise;
}
