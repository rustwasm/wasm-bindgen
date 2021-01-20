#![allow(unused_imports)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = Credential , extends = :: js_sys :: Object , js_name = PublicKeyCredential , typescript_type = "PublicKeyCredential")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `PublicKeyCredential` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PublicKeyCredential)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredential`*"]
    pub type PublicKeyCredential;
    # [wasm_bindgen (structural , method , getter , js_class = "PublicKeyCredential" , js_name = rawId)]
    #[doc = "Getter for the `rawId` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PublicKeyCredential/rawId)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredential`*"]
    pub fn raw_id(this: &PublicKeyCredential) -> ::js_sys::ArrayBuffer;
    #[cfg(feature = "AuthenticatorResponse")]
    # [wasm_bindgen (structural , method , getter , js_class = "PublicKeyCredential" , js_name = response)]
    #[doc = "Getter for the `response` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PublicKeyCredential/response)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticatorResponse`, `PublicKeyCredential`*"]
    pub fn response(this: &PublicKeyCredential) -> AuthenticatorResponse;
    #[cfg(feature = "AuthenticationExtensionsClientOutputs")]
    # [wasm_bindgen (method , structural , js_class = "PublicKeyCredential" , js_name = getClientExtensionResults)]
    #[doc = "The `getClientExtensionResults()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PublicKeyCredential/getClientExtensionResults)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticationExtensionsClientOutputs`, `PublicKeyCredential`*"]
    pub fn get_client_extension_results(
        this: &PublicKeyCredential,
    ) -> AuthenticationExtensionsClientOutputs;
    # [wasm_bindgen (static_method_of = PublicKeyCredential , js_class = "PublicKeyCredential" , js_name = isUserVerifyingPlatformAuthenticatorAvailable)]
    #[doc = "The `isUserVerifyingPlatformAuthenticatorAvailable()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PublicKeyCredential/isUserVerifyingPlatformAuthenticatorAvailable)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredential`*"]
    #[doc = ""]
    #[doc = "Return value: While the Promise can produce any JsValue as far as the type system is concerned, practically it is expected to contain a <code>[bool]</code>. It can be converted like `<code>let result: [bool] = result.await.into();</code>."]
    pub fn is_user_verifying_platform_authenticator_available() -> ::js_sys::Promise;
}
