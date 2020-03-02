use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( is_type_of = | _ | false , extends = :: js_sys :: Object , js_name = WEBGL_compressed_texture_pvrtc , typescript_type = "WEBGL_compressed_texture_pvrtc" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `WebglCompressedTexturePvrtc` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WEBGL_compressed_texture_pvrtc)
    ///
    ///*This API requires the following crate features to be activated: `WebglCompressedTexturePvrtc`*
    pub type WebglCompressedTexturePvrtc;

}

impl WebglCompressedTexturePvrtc {
    ///The `WEBGL_compressed_texture_pvrtc.COMPRESSED_RGB_PVRTC_4BPPV1_IMG` const.
    ///
    ///*This API requires the following crate features to be activated: `WebglCompressedTexturePvrtc`*

    pub const COMPRESSED_RGB_PVRTC_4BPPV1_IMG: u32 = 35840u64 as u32;

    ///The `WEBGL_compressed_texture_pvrtc.COMPRESSED_RGB_PVRTC_2BPPV1_IMG` const.
    ///
    ///*This API requires the following crate features to be activated: `WebglCompressedTexturePvrtc`*

    pub const COMPRESSED_RGB_PVRTC_2BPPV1_IMG: u32 = 35841u64 as u32;

    ///The `WEBGL_compressed_texture_pvrtc.COMPRESSED_RGBA_PVRTC_4BPPV1_IMG` const.
    ///
    ///*This API requires the following crate features to be activated: `WebglCompressedTexturePvrtc`*

    pub const COMPRESSED_RGBA_PVRTC_4BPPV1_IMG: u32 = 35842u64 as u32;

    ///The `WEBGL_compressed_texture_pvrtc.COMPRESSED_RGBA_PVRTC_2BPPV1_IMG` const.
    ///
    ///*This API requires the following crate features to be activated: `WebglCompressedTexturePvrtc`*

    pub const COMPRESSED_RGBA_PVRTC_2BPPV1_IMG: u32 = 35843u64 as u32;
}
