use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = ReadableStream , typescript_type = "ReadableStream" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `ReadableStream` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ReadableStream)
    ///
    ///*This API requires the following crate features to be activated: `ReadableStream`*
    pub type ReadableStream;

}
