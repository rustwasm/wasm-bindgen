use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( is_type_of = | _ | false , extends = :: js_sys :: Object , js_name = WEBGL_compressed_texture_astc , typescript_name = WEBGL_compressed_texture_astc ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `WebglCompressedTextureAstc` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WEBGL_compressed_texture_astc)\n\n*This API requires the following crate features to be activated: `WebglCompressedTextureAstc`*"]
    pub type WebglCompressedTextureAstc;
    # [ wasm_bindgen ( method , structural , js_class = "WEBGL_compressed_texture_astc" , js_name = getSupportedProfiles ) ]
    #[doc = "The `getSupportedProfiles()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WEBGL_compressed_texture_astc/getSupportedProfiles)\n\n*This API requires the following crate features to be activated: `WebglCompressedTextureAstc`*"]
    pub fn get_supported_profiles(this: &WebglCompressedTextureAstc) -> Option<::js_sys::Array>;
}
impl WebglCompressedTextureAstc {
    pub const COMPRESSED_RGBA_ASTC_4X4_KHR: u32 = 37808u64 as u32;
    pub const COMPRESSED_RGBA_ASTC_5X4_KHR: u32 = 37809u64 as u32;
    pub const COMPRESSED_RGBA_ASTC_5X5_KHR: u32 = 37810u64 as u32;
    pub const COMPRESSED_RGBA_ASTC_6X5_KHR: u32 = 37811u64 as u32;
    pub const COMPRESSED_RGBA_ASTC_6X6_KHR: u32 = 37812u64 as u32;
    pub const COMPRESSED_RGBA_ASTC_8X5_KHR: u32 = 37813u64 as u32;
    pub const COMPRESSED_RGBA_ASTC_8X6_KHR: u32 = 37814u64 as u32;
    pub const COMPRESSED_RGBA_ASTC_8X8_KHR: u32 = 37815u64 as u32;
    pub const COMPRESSED_RGBA_ASTC_10X5_KHR: u32 = 37816u64 as u32;
    pub const COMPRESSED_RGBA_ASTC_10X6_KHR: u32 = 37817u64 as u32;
    pub const COMPRESSED_RGBA_ASTC_10X8_KHR: u32 = 37818u64 as u32;
    pub const COMPRESSED_RGBA_ASTC_10X10_KHR: u32 = 37819u64 as u32;
    pub const COMPRESSED_RGBA_ASTC_12X10_KHR: u32 = 37820u64 as u32;
    pub const COMPRESSED_RGBA_ASTC_12X12_KHR: u32 = 37821u64 as u32;
    pub const COMPRESSED_SRGB8_ALPHA8_ASTC_4X4_KHR: u32 = 37840u64 as u32;
    pub const COMPRESSED_SRGB8_ALPHA8_ASTC_5X4_KHR: u32 = 37841u64 as u32;
    pub const COMPRESSED_SRGB8_ALPHA8_ASTC_5X5_KHR: u32 = 37842u64 as u32;
    pub const COMPRESSED_SRGB8_ALPHA8_ASTC_6X5_KHR: u32 = 37843u64 as u32;
    pub const COMPRESSED_SRGB8_ALPHA8_ASTC_6X6_KHR: u32 = 37844u64 as u32;
    pub const COMPRESSED_SRGB8_ALPHA8_ASTC_8X5_KHR: u32 = 37845u64 as u32;
    pub const COMPRESSED_SRGB8_ALPHA8_ASTC_8X6_KHR: u32 = 37846u64 as u32;
    pub const COMPRESSED_SRGB8_ALPHA8_ASTC_8X8_KHR: u32 = 37847u64 as u32;
    pub const COMPRESSED_SRGB8_ALPHA8_ASTC_10X5_KHR: u32 = 37848u64 as u32;
    pub const COMPRESSED_SRGB8_ALPHA8_ASTC_10X6_KHR: u32 = 37849u64 as u32;
    pub const COMPRESSED_SRGB8_ALPHA8_ASTC_10X8_KHR: u32 = 37850u64 as u32;
    pub const COMPRESSED_SRGB8_ALPHA8_ASTC_10X10_KHR: u32 = 37851u64 as u32;
    pub const COMPRESSED_SRGB8_ALPHA8_ASTC_12X10_KHR: u32 = 37852u64 as u32;
    pub const COMPRESSED_SRGB8_ALPHA8_ASTC_12X12_KHR: u32 = 37853u64 as u32;
}
