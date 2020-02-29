use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = WebGLSampler , typescript_name = WebGLSampler ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `WebGlSampler` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLSampler)
    ///
    ///*This API requires the following crate features to be activated: `WebGlSampler`*
    pub type WebGlSampler;

}
