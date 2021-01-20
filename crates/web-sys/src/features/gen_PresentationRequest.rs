#![allow(unused_imports)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = EventTarget , extends = :: js_sys :: Object , js_name = PresentationRequest , typescript_type = "PresentationRequest")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `PresentationRequest` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PresentationRequest)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PresentationRequest`*"]
    pub type PresentationRequest;
    # [wasm_bindgen (structural , method , getter , js_class = "PresentationRequest" , js_name = onconnectionavailable)]
    #[doc = "Getter for the `onconnectionavailable` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PresentationRequest/onconnectionavailable)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PresentationRequest`*"]
    #[doc = ""]
    #[doc = "Return value: See the referenced MDN documentation or the IDL files for the signature of the callback inside the option."]
    pub fn onconnectionavailable(this: &PresentationRequest) -> Option<::js_sys::Function>;
    # [wasm_bindgen (structural , method , setter , js_class = "PresentationRequest" , js_name = onconnectionavailable)]
    #[doc = "Setter for the `onconnectionavailable` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PresentationRequest/onconnectionavailable)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PresentationRequest`*"]
    #[doc = ""]
    #[doc = "Argument: See the referenced MDN documentation or the IDL files for the signature of the callback inside the option."]
    pub fn set_onconnectionavailable(
        this: &PresentationRequest,
        value: Option<&::js_sys::Function>,
    );
    #[wasm_bindgen(catch, constructor, js_class = "PresentationRequest")]
    #[doc = "The `new PresentationRequest(..)` constructor, creating a new instance of `PresentationRequest`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PresentationRequest/PresentationRequest)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PresentationRequest`*"]
    pub fn new_with_url(url: &str) -> Result<PresentationRequest, JsValue>;
    #[wasm_bindgen(catch, constructor, js_class = "PresentationRequest")]
    #[doc = "The `new PresentationRequest(..)` constructor, creating a new instance of `PresentationRequest`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PresentationRequest/PresentationRequest)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PresentationRequest`*"]
    #[doc = ""]
    #[doc = "Argument `urls`: While the iterable or array can produce any JsValue as far as the type system is concerned, practically it is expected to contain a <code>&[str]</code>."]
    pub fn new_with_urls(urls: &::wasm_bindgen::JsValue) -> Result<PresentationRequest, JsValue>;
    # [wasm_bindgen (catch , method , structural , js_class = "PresentationRequest" , js_name = getAvailability)]
    #[doc = "The `getAvailability()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PresentationRequest/getAvailability)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PresentationRequest`*"]
    #[doc = ""]
    #[doc = "Return value: While the Promise of the successful result can produce any JsValue as far as the type system is concerned, practically it is expected to contain a <code>[PresentationAvailability]</code>. It can be converted like `<code>let result: [PresentationAvailability] = result?.await.into();</code>."]
    pub fn get_availability(this: &PresentationRequest) -> Result<::js_sys::Promise, JsValue>;
    # [wasm_bindgen (catch , method , structural , js_class = "PresentationRequest" , js_name = reconnect)]
    #[doc = "The `reconnect()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PresentationRequest/reconnect)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PresentationRequest`*"]
    #[doc = ""]
    #[doc = "Return value: While the Promise of the successful result can produce any JsValue as far as the type system is concerned, practically it is expected to contain a <code>[PresentationConnection]</code>. It can be converted like `<code>let result: [PresentationConnection] = result?.await.into();</code>."]
    pub fn reconnect(
        this: &PresentationRequest,
        presentation_id: &str,
    ) -> Result<::js_sys::Promise, JsValue>;
    # [wasm_bindgen (catch , method , structural , js_class = "PresentationRequest" , js_name = start)]
    #[doc = "The `start()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PresentationRequest/start)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PresentationRequest`*"]
    #[doc = ""]
    #[doc = "Return value: While the Promise of the successful result can produce any JsValue as far as the type system is concerned, practically it is expected to contain a <code>[PresentationConnection]</code>. It can be converted like `<code>let result: [PresentationConnection] = result?.await.into();</code>."]
    pub fn start(this: &PresentationRequest) -> Result<::js_sys::Promise, JsValue>;
}
