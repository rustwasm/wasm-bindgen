use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = EventTarget , extends = :: js_sys :: Object , js_name = PresentationRequest , typescript_type = "PresentationRequest" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `PresentationRequest` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PresentationRequest)
    ///
    ///*This API requires the following crate features to be activated: `PresentationRequest`*
    pub type PresentationRequest;

    # [ wasm_bindgen ( structural , method , getter , js_class = "PresentationRequest" , js_name = onconnectionavailable ) ]
    ///Getter for the `onconnectionavailable` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PresentationRequest/onconnectionavailable)
    ///
    ///*This API requires the following crate features to be activated: `PresentationRequest`*
    pub fn onconnectionavailable(this: &PresentationRequest) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "PresentationRequest" , js_name = onconnectionavailable ) ]
    ///Setter for the `onconnectionavailable` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PresentationRequest/onconnectionavailable)
    ///
    ///*This API requires the following crate features to be activated: `PresentationRequest`*
    pub fn set_onconnectionavailable(
        this: &PresentationRequest,
        value: Option<&::js_sys::Function>,
    );

    #[wasm_bindgen(catch, constructor, js_class = "PresentationRequest")]
    ///The `new PresentationRequest(..)` constructor, creating a new instance of `PresentationRequest`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PresentationRequest/PresentationRequest)
    ///
    ///*This API requires the following crate features to be activated: `PresentationRequest`*
    pub fn new_with_url(url: &str) -> Result<PresentationRequest, JsValue>;

    #[wasm_bindgen(catch, constructor, js_class = "PresentationRequest")]
    ///The `new PresentationRequest(..)` constructor, creating a new instance of `PresentationRequest`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PresentationRequest/PresentationRequest)
    ///
    ///*This API requires the following crate features to be activated: `PresentationRequest`*
    pub fn new_with_urls(urls: &::wasm_bindgen::JsValue) -> Result<PresentationRequest, JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "PresentationRequest" , js_name = getAvailability ) ]
    ///The `getAvailability()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PresentationRequest/getAvailability)
    ///
    ///*This API requires the following crate features to be activated: `PresentationRequest`*
    pub fn get_availability(this: &PresentationRequest) -> Result<::js_sys::Promise, JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "PresentationRequest" , js_name = reconnect ) ]
    ///The `reconnect()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PresentationRequest/reconnect)
    ///
    ///*This API requires the following crate features to be activated: `PresentationRequest`*
    pub fn reconnect(
        this: &PresentationRequest,
        presentation_id: &str,
    ) -> Result<::js_sys::Promise, JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "PresentationRequest" , js_name = start ) ]
    ///The `start()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PresentationRequest/start)
    ///
    ///*This API requires the following crate features to be activated: `PresentationRequest`*
    pub fn start(this: &PresentationRequest) -> Result<::js_sys::Promise, JsValue>;

}
