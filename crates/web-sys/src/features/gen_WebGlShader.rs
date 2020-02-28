use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = WebGLShader , typescript_name = WebGLShader ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `WebGlShader` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLShader)\n\n*This API requires the following crate features to be activated: `WebGlShader`*"]
    pub type WebGlShader;
}
