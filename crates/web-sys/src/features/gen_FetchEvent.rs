use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = ExtendableEvent , extends = Event , extends = :: js_sys :: Object , js_name = FetchEvent , typescript_name = FetchEvent ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `FetchEvent` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FetchEvent)
    ///
    ///*This API requires the following crate features to be activated: `FetchEvent`*
    pub type FetchEvent;

    #[cfg(feature = "Request")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "FetchEvent" , js_name = request ) ]
    ///Getter for the `request` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FetchEvent/request)
    ///
    ///*This API requires the following crate features to be activated: `FetchEvent`, `Request`*
    pub fn request(this: &FetchEvent) -> Request;

    # [ wasm_bindgen ( structural , method , getter , js_class = "FetchEvent" , js_name = clientId ) ]
    ///Getter for the `clientId` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FetchEvent/clientId)
    ///
    ///*This API requires the following crate features to be activated: `FetchEvent`*
    pub fn client_id(this: &FetchEvent) -> Option<String>;

    # [ wasm_bindgen ( structural , method , getter , js_class = "FetchEvent" , js_name = isReload ) ]
    ///Getter for the `isReload` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FetchEvent/isReload)
    ///
    ///*This API requires the following crate features to be activated: `FetchEvent`*
    pub fn is_reload(this: &FetchEvent) -> bool;

    #[cfg(feature = "FetchEventInit")]
    #[wasm_bindgen(catch, constructor, js_class = "FetchEvent")]
    ///The `new FetchEvent(..)` constructor, creating a new instance of `FetchEvent`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FetchEvent/FetchEvent)
    ///
    ///*This API requires the following crate features to be activated: `FetchEvent`, `FetchEventInit`*
    pub fn new(type_: &str, event_init_dict: &FetchEventInit) -> Result<FetchEvent, JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "FetchEvent" , js_name = respondWith ) ]
    ///The `respondWith()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FetchEvent/respondWith)
    ///
    ///*This API requires the following crate features to be activated: `FetchEvent`*
    pub fn respond_with(this: &FetchEvent, r: &::js_sys::Promise) -> Result<(), JsValue>;

}
