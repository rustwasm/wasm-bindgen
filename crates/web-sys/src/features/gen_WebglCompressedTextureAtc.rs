use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( is_type_of = | _ | false , extends = :: js_sys :: Object , js_name = WEBGL_compressed_texture_atc , typescript_type = "WEBGL_compressed_texture_atc" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `WebglCompressedTextureAtc` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WEBGL_compressed_texture_atc)
    ///
    ///*This API requires the following crate features to be activated: `WebglCompressedTextureAtc`*
    pub type WebglCompressedTextureAtc;

}

impl WebglCompressedTextureAtc {
    ///The `WEBGL_compressed_texture_atc.COMPRESSED_RGB_ATC_WEBGL` const.
    ///
    ///*This API requires the following crate features to be activated: `WebglCompressedTextureAtc`*

    pub const COMPRESSED_RGB_ATC_WEBGL: u32 = 35986u64 as u32;

    ///The `WEBGL_compressed_texture_atc.COMPRESSED_RGBA_ATC_EXPLICIT_ALPHA_WEBGL` const.
    ///
    ///*This API requires the following crate features to be activated: `WebglCompressedTextureAtc`*

    pub const COMPRESSED_RGBA_ATC_EXPLICIT_ALPHA_WEBGL: u32 = 35987u64 as u32;

    ///The `WEBGL_compressed_texture_atc.COMPRESSED_RGBA_ATC_INTERPOLATED_ALPHA_WEBGL` const.
    ///
    ///*This API requires the following crate features to be activated: `WebglCompressedTextureAtc`*

    pub const COMPRESSED_RGBA_ATC_INTERPOLATED_ALPHA_WEBGL: u32 = 34798u64 as u32;
}
