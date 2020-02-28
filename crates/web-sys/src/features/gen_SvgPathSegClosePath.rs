use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( is_type_of = | _ | false , extends = SvgPathSeg , extends = :: js_sys :: Object , js_name = SVGPathSegClosePath , typescript_name = SVGPathSegClosePath ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SvgPathSegClosePath` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegClosePath)\n\n*This API requires the following crate features to be activated: `SvgPathSegClosePath`*"]
    pub type SvgPathSegClosePath;
}
