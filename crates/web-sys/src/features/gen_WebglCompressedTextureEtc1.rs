use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( is_type_of = | _ | false , extends = :: js_sys :: Object , js_name = WEBGL_compressed_texture_etc1 , typescript_name = WEBGL_compressed_texture_etc1 ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `WebglCompressedTextureEtc1` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WEBGL_compressed_texture_etc1)
    ///
    ///*This API requires the following crate features to be activated: `WebglCompressedTextureEtc1`*
    pub type WebglCompressedTextureEtc1;

}

impl WebglCompressedTextureEtc1 {
    ///The `WEBGL_compressed_texture_etc1.COMPRESSED_RGB_ETC1_WEBGL` const.
    ///
    ///*This API requires the following crate features to be activated: `WebglCompressedTextureEtc1`*

    pub const COMPRESSED_RGB_ETC1_WEBGL: u32 = 36196u64 as u32;
}
