use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = PushSubscription , typescript_name = PushSubscription ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `PushSubscription` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PushSubscription)\n\n*This API requires the following crate features to be activated: `PushSubscription`*"]
    pub type PushSubscription;
    # [ wasm_bindgen ( structural , method , getter , js_name = endpoint ) ]
    #[doc = "Getter for the `endpoint` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PushSubscription/endpoint)\n\n*This API requires the following crate features to be activated: `PushSubscription`*"]
    pub fn endpoint(this: &PushSubscription) -> String;
    # [ wasm_bindgen ( structural , method , getter , js_name = options ) ]
    #[cfg(feature = "PushSubscriptionOptions")]
    #[doc = "Getter for the `options` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PushSubscription/options)\n\n*This API requires the following crate features to be activated: `PushSubscription`, `PushSubscriptionOptions`*"]
    pub fn options(this: &PushSubscription) -> PushSubscriptionOptions;
    #[cfg(feature = "PushEncryptionKeyName")]
    # [ wasm_bindgen ( catch , method , structural , js_name = getKey ) ]
    #[doc = "The `getKey()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PushSubscription/getKey)\n\n*This API requires the following crate features to be activated: `PushEncryptionKeyName`, `PushSubscription`*"]
    pub fn get_key(
        this: &PushSubscription,
        name: PushEncryptionKeyName,
    ) -> Result<Option<::js_sys::ArrayBuffer>, JsValue>;
    #[cfg(feature = "PushSubscriptionJson")]
    # [ wasm_bindgen ( catch , method , structural , js_name = toJSON ) ]
    #[doc = "The `toJSON()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PushSubscription/toJSON)\n\n*This API requires the following crate features to be activated: `PushSubscription`, `PushSubscriptionJson`*"]
    pub fn to_json(this: &PushSubscription) -> Result<PushSubscriptionJson, JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = unsubscribe ) ]
    #[doc = "The `unsubscribe()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PushSubscription/unsubscribe)\n\n*This API requires the following crate features to be activated: `PushSubscription`*"]
    pub fn unsubscribe(this: &PushSubscription) -> Result<::js_sys::Promise, JsValue>;
}
