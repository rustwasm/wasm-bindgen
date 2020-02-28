use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = WebGLBuffer , typescript_name = WebGLBuffer ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `WebGlBuffer` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLBuffer)\n\n*This API requires the following crate features to be activated: `WebGlBuffer`*"]
    pub type WebGlBuffer;
}
