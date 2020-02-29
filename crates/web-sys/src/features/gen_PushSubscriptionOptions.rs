use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = PushSubscriptionOptions , typescript_name = PushSubscriptionOptions ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `PushSubscriptionOptions` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PushSubscriptionOptions)
    ///
    ///*This API requires the following crate features to be activated: `PushSubscriptionOptions`*
    pub type PushSubscriptionOptions;

    # [ wasm_bindgen ( structural , catch , method , getter , js_class = "PushSubscriptionOptions" , js_name = applicationServerKey ) ]
    ///Getter for the `applicationServerKey` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PushSubscriptionOptions/applicationServerKey)
    ///
    ///*This API requires the following crate features to be activated: `PushSubscriptionOptions`*
    pub fn application_server_key(
        this: &PushSubscriptionOptions,
    ) -> Result<Option<::js_sys::ArrayBuffer>, JsValue>;

}
