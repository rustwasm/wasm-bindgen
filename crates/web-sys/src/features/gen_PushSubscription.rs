use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = PushSubscription , typescript_type = "PushSubscription" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `PushSubscription` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PushSubscription)
    ///
    ///*This API requires the following crate features to be activated: `PushSubscription`*
    pub type PushSubscription;

    # [ wasm_bindgen ( structural , method , getter , js_class = "PushSubscription" , js_name = endpoint ) ]
    ///Getter for the `endpoint` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PushSubscription/endpoint)
    ///
    ///*This API requires the following crate features to be activated: `PushSubscription`*
    pub fn endpoint(this: &PushSubscription) -> String;

    #[cfg(feature = "PushSubscriptionOptions")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "PushSubscription" , js_name = options ) ]
    ///Getter for the `options` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PushSubscription/options)
    ///
    ///*This API requires the following crate features to be activated: `PushSubscription`, `PushSubscriptionOptions`*
    pub fn options(this: &PushSubscription) -> PushSubscriptionOptions;

    #[cfg(feature = "PushEncryptionKeyName")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "PushSubscription" , js_name = getKey ) ]
    ///The `getKey()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PushSubscription/getKey)
    ///
    ///*This API requires the following crate features to be activated: `PushEncryptionKeyName`, `PushSubscription`*
    pub fn get_key(
        this: &PushSubscription,
        name: PushEncryptionKeyName,
    ) -> Result<Option<::js_sys::ArrayBuffer>, JsValue>;

    #[cfg(feature = "PushSubscriptionJson")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "PushSubscription" , js_name = toJSON ) ]
    ///The `toJSON()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PushSubscription/toJSON)
    ///
    ///*This API requires the following crate features to be activated: `PushSubscription`, `PushSubscriptionJson`*
    pub fn to_json(this: &PushSubscription) -> Result<PushSubscriptionJson, JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "PushSubscription" , js_name = unsubscribe ) ]
    ///The `unsubscribe()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PushSubscription/unsubscribe)
    ///
    ///*This API requires the following crate features to be activated: `PushSubscription`*
    pub fn unsubscribe(this: &PushSubscription) -> Result<::js_sys::Promise, JsValue>;

}
