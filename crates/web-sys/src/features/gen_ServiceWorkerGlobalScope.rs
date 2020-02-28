use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = WorkerGlobalScope , extends = EventTarget , extends = :: js_sys :: Object , js_name = ServiceWorkerGlobalScope , typescript_name = ServiceWorkerGlobalScope ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ServiceWorkerGlobalScope` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerGlobalScope)\n\n*This API requires the following crate features to be activated: `ServiceWorkerGlobalScope`*"]
    pub type ServiceWorkerGlobalScope;
    # [ wasm_bindgen ( structural , method , getter , js_name = clients ) ]
    #[cfg(feature = "Clients")]
    #[doc = "Getter for the `clients` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerGlobalScope/clients)\n\n*This API requires the following crate features to be activated: `Clients`, `ServiceWorkerGlobalScope`*"]
    pub fn clients(this: &ServiceWorkerGlobalScope) -> Clients;
    # [ wasm_bindgen ( structural , method , getter , js_name = registration ) ]
    #[cfg(feature = "ServiceWorkerRegistration")]
    #[doc = "Getter for the `registration` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerGlobalScope/registration)\n\n*This API requires the following crate features to be activated: `ServiceWorkerGlobalScope`, `ServiceWorkerRegistration`*"]
    pub fn registration(this: &ServiceWorkerGlobalScope) -> ServiceWorkerRegistration;
    # [ wasm_bindgen ( structural , method , getter , js_name = oninstall ) ]
    #[doc = "Getter for the `oninstall` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerGlobalScope/oninstall)\n\n*This API requires the following crate features to be activated: `ServiceWorkerGlobalScope`*"]
    pub fn oninstall(this: &ServiceWorkerGlobalScope) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = oninstall ) ]
    #[doc = "Setter for the `oninstall` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerGlobalScope/oninstall)\n\n*This API requires the following crate features to be activated: `ServiceWorkerGlobalScope`*"]
    pub fn set_oninstall(this: &ServiceWorkerGlobalScope, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = onactivate ) ]
    #[doc = "Getter for the `onactivate` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerGlobalScope/onactivate)\n\n*This API requires the following crate features to be activated: `ServiceWorkerGlobalScope`*"]
    pub fn onactivate(this: &ServiceWorkerGlobalScope) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onactivate ) ]
    #[doc = "Setter for the `onactivate` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerGlobalScope/onactivate)\n\n*This API requires the following crate features to be activated: `ServiceWorkerGlobalScope`*"]
    pub fn set_onactivate(this: &ServiceWorkerGlobalScope, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = onfetch ) ]
    #[doc = "Getter for the `onfetch` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerGlobalScope/onfetch)\n\n*This API requires the following crate features to be activated: `ServiceWorkerGlobalScope`*"]
    pub fn onfetch(this: &ServiceWorkerGlobalScope) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onfetch ) ]
    #[doc = "Setter for the `onfetch` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerGlobalScope/onfetch)\n\n*This API requires the following crate features to be activated: `ServiceWorkerGlobalScope`*"]
    pub fn set_onfetch(this: &ServiceWorkerGlobalScope, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = onmessage ) ]
    #[doc = "Getter for the `onmessage` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerGlobalScope/onmessage)\n\n*This API requires the following crate features to be activated: `ServiceWorkerGlobalScope`*"]
    pub fn onmessage(this: &ServiceWorkerGlobalScope) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onmessage ) ]
    #[doc = "Setter for the `onmessage` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerGlobalScope/onmessage)\n\n*This API requires the following crate features to be activated: `ServiceWorkerGlobalScope`*"]
    pub fn set_onmessage(this: &ServiceWorkerGlobalScope, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = onpush ) ]
    #[doc = "Getter for the `onpush` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerGlobalScope/onpush)\n\n*This API requires the following crate features to be activated: `ServiceWorkerGlobalScope`*"]
    pub fn onpush(this: &ServiceWorkerGlobalScope) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onpush ) ]
    #[doc = "Setter for the `onpush` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerGlobalScope/onpush)\n\n*This API requires the following crate features to be activated: `ServiceWorkerGlobalScope`*"]
    pub fn set_onpush(this: &ServiceWorkerGlobalScope, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = onpushsubscriptionchange ) ]
    #[doc = "Getter for the `onpushsubscriptionchange` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerGlobalScope/onpushsubscriptionchange)\n\n*This API requires the following crate features to be activated: `ServiceWorkerGlobalScope`*"]
    pub fn onpushsubscriptionchange(this: &ServiceWorkerGlobalScope) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onpushsubscriptionchange ) ]
    #[doc = "Setter for the `onpushsubscriptionchange` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerGlobalScope/onpushsubscriptionchange)\n\n*This API requires the following crate features to be activated: `ServiceWorkerGlobalScope`*"]
    pub fn set_onpushsubscriptionchange(
        this: &ServiceWorkerGlobalScope,
        value: Option<&::js_sys::Function>,
    );
    # [ wasm_bindgen ( structural , method , getter , js_name = onnotificationclick ) ]
    #[doc = "Getter for the `onnotificationclick` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerGlobalScope/onnotificationclick)\n\n*This API requires the following crate features to be activated: `ServiceWorkerGlobalScope`*"]
    pub fn onnotificationclick(this: &ServiceWorkerGlobalScope) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onnotificationclick ) ]
    #[doc = "Setter for the `onnotificationclick` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerGlobalScope/onnotificationclick)\n\n*This API requires the following crate features to be activated: `ServiceWorkerGlobalScope`*"]
    pub fn set_onnotificationclick(
        this: &ServiceWorkerGlobalScope,
        value: Option<&::js_sys::Function>,
    );
    # [ wasm_bindgen ( structural , method , getter , js_name = onnotificationclose ) ]
    #[doc = "Getter for the `onnotificationclose` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerGlobalScope/onnotificationclose)\n\n*This API requires the following crate features to be activated: `ServiceWorkerGlobalScope`*"]
    pub fn onnotificationclose(this: &ServiceWorkerGlobalScope) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onnotificationclose ) ]
    #[doc = "Setter for the `onnotificationclose` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerGlobalScope/onnotificationclose)\n\n*This API requires the following crate features to be activated: `ServiceWorkerGlobalScope`*"]
    pub fn set_onnotificationclose(
        this: &ServiceWorkerGlobalScope,
        value: Option<&::js_sys::Function>,
    );
    # [ wasm_bindgen ( catch , method , structural , js_name = skipWaiting ) ]
    #[doc = "The `skipWaiting()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerGlobalScope/skipWaiting)\n\n*This API requires the following crate features to be activated: `ServiceWorkerGlobalScope`*"]
    pub fn skip_waiting(this: &ServiceWorkerGlobalScope) -> Result<::js_sys::Promise, JsValue>;
}
