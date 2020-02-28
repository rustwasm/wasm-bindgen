use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = ExtendableEvent , extends = Event , extends = :: js_sys :: Object , js_name = FetchEvent , typescript_name = FetchEvent ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `FetchEvent` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FetchEvent)\n\n*This API requires the following crate features to be activated: `FetchEvent`*"]
    pub type FetchEvent;
    # [ wasm_bindgen ( structural , method , getter , js_class = "FetchEvent" , js_name = request ) ]
    #[cfg(feature = "Request")]
    #[doc = "Getter for the `request` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FetchEvent/request)\n\n*This API requires the following crate features to be activated: `FetchEvent`, `Request`*"]
    pub fn request(this: &FetchEvent) -> Request;
    # [ wasm_bindgen ( structural , method , getter , js_class = "FetchEvent" , js_name = clientId ) ]
    #[doc = "Getter for the `clientId` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FetchEvent/clientId)\n\n*This API requires the following crate features to be activated: `FetchEvent`*"]
    pub fn client_id(this: &FetchEvent) -> Option<String>;
    # [ wasm_bindgen ( structural , method , getter , js_class = "FetchEvent" , js_name = isReload ) ]
    #[doc = "Getter for the `isReload` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FetchEvent/isReload)\n\n*This API requires the following crate features to be activated: `FetchEvent`*"]
    pub fn is_reload(this: &FetchEvent) -> bool;
    #[cfg(feature = "FetchEventInit")]
    #[wasm_bindgen(catch, js_class = "FetchEvent", constructor)]
    #[doc = "The `new FetchEvent(..)` constructor, creating a new instance of `FetchEvent`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FetchEvent/FetchEvent)\n\n*This API requires the following crate features to be activated: `FetchEvent`, `FetchEventInit`*"]
    pub fn new(
        this: &FetchEvent,
        type_: &str,
        event_init_dict: &FetchEventInit,
    ) -> Result<FetchEvent, JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_class = "FetchEvent" , js_name = respondWith ) ]
    #[doc = "The `respondWith()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FetchEvent/respondWith)\n\n*This API requires the following crate features to be activated: `FetchEvent`*"]
    pub fn respond_with(this: &FetchEvent, r: &::js_sys::Promise) -> Result<(), JsValue>;
}
