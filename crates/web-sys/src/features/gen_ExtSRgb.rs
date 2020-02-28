use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( is_type_of = | _ | false , extends = :: js_sys :: Object , js_name = EXT_sRGB , typescript_name = EXT_sRGB ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ExtSRgb` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/EXT_sRGB)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ExtSRgb`*"]
    pub type ExtSRgb;
}
impl ExtSRgb {
    pub const SRGB_EXT: u32 = 35904u64 as u32;
    pub const SRGB_ALPHA_EXT: u32 = 35906u64 as u32;
    pub const SRGB8_ALPHA8_EXT: u32 = 35907u64 as u32;
    pub const FRAMEBUFFER_ATTACHMENT_COLOR_ENCODING_EXT: u32 = 33296u64 as u32;
}
