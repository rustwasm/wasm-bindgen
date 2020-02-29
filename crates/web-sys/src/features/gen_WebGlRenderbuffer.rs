use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = WebGLRenderbuffer , typescript_type = "WebGLRenderbuffer" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `WebGlRenderbuffer` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderbuffer)
    ///
    ///*This API requires the following crate features to be activated: `WebGlRenderbuffer`*
    pub type WebGlRenderbuffer;

}
