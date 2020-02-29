use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = CredentialsContainer , typescript_name = CredentialsContainer ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `CredentialsContainer` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CredentialsContainer)
    ///
    ///*This API requires the following crate features to be activated: `CredentialsContainer`*
    pub type CredentialsContainer;

    # [ wasm_bindgen ( catch , method , structural , js_class = "CredentialsContainer" , js_name = create ) ]
    ///The `create()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CredentialsContainer/create)
    ///
    ///*This API requires the following crate features to be activated: `CredentialsContainer`*
    pub fn create(this: &CredentialsContainer) -> Result<::js_sys::Promise, JsValue>;

    #[cfg(feature = "CredentialCreationOptions")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "CredentialsContainer" , js_name = create ) ]
    ///The `create()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CredentialsContainer/create)
    ///
    ///*This API requires the following crate features to be activated: `CredentialCreationOptions`, `CredentialsContainer`*
    pub fn create_with_options(
        this: &CredentialsContainer,
        options: &CredentialCreationOptions,
    ) -> Result<::js_sys::Promise, JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "CredentialsContainer" , js_name = get ) ]
    ///The `get()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CredentialsContainer/get)
    ///
    ///*This API requires the following crate features to be activated: `CredentialsContainer`*
    pub fn get(this: &CredentialsContainer) -> Result<::js_sys::Promise, JsValue>;

    #[cfg(feature = "CredentialRequestOptions")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "CredentialsContainer" , js_name = get ) ]
    ///The `get()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CredentialsContainer/get)
    ///
    ///*This API requires the following crate features to be activated: `CredentialRequestOptions`, `CredentialsContainer`*
    pub fn get_with_options(
        this: &CredentialsContainer,
        options: &CredentialRequestOptions,
    ) -> Result<::js_sys::Promise, JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "CredentialsContainer" , js_name = preventSilentAccess ) ]
    ///The `preventSilentAccess()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CredentialsContainer/preventSilentAccess)
    ///
    ///*This API requires the following crate features to be activated: `CredentialsContainer`*
    pub fn prevent_silent_access(this: &CredentialsContainer)
        -> Result<::js_sys::Promise, JsValue>;

    #[cfg(feature = "Credential")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "CredentialsContainer" , js_name = store ) ]
    ///The `store()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CredentialsContainer/store)
    ///
    ///*This API requires the following crate features to be activated: `Credential`, `CredentialsContainer`*
    pub fn store(
        this: &CredentialsContainer,
        credential: &Credential,
    ) -> Result<::js_sys::Promise, JsValue>;

}
