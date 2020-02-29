use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( is_type_of = | _ | false , extends = SvgPathSeg , extends = :: js_sys :: Object , js_name = SVGPathSegArcAbs , typescript_type = "SVGPathSegArcAbs" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `SvgPathSegArcAbs` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegArcAbs)
    ///
    ///*This API requires the following crate features to be activated: `SvgPathSegArcAbs`*
    pub type SvgPathSegArcAbs;

    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGPathSegArcAbs" , js_name = x ) ]
    ///Getter for the `x` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegArcAbs/x)
    ///
    ///*This API requires the following crate features to be activated: `SvgPathSegArcAbs`*
    pub fn x(this: &SvgPathSegArcAbs) -> f32;

    # [ wasm_bindgen ( structural , method , setter , js_class = "SVGPathSegArcAbs" , js_name = x ) ]
    ///Setter for the `x` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegArcAbs/x)
    ///
    ///*This API requires the following crate features to be activated: `SvgPathSegArcAbs`*
    pub fn set_x(this: &SvgPathSegArcAbs, value: f32);

    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGPathSegArcAbs" , js_name = y ) ]
    ///Getter for the `y` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegArcAbs/y)
    ///
    ///*This API requires the following crate features to be activated: `SvgPathSegArcAbs`*
    pub fn y(this: &SvgPathSegArcAbs) -> f32;

    # [ wasm_bindgen ( structural , method , setter , js_class = "SVGPathSegArcAbs" , js_name = y ) ]
    ///Setter for the `y` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegArcAbs/y)
    ///
    ///*This API requires the following crate features to be activated: `SvgPathSegArcAbs`*
    pub fn set_y(this: &SvgPathSegArcAbs, value: f32);

    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGPathSegArcAbs" , js_name = r1 ) ]
    ///Getter for the `r1` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegArcAbs/r1)
    ///
    ///*This API requires the following crate features to be activated: `SvgPathSegArcAbs`*
    pub fn r1(this: &SvgPathSegArcAbs) -> f32;

    # [ wasm_bindgen ( structural , method , setter , js_class = "SVGPathSegArcAbs" , js_name = r1 ) ]
    ///Setter for the `r1` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegArcAbs/r1)
    ///
    ///*This API requires the following crate features to be activated: `SvgPathSegArcAbs`*
    pub fn set_r1(this: &SvgPathSegArcAbs, value: f32);

    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGPathSegArcAbs" , js_name = r2 ) ]
    ///Getter for the `r2` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegArcAbs/r2)
    ///
    ///*This API requires the following crate features to be activated: `SvgPathSegArcAbs`*
    pub fn r2(this: &SvgPathSegArcAbs) -> f32;

    # [ wasm_bindgen ( structural , method , setter , js_class = "SVGPathSegArcAbs" , js_name = r2 ) ]
    ///Setter for the `r2` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegArcAbs/r2)
    ///
    ///*This API requires the following crate features to be activated: `SvgPathSegArcAbs`*
    pub fn set_r2(this: &SvgPathSegArcAbs, value: f32);

    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGPathSegArcAbs" , js_name = angle ) ]
    ///Getter for the `angle` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegArcAbs/angle)
    ///
    ///*This API requires the following crate features to be activated: `SvgPathSegArcAbs`*
    pub fn angle(this: &SvgPathSegArcAbs) -> f32;

    # [ wasm_bindgen ( structural , method , setter , js_class = "SVGPathSegArcAbs" , js_name = angle ) ]
    ///Setter for the `angle` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegArcAbs/angle)
    ///
    ///*This API requires the following crate features to be activated: `SvgPathSegArcAbs`*
    pub fn set_angle(this: &SvgPathSegArcAbs, value: f32);

    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGPathSegArcAbs" , js_name = largeArcFlag ) ]
    ///Getter for the `largeArcFlag` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegArcAbs/largeArcFlag)
    ///
    ///*This API requires the following crate features to be activated: `SvgPathSegArcAbs`*
    pub fn large_arc_flag(this: &SvgPathSegArcAbs) -> bool;

    # [ wasm_bindgen ( structural , method , setter , js_class = "SVGPathSegArcAbs" , js_name = largeArcFlag ) ]
    ///Setter for the `largeArcFlag` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegArcAbs/largeArcFlag)
    ///
    ///*This API requires the following crate features to be activated: `SvgPathSegArcAbs`*
    pub fn set_large_arc_flag(this: &SvgPathSegArcAbs, value: bool);

    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGPathSegArcAbs" , js_name = sweepFlag ) ]
    ///Getter for the `sweepFlag` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegArcAbs/sweepFlag)
    ///
    ///*This API requires the following crate features to be activated: `SvgPathSegArcAbs`*
    pub fn sweep_flag(this: &SvgPathSegArcAbs) -> bool;

    # [ wasm_bindgen ( structural , method , setter , js_class = "SVGPathSegArcAbs" , js_name = sweepFlag ) ]
    ///Setter for the `sweepFlag` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegArcAbs/sweepFlag)
    ///
    ///*This API requires the following crate features to be activated: `SvgPathSegArcAbs`*
    pub fn set_sweep_flag(this: &SvgPathSegArcAbs, value: bool);

}
