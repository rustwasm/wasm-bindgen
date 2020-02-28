use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = EventTarget , extends = :: js_sys :: Object , js_name = ServiceWorkerRegistration , typescript_name = ServiceWorkerRegistration ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ServiceWorkerRegistration` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerRegistration)\n\n*This API requires the following crate features to be activated: `ServiceWorkerRegistration`*"]
    pub type ServiceWorkerRegistration;
    # [ wasm_bindgen ( structural , method , getter , js_name = installing ) ]
    #[cfg(feature = "ServiceWorker")]
    #[doc = "Getter for the `installing` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerRegistration/installing)\n\n*This API requires the following crate features to be activated: `ServiceWorker`, `ServiceWorkerRegistration`*"]
    pub fn installing(this: &ServiceWorkerRegistration) -> Option<ServiceWorker>;
    # [ wasm_bindgen ( structural , method , getter , js_name = waiting ) ]
    #[cfg(feature = "ServiceWorker")]
    #[doc = "Getter for the `waiting` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerRegistration/waiting)\n\n*This API requires the following crate features to be activated: `ServiceWorker`, `ServiceWorkerRegistration`*"]
    pub fn waiting(this: &ServiceWorkerRegistration) -> Option<ServiceWorker>;
    # [ wasm_bindgen ( structural , method , getter , js_name = active ) ]
    #[cfg(feature = "ServiceWorker")]
    #[doc = "Getter for the `active` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerRegistration/active)\n\n*This API requires the following crate features to be activated: `ServiceWorker`, `ServiceWorkerRegistration`*"]
    pub fn active(this: &ServiceWorkerRegistration) -> Option<ServiceWorker>;
    # [ wasm_bindgen ( structural , method , getter , js_name = scope ) ]
    #[doc = "Getter for the `scope` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerRegistration/scope)\n\n*This API requires the following crate features to be activated: `ServiceWorkerRegistration`*"]
    pub fn scope(this: &ServiceWorkerRegistration) -> String;
    # [ wasm_bindgen ( structural , catch , method , getter , js_name = updateViaCache ) ]
    #[cfg(feature = "ServiceWorkerUpdateViaCache")]
    #[doc = "Getter for the `updateViaCache` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerRegistration/updateViaCache)\n\n*This API requires the following crate features to be activated: `ServiceWorkerRegistration`, `ServiceWorkerUpdateViaCache`*"]
    pub fn update_via_cache(
        this: &ServiceWorkerRegistration,
    ) -> Result<ServiceWorkerUpdateViaCache, JsValue>;
    # [ wasm_bindgen ( structural , method , getter , js_name = onupdatefound ) ]
    #[doc = "Getter for the `onupdatefound` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerRegistration/onupdatefound)\n\n*This API requires the following crate features to be activated: `ServiceWorkerRegistration`*"]
    pub fn onupdatefound(this: &ServiceWorkerRegistration) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onupdatefound ) ]
    #[doc = "Setter for the `onupdatefound` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerRegistration/onupdatefound)\n\n*This API requires the following crate features to be activated: `ServiceWorkerRegistration`*"]
    pub fn set_onupdatefound(this: &ServiceWorkerRegistration, value: Option<::js_sys::Function>);
    # [ wasm_bindgen ( structural , catch , method , getter , js_name = pushManager ) ]
    #[cfg(feature = "PushManager")]
    #[doc = "Getter for the `pushManager` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerRegistration/pushManager)\n\n*This API requires the following crate features to be activated: `PushManager`, `ServiceWorkerRegistration`*"]
    pub fn push_manager(this: &ServiceWorkerRegistration) -> Result<PushManager, JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = getNotifications ) ]
    #[doc = "The `getNotifications()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerRegistration/getNotifications)\n\n*This API requires the following crate features to be activated: `ServiceWorkerRegistration`*"]
    pub fn get_notifications(
        this: &ServiceWorkerRegistration,
    ) -> Result<::js_sys::Promise, JsValue>;
    #[cfg(feature = "GetNotificationOptions")]
    # [ wasm_bindgen ( catch , method , structural , js_name = getNotifications ) ]
    #[doc = "The `getNotifications()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerRegistration/getNotifications)\n\n*This API requires the following crate features to be activated: `GetNotificationOptions`, `ServiceWorkerRegistration`*"]
    pub fn get_notifications_with_filter(
        this: &ServiceWorkerRegistration,
        filter: &GetNotificationOptions,
    ) -> Result<::js_sys::Promise, JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = showNotification ) ]
    #[doc = "The `showNotification()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerRegistration/showNotification)\n\n*This API requires the following crate features to be activated: `ServiceWorkerRegistration`*"]
    pub fn show_notification(
        this: &ServiceWorkerRegistration,
        title: &str,
    ) -> Result<::js_sys::Promise, JsValue>;
    #[cfg(feature = "NotificationOptions")]
    # [ wasm_bindgen ( catch , method , structural , js_name = showNotification ) ]
    #[doc = "The `showNotification()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerRegistration/showNotification)\n\n*This API requires the following crate features to be activated: `NotificationOptions`, `ServiceWorkerRegistration`*"]
    pub fn show_notification_with_options(
        this: &ServiceWorkerRegistration,
        title: &str,
        options: &NotificationOptions,
    ) -> Result<::js_sys::Promise, JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = unregister ) ]
    #[doc = "The `unregister()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerRegistration/unregister)\n\n*This API requires the following crate features to be activated: `ServiceWorkerRegistration`*"]
    pub fn unregister(this: &ServiceWorkerRegistration) -> Result<::js_sys::Promise, JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = update ) ]
    #[doc = "The `update()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerRegistration/update)\n\n*This API requires the following crate features to be activated: `ServiceWorkerRegistration`*"]
    pub fn update(this: &ServiceWorkerRegistration) -> Result<::js_sys::Promise, JsValue>;
}
