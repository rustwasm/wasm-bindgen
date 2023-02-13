#![allow(unused_imports)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = WebTransportBidirectionalStream , typescript_type = "WebTransportBidirectionalStream")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `WebTransportBidirectionalStream` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebTransportBidirectionalStream)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebTransportBidirectionalStream`*"]
    pub type WebTransportBidirectionalStream;
    #[cfg(feature = "WebTransportReceiveStream")]
    # [wasm_bindgen (structural , method , getter , js_class = "WebTransportBidirectionalStream" , js_name = readable)]
    #[doc = "Getter for the `readable` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebTransportBidirectionalStream/readable)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebTransportBidirectionalStream`, `WebTransportReceiveStream`*"]
    pub fn readable(this: &WebTransportBidirectionalStream) -> WebTransportReceiveStream;
}
