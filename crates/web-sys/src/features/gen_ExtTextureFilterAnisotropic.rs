use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( is_type_of = | _ | false , extends = :: js_sys :: Object , js_name = EXT_texture_filter_anisotropic , typescript_name = EXT_texture_filter_anisotropic ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `ExtTextureFilterAnisotropic` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/EXT_texture_filter_anisotropic)
    ///
    ///*This API requires the following crate features to be activated: `ExtTextureFilterAnisotropic`*
    pub type ExtTextureFilterAnisotropic;

}

impl ExtTextureFilterAnisotropic {
    ///The `EXT_texture_filter_anisotropic.TEXTURE_MAX_ANISOTROPY_EXT` const.
    ///
    ///*This API requires the following crate features to be activated: `ExtTextureFilterAnisotropic`*

    pub const TEXTURE_MAX_ANISOTROPY_EXT: u32 = 34046u64 as u32;

    ///The `EXT_texture_filter_anisotropic.MAX_TEXTURE_MAX_ANISOTROPY_EXT` const.
    ///
    ///*This API requires the following crate features to be activated: `ExtTextureFilterAnisotropic`*

    pub const MAX_TEXTURE_MAX_ANISOTROPY_EXT: u32 = 34047u64 as u32;
}
