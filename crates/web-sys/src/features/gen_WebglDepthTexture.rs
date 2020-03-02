use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( is_type_of = | _ | false , extends = :: js_sys :: Object , js_name = WEBGL_depth_texture , typescript_type = "WEBGL_depth_texture" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `WebglDepthTexture` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WEBGL_depth_texture)
    ///
    ///*This API requires the following crate features to be activated: `WebglDepthTexture`*
    pub type WebglDepthTexture;

}

impl WebglDepthTexture {
    ///The `WEBGL_depth_texture.UNSIGNED_INT_24_8_WEBGL` const.
    ///
    ///*This API requires the following crate features to be activated: `WebglDepthTexture`*

    pub const UNSIGNED_INT_24_8_WEBGL: u32 = 34042u64 as u32;
}
