use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = WebGLBuffer , typescript_type = "WebGLBuffer" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `WebGlBuffer` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLBuffer)
    ///
    ///*This API requires the following crate features to be activated: `WebGlBuffer`*
    pub type WebGlBuffer;

}
