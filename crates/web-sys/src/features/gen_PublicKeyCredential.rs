use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = Credential , extends = :: js_sys :: Object , js_name = PublicKeyCredential , typescript_name = PublicKeyCredential ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `PublicKeyCredential` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PublicKeyCredential)
    ///
    ///*This API requires the following crate features to be activated: `PublicKeyCredential`*
    pub type PublicKeyCredential;

    # [ wasm_bindgen ( structural , method , getter , js_class = "PublicKeyCredential" , js_name = rawId ) ]
    ///Getter for the `rawId` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PublicKeyCredential/rawId)
    ///
    ///*This API requires the following crate features to be activated: `PublicKeyCredential`*
    pub fn raw_id(this: &PublicKeyCredential) -> ::js_sys::ArrayBuffer;

    #[cfg(feature = "AuthenticatorResponse")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "PublicKeyCredential" , js_name = response ) ]
    ///Getter for the `response` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PublicKeyCredential/response)
    ///
    ///*This API requires the following crate features to be activated: `AuthenticatorResponse`, `PublicKeyCredential`*
    pub fn response(this: &PublicKeyCredential) -> AuthenticatorResponse;

    #[cfg(feature = "AuthenticationExtensionsClientOutputs")]
    # [ wasm_bindgen ( method , structural , js_class = "PublicKeyCredential" , js_name = getClientExtensionResults ) ]
    ///The `getClientExtensionResults()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PublicKeyCredential/getClientExtensionResults)
    ///
    ///*This API requires the following crate features to be activated: `AuthenticationExtensionsClientOutputs`, `PublicKeyCredential`*
    pub fn get_client_extension_results(
        this: &PublicKeyCredential,
    ) -> AuthenticationExtensionsClientOutputs;

    # [ wasm_bindgen ( static_method_of = PublicKeyCredential , js_class = "PublicKeyCredential" , js_name = isUserVerifyingPlatformAuthenticatorAvailable ) ]
    ///The `isUserVerifyingPlatformAuthenticatorAvailable()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PublicKeyCredential/isUserVerifyingPlatformAuthenticatorAvailable)
    ///
    ///*This API requires the following crate features to be activated: `PublicKeyCredential`*
    pub fn is_user_verifying_platform_authenticator_available() -> ::js_sys::Promise;

}
