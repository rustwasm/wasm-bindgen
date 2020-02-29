use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( is_type_of = | _ | false , extends = :: js_sys :: Object , js_name = OES_texture_half_float , typescript_type = "OES_texture_half_float" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `OesTextureHalfFloat` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OES_texture_half_float)
    ///
    ///*This API requires the following crate features to be activated: `OesTextureHalfFloat`*
    pub type OesTextureHalfFloat;

}

impl OesTextureHalfFloat {
    ///The `OES_texture_half_float.HALF_FLOAT_OES` const.
    ///
    ///*This API requires the following crate features to be activated: `OesTextureHalfFloat`*

    pub const HALF_FLOAT_OES: u32 = 36193u64 as u32;
}
