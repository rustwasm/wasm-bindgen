use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = WebGLProgram , typescript_name = WebGLProgram ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `WebGlProgram` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLProgram)
    ///
    ///*This API requires the following crate features to be activated: `WebGlProgram`*
    pub type WebGlProgram;

}
