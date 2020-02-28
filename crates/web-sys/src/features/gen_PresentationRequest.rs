use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = EventTarget , extends = :: js_sys :: Object , js_name = PresentationRequest , typescript_name = PresentationRequest ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `PresentationRequest` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PresentationRequest)\n\n*This API requires the following crate features to be activated: `PresentationRequest`*"]
    pub type PresentationRequest;
    # [ wasm_bindgen ( structural , method , getter , js_name = onconnectionavailable ) ]
    #[doc = "Getter for the `onconnectionavailable` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PresentationRequest/onconnectionavailable)\n\n*This API requires the following crate features to be activated: `PresentationRequest`*"]
    pub fn onconnectionavailable(this: &PresentationRequest) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onconnectionavailable ) ]
    #[doc = "Setter for the `onconnectionavailable` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PresentationRequest/onconnectionavailable)\n\n*This API requires the following crate features to be activated: `PresentationRequest`*"]
    pub fn set_onconnectionavailable(this: &PresentationRequest, value: Option<::js_sys::Function>);
    #[wasm_bindgen(catch, constructor)]
    #[doc = "The `new PresentationRequest(..)` constructor, creating a new instance of `PresentationRequest`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PresentationRequest/PresentationRequest)\n\n*This API requires the following crate features to be activated: `PresentationRequest`*"]
    pub fn new_with_url(
        this: &PresentationRequest,
        url: &str,
    ) -> Result<PresentationRequest, JsValue>;
    #[wasm_bindgen(catch, constructor)]
    #[doc = "The `new PresentationRequest(..)` constructor, creating a new instance of `PresentationRequest`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PresentationRequest/PresentationRequest)\n\n*This API requires the following crate features to be activated: `PresentationRequest`*"]
    pub fn new_with_urls(
        this: &PresentationRequest,
        urls: &::wasm_bindgen::JsValue,
    ) -> Result<PresentationRequest, JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = getAvailability ) ]
    #[doc = "The `getAvailability()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PresentationRequest/getAvailability)\n\n*This API requires the following crate features to be activated: `PresentationRequest`*"]
    pub fn get_availability(this: &PresentationRequest) -> Result<::js_sys::Promise, JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = reconnect ) ]
    #[doc = "The `reconnect()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PresentationRequest/reconnect)\n\n*This API requires the following crate features to be activated: `PresentationRequest`*"]
    pub fn reconnect(
        this: &PresentationRequest,
        presentation_id: &str,
    ) -> Result<::js_sys::Promise, JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = start ) ]
    #[doc = "The `start()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PresentationRequest/start)\n\n*This API requires the following crate features to be activated: `PresentationRequest`*"]
    pub fn start(this: &PresentationRequest) -> Result<::js_sys::Promise, JsValue>;
}
