use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( is_type_of = | _ | false , extends = SvgPathSeg , extends = :: js_sys :: Object , js_name = SVGPathSegLinetoHorizontalRel , typescript_name = SVGPathSegLinetoHorizontalRel ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SvgPathSegLinetoHorizontalRel` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegLinetoHorizontalRel)\n\n*This API requires the following crate features to be activated: `SvgPathSegLinetoHorizontalRel`*"]
    pub type SvgPathSegLinetoHorizontalRel;
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGPathSegLinetoHorizontalRel" , js_name = x ) ]
    #[doc = "Getter for the `x` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegLinetoHorizontalRel/x)\n\n*This API requires the following crate features to be activated: `SvgPathSegLinetoHorizontalRel`*"]
    pub fn x(this: &SvgPathSegLinetoHorizontalRel) -> f32;
    # [ wasm_bindgen ( structural , method , setter , js_class = "SVGPathSegLinetoHorizontalRel" , js_name = x ) ]
    #[doc = "Setter for the `x` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegLinetoHorizontalRel/x)\n\n*This API requires the following crate features to be activated: `SvgPathSegLinetoHorizontalRel`*"]
    pub fn set_x(this: &SvgPathSegLinetoHorizontalRel, value: f32);
}
