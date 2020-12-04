#![allow(unused_imports)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = CredentialsContainer , typescript_type = "CredentialsContainer")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `CredentialsContainer` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CredentialsContainer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CredentialsContainer`*"]
    pub type CredentialsContainer;
    # [wasm_bindgen (catch , method , structural , js_class = "CredentialsContainer" , js_name = create)]
    #[doc = "The `create()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CredentialsContainer/create)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CredentialsContainer`*"]
    #[doc = ""]
    #[doc = "Return value: While the Promise can produce any JsValue as far as the type system is concerned, practically it is expected to contain a <code>[Option]<[Credential]></code>. It can be converted like `<code>let result: [Option]<[Credential]> = result?.await.into();</code>."]
    pub fn create(this: &CredentialsContainer) -> Result<::js_sys::Promise, JsValue>;
    #[cfg(feature = "CredentialCreationOptions")]
    # [wasm_bindgen (catch , method , structural , js_class = "CredentialsContainer" , js_name = create)]
    #[doc = "The `create()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CredentialsContainer/create)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CredentialCreationOptions`, `CredentialsContainer`*"]
    #[doc = ""]
    #[doc = "Return value: While the Promise can produce any JsValue as far as the type system is concerned, practically it is expected to contain a <code>[Option]<[Credential]></code>. It can be converted like `<code>let result: [Option]<[Credential]> = result?.await.into();</code>."]
    pub fn create_with_options(
        this: &CredentialsContainer,
        options: &CredentialCreationOptions,
    ) -> Result<::js_sys::Promise, JsValue>;
    # [wasm_bindgen (catch , method , structural , js_class = "CredentialsContainer" , js_name = get)]
    #[doc = "The `get()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CredentialsContainer/get)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CredentialsContainer`*"]
    #[doc = ""]
    #[doc = "Return value: While the Promise can produce any JsValue as far as the type system is concerned, practically it is expected to contain a <code>[Option]<[Credential]></code>. It can be converted like `<code>let result: [Option]<[Credential]> = result?.await.into();</code>."]
    pub fn get(this: &CredentialsContainer) -> Result<::js_sys::Promise, JsValue>;
    #[cfg(feature = "CredentialRequestOptions")]
    # [wasm_bindgen (catch , method , structural , js_class = "CredentialsContainer" , js_name = get)]
    #[doc = "The `get()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CredentialsContainer/get)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CredentialRequestOptions`, `CredentialsContainer`*"]
    #[doc = ""]
    #[doc = "Return value: While the Promise can produce any JsValue as far as the type system is concerned, practically it is expected to contain a <code>[Option]<[Credential]></code>. It can be converted like `<code>let result: [Option]<[Credential]> = result?.await.into();</code>."]
    pub fn get_with_options(
        this: &CredentialsContainer,
        options: &CredentialRequestOptions,
    ) -> Result<::js_sys::Promise, JsValue>;
    # [wasm_bindgen (catch , method , structural , js_class = "CredentialsContainer" , js_name = preventSilentAccess)]
    #[doc = "The `preventSilentAccess()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CredentialsContainer/preventSilentAccess)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CredentialsContainer`*"]
    #[doc = ""]
    #[doc = "Return value: There is additional information in the IDL file about the content of the promise, but it can not yet be explained any better."]
    pub fn prevent_silent_access(this: &CredentialsContainer)
        -> Result<::js_sys::Promise, JsValue>;
    #[cfg(feature = "Credential")]
    # [wasm_bindgen (catch , method , structural , js_class = "CredentialsContainer" , js_name = store)]
    #[doc = "The `store()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CredentialsContainer/store)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Credential`, `CredentialsContainer`*"]
    #[doc = ""]
    #[doc = "Return value: While the Promise can produce any JsValue as far as the type system is concerned, practically it is expected to contain a <code>[Credential]</code>. It can be converted like `<code>let result: [Credential] = result?.await.into();</code>."]
    pub fn store(
        this: &CredentialsContainer,
        credential: &Credential,
    ) -> Result<::js_sys::Promise, JsValue>;
}
