use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = WorkerGlobalScope , extends = EventTarget , extends = :: js_sys :: Object , js_name = ServiceWorkerGlobalScope , typescript_type = "ServiceWorkerGlobalScope" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `ServiceWorkerGlobalScope` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerGlobalScope)
    ///
    ///*This API requires the following crate features to be activated: `ServiceWorkerGlobalScope`*
    pub type ServiceWorkerGlobalScope;

    #[cfg(feature = "Clients")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "ServiceWorkerGlobalScope" , js_name = clients ) ]
    ///Getter for the `clients` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerGlobalScope/clients)
    ///
    ///*This API requires the following crate features to be activated: `Clients`, `ServiceWorkerGlobalScope`*
    pub fn clients(this: &ServiceWorkerGlobalScope) -> Clients;

    #[cfg(feature = "ServiceWorkerRegistration")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "ServiceWorkerGlobalScope" , js_name = registration ) ]
    ///Getter for the `registration` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerGlobalScope/registration)
    ///
    ///*This API requires the following crate features to be activated: `ServiceWorkerGlobalScope`, `ServiceWorkerRegistration`*
    pub fn registration(this: &ServiceWorkerGlobalScope) -> ServiceWorkerRegistration;

    # [ wasm_bindgen ( structural , method , getter , js_class = "ServiceWorkerGlobalScope" , js_name = oninstall ) ]
    ///Getter for the `oninstall` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerGlobalScope/oninstall)
    ///
    ///*This API requires the following crate features to be activated: `ServiceWorkerGlobalScope`*
    pub fn oninstall(this: &ServiceWorkerGlobalScope) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "ServiceWorkerGlobalScope" , js_name = oninstall ) ]
    ///Setter for the `oninstall` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerGlobalScope/oninstall)
    ///
    ///*This API requires the following crate features to be activated: `ServiceWorkerGlobalScope`*
    pub fn set_oninstall(this: &ServiceWorkerGlobalScope, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "ServiceWorkerGlobalScope" , js_name = onactivate ) ]
    ///Getter for the `onactivate` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerGlobalScope/onactivate)
    ///
    ///*This API requires the following crate features to be activated: `ServiceWorkerGlobalScope`*
    pub fn onactivate(this: &ServiceWorkerGlobalScope) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "ServiceWorkerGlobalScope" , js_name = onactivate ) ]
    ///Setter for the `onactivate` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerGlobalScope/onactivate)
    ///
    ///*This API requires the following crate features to be activated: `ServiceWorkerGlobalScope`*
    pub fn set_onactivate(this: &ServiceWorkerGlobalScope, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "ServiceWorkerGlobalScope" , js_name = onfetch ) ]
    ///Getter for the `onfetch` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerGlobalScope/onfetch)
    ///
    ///*This API requires the following crate features to be activated: `ServiceWorkerGlobalScope`*
    pub fn onfetch(this: &ServiceWorkerGlobalScope) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "ServiceWorkerGlobalScope" , js_name = onfetch ) ]
    ///Setter for the `onfetch` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerGlobalScope/onfetch)
    ///
    ///*This API requires the following crate features to be activated: `ServiceWorkerGlobalScope`*
    pub fn set_onfetch(this: &ServiceWorkerGlobalScope, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "ServiceWorkerGlobalScope" , js_name = onmessage ) ]
    ///Getter for the `onmessage` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerGlobalScope/onmessage)
    ///
    ///*This API requires the following crate features to be activated: `ServiceWorkerGlobalScope`*
    pub fn onmessage(this: &ServiceWorkerGlobalScope) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "ServiceWorkerGlobalScope" , js_name = onmessage ) ]
    ///Setter for the `onmessage` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerGlobalScope/onmessage)
    ///
    ///*This API requires the following crate features to be activated: `ServiceWorkerGlobalScope`*
    pub fn set_onmessage(this: &ServiceWorkerGlobalScope, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "ServiceWorkerGlobalScope" , js_name = onpush ) ]
    ///Getter for the `onpush` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerGlobalScope/onpush)
    ///
    ///*This API requires the following crate features to be activated: `ServiceWorkerGlobalScope`*
    pub fn onpush(this: &ServiceWorkerGlobalScope) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "ServiceWorkerGlobalScope" , js_name = onpush ) ]
    ///Setter for the `onpush` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerGlobalScope/onpush)
    ///
    ///*This API requires the following crate features to be activated: `ServiceWorkerGlobalScope`*
    pub fn set_onpush(this: &ServiceWorkerGlobalScope, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "ServiceWorkerGlobalScope" , js_name = onpushsubscriptionchange ) ]
    ///Getter for the `onpushsubscriptionchange` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerGlobalScope/onpushsubscriptionchange)
    ///
    ///*This API requires the following crate features to be activated: `ServiceWorkerGlobalScope`*
    pub fn onpushsubscriptionchange(this: &ServiceWorkerGlobalScope) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "ServiceWorkerGlobalScope" , js_name = onpushsubscriptionchange ) ]
    ///Setter for the `onpushsubscriptionchange` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerGlobalScope/onpushsubscriptionchange)
    ///
    ///*This API requires the following crate features to be activated: `ServiceWorkerGlobalScope`*
    pub fn set_onpushsubscriptionchange(
        this: &ServiceWorkerGlobalScope,
        value: Option<&::js_sys::Function>,
    );

    # [ wasm_bindgen ( structural , method , getter , js_class = "ServiceWorkerGlobalScope" , js_name = onnotificationclick ) ]
    ///Getter for the `onnotificationclick` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerGlobalScope/onnotificationclick)
    ///
    ///*This API requires the following crate features to be activated: `ServiceWorkerGlobalScope`*
    pub fn onnotificationclick(this: &ServiceWorkerGlobalScope) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "ServiceWorkerGlobalScope" , js_name = onnotificationclick ) ]
    ///Setter for the `onnotificationclick` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerGlobalScope/onnotificationclick)
    ///
    ///*This API requires the following crate features to be activated: `ServiceWorkerGlobalScope`*
    pub fn set_onnotificationclick(
        this: &ServiceWorkerGlobalScope,
        value: Option<&::js_sys::Function>,
    );

    # [ wasm_bindgen ( structural , method , getter , js_class = "ServiceWorkerGlobalScope" , js_name = onnotificationclose ) ]
    ///Getter for the `onnotificationclose` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerGlobalScope/onnotificationclose)
    ///
    ///*This API requires the following crate features to be activated: `ServiceWorkerGlobalScope`*
    pub fn onnotificationclose(this: &ServiceWorkerGlobalScope) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "ServiceWorkerGlobalScope" , js_name = onnotificationclose ) ]
    ///Setter for the `onnotificationclose` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerGlobalScope/onnotificationclose)
    ///
    ///*This API requires the following crate features to be activated: `ServiceWorkerGlobalScope`*
    pub fn set_onnotificationclose(
        this: &ServiceWorkerGlobalScope,
        value: Option<&::js_sys::Function>,
    );

    # [ wasm_bindgen ( catch , method , structural , js_class = "ServiceWorkerGlobalScope" , js_name = skipWaiting ) ]
    ///The `skipWaiting()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerGlobalScope/skipWaiting)
    ///
    ///*This API requires the following crate features to be activated: `ServiceWorkerGlobalScope`*
    pub fn skip_waiting(this: &ServiceWorkerGlobalScope) -> Result<::js_sys::Promise, JsValue>;

}
