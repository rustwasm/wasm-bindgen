use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( is_type_of = | _ | false , extends = :: js_sys :: Object , js_name = WEBGL_color_buffer_float , typescript_name = WEBGL_color_buffer_float ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `WebglColorBufferFloat` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WEBGL_color_buffer_float)\n\n*This API requires the following crate features to be activated: `WebglColorBufferFloat`*"]
    pub type WebglColorBufferFloat;
}
impl WebglColorBufferFloat {
    pub const RGBA32F_EXT: u32 = 34836u64 as u32;
    pub const RGB32F_EXT: u32 = 34837u64 as u32;
    pub const FRAMEBUFFER_ATTACHMENT_COMPONENT_TYPE_EXT: u32 = 33297u64 as u32;
    pub const UNSIGNED_NORMALIZED_EXT: u32 = 35863u64 as u32;
}
