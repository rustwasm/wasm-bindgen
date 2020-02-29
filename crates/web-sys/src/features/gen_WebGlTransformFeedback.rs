use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = WebGLTransformFeedback , typescript_name = WebGLTransformFeedback ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `WebGlTransformFeedback` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLTransformFeedback)
    ///
    ///*This API requires the following crate features to be activated: `WebGlTransformFeedback`*
    pub type WebGlTransformFeedback;

}
