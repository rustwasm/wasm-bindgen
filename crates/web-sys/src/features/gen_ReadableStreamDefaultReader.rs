#![allow(unused_imports)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = ReadableStreamDefaultReader , typescript_type = "ReadableStreamDefaultReader")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ReadableStreamDefaultReader` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ReadableStreamDefaultReader)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ReadableStreamDefaultReader`*"]
    pub type ReadableStreamDefaultReader;
    # [wasm_bindgen (structural , method , js_class = "ReadableStreamDefaultReader" , js_name = read)]
    #[doc = "The `read()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ReadableStreamDefaultReader/read)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ReadableStreamDefaultReader`*"]
    pub fn read(this: &ReadableStreamDefaultReader) -> ::js_sys::Promise;
    # [wasm_bindgen (structural , method , js_class = "ReadableStreamDefaultReader" , js_name = releaseLock)]
    #[doc = "The `releaseLock()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ReadableStreamDefaultReader/releaseLock)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ReadableStreamDefaultReader`*"]
    pub fn release_lock(this: &ReadableStreamDefaultReader);
    # [wasm_bindgen (structural , method , getter , js_class = "ReadableStreamDefaultReader" , js_name = closed)]
    #[doc = "Getter for the `closed` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ReadableStreamDefaultReader/closed)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ReadableStreamDefaultReader`*"]
    pub fn closed(this: &ReadableStreamDefaultReader) -> ::js_sys::Promise;
    # [wasm_bindgen (structural , method , js_class = "ReadableStreamDefaultReader" , js_name = cancel)]
    #[doc = "The `cancel()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ReadableStreamDefaultReader/cancel)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ReadableStreamDefaultReader`*"]
    pub fn cancel(this: &ReadableStreamDefaultReader) -> ::js_sys::Promise;
    # [wasm_bindgen (structural , method , js_class = "ReadableStreamDefaultReader" , js_name = cancel)]
    #[doc = "The `cancel()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ReadableStreamDefaultReader/cancel)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ReadableStreamDefaultReader`*"]
    pub fn cancel_with_reason(
        this: &ReadableStreamDefaultReader,
        reason: &str,
    ) -> ::js_sys::Promise;
}
