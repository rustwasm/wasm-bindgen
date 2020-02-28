use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = EventTarget , extends = :: js_sys :: Object , js_name = ServiceWorkerContainer , typescript_name = ServiceWorkerContainer ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ServiceWorkerContainer` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerContainer)\n\n*This API requires the following crate features to be activated: `ServiceWorkerContainer`*"]
    pub type ServiceWorkerContainer;
    # [ wasm_bindgen ( structural , method , getter , js_name = controller ) ]
    #[cfg(feature = "ServiceWorker")]
    #[doc = "Getter for the `controller` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerContainer/controller)\n\n*This API requires the following crate features to be activated: `ServiceWorker`, `ServiceWorkerContainer`*"]
    pub fn controller(this: &ServiceWorkerContainer) -> Option<ServiceWorker>;
    # [ wasm_bindgen ( structural , catch , method , getter , js_name = ready ) ]
    #[doc = "Getter for the `ready` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerContainer/ready)\n\n*This API requires the following crate features to be activated: `ServiceWorkerContainer`*"]
    pub fn ready(this: &ServiceWorkerContainer) -> Result<::js_sys::Promise, JsValue>;
    # [ wasm_bindgen ( structural , method , getter , js_name = oncontrollerchange ) ]
    #[doc = "Getter for the `oncontrollerchange` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerContainer/oncontrollerchange)\n\n*This API requires the following crate features to be activated: `ServiceWorkerContainer`*"]
    pub fn oncontrollerchange(this: &ServiceWorkerContainer) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = oncontrollerchange ) ]
    #[doc = "Setter for the `oncontrollerchange` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerContainer/oncontrollerchange)\n\n*This API requires the following crate features to be activated: `ServiceWorkerContainer`*"]
    pub fn set_oncontrollerchange(this: &ServiceWorkerContainer, value: Option<::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = onerror ) ]
    #[doc = "Getter for the `onerror` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerContainer/onerror)\n\n*This API requires the following crate features to be activated: `ServiceWorkerContainer`*"]
    pub fn onerror(this: &ServiceWorkerContainer) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onerror ) ]
    #[doc = "Setter for the `onerror` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerContainer/onerror)\n\n*This API requires the following crate features to be activated: `ServiceWorkerContainer`*"]
    pub fn set_onerror(this: &ServiceWorkerContainer, value: Option<::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = onmessage ) ]
    #[doc = "Getter for the `onmessage` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerContainer/onmessage)\n\n*This API requires the following crate features to be activated: `ServiceWorkerContainer`*"]
    pub fn onmessage(this: &ServiceWorkerContainer) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onmessage ) ]
    #[doc = "Setter for the `onmessage` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerContainer/onmessage)\n\n*This API requires the following crate features to be activated: `ServiceWorkerContainer`*"]
    pub fn set_onmessage(this: &ServiceWorkerContainer, value: Option<::js_sys::Function>);
    # [ wasm_bindgen ( method , structural , js_name = getRegistration ) ]
    #[doc = "The `getRegistration()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerContainer/getRegistration)\n\n*This API requires the following crate features to be activated: `ServiceWorkerContainer`*"]
    pub fn get_registration(this: &ServiceWorkerContainer) -> ::js_sys::Promise;
    # [ wasm_bindgen ( method , structural , js_name = getRegistration ) ]
    #[doc = "The `getRegistration()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerContainer/getRegistration)\n\n*This API requires the following crate features to be activated: `ServiceWorkerContainer`*"]
    pub fn get_registration_with_document_url(
        this: &ServiceWorkerContainer,
        document_url: &str,
    ) -> ::js_sys::Promise;
    # [ wasm_bindgen ( method , structural , js_name = getRegistrations ) ]
    #[doc = "The `getRegistrations()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerContainer/getRegistrations)\n\n*This API requires the following crate features to be activated: `ServiceWorkerContainer`*"]
    pub fn get_registrations(this: &ServiceWorkerContainer) -> ::js_sys::Promise;
    # [ wasm_bindgen ( catch , method , structural , js_name = getScopeForUrl ) ]
    #[doc = "The `getScopeForUrl()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerContainer/getScopeForUrl)\n\n*This API requires the following crate features to be activated: `ServiceWorkerContainer`*"]
    pub fn get_scope_for_url(this: &ServiceWorkerContainer, url: &str) -> Result<String, JsValue>;
    # [ wasm_bindgen ( method , structural , js_name = register ) ]
    #[doc = "The `register()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerContainer/register)\n\n*This API requires the following crate features to be activated: `ServiceWorkerContainer`*"]
    pub fn register(this: &ServiceWorkerContainer, script_url: &str) -> ::js_sys::Promise;
    #[cfg(feature = "RegistrationOptions")]
    # [ wasm_bindgen ( method , structural , js_name = register ) ]
    #[doc = "The `register()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerContainer/register)\n\n*This API requires the following crate features to be activated: `RegistrationOptions`, `ServiceWorkerContainer`*"]
    pub fn register_with_options(
        this: &ServiceWorkerContainer,
        script_url: &str,
        options: &RegistrationOptions,
    ) -> ::js_sys::Promise;
}
