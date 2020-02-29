use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( is_type_of = | _ | false , extends = :: js_sys :: Object , js_name = OES_texture_half_float_linear , typescript_type = "OES_texture_half_float_linear" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `OesTextureHalfFloatLinear` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OES_texture_half_float_linear)
    ///
    ///*This API requires the following crate features to be activated: `OesTextureHalfFloatLinear`*
    pub type OesTextureHalfFloatLinear;

}
