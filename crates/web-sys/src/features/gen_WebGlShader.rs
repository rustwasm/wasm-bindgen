use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = WebGLShader , typescript_type = "WebGLShader" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `WebGlShader` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLShader)
    ///
    ///*This API requires the following crate features to be activated: `WebGlShader`*
    pub type WebGlShader;

}
