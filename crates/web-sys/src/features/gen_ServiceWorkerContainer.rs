use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = EventTarget , extends = :: js_sys :: Object , js_name = ServiceWorkerContainer , typescript_name = ServiceWorkerContainer ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `ServiceWorkerContainer` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerContainer)
    ///
    ///*This API requires the following crate features to be activated: `ServiceWorkerContainer`*
    pub type ServiceWorkerContainer;

    #[cfg(feature = "ServiceWorker")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "ServiceWorkerContainer" , js_name = controller ) ]
    ///Getter for the `controller` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerContainer/controller)
    ///
    ///*This API requires the following crate features to be activated: `ServiceWorker`, `ServiceWorkerContainer`*
    pub fn controller(this: &ServiceWorkerContainer) -> Option<ServiceWorker>;

    # [ wasm_bindgen ( structural , catch , method , getter , js_class = "ServiceWorkerContainer" , js_name = ready ) ]
    ///Getter for the `ready` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerContainer/ready)
    ///
    ///*This API requires the following crate features to be activated: `ServiceWorkerContainer`*
    pub fn ready(this: &ServiceWorkerContainer) -> Result<::js_sys::Promise, JsValue>;

    # [ wasm_bindgen ( structural , method , getter , js_class = "ServiceWorkerContainer" , js_name = oncontrollerchange ) ]
    ///Getter for the `oncontrollerchange` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerContainer/oncontrollerchange)
    ///
    ///*This API requires the following crate features to be activated: `ServiceWorkerContainer`*
    pub fn oncontrollerchange(this: &ServiceWorkerContainer) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "ServiceWorkerContainer" , js_name = oncontrollerchange ) ]
    ///Setter for the `oncontrollerchange` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerContainer/oncontrollerchange)
    ///
    ///*This API requires the following crate features to be activated: `ServiceWorkerContainer`*
    pub fn set_oncontrollerchange(
        this: &ServiceWorkerContainer,
        value: Option<&::js_sys::Function>,
    );

    # [ wasm_bindgen ( structural , method , getter , js_class = "ServiceWorkerContainer" , js_name = onerror ) ]
    ///Getter for the `onerror` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerContainer/onerror)
    ///
    ///*This API requires the following crate features to be activated: `ServiceWorkerContainer`*
    pub fn onerror(this: &ServiceWorkerContainer) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "ServiceWorkerContainer" , js_name = onerror ) ]
    ///Setter for the `onerror` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerContainer/onerror)
    ///
    ///*This API requires the following crate features to be activated: `ServiceWorkerContainer`*
    pub fn set_onerror(this: &ServiceWorkerContainer, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "ServiceWorkerContainer" , js_name = onmessage ) ]
    ///Getter for the `onmessage` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerContainer/onmessage)
    ///
    ///*This API requires the following crate features to be activated: `ServiceWorkerContainer`*
    pub fn onmessage(this: &ServiceWorkerContainer) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "ServiceWorkerContainer" , js_name = onmessage ) ]
    ///Setter for the `onmessage` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerContainer/onmessage)
    ///
    ///*This API requires the following crate features to be activated: `ServiceWorkerContainer`*
    pub fn set_onmessage(this: &ServiceWorkerContainer, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( method , structural , js_class = "ServiceWorkerContainer" , js_name = getRegistration ) ]
    ///The `getRegistration()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerContainer/getRegistration)
    ///
    ///*This API requires the following crate features to be activated: `ServiceWorkerContainer`*
    pub fn get_registration(this: &ServiceWorkerContainer) -> ::js_sys::Promise;

    # [ wasm_bindgen ( method , structural , js_class = "ServiceWorkerContainer" , js_name = getRegistration ) ]
    ///The `getRegistration()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerContainer/getRegistration)
    ///
    ///*This API requires the following crate features to be activated: `ServiceWorkerContainer`*
    pub fn get_registration_with_document_url(
        this: &ServiceWorkerContainer,
        document_url: &str,
    ) -> ::js_sys::Promise;

    # [ wasm_bindgen ( method , structural , js_class = "ServiceWorkerContainer" , js_name = getRegistrations ) ]
    ///The `getRegistrations()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerContainer/getRegistrations)
    ///
    ///*This API requires the following crate features to be activated: `ServiceWorkerContainer`*
    pub fn get_registrations(this: &ServiceWorkerContainer) -> ::js_sys::Promise;

    # [ wasm_bindgen ( catch , method , structural , js_class = "ServiceWorkerContainer" , js_name = getScopeForUrl ) ]
    ///The `getScopeForUrl()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerContainer/getScopeForUrl)
    ///
    ///*This API requires the following crate features to be activated: `ServiceWorkerContainer`*
    pub fn get_scope_for_url(this: &ServiceWorkerContainer, url: &str) -> Result<String, JsValue>;

    # [ wasm_bindgen ( method , structural , js_class = "ServiceWorkerContainer" , js_name = register ) ]
    ///The `register()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerContainer/register)
    ///
    ///*This API requires the following crate features to be activated: `ServiceWorkerContainer`*
    pub fn register(this: &ServiceWorkerContainer, script_url: &str) -> ::js_sys::Promise;

    #[cfg(feature = "RegistrationOptions")]
    # [ wasm_bindgen ( method , structural , js_class = "ServiceWorkerContainer" , js_name = register ) ]
    ///The `register()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerContainer/register)
    ///
    ///*This API requires the following crate features to be activated: `RegistrationOptions`, `ServiceWorkerContainer`*
    pub fn register_with_options(
        this: &ServiceWorkerContainer,
        script_url: &str,
        options: &RegistrationOptions,
    ) -> ::js_sys::Promise;

}
