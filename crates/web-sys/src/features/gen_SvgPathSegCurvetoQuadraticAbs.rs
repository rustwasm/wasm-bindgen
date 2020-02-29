use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( is_type_of = | _ | false , extends = SvgPathSeg , extends = :: js_sys :: Object , js_name = SVGPathSegCurvetoQuadraticAbs , typescript_type = "SVGPathSegCurvetoQuadraticAbs" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `SvgPathSegCurvetoQuadraticAbs` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegCurvetoQuadraticAbs)
    ///
    ///*This API requires the following crate features to be activated: `SvgPathSegCurvetoQuadraticAbs`*
    pub type SvgPathSegCurvetoQuadraticAbs;

    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGPathSegCurvetoQuadraticAbs" , js_name = x ) ]
    ///Getter for the `x` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegCurvetoQuadraticAbs/x)
    ///
    ///*This API requires the following crate features to be activated: `SvgPathSegCurvetoQuadraticAbs`*
    pub fn x(this: &SvgPathSegCurvetoQuadraticAbs) -> f32;

    # [ wasm_bindgen ( structural , method , setter , js_class = "SVGPathSegCurvetoQuadraticAbs" , js_name = x ) ]
    ///Setter for the `x` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegCurvetoQuadraticAbs/x)
    ///
    ///*This API requires the following crate features to be activated: `SvgPathSegCurvetoQuadraticAbs`*
    pub fn set_x(this: &SvgPathSegCurvetoQuadraticAbs, value: f32);

    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGPathSegCurvetoQuadraticAbs" , js_name = y ) ]
    ///Getter for the `y` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegCurvetoQuadraticAbs/y)
    ///
    ///*This API requires the following crate features to be activated: `SvgPathSegCurvetoQuadraticAbs`*
    pub fn y(this: &SvgPathSegCurvetoQuadraticAbs) -> f32;

    # [ wasm_bindgen ( structural , method , setter , js_class = "SVGPathSegCurvetoQuadraticAbs" , js_name = y ) ]
    ///Setter for the `y` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegCurvetoQuadraticAbs/y)
    ///
    ///*This API requires the following crate features to be activated: `SvgPathSegCurvetoQuadraticAbs`*
    pub fn set_y(this: &SvgPathSegCurvetoQuadraticAbs, value: f32);

    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGPathSegCurvetoQuadraticAbs" , js_name = x1 ) ]
    ///Getter for the `x1` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegCurvetoQuadraticAbs/x1)
    ///
    ///*This API requires the following crate features to be activated: `SvgPathSegCurvetoQuadraticAbs`*
    pub fn x1(this: &SvgPathSegCurvetoQuadraticAbs) -> f32;

    # [ wasm_bindgen ( structural , method , setter , js_class = "SVGPathSegCurvetoQuadraticAbs" , js_name = x1 ) ]
    ///Setter for the `x1` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegCurvetoQuadraticAbs/x1)
    ///
    ///*This API requires the following crate features to be activated: `SvgPathSegCurvetoQuadraticAbs`*
    pub fn set_x1(this: &SvgPathSegCurvetoQuadraticAbs, value: f32);

    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGPathSegCurvetoQuadraticAbs" , js_name = y1 ) ]
    ///Getter for the `y1` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegCurvetoQuadraticAbs/y1)
    ///
    ///*This API requires the following crate features to be activated: `SvgPathSegCurvetoQuadraticAbs`*
    pub fn y1(this: &SvgPathSegCurvetoQuadraticAbs) -> f32;

    # [ wasm_bindgen ( structural , method , setter , js_class = "SVGPathSegCurvetoQuadraticAbs" , js_name = y1 ) ]
    ///Setter for the `y1` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegCurvetoQuadraticAbs/y1)
    ///
    ///*This API requires the following crate features to be activated: `SvgPathSegCurvetoQuadraticAbs`*
    pub fn set_y1(this: &SvgPathSegCurvetoQuadraticAbs, value: f32);

}
