use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( is_type_of = | _ | false , extends = SvgPathSeg , extends = :: js_sys :: Object , js_name = SVGPathSegLinetoVerticalRel , typescript_name = SVGPathSegLinetoVerticalRel ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SvgPathSegLinetoVerticalRel` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegLinetoVerticalRel)\n\n*This API requires the following crate features to be activated: `SvgPathSegLinetoVerticalRel`*"]
    pub type SvgPathSegLinetoVerticalRel;
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGPathSegLinetoVerticalRel" , js_name = y ) ]
    #[doc = "Getter for the `y` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegLinetoVerticalRel/y)\n\n*This API requires the following crate features to be activated: `SvgPathSegLinetoVerticalRel`*"]
    pub fn y(this: &SvgPathSegLinetoVerticalRel) -> f32;
    # [ wasm_bindgen ( structural , method , setter , js_class = "SVGPathSegLinetoVerticalRel" , js_name = y ) ]
    #[doc = "Setter for the `y` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegLinetoVerticalRel/y)\n\n*This API requires the following crate features to be activated: `SvgPathSegLinetoVerticalRel`*"]
    pub fn set_y(this: &SvgPathSegLinetoVerticalRel, value: f32);
}
