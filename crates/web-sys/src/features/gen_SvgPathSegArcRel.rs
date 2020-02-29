use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( is_type_of = | _ | false , extends = SvgPathSeg , extends = :: js_sys :: Object , js_name = SVGPathSegArcRel , typescript_type = "SVGPathSegArcRel" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `SvgPathSegArcRel` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegArcRel)
    ///
    ///*This API requires the following crate features to be activated: `SvgPathSegArcRel`*
    pub type SvgPathSegArcRel;

    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGPathSegArcRel" , js_name = x ) ]
    ///Getter for the `x` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegArcRel/x)
    ///
    ///*This API requires the following crate features to be activated: `SvgPathSegArcRel`*
    pub fn x(this: &SvgPathSegArcRel) -> f32;

    # [ wasm_bindgen ( structural , method , setter , js_class = "SVGPathSegArcRel" , js_name = x ) ]
    ///Setter for the `x` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegArcRel/x)
    ///
    ///*This API requires the following crate features to be activated: `SvgPathSegArcRel`*
    pub fn set_x(this: &SvgPathSegArcRel, value: f32);

    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGPathSegArcRel" , js_name = y ) ]
    ///Getter for the `y` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegArcRel/y)
    ///
    ///*This API requires the following crate features to be activated: `SvgPathSegArcRel`*
    pub fn y(this: &SvgPathSegArcRel) -> f32;

    # [ wasm_bindgen ( structural , method , setter , js_class = "SVGPathSegArcRel" , js_name = y ) ]
    ///Setter for the `y` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegArcRel/y)
    ///
    ///*This API requires the following crate features to be activated: `SvgPathSegArcRel`*
    pub fn set_y(this: &SvgPathSegArcRel, value: f32);

    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGPathSegArcRel" , js_name = r1 ) ]
    ///Getter for the `r1` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegArcRel/r1)
    ///
    ///*This API requires the following crate features to be activated: `SvgPathSegArcRel`*
    pub fn r1(this: &SvgPathSegArcRel) -> f32;

    # [ wasm_bindgen ( structural , method , setter , js_class = "SVGPathSegArcRel" , js_name = r1 ) ]
    ///Setter for the `r1` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegArcRel/r1)
    ///
    ///*This API requires the following crate features to be activated: `SvgPathSegArcRel`*
    pub fn set_r1(this: &SvgPathSegArcRel, value: f32);

    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGPathSegArcRel" , js_name = r2 ) ]
    ///Getter for the `r2` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegArcRel/r2)
    ///
    ///*This API requires the following crate features to be activated: `SvgPathSegArcRel`*
    pub fn r2(this: &SvgPathSegArcRel) -> f32;

    # [ wasm_bindgen ( structural , method , setter , js_class = "SVGPathSegArcRel" , js_name = r2 ) ]
    ///Setter for the `r2` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegArcRel/r2)
    ///
    ///*This API requires the following crate features to be activated: `SvgPathSegArcRel`*
    pub fn set_r2(this: &SvgPathSegArcRel, value: f32);

    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGPathSegArcRel" , js_name = angle ) ]
    ///Getter for the `angle` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegArcRel/angle)
    ///
    ///*This API requires the following crate features to be activated: `SvgPathSegArcRel`*
    pub fn angle(this: &SvgPathSegArcRel) -> f32;

    # [ wasm_bindgen ( structural , method , setter , js_class = "SVGPathSegArcRel" , js_name = angle ) ]
    ///Setter for the `angle` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegArcRel/angle)
    ///
    ///*This API requires the following crate features to be activated: `SvgPathSegArcRel`*
    pub fn set_angle(this: &SvgPathSegArcRel, value: f32);

    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGPathSegArcRel" , js_name = largeArcFlag ) ]
    ///Getter for the `largeArcFlag` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegArcRel/largeArcFlag)
    ///
    ///*This API requires the following crate features to be activated: `SvgPathSegArcRel`*
    pub fn large_arc_flag(this: &SvgPathSegArcRel) -> bool;

    # [ wasm_bindgen ( structural , method , setter , js_class = "SVGPathSegArcRel" , js_name = largeArcFlag ) ]
    ///Setter for the `largeArcFlag` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegArcRel/largeArcFlag)
    ///
    ///*This API requires the following crate features to be activated: `SvgPathSegArcRel`*
    pub fn set_large_arc_flag(this: &SvgPathSegArcRel, value: bool);

    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGPathSegArcRel" , js_name = sweepFlag ) ]
    ///Getter for the `sweepFlag` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegArcRel/sweepFlag)
    ///
    ///*This API requires the following crate features to be activated: `SvgPathSegArcRel`*
    pub fn sweep_flag(this: &SvgPathSegArcRel) -> bool;

    # [ wasm_bindgen ( structural , method , setter , js_class = "SVGPathSegArcRel" , js_name = sweepFlag ) ]
    ///Setter for the `sweepFlag` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegArcRel/sweepFlag)
    ///
    ///*This API requires the following crate features to be activated: `SvgPathSegArcRel`*
    pub fn set_sweep_flag(this: &SvgPathSegArcRel, value: bool);

}
