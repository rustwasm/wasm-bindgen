use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( is_type_of = | _ | false , extends = SvgPathSeg , extends = :: js_sys :: Object , js_name = SVGPathSegCurvetoQuadraticSmoothRel , typescript_name = SVGPathSegCurvetoQuadraticSmoothRel ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SvgPathSegCurvetoQuadraticSmoothRel` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegCurvetoQuadraticSmoothRel)\n\n*This API requires the following crate features to be activated: `SvgPathSegCurvetoQuadraticSmoothRel`*"]
    pub type SvgPathSegCurvetoQuadraticSmoothRel;
    # [ wasm_bindgen ( structural , method , getter , js_name = x ) ]
    #[doc = "Getter for the `x` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegCurvetoQuadraticSmoothRel/x)\n\n*This API requires the following crate features to be activated: `SvgPathSegCurvetoQuadraticSmoothRel`*"]
    pub fn x(this: &SvgPathSegCurvetoQuadraticSmoothRel) -> f32;
    # [ wasm_bindgen ( structural , method , setter , js_name = x ) ]
    #[doc = "Setter for the `x` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegCurvetoQuadraticSmoothRel/x)\n\n*This API requires the following crate features to be activated: `SvgPathSegCurvetoQuadraticSmoothRel`*"]
    pub fn set_x(this: &SvgPathSegCurvetoQuadraticSmoothRel, value: f32);
    # [ wasm_bindgen ( structural , method , getter , js_name = y ) ]
    #[doc = "Getter for the `y` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegCurvetoQuadraticSmoothRel/y)\n\n*This API requires the following crate features to be activated: `SvgPathSegCurvetoQuadraticSmoothRel`*"]
    pub fn y(this: &SvgPathSegCurvetoQuadraticSmoothRel) -> f32;
    # [ wasm_bindgen ( structural , method , setter , js_name = y ) ]
    #[doc = "Setter for the `y` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegCurvetoQuadraticSmoothRel/y)\n\n*This API requires the following crate features to be activated: `SvgPathSegCurvetoQuadraticSmoothRel`*"]
    pub fn set_y(this: &SvgPathSegCurvetoQuadraticSmoothRel, value: f32);
}
