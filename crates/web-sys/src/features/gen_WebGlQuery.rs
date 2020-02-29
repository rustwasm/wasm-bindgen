use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = WebGLQuery , typescript_name = WebGLQuery ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `WebGlQuery` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLQuery)
    ///
    ///*This API requires the following crate features to be activated: `WebGlQuery`*
    pub type WebGlQuery;

}
