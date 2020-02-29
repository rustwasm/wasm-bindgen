use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( is_type_of = | _ | false , extends = :: js_sys :: Object , js_name = EXT_color_buffer_half_float , typescript_name = EXT_color_buffer_half_float ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `ExtColorBufferHalfFloat` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/EXT_color_buffer_half_float)
    ///
    ///*This API requires the following crate features to be activated: `ExtColorBufferHalfFloat`*
    pub type ExtColorBufferHalfFloat;

}

impl ExtColorBufferHalfFloat {
    ///The `EXT_color_buffer_half_float.RGBA16F_EXT` const.
    ///
    ///*This API requires the following crate features to be activated: `ExtColorBufferHalfFloat`*

    pub const RGBA16F_EXT: u32 = 34842u64 as u32;

    ///The `EXT_color_buffer_half_float.RGB16F_EXT` const.
    ///
    ///*This API requires the following crate features to be activated: `ExtColorBufferHalfFloat`*

    pub const RGB16F_EXT: u32 = 34843u64 as u32;

    ///The `EXT_color_buffer_half_float.FRAMEBUFFER_ATTACHMENT_COMPONENT_TYPE_EXT` const.
    ///
    ///*This API requires the following crate features to be activated: `ExtColorBufferHalfFloat`*

    pub const FRAMEBUFFER_ATTACHMENT_COMPONENT_TYPE_EXT: u32 = 33297u64 as u32;

    ///The `EXT_color_buffer_half_float.UNSIGNED_NORMALIZED_EXT` const.
    ///
    ///*This API requires the following crate features to be activated: `ExtColorBufferHalfFloat`*

    pub const UNSIGNED_NORMALIZED_EXT: u32 = 35863u64 as u32;
}
