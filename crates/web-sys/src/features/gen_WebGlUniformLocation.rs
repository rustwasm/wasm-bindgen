use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = WebGLUniformLocation , typescript_name = WebGLUniformLocation ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `WebGlUniformLocation` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLUniformLocation)
    ///
    ///*This API requires the following crate features to be activated: `WebGlUniformLocation`*
    pub type WebGlUniformLocation;

}
