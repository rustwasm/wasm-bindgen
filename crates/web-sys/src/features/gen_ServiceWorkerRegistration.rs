use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = EventTarget , extends = :: js_sys :: Object , js_name = ServiceWorkerRegistration , typescript_type = "ServiceWorkerRegistration" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `ServiceWorkerRegistration` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerRegistration)
    ///
    ///*This API requires the following crate features to be activated: `ServiceWorkerRegistration`*
    pub type ServiceWorkerRegistration;

    #[cfg(feature = "ServiceWorker")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "ServiceWorkerRegistration" , js_name = installing ) ]
    ///Getter for the `installing` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerRegistration/installing)
    ///
    ///*This API requires the following crate features to be activated: `ServiceWorker`, `ServiceWorkerRegistration`*
    pub fn installing(this: &ServiceWorkerRegistration) -> Option<ServiceWorker>;

    #[cfg(feature = "ServiceWorker")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "ServiceWorkerRegistration" , js_name = waiting ) ]
    ///Getter for the `waiting` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerRegistration/waiting)
    ///
    ///*This API requires the following crate features to be activated: `ServiceWorker`, `ServiceWorkerRegistration`*
    pub fn waiting(this: &ServiceWorkerRegistration) -> Option<ServiceWorker>;

    #[cfg(feature = "ServiceWorker")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "ServiceWorkerRegistration" , js_name = active ) ]
    ///Getter for the `active` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerRegistration/active)
    ///
    ///*This API requires the following crate features to be activated: `ServiceWorker`, `ServiceWorkerRegistration`*
    pub fn active(this: &ServiceWorkerRegistration) -> Option<ServiceWorker>;

    # [ wasm_bindgen ( structural , method , getter , js_class = "ServiceWorkerRegistration" , js_name = scope ) ]
    ///Getter for the `scope` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerRegistration/scope)
    ///
    ///*This API requires the following crate features to be activated: `ServiceWorkerRegistration`*
    pub fn scope(this: &ServiceWorkerRegistration) -> String;

    #[cfg(feature = "ServiceWorkerUpdateViaCache")]
    # [ wasm_bindgen ( structural , catch , method , getter , js_class = "ServiceWorkerRegistration" , js_name = updateViaCache ) ]
    ///Getter for the `updateViaCache` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerRegistration/updateViaCache)
    ///
    ///*This API requires the following crate features to be activated: `ServiceWorkerRegistration`, `ServiceWorkerUpdateViaCache`*
    pub fn update_via_cache(
        this: &ServiceWorkerRegistration,
    ) -> Result<ServiceWorkerUpdateViaCache, JsValue>;

    # [ wasm_bindgen ( structural , method , getter , js_class = "ServiceWorkerRegistration" , js_name = onupdatefound ) ]
    ///Getter for the `onupdatefound` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerRegistration/onupdatefound)
    ///
    ///*This API requires the following crate features to be activated: `ServiceWorkerRegistration`*
    pub fn onupdatefound(this: &ServiceWorkerRegistration) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "ServiceWorkerRegistration" , js_name = onupdatefound ) ]
    ///Setter for the `onupdatefound` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerRegistration/onupdatefound)
    ///
    ///*This API requires the following crate features to be activated: `ServiceWorkerRegistration`*
    pub fn set_onupdatefound(this: &ServiceWorkerRegistration, value: Option<&::js_sys::Function>);

    #[cfg(feature = "PushManager")]
    # [ wasm_bindgen ( structural , catch , method , getter , js_class = "ServiceWorkerRegistration" , js_name = pushManager ) ]
    ///Getter for the `pushManager` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerRegistration/pushManager)
    ///
    ///*This API requires the following crate features to be activated: `PushManager`, `ServiceWorkerRegistration`*
    pub fn push_manager(this: &ServiceWorkerRegistration) -> Result<PushManager, JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "ServiceWorkerRegistration" , js_name = getNotifications ) ]
    ///The `getNotifications()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerRegistration/getNotifications)
    ///
    ///*This API requires the following crate features to be activated: `ServiceWorkerRegistration`*
    pub fn get_notifications(
        this: &ServiceWorkerRegistration,
    ) -> Result<::js_sys::Promise, JsValue>;

    #[cfg(feature = "GetNotificationOptions")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "ServiceWorkerRegistration" , js_name = getNotifications ) ]
    ///The `getNotifications()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerRegistration/getNotifications)
    ///
    ///*This API requires the following crate features to be activated: `GetNotificationOptions`, `ServiceWorkerRegistration`*
    pub fn get_notifications_with_filter(
        this: &ServiceWorkerRegistration,
        filter: &GetNotificationOptions,
    ) -> Result<::js_sys::Promise, JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "ServiceWorkerRegistration" , js_name = showNotification ) ]
    ///The `showNotification()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerRegistration/showNotification)
    ///
    ///*This API requires the following crate features to be activated: `ServiceWorkerRegistration`*
    pub fn show_notification(
        this: &ServiceWorkerRegistration,
        title: &str,
    ) -> Result<::js_sys::Promise, JsValue>;

    #[cfg(feature = "NotificationOptions")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "ServiceWorkerRegistration" , js_name = showNotification ) ]
    ///The `showNotification()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerRegistration/showNotification)
    ///
    ///*This API requires the following crate features to be activated: `NotificationOptions`, `ServiceWorkerRegistration`*
    pub fn show_notification_with_options(
        this: &ServiceWorkerRegistration,
        title: &str,
        options: &NotificationOptions,
    ) -> Result<::js_sys::Promise, JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "ServiceWorkerRegistration" , js_name = unregister ) ]
    ///The `unregister()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerRegistration/unregister)
    ///
    ///*This API requires the following crate features to be activated: `ServiceWorkerRegistration`*
    pub fn unregister(this: &ServiceWorkerRegistration) -> Result<::js_sys::Promise, JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "ServiceWorkerRegistration" , js_name = update ) ]
    ///The `update()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerRegistration/update)
    ///
    ///*This API requires the following crate features to be activated: `ServiceWorkerRegistration`*
    pub fn update(this: &ServiceWorkerRegistration) -> Result<::js_sys::Promise, JsValue>;

}
