use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = WebGLTexture , typescript_type = "WebGLTexture" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `WebGlTexture` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLTexture)
    ///
    ///*This API requires the following crate features to be activated: `WebGlTexture`*
    pub type WebGlTexture;

}
