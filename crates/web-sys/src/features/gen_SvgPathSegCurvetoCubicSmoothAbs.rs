use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( is_type_of = | _ | false , extends = SvgPathSeg , extends = :: js_sys :: Object , js_name = SVGPathSegCurvetoCubicSmoothAbs , typescript_name = SVGPathSegCurvetoCubicSmoothAbs ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SvgPathSegCurvetoCubicSmoothAbs` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegCurvetoCubicSmoothAbs)\n\n*This API requires the following crate features to be activated: `SvgPathSegCurvetoCubicSmoothAbs`*"]
    pub type SvgPathSegCurvetoCubicSmoothAbs;
    # [ wasm_bindgen ( structural , method , getter , js_name = x ) ]
    #[doc = "Getter for the `x` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegCurvetoCubicSmoothAbs/x)\n\n*This API requires the following crate features to be activated: `SvgPathSegCurvetoCubicSmoothAbs`*"]
    pub fn x(this: &SvgPathSegCurvetoCubicSmoothAbs) -> f32;
    # [ wasm_bindgen ( structural , method , setter , js_name = x ) ]
    #[doc = "Setter for the `x` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegCurvetoCubicSmoothAbs/x)\n\n*This API requires the following crate features to be activated: `SvgPathSegCurvetoCubicSmoothAbs`*"]
    pub fn set_x(this: &SvgPathSegCurvetoCubicSmoothAbs, value: f32);
    # [ wasm_bindgen ( structural , method , getter , js_name = y ) ]
    #[doc = "Getter for the `y` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegCurvetoCubicSmoothAbs/y)\n\n*This API requires the following crate features to be activated: `SvgPathSegCurvetoCubicSmoothAbs`*"]
    pub fn y(this: &SvgPathSegCurvetoCubicSmoothAbs) -> f32;
    # [ wasm_bindgen ( structural , method , setter , js_name = y ) ]
    #[doc = "Setter for the `y` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegCurvetoCubicSmoothAbs/y)\n\n*This API requires the following crate features to be activated: `SvgPathSegCurvetoCubicSmoothAbs`*"]
    pub fn set_y(this: &SvgPathSegCurvetoCubicSmoothAbs, value: f32);
    # [ wasm_bindgen ( structural , method , getter , js_name = x2 ) ]
    #[doc = "Getter for the `x2` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegCurvetoCubicSmoothAbs/x2)\n\n*This API requires the following crate features to be activated: `SvgPathSegCurvetoCubicSmoothAbs`*"]
    pub fn x2(this: &SvgPathSegCurvetoCubicSmoothAbs) -> f32;
    # [ wasm_bindgen ( structural , method , setter , js_name = x2 ) ]
    #[doc = "Setter for the `x2` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegCurvetoCubicSmoothAbs/x2)\n\n*This API requires the following crate features to be activated: `SvgPathSegCurvetoCubicSmoothAbs`*"]
    pub fn set_x2(this: &SvgPathSegCurvetoCubicSmoothAbs, value: f32);
    # [ wasm_bindgen ( structural , method , getter , js_name = y2 ) ]
    #[doc = "Getter for the `y2` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegCurvetoCubicSmoothAbs/y2)\n\n*This API requires the following crate features to be activated: `SvgPathSegCurvetoCubicSmoothAbs`*"]
    pub fn y2(this: &SvgPathSegCurvetoCubicSmoothAbs) -> f32;
    # [ wasm_bindgen ( structural , method , setter , js_name = y2 ) ]
    #[doc = "Setter for the `y2` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegCurvetoCubicSmoothAbs/y2)\n\n*This API requires the following crate features to be activated: `SvgPathSegCurvetoCubicSmoothAbs`*"]
    pub fn set_y2(this: &SvgPathSegCurvetoCubicSmoothAbs, value: f32);
}
