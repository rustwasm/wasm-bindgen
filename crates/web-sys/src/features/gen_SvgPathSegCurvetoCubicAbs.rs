use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( is_type_of = | _ | false , extends = SvgPathSeg , extends = :: js_sys :: Object , js_name = SVGPathSegCurvetoCubicAbs , typescript_name = SVGPathSegCurvetoCubicAbs ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SvgPathSegCurvetoCubicAbs` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegCurvetoCubicAbs)\n\n*This API requires the following crate features to be activated: `SvgPathSegCurvetoCubicAbs`*"]
    pub type SvgPathSegCurvetoCubicAbs;
    # [ wasm_bindgen ( structural , method , getter , js_name = x ) ]
    #[doc = "Getter for the `x` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegCurvetoCubicAbs/x)\n\n*This API requires the following crate features to be activated: `SvgPathSegCurvetoCubicAbs`*"]
    pub fn x(this: &SvgPathSegCurvetoCubicAbs) -> f32;
    # [ wasm_bindgen ( structural , method , setter , js_name = x ) ]
    #[doc = "Setter for the `x` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegCurvetoCubicAbs/x)\n\n*This API requires the following crate features to be activated: `SvgPathSegCurvetoCubicAbs`*"]
    pub fn set_x(this: &SvgPathSegCurvetoCubicAbs, value: f32);
    # [ wasm_bindgen ( structural , method , getter , js_name = y ) ]
    #[doc = "Getter for the `y` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegCurvetoCubicAbs/y)\n\n*This API requires the following crate features to be activated: `SvgPathSegCurvetoCubicAbs`*"]
    pub fn y(this: &SvgPathSegCurvetoCubicAbs) -> f32;
    # [ wasm_bindgen ( structural , method , setter , js_name = y ) ]
    #[doc = "Setter for the `y` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegCurvetoCubicAbs/y)\n\n*This API requires the following crate features to be activated: `SvgPathSegCurvetoCubicAbs`*"]
    pub fn set_y(this: &SvgPathSegCurvetoCubicAbs, value: f32);
    # [ wasm_bindgen ( structural , method , getter , js_name = x1 ) ]
    #[doc = "Getter for the `x1` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegCurvetoCubicAbs/x1)\n\n*This API requires the following crate features to be activated: `SvgPathSegCurvetoCubicAbs`*"]
    pub fn x1(this: &SvgPathSegCurvetoCubicAbs) -> f32;
    # [ wasm_bindgen ( structural , method , setter , js_name = x1 ) ]
    #[doc = "Setter for the `x1` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegCurvetoCubicAbs/x1)\n\n*This API requires the following crate features to be activated: `SvgPathSegCurvetoCubicAbs`*"]
    pub fn set_x1(this: &SvgPathSegCurvetoCubicAbs, value: f32);
    # [ wasm_bindgen ( structural , method , getter , js_name = y1 ) ]
    #[doc = "Getter for the `y1` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegCurvetoCubicAbs/y1)\n\n*This API requires the following crate features to be activated: `SvgPathSegCurvetoCubicAbs`*"]
    pub fn y1(this: &SvgPathSegCurvetoCubicAbs) -> f32;
    # [ wasm_bindgen ( structural , method , setter , js_name = y1 ) ]
    #[doc = "Setter for the `y1` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegCurvetoCubicAbs/y1)\n\n*This API requires the following crate features to be activated: `SvgPathSegCurvetoCubicAbs`*"]
    pub fn set_y1(this: &SvgPathSegCurvetoCubicAbs, value: f32);
    # [ wasm_bindgen ( structural , method , getter , js_name = x2 ) ]
    #[doc = "Getter for the `x2` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegCurvetoCubicAbs/x2)\n\n*This API requires the following crate features to be activated: `SvgPathSegCurvetoCubicAbs`*"]
    pub fn x2(this: &SvgPathSegCurvetoCubicAbs) -> f32;
    # [ wasm_bindgen ( structural , method , setter , js_name = x2 ) ]
    #[doc = "Setter for the `x2` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegCurvetoCubicAbs/x2)\n\n*This API requires the following crate features to be activated: `SvgPathSegCurvetoCubicAbs`*"]
    pub fn set_x2(this: &SvgPathSegCurvetoCubicAbs, value: f32);
    # [ wasm_bindgen ( structural , method , getter , js_name = y2 ) ]
    #[doc = "Getter for the `y2` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegCurvetoCubicAbs/y2)\n\n*This API requires the following crate features to be activated: `SvgPathSegCurvetoCubicAbs`*"]
    pub fn y2(this: &SvgPathSegCurvetoCubicAbs) -> f32;
    # [ wasm_bindgen ( structural , method , setter , js_name = y2 ) ]
    #[doc = "Setter for the `y2` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegCurvetoCubicAbs/y2)\n\n*This API requires the following crate features to be activated: `SvgPathSegCurvetoCubicAbs`*"]
    pub fn set_y2(this: &SvgPathSegCurvetoCubicAbs, value: f32);
}
