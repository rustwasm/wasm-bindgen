use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( is_type_of = | _ | false , extends = SvgPathSeg , extends = :: js_sys :: Object , js_name = SVGPathSegLinetoRel , typescript_name = SVGPathSegLinetoRel ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SvgPathSegLinetoRel` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegLinetoRel)\n\n*This API requires the following crate features to be activated: `SvgPathSegLinetoRel`*"]
    pub type SvgPathSegLinetoRel;
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGPathSegLinetoRel" , js_name = x ) ]
    #[doc = "Getter for the `x` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegLinetoRel/x)\n\n*This API requires the following crate features to be activated: `SvgPathSegLinetoRel`*"]
    pub fn x(this: &SvgPathSegLinetoRel) -> f32;
    # [ wasm_bindgen ( structural , method , setter , js_class = "SVGPathSegLinetoRel" , js_name = x ) ]
    #[doc = "Setter for the `x` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegLinetoRel/x)\n\n*This API requires the following crate features to be activated: `SvgPathSegLinetoRel`*"]
    pub fn set_x(this: &SvgPathSegLinetoRel, value: f32);
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGPathSegLinetoRel" , js_name = y ) ]
    #[doc = "Getter for the `y` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegLinetoRel/y)\n\n*This API requires the following crate features to be activated: `SvgPathSegLinetoRel`*"]
    pub fn y(this: &SvgPathSegLinetoRel) -> f32;
    # [ wasm_bindgen ( structural , method , setter , js_class = "SVGPathSegLinetoRel" , js_name = y ) ]
    #[doc = "Setter for the `y` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegLinetoRel/y)\n\n*This API requires the following crate features to be activated: `SvgPathSegLinetoRel`*"]
    pub fn set_y(this: &SvgPathSegLinetoRel, value: f32);
}
