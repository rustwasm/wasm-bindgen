use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( is_type_of = | _ | false , extends = SvgPathSeg , extends = :: js_sys :: Object , js_name = SVGPathSegArcRel , typescript_name = SVGPathSegArcRel ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SvgPathSegArcRel` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegArcRel)\n\n*This API requires the following crate features to be activated: `SvgPathSegArcRel`*"]
    pub type SvgPathSegArcRel;
    # [ wasm_bindgen ( structural , method , getter , js_name = x ) ]
    #[doc = "Getter for the `x` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegArcRel/x)\n\n*This API requires the following crate features to be activated: `SvgPathSegArcRel`*"]
    pub fn x(this: &SvgPathSegArcRel) -> f32;
    # [ wasm_bindgen ( structural , method , setter , js_name = x ) ]
    #[doc = "Setter for the `x` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegArcRel/x)\n\n*This API requires the following crate features to be activated: `SvgPathSegArcRel`*"]
    pub fn set_x(this: &SvgPathSegArcRel, value: f32);
    # [ wasm_bindgen ( structural , method , getter , js_name = y ) ]
    #[doc = "Getter for the `y` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegArcRel/y)\n\n*This API requires the following crate features to be activated: `SvgPathSegArcRel`*"]
    pub fn y(this: &SvgPathSegArcRel) -> f32;
    # [ wasm_bindgen ( structural , method , setter , js_name = y ) ]
    #[doc = "Setter for the `y` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegArcRel/y)\n\n*This API requires the following crate features to be activated: `SvgPathSegArcRel`*"]
    pub fn set_y(this: &SvgPathSegArcRel, value: f32);
    # [ wasm_bindgen ( structural , method , getter , js_name = r1 ) ]
    #[doc = "Getter for the `r1` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegArcRel/r1)\n\n*This API requires the following crate features to be activated: `SvgPathSegArcRel`*"]
    pub fn r1(this: &SvgPathSegArcRel) -> f32;
    # [ wasm_bindgen ( structural , method , setter , js_name = r1 ) ]
    #[doc = "Setter for the `r1` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegArcRel/r1)\n\n*This API requires the following crate features to be activated: `SvgPathSegArcRel`*"]
    pub fn set_r1(this: &SvgPathSegArcRel, value: f32);
    # [ wasm_bindgen ( structural , method , getter , js_name = r2 ) ]
    #[doc = "Getter for the `r2` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegArcRel/r2)\n\n*This API requires the following crate features to be activated: `SvgPathSegArcRel`*"]
    pub fn r2(this: &SvgPathSegArcRel) -> f32;
    # [ wasm_bindgen ( structural , method , setter , js_name = r2 ) ]
    #[doc = "Setter for the `r2` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegArcRel/r2)\n\n*This API requires the following crate features to be activated: `SvgPathSegArcRel`*"]
    pub fn set_r2(this: &SvgPathSegArcRel, value: f32);
    # [ wasm_bindgen ( structural , method , getter , js_name = angle ) ]
    #[doc = "Getter for the `angle` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegArcRel/angle)\n\n*This API requires the following crate features to be activated: `SvgPathSegArcRel`*"]
    pub fn angle(this: &SvgPathSegArcRel) -> f32;
    # [ wasm_bindgen ( structural , method , setter , js_name = angle ) ]
    #[doc = "Setter for the `angle` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegArcRel/angle)\n\n*This API requires the following crate features to be activated: `SvgPathSegArcRel`*"]
    pub fn set_angle(this: &SvgPathSegArcRel, value: f32);
    # [ wasm_bindgen ( structural , method , getter , js_name = largeArcFlag ) ]
    #[doc = "Getter for the `largeArcFlag` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegArcRel/largeArcFlag)\n\n*This API requires the following crate features to be activated: `SvgPathSegArcRel`*"]
    pub fn large_arc_flag(this: &SvgPathSegArcRel) -> bool;
    # [ wasm_bindgen ( structural , method , setter , js_name = largeArcFlag ) ]
    #[doc = "Setter for the `largeArcFlag` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegArcRel/largeArcFlag)\n\n*This API requires the following crate features to be activated: `SvgPathSegArcRel`*"]
    pub fn set_large_arc_flag(this: &SvgPathSegArcRel, value: bool);
    # [ wasm_bindgen ( structural , method , getter , js_name = sweepFlag ) ]
    #[doc = "Getter for the `sweepFlag` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegArcRel/sweepFlag)\n\n*This API requires the following crate features to be activated: `SvgPathSegArcRel`*"]
    pub fn sweep_flag(this: &SvgPathSegArcRel) -> bool;
    # [ wasm_bindgen ( structural , method , setter , js_name = sweepFlag ) ]
    #[doc = "Setter for the `sweepFlag` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegArcRel/sweepFlag)\n\n*This API requires the following crate features to be activated: `SvgPathSegArcRel`*"]
    pub fn set_sweep_flag(this: &SvgPathSegArcRel, value: bool);
}
