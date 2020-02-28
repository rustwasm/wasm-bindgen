use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( is_type_of = | _ | false , extends = SvgPathSeg , extends = :: js_sys :: Object , js_name = SVGPathSegCurvetoQuadraticSmoothAbs , typescript_name = SVGPathSegCurvetoQuadraticSmoothAbs ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SvgPathSegCurvetoQuadraticSmoothAbs` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegCurvetoQuadraticSmoothAbs)\n\n*This API requires the following crate features to be activated: `SvgPathSegCurvetoQuadraticSmoothAbs`*"]
    pub type SvgPathSegCurvetoQuadraticSmoothAbs;
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGPathSegCurvetoQuadraticSmoothAbs" , js_name = x ) ]
    #[doc = "Getter for the `x` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegCurvetoQuadraticSmoothAbs/x)\n\n*This API requires the following crate features to be activated: `SvgPathSegCurvetoQuadraticSmoothAbs`*"]
    pub fn x(this: &SvgPathSegCurvetoQuadraticSmoothAbs) -> f32;
    # [ wasm_bindgen ( structural , method , setter , js_class = "SVGPathSegCurvetoQuadraticSmoothAbs" , js_name = x ) ]
    #[doc = "Setter for the `x` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegCurvetoQuadraticSmoothAbs/x)\n\n*This API requires the following crate features to be activated: `SvgPathSegCurvetoQuadraticSmoothAbs`*"]
    pub fn set_x(this: &SvgPathSegCurvetoQuadraticSmoothAbs, value: f32);
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGPathSegCurvetoQuadraticSmoothAbs" , js_name = y ) ]
    #[doc = "Getter for the `y` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegCurvetoQuadraticSmoothAbs/y)\n\n*This API requires the following crate features to be activated: `SvgPathSegCurvetoQuadraticSmoothAbs`*"]
    pub fn y(this: &SvgPathSegCurvetoQuadraticSmoothAbs) -> f32;
    # [ wasm_bindgen ( structural , method , setter , js_class = "SVGPathSegCurvetoQuadraticSmoothAbs" , js_name = y ) ]
    #[doc = "Setter for the `y` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegCurvetoQuadraticSmoothAbs/y)\n\n*This API requires the following crate features to be activated: `SvgPathSegCurvetoQuadraticSmoothAbs`*"]
    pub fn set_y(this: &SvgPathSegCurvetoQuadraticSmoothAbs, value: f32);
}
