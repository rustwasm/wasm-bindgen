#![allow(unused_imports)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    #[cfg(web_sys_unstable_apis)]
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = TransformStream , typescript_type = "TransformStream")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `TransformStream` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TransformStream)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TransformStream`*"]
    pub type TransformStream;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(catch, constructor, js_class = "TransformStream")]
    #[doc = "The `new TransformStream(..)` constructor, creating a new instance of `TransformStream`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TransformStream)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TransformStream`*"]
    pub fn new() -> Result<TransformStream, JsValue>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "ReadableStream")]
    # [wasm_bindgen (structural , method , getter , js_class = "TransformStream" , js_name = readable)]
    #[doc = "Getter for the `readable` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TransformStream)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TransformStream`, `ReadableStream`*"]
    pub fn readable(this: &TransformStream) -> ReadableStream;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "WritableStream")]
    # [wasm_bindgen (structural , method , getter , js_class = "TransformStream" , js_name = writable)]
    #[doc = "Getter for the `writable` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TransformStream)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TransformStream`, `WritableStream`*"]
    pub fn writable(this: &TransformStream) -> WritableStream;
}
