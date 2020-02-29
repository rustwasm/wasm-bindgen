use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( is_type_of = | _ | false , extends = SvgPathSeg , extends = :: js_sys :: Object , js_name = SVGPathSegCurvetoQuadraticSmoothRel , typescript_type = "SVGPathSegCurvetoQuadraticSmoothRel" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `SvgPathSegCurvetoQuadraticSmoothRel` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegCurvetoQuadraticSmoothRel)
    ///
    ///*This API requires the following crate features to be activated: `SvgPathSegCurvetoQuadraticSmoothRel`*
    pub type SvgPathSegCurvetoQuadraticSmoothRel;

    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGPathSegCurvetoQuadraticSmoothRel" , js_name = x ) ]
    ///Getter for the `x` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegCurvetoQuadraticSmoothRel/x)
    ///
    ///*This API requires the following crate features to be activated: `SvgPathSegCurvetoQuadraticSmoothRel`*
    pub fn x(this: &SvgPathSegCurvetoQuadraticSmoothRel) -> f32;

    # [ wasm_bindgen ( structural , method , setter , js_class = "SVGPathSegCurvetoQuadraticSmoothRel" , js_name = x ) ]
    ///Setter for the `x` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegCurvetoQuadraticSmoothRel/x)
    ///
    ///*This API requires the following crate features to be activated: `SvgPathSegCurvetoQuadraticSmoothRel`*
    pub fn set_x(this: &SvgPathSegCurvetoQuadraticSmoothRel, value: f32);

    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGPathSegCurvetoQuadraticSmoothRel" , js_name = y ) ]
    ///Getter for the `y` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegCurvetoQuadraticSmoothRel/y)
    ///
    ///*This API requires the following crate features to be activated: `SvgPathSegCurvetoQuadraticSmoothRel`*
    pub fn y(this: &SvgPathSegCurvetoQuadraticSmoothRel) -> f32;

    # [ wasm_bindgen ( structural , method , setter , js_class = "SVGPathSegCurvetoQuadraticSmoothRel" , js_name = y ) ]
    ///Setter for the `y` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegCurvetoQuadraticSmoothRel/y)
    ///
    ///*This API requires the following crate features to be activated: `SvgPathSegCurvetoQuadraticSmoothRel`*
    pub fn set_y(this: &SvgPathSegCurvetoQuadraticSmoothRel, value: f32);

}
