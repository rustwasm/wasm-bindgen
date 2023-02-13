#![allow(unused_imports)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = DomException , extends = :: js_sys :: Object , js_name = WebTransportError , typescript_type = "WebTransportError")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `WebTransportError` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebTransportError)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebTransportError`*"]
    pub type WebTransportError;
    #[cfg(feature = "WebTransportErrorSource")]
    # [wasm_bindgen (structural , method , getter , js_class = "WebTransportError" , js_name = source)]
    #[doc = "Getter for the `source` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebTransportError/source)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebTransportError`, `WebTransportErrorSource`*"]
    pub fn source(this: &WebTransportError) -> WebTransportErrorSource;
    # [wasm_bindgen (structural , method , getter , js_class = "WebTransportError" , js_name = streamErrorCode)]
    #[doc = "Getter for the `streamErrorCode` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebTransportError/streamErrorCode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebTransportError`*"]
    pub fn stream_error_code(this: &WebTransportError) -> Option<u8>;
    #[wasm_bindgen(catch, constructor, js_class = "WebTransportError")]
    #[doc = "The `new WebTransportError(..)` constructor, creating a new instance of `WebTransportError`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebTransportError/WebTransportError)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebTransportError`*"]
    pub fn new() -> Result<WebTransportError, JsValue>;
    #[cfg(feature = "WebTransportErrorInit")]
    #[wasm_bindgen(catch, constructor, js_class = "WebTransportError")]
    #[doc = "The `new WebTransportError(..)` constructor, creating a new instance of `WebTransportError`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebTransportError/WebTransportError)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebTransportError`, `WebTransportErrorInit`*"]
    pub fn new_with_init(init: &WebTransportErrorInit) -> Result<WebTransportError, JsValue>;
}
