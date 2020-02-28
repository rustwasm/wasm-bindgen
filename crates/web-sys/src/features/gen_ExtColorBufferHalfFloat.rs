use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( is_type_of = | _ | false , extends = :: js_sys :: Object , js_name = EXT_color_buffer_half_float , typescript_name = EXT_color_buffer_half_float ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ExtColorBufferHalfFloat` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/EXT_color_buffer_half_float)\n\n*This API requires the following crate features to be activated: `ExtColorBufferHalfFloat`*"]
    pub type ExtColorBufferHalfFloat;
}
impl ExtColorBufferHalfFloat {
    pub const RGBA16F_EXT: u32 = 34842u64 as u32;
    pub const RGB16F_EXT: u32 = 34843u64 as u32;
    pub const FRAMEBUFFER_ATTACHMENT_COMPONENT_TYPE_EXT: u32 = 33297u64 as u32;
    pub const UNSIGNED_NORMALIZED_EXT: u32 = 35863u64 as u32;
}
