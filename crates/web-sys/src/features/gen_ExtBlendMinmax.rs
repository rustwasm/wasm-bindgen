use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( is_type_of = | _ | false , extends = :: js_sys :: Object , js_name = EXT_blend_minmax , typescript_type = "EXT_blend_minmax" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `ExtBlendMinmax` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/EXT_blend_minmax)
    ///
    ///*This API requires the following crate features to be activated: `ExtBlendMinmax`*
    pub type ExtBlendMinmax;

}

impl ExtBlendMinmax {
    ///The `EXT_blend_minmax.MIN_EXT` const.
    ///
    ///*This API requires the following crate features to be activated: `ExtBlendMinmax`*

    pub const MIN_EXT: u32 = 32775u64 as u32;

    ///The `EXT_blend_minmax.MAX_EXT` const.
    ///
    ///*This API requires the following crate features to be activated: `ExtBlendMinmax`*

    pub const MAX_EXT: u32 = 32776u64 as u32;
}
