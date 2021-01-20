#![allow(unused_imports)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = PushManager , typescript_type = "PushManager")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `PushManager` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PushManager)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PushManager`*"]
    pub type PushManager;
    # [wasm_bindgen (catch , method , structural , js_class = "PushManager" , js_name = getSubscription)]
    #[doc = "The `getSubscription()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PushManager/getSubscription)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PushManager`*"]
    #[doc = ""]
    #[doc = "Return value: While the Promise of the successful result can produce any JsValue as far as the type system is concerned, practically it is expected to contain a <code>[Option]<[PushSubscription]></code>. It can be converted like `<code>let result: [Option]<[PushSubscription]> = result?.await.into();</code>."]
    pub fn get_subscription(this: &PushManager) -> Result<::js_sys::Promise, JsValue>;
    # [wasm_bindgen (catch , method , structural , js_class = "PushManager" , js_name = permissionState)]
    #[doc = "The `permissionState()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PushManager/permissionState)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PushManager`*"]
    #[doc = ""]
    #[doc = "Return value: While the Promise of the successful result can produce any JsValue as far as the type system is concerned, practically it is expected to contain a <code>[PushPermissionState]</code>. It can be converted like `<code>let result: [PushPermissionState] = result?.await.into();</code>."]
    pub fn permission_state(this: &PushManager) -> Result<::js_sys::Promise, JsValue>;
    #[cfg(feature = "PushSubscriptionOptionsInit")]
    # [wasm_bindgen (catch , method , structural , js_class = "PushManager" , js_name = permissionState)]
    #[doc = "The `permissionState()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PushManager/permissionState)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PushManager`, `PushSubscriptionOptionsInit`*"]
    #[doc = ""]
    #[doc = "Return value: While the Promise of the successful result can produce any JsValue as far as the type system is concerned, practically it is expected to contain a <code>[PushPermissionState]</code>. It can be converted like `<code>let result: [PushPermissionState] = result?.await.into();</code>."]
    pub fn permission_state_with_options(
        this: &PushManager,
        options: &PushSubscriptionOptionsInit,
    ) -> Result<::js_sys::Promise, JsValue>;
    # [wasm_bindgen (catch , method , structural , js_class = "PushManager" , js_name = subscribe)]
    #[doc = "The `subscribe()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PushManager/subscribe)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PushManager`*"]
    #[doc = ""]
    #[doc = "Return value: While the Promise of the successful result can produce any JsValue as far as the type system is concerned, practically it is expected to contain a <code>[PushSubscription]</code>. It can be converted like `<code>let result: [PushSubscription] = result?.await.into();</code>."]
    pub fn subscribe(this: &PushManager) -> Result<::js_sys::Promise, JsValue>;
    #[cfg(feature = "PushSubscriptionOptionsInit")]
    # [wasm_bindgen (catch , method , structural , js_class = "PushManager" , js_name = subscribe)]
    #[doc = "The `subscribe()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PushManager/subscribe)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PushManager`, `PushSubscriptionOptionsInit`*"]
    #[doc = ""]
    #[doc = "Return value: While the Promise of the successful result can produce any JsValue as far as the type system is concerned, practically it is expected to contain a <code>[PushSubscription]</code>. It can be converted like `<code>let result: [PushSubscription] = result?.await.into();</code>."]
    pub fn subscribe_with_options(
        this: &PushManager,
        options: &PushSubscriptionOptionsInit,
    ) -> Result<::js_sys::Promise, JsValue>;
}
