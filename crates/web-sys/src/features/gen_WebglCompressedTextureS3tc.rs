use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( is_type_of = | _ | false , extends = :: js_sys :: Object , js_name = WEBGL_compressed_texture_s3tc , typescript_name = WEBGL_compressed_texture_s3tc ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `WebglCompressedTextureS3tc` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WEBGL_compressed_texture_s3tc)\n\n*This API requires the following crate features to be activated: `WebglCompressedTextureS3tc`*"]
    pub type WebglCompressedTextureS3tc;
}
impl WebglCompressedTextureS3tc {
    pub const COMPRESSED_RGB_S3TC_DXT1_EXT: u32 = 33776u64 as u32;
    pub const COMPRESSED_RGBA_S3TC_DXT1_EXT: u32 = 33777u64 as u32;
    pub const COMPRESSED_RGBA_S3TC_DXT3_EXT: u32 = 33778u64 as u32;
    pub const COMPRESSED_RGBA_S3TC_DXT5_EXT: u32 = 33779u64 as u32;
}
