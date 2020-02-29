use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( is_type_of = | _ | false , extends = SvgPathSeg , extends = :: js_sys :: Object , js_name = SVGPathSegMovetoAbs , typescript_name = SVGPathSegMovetoAbs ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `SvgPathSegMovetoAbs` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegMovetoAbs)
    ///
    ///*This API requires the following crate features to be activated: `SvgPathSegMovetoAbs`*
    pub type SvgPathSegMovetoAbs;

    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGPathSegMovetoAbs" , js_name = x ) ]
    ///Getter for the `x` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegMovetoAbs/x)
    ///
    ///*This API requires the following crate features to be activated: `SvgPathSegMovetoAbs`*
    pub fn x(this: &SvgPathSegMovetoAbs) -> f32;

    # [ wasm_bindgen ( structural , method , setter , js_class = "SVGPathSegMovetoAbs" , js_name = x ) ]
    ///Setter for the `x` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegMovetoAbs/x)
    ///
    ///*This API requires the following crate features to be activated: `SvgPathSegMovetoAbs`*
    pub fn set_x(this: &SvgPathSegMovetoAbs, value: f32);

    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGPathSegMovetoAbs" , js_name = y ) ]
    ///Getter for the `y` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegMovetoAbs/y)
    ///
    ///*This API requires the following crate features to be activated: `SvgPathSegMovetoAbs`*
    pub fn y(this: &SvgPathSegMovetoAbs) -> f32;

    # [ wasm_bindgen ( structural , method , setter , js_class = "SVGPathSegMovetoAbs" , js_name = y ) ]
    ///Setter for the `y` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegMovetoAbs/y)
    ///
    ///*This API requires the following crate features to be activated: `SvgPathSegMovetoAbs`*
    pub fn set_y(this: &SvgPathSegMovetoAbs, value: f32);

}
