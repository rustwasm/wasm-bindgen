use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = PushManager , typescript_name = PushManager ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `PushManager` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PushManager)\n\n*This API requires the following crate features to be activated: `PushManager`*"]
    pub type PushManager;
    # [ wasm_bindgen ( catch , method , structural , js_class = "PushManager" , js_name = getSubscription ) ]
    #[doc = "The `getSubscription()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PushManager/getSubscription)\n\n*This API requires the following crate features to be activated: `PushManager`*"]
    pub fn get_subscription(this: &PushManager) -> Result<::js_sys::Promise, JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_class = "PushManager" , js_name = permissionState ) ]
    #[doc = "The `permissionState()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PushManager/permissionState)\n\n*This API requires the following crate features to be activated: `PushManager`*"]
    pub fn permission_state(this: &PushManager) -> Result<::js_sys::Promise, JsValue>;
    #[cfg(feature = "PushSubscriptionOptionsInit")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "PushManager" , js_name = permissionState ) ]
    #[doc = "The `permissionState()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PushManager/permissionState)\n\n*This API requires the following crate features to be activated: `PushManager`, `PushSubscriptionOptionsInit`*"]
    pub fn permission_state_with_options(
        this: &PushManager,
        options: &PushSubscriptionOptionsInit,
    ) -> Result<::js_sys::Promise, JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_class = "PushManager" , js_name = subscribe ) ]
    #[doc = "The `subscribe()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PushManager/subscribe)\n\n*This API requires the following crate features to be activated: `PushManager`*"]
    pub fn subscribe(this: &PushManager) -> Result<::js_sys::Promise, JsValue>;
    #[cfg(feature = "PushSubscriptionOptionsInit")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "PushManager" , js_name = subscribe ) ]
    #[doc = "The `subscribe()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PushManager/subscribe)\n\n*This API requires the following crate features to be activated: `PushManager`, `PushSubscriptionOptionsInit`*"]
    pub fn subscribe_with_options(
        this: &PushManager,
        options: &PushSubscriptionOptionsInit,
    ) -> Result<::js_sys::Promise, JsValue>;
}
