#![allow(unused_imports)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = ReadableStream , extends = :: js_sys :: Object , js_name = WebTransportReceiveStream , typescript_type = "WebTransportReceiveStream")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `WebTransportReceiveStream` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebTransportReceiveStream)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebTransportReceiveStream`*"]
    pub type WebTransportReceiveStream;
    # [wasm_bindgen (method , structural , js_class = "WebTransportReceiveStream" , js_name = getStats)]
    #[doc = "The `getStats()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebTransportReceiveStream/getStats)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebTransportReceiveStream`*"]
    pub fn get_stats(this: &WebTransportReceiveStream) -> ::js_sys::Promise;
}
