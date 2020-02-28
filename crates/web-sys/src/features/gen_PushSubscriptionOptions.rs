use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = PushSubscriptionOptions , typescript_name = PushSubscriptionOptions ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `PushSubscriptionOptions` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PushSubscriptionOptions)\n\n*This API requires the following crate features to be activated: `PushSubscriptionOptions`*"]
    pub type PushSubscriptionOptions;
    # [ wasm_bindgen ( structural , catch , method , getter , js_name = applicationServerKey ) ]
    #[doc = "Getter for the `applicationServerKey` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PushSubscriptionOptions/applicationServerKey)\n\n*This API requires the following crate features to be activated: `PushSubscriptionOptions`*"]
    pub fn application_server_key(
        this: &PushSubscriptionOptions,
    ) -> Result<Option<::js_sys::ArrayBuffer>, JsValue>;
}
