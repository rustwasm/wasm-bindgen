#![allow(unused_imports)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = ReadableStream , typescript_type = "ReadableStream")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ReadableStream` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ReadableStream)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ReadableStream`*"]
    pub type ReadableStream;
    # [wasm_bindgen (structural , method , getter , js_class = "ReadableStream" , js_name = locked)]
    #[doc = "Getter for the `locked` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ReadableStream/locked)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ReadableStream`*"]
    pub fn locked(this: &ReadableStream) -> bool;
    # [wasm_bindgen (method , structural , js_class = "ReadableStream" , js_name = cancel)]
    #[doc = "The `cancel()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ReadableStream/cancel)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ReadableStream`*"]
    pub fn cancel(this: &ReadableStream) -> ::js_sys::Promise;
    # [wasm_bindgen (method , structural , js_class = "ReadableStream" , js_name = cancel)]
    #[doc = "The `cancel()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ReadableStream/cancel)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ReadableStream`*"]
    pub fn cancel_with_reason(
        this: &ReadableStream,
        reason: &::wasm_bindgen::JsValue,
    ) -> ::js_sys::Promise;
    # [wasm_bindgen (method , structural , js_class = "ReadableStream" , js_name = getReader)]
    #[doc = "The `getReader()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ReadableStream/getReader)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ReadableStream`*"]
    pub fn get_reader(this: &ReadableStream) -> ::wasm_bindgen::JsValue;
    #[cfg(feature = "ReadableStreamGetReaderOptions")]
    # [wasm_bindgen (method , structural , js_class = "ReadableStream" , js_name = getReader)]
    #[doc = "The `getReader()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ReadableStream/getReader)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ReadableStream`, `ReadableStreamGetReaderOptions`*"]
    pub fn get_reader_with_options(
        this: &ReadableStream,
        options: &ReadableStreamGetReaderOptions,
    ) -> ::wasm_bindgen::JsValue;
    #[cfg(feature = "ReadableWritablePair")]
    # [wasm_bindgen (method , structural , js_class = "ReadableStream" , js_name = pipeThrough)]
    #[doc = "The `pipeThrough()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ReadableStream/pipeThrough)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ReadableStream`, `ReadableWritablePair`*"]
    pub fn pipe_through(this: &ReadableStream, transform: &ReadableWritablePair) -> ReadableStream;
    #[cfg(all(feature = "ReadableWritablePair", feature = "StreamPipeOptions",))]
    # [wasm_bindgen (method , structural , js_class = "ReadableStream" , js_name = pipeThrough)]
    #[doc = "The `pipeThrough()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ReadableStream/pipeThrough)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ReadableStream`, `ReadableWritablePair`, `StreamPipeOptions`*"]
    pub fn pipe_through_with_options(
        this: &ReadableStream,
        transform: &ReadableWritablePair,
        options: &StreamPipeOptions,
    ) -> ReadableStream;
    # [wasm_bindgen (method , structural , js_class = "ReadableStream" , js_name = tee)]
    #[doc = "The `tee()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ReadableStream/tee)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ReadableStream`*"]
    pub fn tee(this: &ReadableStream) -> ::js_sys::Array;
}
