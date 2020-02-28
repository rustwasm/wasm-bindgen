use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( is_type_of = | _ | false , extends = SvgPathSeg , extends = :: js_sys :: Object , js_name = SVGPathSegLinetoVerticalAbs , typescript_name = SVGPathSegLinetoVerticalAbs ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SvgPathSegLinetoVerticalAbs` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegLinetoVerticalAbs)\n\n*This API requires the following crate features to be activated: `SvgPathSegLinetoVerticalAbs`*"]
    pub type SvgPathSegLinetoVerticalAbs;
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGPathSegLinetoVerticalAbs" , js_name = y ) ]
    #[doc = "Getter for the `y` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegLinetoVerticalAbs/y)\n\n*This API requires the following crate features to be activated: `SvgPathSegLinetoVerticalAbs`*"]
    pub fn y(this: &SvgPathSegLinetoVerticalAbs) -> f32;
    # [ wasm_bindgen ( structural , method , setter , js_class = "SVGPathSegLinetoVerticalAbs" , js_name = y ) ]
    #[doc = "Setter for the `y` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegLinetoVerticalAbs/y)\n\n*This API requires the following crate features to be activated: `SvgPathSegLinetoVerticalAbs`*"]
    pub fn set_y(this: &SvgPathSegLinetoVerticalAbs, value: f32);
}
